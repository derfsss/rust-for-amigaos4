//
// Network Demo — AmigaOS 4 + Rust + clib4
//
// Demonstrates the amigaos4::net, amigaos4::dns, and amigaos4::http modules.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::net::{TcpStream, TcpListener, SocketAddr};
use amigaos4::dns;
use amigaos4::http;
use amigaos4::io::{Read, Write};

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("=== Network Demo ===");

    // --- DNS Resolution ---
    amigaos4::serial_println!("\n--- DNS Resolution ---");
    match dns::resolve(b"example.com\0") {
        Ok(addrs) => {
            amigaos4::serial_println!("Resolved example.com to {} address(es):", addrs.len());
            for addr in &addrs {
                amigaos4::serial_println!("  {}.{}.{}.{}", addr[0], addr[1], addr[2], addr[3]);
            }
        }
        Err(e) => {
            amigaos4::serial_println!("DNS resolution failed: {}", e);
            amigaos4::serial_println!("(This is expected if no network is available)");
        }
    }

    // --- TCP Connection ---
    amigaos4::serial_println!("\n--- TCP Connection ---");
    // Try connecting to a well-known address
    let addr = SocketAddr::new([93, 184, 216, 34], 80); // example.com
    amigaos4::serial_println!("Connecting to {}.{}.{}.{}:{}...",
        addr.ip[0], addr.ip[1], addr.ip[2], addr.ip[3], addr.port);

    match TcpStream::connect(&addr) {
        Ok(mut stream) => {
            amigaos4::serial_println!("Connected!");

            // Send a simple HTTP request
            let request = b"GET / HTTP/1.1\r\nHost: example.com\r\nConnection: close\r\n\r\n";
            match stream.write_all(request) {
                Ok(()) => amigaos4::serial_println!("Request sent."),
                Err(e) => {
                    amigaos4::serial_println!("Failed to send: {}", e);
                    return 1;
                }
            }

            // Read first chunk of response
            let mut buf = [0u8; 512];
            match stream.read(&mut buf) {
                Ok(n) => {
                    amigaos4::serial_println!("Received {} bytes:", n);
                    if let Ok(text) = core::str::from_utf8(&buf[..n.min(256)]) {
                        amigaos4::serial_println!("{}", text);
                    }
                }
                Err(e) => amigaos4::serial_println!("Read failed: {}", e),
            }

            // Stream is closed on drop
        }
        Err(e) => {
            amigaos4::serial_println!("Connection failed: {}", e);
            amigaos4::serial_println!("(This is expected if no network is available)");
        }
    }

    // --- HTTP GET ---
    amigaos4::serial_println!("\n--- HTTP GET ---");
    match http::get(b"example.com\0", 80, b"/\0") {
        Ok(response) => {
            amigaos4::serial_println!("HTTP Status: {}", response.status_code);
            amigaos4::serial_println!("Body length: {} bytes", response.body.len());
            // Print first 256 bytes of body
            let preview_len = response.body.len().min(256);
            if let Ok(text) = core::str::from_utf8(&response.body[..preview_len]) {
                amigaos4::serial_println!("Body preview:\n{}", text);
            }
        }
        Err(e) => {
            amigaos4::serial_println!("HTTP GET failed: {}", e);
            amigaos4::serial_println!("(This is expected if no network is available)");
        }
    }

    amigaos4::serial_println!("\n=== Network Demo complete ===");
    0
}
