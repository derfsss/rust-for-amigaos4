//! Pure parsing helpers shared by the network modules.
//!
//! The `net`, `dns`, and `http` modules are compiled only for the PPC
//! target (they wrap clib4 symbols that exist only at cross-link time),
//! which means `#[cfg(test)]` blocks inside them never run on the host.
//! Everything in this module is pure Rust with no FFI, compiled on every
//! target, so its tests run on every host `cargo test`.

// The consumers (net/dns/http) only compile for PPC application builds,
// so everywhere else these helpers are exercised only by the tests below.
#![cfg_attr(
    not(all(feature = "net", target_arch = "powerpc")),
    allow(dead_code)
)]

use alloc::vec::Vec;
use crate::error::{AmigaError, Result};

// ---------------------------------------------------------------------------
// Byte / number primitives
// ---------------------------------------------------------------------------

/// Find the first occurrence of `needle` in `data`.
pub(crate) fn find_byte(data: &[u8], needle: u8) -> Option<usize> {
    let mut i = 0;
    while i < data.len() {
        if data[i] == needle {
            return Some(i);
        }
        i += 1;
    }
    None
}

/// Strip one trailing `\0` if present.
pub(crate) fn strip_nul(data: &[u8]) -> &[u8] {
    if data.last() == Some(&0) {
        &data[..data.len() - 1]
    } else {
        data
    }
}

/// Parse a dotted-quad IPv4 address (`"1.2.3.4"`).
pub(crate) fn parse_ipv4(data: &[u8]) -> Result<[u8; 4]> {
    let mut octets = [0u8; 4];
    let mut octet_idx = 0usize;
    let mut acc: u16 = 0;
    let mut digits = 0u8;

    for &b in data {
        if b == b'.' {
            if digits == 0 || octet_idx >= 3 || acc > 255 {
                return Err(AmigaError::IoError(0));
            }
            octets[octet_idx] = acc as u8;
            octet_idx += 1;
            acc = 0;
            digits = 0;
        } else if b >= b'0' && b <= b'9' {
            acc = acc * 10 + (b - b'0') as u16;
            digits += 1;
            if digits > 3 || acc > 255 {
                return Err(AmigaError::IoError(0));
            }
        } else {
            return Err(AmigaError::IoError(0));
        }
    }

    // Final octet
    if digits == 0 || octet_idx != 3 || acc > 255 {
        return Err(AmigaError::IoError(0));
    }
    octets[3] = acc as u8;

    Ok(octets)
}

/// Parse a decimal number that must fit in a `u16`.
pub(crate) fn parse_u16(data: &[u8]) -> Result<u16> {
    if data.is_empty() {
        return Err(AmigaError::IoError(0));
    }
    let mut acc: u32 = 0;
    for &b in data {
        if b >= b'0' && b <= b'9' {
            acc = acc * 10 + (b - b'0') as u32;
            if acc > 65535 {
                return Err(AmigaError::IoError(0));
            }
        } else {
            return Err(AmigaError::IoError(0));
        }
    }
    Ok(acc as u16)
}

/// Parse `"1.2.3.4:80"` into IP octets and a port. A trailing `\0` on the
/// input is tolerated.
pub(crate) fn parse_socket_addr(addr: &[u8]) -> Result<([u8; 4], u16)> {
    let data = strip_nul(addr);
    let colon_pos = find_byte(data, b':').ok_or(AmigaError::IoError(0))?;
    let ip = parse_ipv4(&data[..colon_pos])?;
    let port = parse_u16(&data[colon_pos + 1..])?;
    Ok((ip, port))
}

// ---------------------------------------------------------------------------
// HTTP response parsing
// ---------------------------------------------------------------------------

/// Extract the three-digit status code starting at byte 9 of the response.
///
/// Returns 0 if the response is too short or the code is not exactly
/// three digits.
pub(crate) fn parse_status_code(data: &[u8]) -> u16 {
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
/// Returns `data.len()` if no header terminator is found.
pub(crate) fn find_body_start(data: &[u8]) -> usize {
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

/// ASCII case-insensitive equality.
fn eq_ignore_case(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len()
        && a.iter()
            .zip(b.iter())
            .all(|(x, y)| x.to_ascii_lowercase() == y.to_ascii_lowercase())
}

/// Look up a header value in the header block of `response` (everything
/// before the `\r\n\r\n`). `name` is matched case-insensitively without
/// a trailing colon. The returned value has surrounding whitespace
/// trimmed.
pub(crate) fn find_header<'a>(response: &'a [u8], name: &[u8]) -> Option<&'a [u8]> {
    let header_end = find_body_start(response);
    let headers = &response[..header_end.min(response.len())];

    // Walk line by line; skip the status line.
    let mut line_start = match find_byte(headers, b'\n') {
        Some(p) => p + 1,
        None => return None,
    };
    while line_start < headers.len() {
        let rest = &headers[line_start..];
        let line_end = find_byte(rest, b'\n').map(|p| line_start + p).unwrap_or(headers.len());
        let line = &headers[line_start..line_end];
        let line = if line.last() == Some(&b'\r') {
            &line[..line.len() - 1]
        } else {
            line
        };
        if let Some(colon) = find_byte(line, b':') {
            if eq_ignore_case(&line[..colon], name) {
                let mut value = &line[colon + 1..];
                while value.first() == Some(&b' ') || value.first() == Some(&b'\t') {
                    value = &value[1..];
                }
                while value.last() == Some(&b' ') || value.last() == Some(&b'\t') {
                    value = &value[..value.len() - 1];
                }
                return Some(value);
            }
        }
        line_start = line_end + 1;
    }
    None
}

