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
use amigaos4_sys::*;
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { amiga_debug_str(b"PANIC!\n\0".as_ptr()); }
    loop { core::hint::spin_loop(); }
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    unsafe {
        amiga_debug_str(b"Hello from Rust on AmigaOS 4!\n\0".as_ptr());
    }

    // Vec, String, format! all work via Clib4Allocator (malloc/free)
    let v = alloc::vec![1u32, 2, 3];
    unsafe {
        amiga_debug_fmt_u32(b"Vec has %lu elements\n\0".as_ptr(), v.len() as u32);
    }

    0
}
