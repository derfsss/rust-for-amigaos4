//! HTTPS client over the clib4-built OpenSSL (application mode only).
//!
//! [`get`] mirrors [`http::get`](crate::http::get) for `https://`
//! endpoints; [`TlsStream`] is the underlying TLS connection (with
//! [`Read`]/[`Write`] impls) for manual protocols.
//!
//! # Linking
//!
//! Requires the `tls` feature **and** OpenSSL on the link line — add
//! to the project Makefile:
//!
//! ```make
//! LIBS = -lssl -lcrypto -lz -lpthread -lauto
//! ```
//!
//! The Docker SDK ships OpenSSL built against clib4
//! (`SDK/local/clib4/lib`), so `SSL_set_fd` works directly on the
//! clib4 socket fd from [`TcpStream`] — no extra glue.
//!
//! # Certificate verification
//!
//! AmigaOS has no system CA store, so verification is **off** by
//! default (`SSL_VERIFY_NONE`) — the connection is encrypted but the
//! peer is not authenticated. Ship a CA bundle and verify with
//! [`TlsConfig::ca_file`] where authentication matters.
//!
//! # Example
//!
//! ```ignore
//! let resp = amigaos4::https::get(amstr!("example.com"), 443, amstr!("/"))?;
//! if resp.status_code == 200 { /* use resp.body */ }
//! ```

use alloc::vec::Vec;
use core::ffi::c_void;
use crate::cstr::require_nul;
use crate::dns;
use crate::error::{AmigaError, Result};
use crate::http::HttpResponse;
use crate::io::{Read, Write};
use crate::net::{SocketAddr, TcpStream};
use crate::parse::{self, UrlScheme};

/// Maximum number of redirect hops [`get`] follows before giving up.
pub const MAX_REDIRECTS: usize = 5;

// ---------------------------------------------------------------------------
// OpenSSL 1.1 FFI (libssl/libcrypto built against clib4).
// `long` is 32-bit on PPC32, so SSL_ctrl maps to i32.
// ---------------------------------------------------------------------------

#[allow(non_camel_case_types)]
type SSL = c_void;
#[allow(non_camel_case_types)]
type SSL_CTX = c_void;

extern "C" {
    fn TLS_client_method() -> *const c_void;
    fn SSL_CTX_new(method: *const c_void) -> *mut SSL_CTX;
    fn SSL_CTX_free(ctx: *mut SSL_CTX);
    fn SSL_CTX_set_verify(ctx: *mut SSL_CTX, mode: i32, callback: *const c_void);
    fn SSL_CTX_load_verify_locations(
        ctx: *mut SSL_CTX,
        ca_file: *const u8,
        ca_path: *const u8,
    ) -> i32;
    fn SSL_new(ctx: *mut SSL_CTX) -> *mut SSL;
    fn SSL_free(ssl: *mut SSL);
    fn SSL_set_fd(ssl: *mut SSL, fd: i32) -> i32;
    fn SSL_ctrl(ssl: *mut SSL, cmd: i32, larg: i32, parg: *mut c_void) -> i32;
    fn SSL_connect(ssl: *mut SSL) -> i32;
    fn SSL_read(ssl: *mut SSL, buf: *mut u8, num: i32) -> i32;
    fn SSL_write(ssl: *mut SSL, buf: *const u8, num: i32) -> i32;
    fn SSL_shutdown(ssl: *mut SSL) -> i32;
    fn SSL_get_error(ssl: *const SSL, ret: i32) -> i32;
}

/// `SSL_set_tlsext_host_name` is a macro over SSL_ctrl in the headers.
const SSL_CTRL_SET_TLSEXT_HOSTNAME: i32 = 55;
const TLSEXT_NAMETYPE_HOST_NAME: i32 = 0;

const SSL_VERIFY_NONE: i32 = 0;
const SSL_VERIFY_PEER: i32 = 1;

/// `SSL_ERROR_ZERO_RETURN` — clean TLS close.
const SSL_ERROR_ZERO_RETURN: i32 = 6;

