//
// AmigaOS 4 HTTP client - opens a TCP connection to a hard-coded
// IPv4 endpoint on port 80, sends an HTTP/1.1 GET, prints the first
// line of the response.
//
// Demonstrates amigaos4::net::TcpStream end-to-end with real outbound
// network traffic (no DNS — connects to 1.1.1.1:80 so the example
// runs anywhere with bsdsocket reachable to the wider internet).
//
// HTTPS is intentionally not attempted here. amigaos4-sys's AmiSSL +
// AmiSSLMaster wrappers load and round-trip SSL_CTX through the
// binding (see examples/thread-amissl-probe/), but a real TLS
// handshake over a clib4-allocated bsdsocket fd needs additional
// per-task glue we don't ship yet; that work belongs in its own
// example once the integration story is settled.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_sys::*;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::net::{SocketAddr, TcpStream};
use amigaos4::io::{Read, Write};

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])                  { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(prefix: &[u8], v: u32)   { unsafe { amiga_debug_str(prefix.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - HTTP client demo\n\0");
    say(b"========================================\n\0");

    // 1. TCP connect to 1.1.1.1:80. Cloudflare's edge serves an
    //    HTTP 301 redirect to HTTPS on port 80, so we always get a
    //    valid status line back without needing DNS or TLS.
    let addr = SocketAddr::new([1, 1, 1, 1], 80);
    let mut stream = match TcpStream::connect(&addr) {
        Ok(s) => s,
        Err(_) => { say(b"FAIL: TCP connect to 1.1.1.1:80 refused\n\0"); return 1; }
    };
    say_d(b"  TCP connected, fd                  = \0", stream.as_raw_fd() as u32);

    // 2. Send HTTP/1.1 GET. IP literal in Host: header is accepted by
    //    Cloudflare's edge for the redirect.
    let req: &[u8] = b"GET / HTTP/1.1\r\nHost: 1.1.1.1\r\nConnection: close\r\nUser-Agent: rust-for-amigaos4\r\n\r\n";
    match stream.write(req) {
        Ok(n) => say_d(b"  Sent bytes                         = \0", n as u32),
        Err(_) => { say(b"FAIL: write\n\0"); return 2; }
    }

    // 3. Read response, print the first line.
    let mut buf = [0u8; 256];
    let last = buf.len() - 1;
    let got = match stream.read(&mut buf[..last]) {
        Ok(n) => n,
        Err(_) => { say(b"FAIL: read\n\0"); return 3; }
    };
    say_d(b"  Received bytes                     = \0", got as u32);
    if got > 0 {
        let end = buf[..got].iter().position(|&b| b == b'\r' || b == b'\n').unwrap_or(got);
        buf[end] = 0;
        say(b"  Status line: \0");
        unsafe { amiga_debug_str(buf.as_ptr()); }
        say(b"\n\0");
    }

    say(b"\n  => PASS: TCP open + HTTP GET + status-line round-trip ok\n\0");
    say(b"========================================\n\0");
    0
}