/// True if the response declares `Transfer-Encoding: chunked`.
pub(crate) fn is_chunked(response: &[u8]) -> bool {
    match find_header(response, b"Transfer-Encoding") {
        Some(v) => eq_ignore_case(v, b"chunked"),
        None => false,
    }
}

/// True for status codes that redirect with a `Location` header.
pub(crate) fn is_redirect(status: u16) -> bool {
    matches!(status, 301 | 302 | 303 | 307 | 308)
}

/// Decode a `Transfer-Encoding: chunked` body.
///
/// Each chunk is `<hex-size>[;extensions]\r\n<data>\r\n`; a zero-size
/// chunk terminates the body (trailing headers are ignored).
pub(crate) fn decode_chunked(body: &[u8]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    let mut pos = 0usize;

    loop {
        // Parse the hex chunk-size line.
        let rest = &body[pos.min(body.len())..];
        let line_end = find_byte(rest, b'\n').ok_or(AmigaError::UnexpectedEof)?;
        let line = &rest[..line_end];
        let line = if line.last() == Some(&b'\r') {
            &line[..line.len() - 1]
        } else {
            line
        };
        // Chunk extensions (";...") are allowed after the size.
        let size_part = match find_byte(line, b';') {
            Some(p) => &line[..p],
            None => line,
        };
        if size_part.is_empty() {
            return Err(AmigaError::IoError(0));
        }
        let mut size: usize = 0;
        for &b in size_part {
            let digit = match b {
                b'0'..=b'9' => (b - b'0') as usize,
                b'a'..=b'f' => (b - b'a' + 10) as usize,
                b'A'..=b'F' => (b - b'A' + 10) as usize,
                _ => return Err(AmigaError::IoError(0)),
            };
            size = size.checked_mul(16).and_then(|s| s.checked_add(digit))
                .ok_or(AmigaError::IoError(0))?;
        }

        pos += line_end + 1;
        if size == 0 {
            // Terminating chunk; ignore any trailing headers.
            return Ok(out);
        }
        if pos + size > body.len() {
            return Err(AmigaError::UnexpectedEof);
        }
        out.extend_from_slice(&body[pos..pos + size]);
        pos += size;
        // Skip the CRLF after the chunk data.
        if body.get(pos) == Some(&b'\r') {
            pos += 1;
        }
        if body.get(pos) == Some(&b'\n') {
            pos += 1;
        } else {
            return Err(AmigaError::IoError(0));
        }
    }
}

// ---------------------------------------------------------------------------
// URL parsing (for HTTP redirects)
// ---------------------------------------------------------------------------

/// Components of a parsed `http://` URL. Byte slices borrow from the input.
pub(crate) struct UrlParts<'a> {
    pub host: &'a [u8],
    pub port: u16,
    pub path: &'a [u8],
}

/// Parse an absolute `http://host[:port][/path]` URL.
///
/// Returns `None` for any other scheme (including `https`, which this
/// client cannot speak).
pub(crate) fn parse_http_url(url: &[u8]) -> Option<UrlParts<'_>> {
    const PREFIX: &[u8] = b"http://";
    if url.len() < PREFIX.len() || !eq_ignore_case(&url[..PREFIX.len()], PREFIX) {
        return None;
    }
    let rest = &url[PREFIX.len()..];
    let (authority, path) = match find_byte(rest, b'/') {
        Some(p) => (&rest[..p], &rest[p..]),
        None => (rest, b"/".as_slice()),
    };
    if authority.is_empty() {
        return None;
    }
    let (host, port) = match find_byte(authority, b':') {
        Some(p) => {
            let port = parse_u16(&authority[p + 1..]).ok()?;
            (&authority[..p], port)
        }
        None => (authority, 80),
    };
    if host.is_empty() {
        return None;
    }
    Some(UrlParts { host, port, path })
}