// ---------------------------------------------------------------------------
// TlsConfig
// ---------------------------------------------------------------------------

/// TLS settings for [`TlsStream::connect_with`].
pub struct TlsConfig {
    ca_file: Option<Vec<u8>>,
}

impl TlsConfig {
    /// Default config: encrypted, peer **not** authenticated
    /// (`SSL_VERIFY_NONE` — see the module docs).
    pub fn new() -> Self {
        Self { ca_file: None }
    }

    /// Verify the peer against a PEM CA bundle at `path`
    /// (null-terminated, e.g. `amstr!("PROGDIR:cacert.pem")`).
    /// Enables `SSL_VERIFY_PEER`; the handshake fails on an
    /// untrusted certificate.
    pub fn ca_file(mut self, path: &[u8]) -> Result<Self> {
        require_nul(path)?;
        self.ca_file = Some(path.to_vec());
        Ok(self)
    }
}

impl Default for TlsConfig {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// TlsStream
// ---------------------------------------------------------------------------

/// A TLS connection over a [`TcpStream`]. Implements [`Read`]/[`Write`];
/// shuts down and frees the OpenSSL state on drop, then closes the
/// socket.
pub struct TlsStream {
    ssl: *mut SSL,
    ctx: *mut SSL_CTX,
    /// Owns the fd; dropped (closing the socket) after Drop::drop has
    /// freed the SSL state above it.
    _stream: TcpStream,
}

impl TlsStream {
    /// Resolve `host` (null-terminated), connect TCP to `port`, and
    /// perform a TLS handshake with SNI set to `host`.
    pub fn connect(host: &[u8], port: u16) -> Result<Self> {
        Self::connect_with(host, port, &TlsConfig::new())
    }

    /// Like [`connect`](Self::connect), with explicit [`TlsConfig`].
    pub fn connect_with(host: &[u8], port: u16, config: &TlsConfig) -> Result<Self> {
        let host_ptr = require_nul(host)?;

        // TCP first.
        let addrs = dns::resolve(host)?;
        let stream = TcpStream::connect(&SocketAddr::new(addrs[0], port))?;

        // SAFETY: OpenSSL 1.1 auto-initialises; null returns checked.
        unsafe {
            let ctx = SSL_CTX_new(TLS_client_method());
            if ctx.is_null() {
                return Err(AmigaError::AllocationFailed);
            }

            if let Some(ref ca) = config.ca_file {
                if SSL_CTX_load_verify_locations(ctx, ca.as_ptr(), core::ptr::null()) != 1 {
                    SSL_CTX_free(ctx);
                    return Err(AmigaError::IoError(0));
                }
                SSL_CTX_set_verify(ctx, SSL_VERIFY_PEER, core::ptr::null());
            } else {
                SSL_CTX_set_verify(ctx, SSL_VERIFY_NONE, core::ptr::null());
            }

            let ssl = SSL_new(ctx);
            if ssl.is_null() {
                SSL_CTX_free(ctx);
                return Err(AmigaError::AllocationFailed);
            }

            // SNI — required by most modern servers. The host buffer is
            // only read during the handshake below.
            SSL_ctrl(
                ssl,
                SSL_CTRL_SET_TLSEXT_HOSTNAME,
                TLSEXT_NAMETYPE_HOST_NAME,
                host_ptr as *mut c_void,
            );

            if SSL_set_fd(ssl, stream.as_raw_fd()) != 1 {
                SSL_free(ssl);
                SSL_CTX_free(ctx);
                return Err(AmigaError::IoError(0));
            }

            let rc = SSL_connect(ssl);
            if rc != 1 {
                let err = SSL_get_error(ssl, rc);
                SSL_free(ssl);
                SSL_CTX_free(ctx);
                return Err(AmigaError::IoError(err));
            }

            Ok(Self { ssl, ctx, _stream: stream })
        }
    }
}

impl Read for TlsStream {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len().min(i32::MAX as usize) as i32;
        // SAFETY: ssl is a live connection; buf bounds are passed.
        let n = unsafe { SSL_read(self.ssl, buf.as_mut_ptr(), len) };
        if n > 0 {
            return Ok(n as usize);
        }
        // SAFETY: querying the error state for the failed call.
        let err = unsafe { SSL_get_error(self.ssl, n) };
        if err == SSL_ERROR_ZERO_RETURN {
            Ok(0) // clean TLS close == EOF
        } else {
            Err(AmigaError::IoError(err))
        }
    }
}

