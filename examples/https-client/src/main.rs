//
// AmigaOS 4 HTTPS client — a real TLS GET from Rust.
//
// Uses amigaos4::https (the `tls` feature) over the clib4-built
// OpenSSL static libs: DNS resolve, TCP connect, TLS handshake with
// SNI, HTTP/1.1 GET, redirect following, chunked decoding.
//
// Certificate verification is off (no system CA store on AmigaOS) —
// the connection is encrypted but unauthenticated; see the https
// module docs for the opt-in CA-file API.
//
// Link with: -lssl -lcrypto -lz -lpthread -lauto (see Makefile).
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4::amstr;
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("========================================");
    amigaos4::serial_println!("  Rust for AmigaOS 4 - HTTPS client");
    amigaos4::serial_println!("========================================");

    // example.com serves a stable small page over TLS 1.2/1.3.
    let resp = match amigaos4::https::get(amstr!("example.com"), 443, amstr!("/")) {
        Ok(r) => r,
        Err(e) => {
            amigaos4::serial_println!("FAIL: https::get: {}", e);
            return 1;
        }
    };

    amigaos4::serial_println!("  status = {}", resp.status_code);
    amigaos4::serial_println!("  body   = {} bytes", resp.body.len());

    if resp.status_code != 200 {
        amigaos4::serial_println!("FAIL: expected status 200");
        return 2;
    }

    // The page title is a stable content marker.
    let marker = b"Example Domain";
    let found = resp
        .body
        .windows(marker.len())
        .any(|w| w == marker);
    if !found {
        amigaos4::serial_println!("FAIL: body marker not found");
        return 3;
    }

    amigaos4::serial_println!("  => PASS: TLS GET over OpenSSL from Rust");
    amigaos4::serial_println!("========================================");
    0
}
