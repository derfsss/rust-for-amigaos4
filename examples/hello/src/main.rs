//
// AmigaOS 4 Application Template — Rust + clib4
//
// Linked with clib4 (-mcrt=clib4) and -lauto (auto-open libraries).
// clib4 CRT calls main() after startup initialisation.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("Hello from Rust on AmigaOS 4!");

    let v = alloc::vec![1u32, 2, 3];
    amigaos4::serial_println!("Vec has {} elements", v.len());

    0
}
