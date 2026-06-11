//! Minimal HTTP/1.1 client (application mode only).
//!
//! Provides [`get`] for simple HTTP GET requests over plain TCP. This is
//! intentionally barebones — no chunked transfer decoding, no redirects,
//! no TLS. It is useful for fetching small payloads from local or
//! Aminet-style servers.
//!
//! Requires the `net` feature.

use alloc::vec::Vec;
use crate::error::Result;
use crate::net::{TcpStream, SocketAddr};
use crate::dns;
use crate::io::{Read, Write};

/// An HTTP response with status code and raw body.
pub struct HttpResponse {
    /// HTTP status code (e.g. 200, 404).
    pub status_code: u16,
    /// Response body bytes (everything after the `\r\n\r\n` header separator).
    pub body: Vec<u8>,
}

/// Perform an HTTP GET request.
///
/// `host` and `path` must be null-terminated byte slices. `port` is typically
/// 80 for plain HTTP.
///
/// # Example
///
/// ```ignore
/// let resp = amigaos4::http::get(b"example.com\0", 80, b"/index.html\0")?;
/// if resp.status_code == 200 {
///     // use resp.body
/// }
/// ```
///
/// # Errors
///
/// Returns errors from DNS resolution, TCP connection, or I/O operations.
pub fn get(host: &[u8], port: u16, path: &[u8]) -> Result<HttpResponse> {
    // Resolve hostname to an IPv4 address.
    let addrs = dns::resolve(host)?;
    let addr = SocketAddr::new(addrs[0], port);

    // Connect.
    let mut stream = TcpStream::connect(&addr)?;

    // Strip null terminators for inclusion in the HTTP request text.
    let host_bytes = strip_nul(host);
    let path_bytes = strip_nul(path);

    // Build request: "GET /path HTTP/1.1\r\nHost: hostname\r\nConnection: close\r\n\r\n"
    let mut request = Vec::new();
    request.extend_from_slice(b"GET ");
    request.extend_from_slice(path_bytes);
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

    // Parse status code from the first line ("HTTP/1.x NNN ...").
    let status_code = parse_status_code(&response_buf);

    // Locate the body (everything after the first "\r\n\r\n").
    let body_start = find_body_start(&response_buf);
    let body = if body_start < response_buf.len() {
        response_buf[body_start..].to_vec()
    } else {
        Vec::new()
    };

    Ok(HttpResponse { status_code, body })
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn strip_nul(data: &[u8]) -> &[u8] {
    if data.last() == Some(&0) {
        &data[..data.len() - 1]
    } else {
        data
    }
}

/// Extract the three-digit status code starting at byte 9 of the response.
fn parse_status_code(data: &[u8]) -> u16 {
    // "HTTP/1.x NNN" — the status code occupies bytes 9..12.
    if data.len() < 12 {
        return 0;
    }
    let code_bytes = &data[9..12];
    let mut code: u16 = 0;
    for &b in code_bytes {
        if b >= b'0' && b <= b'9' {
            code = code * 10 + (b - b'0') as u16;
        } else {
            // Status codes are exactly three digits; a partial parse
            // (e.g. "2X4") would otherwise return a bogus small value.
            return 0;
        }
    }
    code
}

/// Return the byte offset where the body begins (after `\r\n\r\n`).
fn find_body_start(data: &[u8]) -> usize {
    let mut i = 0;
    let end = data.len().saturating_sub(3);
    while i < end {
        if data[i] == b'\r'
            && data[i + 1] == b'\n'
            && data[i + 2] == b'\r'
            && data[i + 3] == b'\n'
        {
            return i + 4;
        }
        i += 1;
    }
    data.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_code_200() {
        let data = b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n";
        assert_eq!(parse_status_code(data), 200);
    }

    #[test]
    fn status_code_404() {
        let data = b"HTTP/1.1 404 Not Found\r\n\r\n";
        assert_eq!(parse_status_code(data), 404);
    }

    #[test]
    fn status_code_short() {
        assert_eq!(parse_status_code(b"HTTP"), 0);
    }

    #[test]
    fn body_start_normal() {
        let data = b"HTTP/1.1 200 OK\r\nHost: x\r\n\r\nHello";
        let start = find_body_start(data);
        assert_eq!(&data[start..], b"Hello");
    }

    #[test]
    fn body_start_no_body() {
        let data = b"HTTP/1.1 200 OK\r\n";
        assert_eq!(find_body_start(data), data.len());
    }

    #[test]
    fn body_start_empty() {
        assert_eq!(find_body_start(b""), 0);
    }

    #[test]
    fn strip_nul_present() {
        assert_eq!(strip_nul(b"hello\0"), b"hello");
    }

    #[test]
    fn strip_nul_absent() {
        assert_eq!(strip_nul(b"hello"), b"hello");
    }

    #[test]
    fn status_code_301() {
        let data = b"HTTP/1.1 301 Moved Permanently\r\n\r\n";
        assert_eq!(parse_status_code(data), 301);
    }

    #[test]
    fn status_code_500() {
        let data = b"HTTP/1.1 500 Internal Server Error\r\n\r\n";
        assert_eq!(parse_status_code(data), 500);
    }

    #[test]
    fn status_code_http10() {
        let data = b"HTTP/1.0 200 OK\r\n\r\n";
        assert_eq!(parse_status_code(data), 200);
    }

    #[test]
    fn status_code_malformed() {
        // Non-digit at position 9
        let data = b"HTTP/1.1 X00 Bad\r\n\r\n";
        assert_eq!(parse_status_code(data), 0);
    }

    #[test]
    fn status_code_partial_digits() {
        // Non-digit in the middle of the code must not yield a partial parse.
        let data = b"HTTP/1.1 2X4 Bad\r\n\r\n";
        assert_eq!(parse_status_code(data), 0);
        let data = b"HTTP/1.1 20X Bad\r\n\r\n";
        assert_eq!(parse_status_code(data), 0);
    }

    #[test]
    fn body_start_double_newline_only() {
        let data = b"\r\n\r\n";
        assert_eq!(find_body_start(data), 4);
    }

    #[test]
    fn body_start_with_headers() {
        let data = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nServer: test\r\n\r\nbody here";
        let start = find_body_start(data);
        assert_eq!(&data[start..], b"body here");
    }

    #[test]
    fn strip_nul_empty() {
        assert_eq!(strip_nul(b""), b"");
    }

    #[test]
    fn strip_nul_only_nul() {
        assert_eq!(strip_nul(b"\0"), b"");
    }
}
