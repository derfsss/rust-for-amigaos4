//
// AmigaOS 4 Driver/Handler Template — Rust (no clib4)
//
// Built with -nostartfiles -nodefaultlibs -lgcc (no CRT).
// Entry point is _start() in driver_glue.c → rust_handler_main().
// Global allocator backed by IExec->AllocVecTagList/FreeVec.
//

#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::panic::PanicInfo;

use amigaos4_sys::*;
use amigaos4_alloc::ExecAllocator;

#[global_allocator]
static ALLOCATOR: ExecAllocator = ExecAllocator;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop { core::hint::spin_loop(); }
}

/// Called from driver_glue.c after IExec is initialised.
#[no_mangle]
pub extern "C" fn rust_handler_main(_sysbase: *mut u8) -> i32 {
    unsafe {
        amiga_debug_str(b"Hello from Rust driver!\n\0".as_ptr());

        // Vec and String work — backed by IExec->AllocVecTagList
        let mut v: Vec<u32> = Vec::new();
        v.push(42);
        v.push(99);

        let s = String::from("Rust on AmigaOS");
        amiga_debug_fmt_u32(b"Vec length: %lu\n\0".as_ptr(), v.len() as u32);
        amiga_debug_fmt_u32(b"String length: %lu\n\0".as_ptr(), s.len() as u32);
    }
    0
}
