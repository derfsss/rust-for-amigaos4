//
// AmigaOS 4 Driver/Handler Template — Rust (no clib4)
//
// Built with -nostartfiles -nodefaultlibs -lgcc (no CRT).
// Entry point is _start() in driver_glue.c -> rust_handler_main().
// Global allocator backed by IExec->AllocVecTagList/FreeVec.
//

#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::panic::PanicInfo;

use amigaos4_alloc::ExecAllocator;

#[global_allocator]
static ALLOCATOR: ExecAllocator = ExecAllocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

/// Called from driver_glue.c after IExec is initialised.
#[no_mangle]
pub extern "C" fn rust_handler_main(_sysbase: *mut u8) -> i32 {
    // Note: IExec is available at this point (set up by driver_glue.c).
    amigaos4::serial_println!("Hello from Rust driver!");

    // Vec and String work — backed by IExec->AllocVecTagList
    let mut v: Vec<u32> = Vec::new();
    v.push(42);
    v.push(99);

    let s = String::from("Rust on AmigaOS");
    amigaos4::serial_println!("Vec length: {}", v.len());
    amigaos4::serial_println!("String length: {}", s.len());

    0
}
