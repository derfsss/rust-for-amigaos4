//! Minimal HTTP/1.1 client (application mode only).
//!
//! Provides [`get`] for simple HTTP GET requests over plain TCP. The
//! client follows redirects (up to [`MAX_REDIRECTS`] hops) and decodes
//! `Transfer-Encoding: chunked` bodies. There is no TLS — an `https://`
//! redirect target cannot be followed and is returned as-is.
//!
//! All pure parsing logic lives in the host-testable `parse` module.
//!
//! Requires the `net` feature.

use alloc::vec::Vec;
use crate::error::{AmigaError, Result};
use crate::net::{TcpStream, SocketAddr};
use crate::dns;
use crate::io::{Read, Write};
use crate::parse;

/// Maximum number of redirect hops [`get`] follows before giving up.
pub const MAX_REDIRECTS: usize = 5;

/// An HTTP response with status code and raw body.
pub struct HttpResponse {
    /// HTTP status code (e.g. 200, 404).
    pub status_code: u16,
    /// Response body bytes (after the `\r\n\r\n` header separator,
    /// chunked-decoded if the server used chunked transfer encoding).
    pub body: Vec<u8>,
}

/// Perform an HTTP GET request, following redirects.
///
/// `host` must be a null-terminated byte slice (build it with
/// [`amstr!`](crate::amstr)); `path` may or may not be terminated.
/// `port` is typically 80 for plain HTTP.
///
/// Redirect responses (301/302/303/307/308) with an `http://` or
/// relative `Location` are followed, up to [`MAX_REDIRECTS`] hops. A
/// redirect this client cannot follow (e.g. to `https://`) is returned
/// to the caller unchanged, status code and all.
///
/// # Example
///
/// ```ignore
/// let resp = amigaos4::http::get(amstr!("example.com"), 80, amstr!("/index.html"))?;
/// if resp.status_code == 200 {
///     // use resp.body
/// }
/// ```
///
/// # Errors
///
/// Returns errors from DNS resolution (`NotNulTerminated`,
/// `HostNotFound`), TCP connection, or I/O operations; `IoError(0)` if
/// the redirect limit is exceeded.
pub fn get(host: &[u8], port: u16, path: &[u8]) -> Result<HttpResponse> {
    let mut host_buf = nul_terminated(host);
    let mut port = port;
    let mut path_buf: Vec<u8> = parse::strip_nul(path).to_vec();

    for _hop in 0..=MAX_REDIRECTS {
        let raw = fetch_raw(&host_buf, port, &path_buf)?;
        let status = parse::parse_status_code(&raw);

        if parse::is_redirect(status) {
            if let Some(location) = parse::find_header(&raw, b"Location") {
                if let Some(url) = parse::parse_http_url(location) {
                    // Absolute http:// URL — possibly a different host.
                    host_buf = nul_terminated(url.host);
                    port = url.port;
                    path_buf = url.path.to_vec();
                    continue;
                }
                if location.first() == Some(&b'/') {
                    // Relative path on the same host.
                    path_buf = location.to_vec();
                    continue;
                }
                // Unfollowable target (https://, protocol-relative, ...):
                // fall through and hand the redirect response to the caller.
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

    // Redirect limit exceeded.
    Err(AmigaError::IoError(0))
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Copy `s` into an owned vector with exactly one trailing `\0`.
fn nul_terminated(s: &[u8]) -> Vec<u8> {
    let mut v = parse::strip_nul(s).to_vec();
    v.push(0);
    v
}

/// Issue one GET request and read the entire raw response (headers +
/// body) until the server closes the connection.
///
/// `host_nul` must be null-terminated; `path` must not be.
fn fetch_raw(host_nul: &[u8], port: u16, path: &[u8]) -> Result<Vec<u8>> {
    // Resolve hostname to an IPv4 address.
    let addrs = dns::resolve(host_nul)?;
    let addr = SocketAddr::new(addrs[0], port);

    // Connect.
    let mut stream = TcpStream::connect(&addr)?;

    let host_bytes = parse::strip_nul(host_nul);

    // Build request: "GET /path HTTP/1.1\r\nHost: hostname\r\nConnection: close\r\n\r\n"
    let mut request = Vec::new();
    request.extend_from_slice(b"GET ");
    request.extend_from_slice(path);
    request.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    request.extend_from_slice(host_bytes);
    request.extend_from_slice(b"\r\nConnection: close\r\n\r\n");

    stream.write_all(&request)?;

    // Read entire response until the server closes the connection.
    let mut response_buf = Vec::new();
    let mut chunk = [0u8; 1024];
    loop {
        let n = stream.read(&mut chunk)?;
        if n == 0 {
            break;
        }
        response_buf.extend_from_slice(&chunk[..n]);
    }
    Ok(response_buf)
}