// ---------------------------------------------------------------------------
// Tests (run on host — that is the point of this module)
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // ---- socket address ----

    #[test]
    fn socket_addr_basic() {
        let (ip, port) = parse_socket_addr(b"127.0.0.1:8080").unwrap();
        assert_eq!(ip, [127, 0, 0, 1]);
        assert_eq!(port, 8080);
    }

    #[test]
    fn socket_addr_nul_terminated() {
        let (ip, port) = parse_socket_addr(b"10.0.0.1:80\0").unwrap();
        assert_eq!(ip, [10, 0, 0, 1]);
        assert_eq!(port, 80);
    }

    #[test]
    fn socket_addr_max() {
        let (ip, port) = parse_socket_addr(b"255.255.255.255:65535").unwrap();
        assert_eq!(ip, [255, 255, 255, 255]);
        assert_eq!(port, 65535);
    }

    #[test]
    fn socket_addr_zero() {
        let (ip, port) = parse_socket_addr(b"0.0.0.0:0").unwrap();
        assert_eq!(ip, [0, 0, 0, 0]);
        assert_eq!(port, 0);
    }

    #[test]
    fn socket_addr_no_colon() {
        assert!(parse_socket_addr(b"127.0.0.1").is_err());
    }

    #[test]
    fn socket_addr_invalid_octet() {
        assert!(parse_socket_addr(b"256.0.0.1:80").is_err());
    }

    #[test]
    fn socket_addr_port_overflow() {
        assert!(parse_socket_addr(b"1.2.3.4:99999").is_err());
    }

    #[test]
    fn socket_addr_empty() {
        assert!(parse_socket_addr(b"").is_err());
    }

    // ---- IPv4 edge cases ----

    #[test]
    fn ipv4_one_octet() {
        assert!(parse_ipv4(b"1").is_err());
    }

    #[test]
    fn ipv4_three_octets() {
        assert!(parse_ipv4(b"1.2.3").is_err());
    }

    #[test]
    fn ipv4_five_octets() {
        assert!(parse_ipv4(b"1.2.3.4.5").is_err());
    }

    #[test]
    fn ipv4_leading_dot() {
        assert!(parse_ipv4(b".1.2.3").is_err());
    }

    #[test]
    fn ipv4_trailing_dot() {
        assert!(parse_ipv4(b"1.2.3.").is_err());
    }

    #[test]
    fn ipv4_double_dot() {
        assert!(parse_ipv4(b"1..2.3").is_err());
    }

    #[test]
    fn ipv4_non_digit() {
        assert!(parse_ipv4(b"1.2.a.3").is_err());
    }

    // ---- u16 ----

    #[test]
    fn u16_empty() {
        assert!(parse_u16(b"").is_err());
    }

    #[test]
    fn u16_zero() {
        assert_eq!(parse_u16(b"0").unwrap(), 0);
    }

    #[test]
    fn u16_max() {
        assert_eq!(parse_u16(b"65535").unwrap(), 65535);
    }

    #[test]
    fn u16_overflow() {
        assert!(parse_u16(b"65536").is_err());
    }

    #[test]
    fn u16_non_digit() {
        assert!(parse_u16(b"80ab").is_err());
    }

    // ---- strip_nul ----

    #[test]
    fn strip_nul_present() {
        assert_eq!(strip_nul(b"hello\0"), b"hello");
    }

    #[test]
    fn strip_nul_absent() {
        assert_eq!(strip_nul(b"hello"), b"hello");
    }

    #[test]
    fn strip_nul_empty() {
        assert_eq!(strip_nul(b""), b"");
    }

    #[test]
    fn strip_nul_only_nul() {
        assert_eq!(strip_nul(b"\0"), b"");
    }

    // ---- status code ----

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
    fn status_code_short() {
        assert_eq!(parse_status_code(b"HTTP"), 0);
    }

    #[test]
    fn status_code_malformed() {
        let data = b"HTTP/1.1 X00 Bad\r\n\r\n";
        assert_eq!(parse_status_code(data), 0);
    }

    #[test]
    fn status_code_partial_digits() {
        assert_eq!(parse_status_code(b"HTTP/1.1 2X4 Bad\r\n\r\n"), 0);
        assert_eq!(parse_status_code(b"HTTP/1.1 20X Bad\r\n\r\n"), 0);
    }

    // ---- body start ----

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
    fn body_start_double_newline_only() {
        assert_eq!(find_body_start(b"\r\n\r\n"), 4);
    }

    #[test]
    fn body_start_with_headers() {
        let data = b"HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nServer: test\r\n\r\nbody here";
        let start = find_body_start(data);
        assert_eq!(&data[start..], b"body here");
    }

    // ---- headers ----

    #[test]
    fn header_lookup_basic() {
        let data = b"HTTP/1.1 301 Moved\r\nLocation: http://example.com/x\r\n\r\n";
        assert_eq!(
            find_header(data, b"Location").unwrap(),
            b"http://example.com/x"
        );
    }

    #[test]
    fn header_lookup_case_insensitive() {
        let data = b"HTTP/1.1 200 OK\r\ncOnTeNt-TyPe: text/html\r\n\r\n";
        assert_eq!(find_header(data, b"Content-Type").unwrap(), b"text/html");
    }

    #[test]
    fn header_lookup_trims_whitespace() {
        let data = b"HTTP/1.1 200 OK\r\nServer:   spaced   \r\n\r\n";
        assert_eq!(find_header(data, b"Server").unwrap(), b"spaced");
    }

    #[test]
    fn header_lookup_missing() {
        let data = b"HTTP/1.1 200 OK\r\nServer: x\r\n\r\n";
        assert!(find_header(data, b"Location").is_none());
    }

    #[test]
    fn header_lookup_not_in_body() {
        // A "header" appearing in the body must not match.
        let data = b"HTTP/1.1 200 OK\r\nServer: x\r\n\r\nLocation: http://evil/\r\n";
        assert!(find_header(data, b"Location").is_none());
    }

    #[test]
    fn header_status_line_not_matched() {
        // The status line has no colon-name; must not panic or match.
        let data = b"HTTP/1.1 200 OK\r\n\r\n";
        assert!(find_header(data, b"HTTP/1.1 200 OK").is_none());
    }

    // ---- chunked ----

    #[test]
    fn chunked_detection() {
        let data = b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n";
        assert!(is_chunked(data));
        let plain = b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\n";
        assert!(!is_chunked(plain));
    }

    #[test]
    fn chunked_decode_basic() {
        let body = b"5\r\nHello\r\n7\r\n, World\r\n0\r\n\r\n";
        assert_eq!(decode_chunked(body).unwrap(), b"Hello, World");
    }

    #[test]
    fn chunked_decode_hex_sizes() {
        let body = b"A\r\n0123456789\r\n0\r\n\r\n";
        assert_eq!(decode_chunked(body).unwrap(), b"0123456789");
    }

    #[test]
    fn chunked_decode_uppercase_hex() {
        let body = b"A\r\nabcdefghij\r\n0\r\n\r\n";
        assert_eq!(decode_chunked(body).unwrap(), b"abcdefghij");
    }

    #[test]
    fn chunked_decode_with_extension() {
        let body = b"5;name=value\r\nHello\r\n0\r\n\r\n";
        assert_eq!(decode_chunked(body).unwrap(), b"Hello");
    }

    #[test]
    fn chunked_decode_empty_body() {
        let body = b"0\r\n\r\n";
        assert_eq!(decode_chunked(body).unwrap(), b"");
    }

    #[test]
    fn chunked_decode_truncated_chunk() {
        let body = b"A\r\n0123\r\n";
        assert!(decode_chunked(body).is_err());
    }

    #[test]
    fn chunked_decode_bad_size() {
        let body = b"XYZ\r\ndata\r\n0\r\n\r\n";
        assert!(decode_chunked(body).is_err());
    }

    #[test]
    fn chunked_decode_missing_terminator() {
        let body = b"5\r\nHello";
        assert!(decode_chunked(body).is_err());
    }

    // ---- redirect status ----

    #[test]
    fn redirect_statuses() {
        for s in [301u16, 302, 303, 307, 308] {
            assert!(is_redirect(s), "{} should redirect", s);
        }
        for s in [200u16, 204, 304, 400, 404, 500] {
            assert!(!is_redirect(s), "{} should not redirect", s);
        }
    }

    // ---- URL parsing ----

    #[test]
    fn url_basic() {
        let u = parse_http_url(b"http://example.com/path/x").unwrap();
        assert_eq!(u.host, b"example.com");
        assert_eq!(u.port, 80);
        assert_eq!(u.path, b"/path/x");
    }

    #[test]
    fn url_with_port() {
        let u = parse_http_url(b"http://example.com:8080/x").unwrap();
        assert_eq!(u.host, b"example.com");
        assert_eq!(u.port, 8080);
        assert_eq!(u.path, b"/x");
    }

    #[test]
    fn url_no_path() {
        let u = parse_http_url(b"http://example.com").unwrap();
        assert_eq!(u.host, b"example.com");
        assert_eq!(u.port, 80);
        assert_eq!(u.path, b"/");
    }

    #[test]
    fn url_scheme_case_insensitive() {
        assert!(parse_http_url(b"HTTP://example.com/").is_some());
    }

    #[test]
    fn url_https_rejected() {
        assert!(parse_http_url(b"https://example.com/").is_none());
    }

    #[test]
    fn url_empty_host_rejected() {
        assert!(parse_http_url(b"http:///path").is_none());
        assert!(parse_http_url(b"http://:80/path").is_none());
    }

    #[test]
    fn url_bad_port_rejected() {
        assert!(parse_http_url(b"http://example.com:99999/").is_none());
    }
}