impl Write for TlsStream {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let len = buf.len().min(i32::MAX as usize) as i32;
        // SAFETY: ssl is a live connection; buf bounds are passed.
        let n = unsafe { SSL_write(self.ssl, buf.as_ptr(), len) };
        if n > 0 {
            Ok(n as usize)
        } else {
            // SAFETY: querying the error state for the failed call.
            Err(AmigaError::IoError(unsafe { SSL_get_error(self.ssl, n) }))
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Drop for TlsStream {
    fn drop(&mut self) {
        // SAFETY: ssl/ctx are live and freed exactly once; the socket
        // (self._stream) closes afterwards in its own Drop.
        unsafe {
            SSL_shutdown(self.ssl);
            SSL_free(self.ssl);
            SSL_CTX_free(self.ctx);
        }
    }
}

// ---------------------------------------------------------------------------
// get
// ---------------------------------------------------------------------------

/// Perform an HTTPS GET request, following `https://` redirects.
///
/// Mirrors [`http::get`](crate::http::get): `host` must be
/// null-terminated (use [`amstr!`](crate::amstr)); `path` may or may
/// not be. Redirects to plain `http://` are **not** followed (no
/// silent downgrade) — the redirect response is returned as-is.
pub fn get(host: &[u8], port: u16, path: &[u8]) -> Result<HttpResponse> {
    let mut host_buf = nul_terminated(host);
    let mut port = port;
    let mut path_buf: Vec<u8> = parse::strip_nul(path).to_vec();

    for _hop in 0..=MAX_REDIRECTS {
        let raw = fetch_raw(&host_buf, port, &path_buf)?;
        let status = parse::parse_status_code(&raw);

        if parse::is_redirect(status) {
            if let Some(location) = parse::find_header(&raw, b"Location") {
                if let Some(url) = parse::parse_url(location) {
                    if url.scheme == UrlScheme::Https {
                        host_buf = nul_terminated(url.host);
                        port = url.port;
                        path_buf = url.path.to_vec();
                        continue;
                    }
                    // http:// target — refuse the downgrade, return as-is.
                } else if location.first() == Some(&b'/') {
                    path_buf = location.to_vec();
                    continue;
                }
            }
        }

        let body_start = parse::find_body_start(&raw);
        let body_slice: &[u8] = if body_start < raw.len() {
            &raw[body_start..]
        } else {
            &[]
        };
        let body = if parse::is_chunked(&raw) {
            parse::decode_chunked(body_slice)?
        } else {
            body_slice.to_vec()
        };

        return Ok(HttpResponse { status_code: status, body });
    }

    Err(AmigaError::IoError(0))
}

/// Copy `s` into an owned vector with exactly one trailing `\0`.
fn nul_terminated(s: &[u8]) -> Vec<u8> {
    let mut v = parse::strip_nul(s).to_vec();
    v.push(0);
    v
}

/// One GET over a fresh TLS connection; reads until close.
fn fetch_raw(host_nul: &[u8], port: u16, path: &[u8]) -> Result<Vec<u8>> {
    let mut tls = TlsStream::connect(host_nul, port)?;

    let host_bytes = parse::strip_nul(host_nul);
    let mut request = Vec::new();
    request.extend_from_slice(b"GET ");
    request.extend_from_slice(path);
    request.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    request.extend_from_slice(host_bytes);
    request.extend_from_slice(b"\r\nConnection: close\r\n\r\n");
    tls.write_all(&request)?;

    let mut response = Vec::new();
    let mut chunk = [0u8; 1024];
    loop {
        let n = tls.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        response.extend_from_slice(&chunk[..n]);
    }
    Ok(response)
}
