//
// hello.library — Example AmigaOS 4 shared library in Rust
//
// Demonstrates the shared library pattern:
//   - library_glue.c provides the Resident struct + interface vector table
//   - This file provides the actual library logic as #[no_mangle] extern "C" functions
//   - The Makefile links them into a .library file
//
// To add a new function:
//   1. Write it here as `#[no_mangle] pub extern "C" fn rust_lib_xxx(...)`
//   2. Declare it in library_glue.c: `extern uint32 rust_lib_xxx(...);`
//   3. Add `(void *)rust_lib_xxx` to mainVectors[] in library_glue.c
//

#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use core::panic::PanicInfo;

use amigaos4_alloc::ExecAllocator;

#[global_allocator]
static ALLOCATOR: ExecAllocator = ExecAllocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// Library lifecycle
// ---------------------------------------------------------------------------

/// Called once when the library is first loaded by AmigaOS.
/// Return 0 for success, non-zero to abort loading.
#[no_mangle]
pub extern "C" fn rust_lib_init() -> i32 {
    amigaos4::serial_println!("hello.library: initialized");
    0
}

/// Called each time a process opens the library via OpenLibrary().
/// Return 0 for success.
#[no_mangle]
pub extern "C" fn rust_lib_open() -> i32 {
    amigaos4::serial_println!("hello.library: opened by process");
    0
}

/// Called each time a process closes the library via CloseLibrary().
#[no_mangle]
pub extern "C" fn rust_lib_close() {
    amigaos4::serial_println!("hello.library: closed");
}

// ---------------------------------------------------------------------------
// Public library API
// ---------------------------------------------------------------------------

/// Add two unsigned integers. (Interface index 4)
#[no_mangle]
pub extern "C" fn rust_lib_add(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

/// Multiply two unsigned integers. (Interface index 5)
#[no_mangle]
pub extern "C" fn rust_lib_multiply(a: u32, b: u32) -> u32 {
    a.wrapping_mul(b)
}

/// Return the library version number. (Interface index 6)
#[no_mangle]
pub extern "C" fn rust_lib_version() -> u32 {
    1
}
