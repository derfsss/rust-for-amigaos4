//
// AmigaOS 4 Shared Library Template — Rust (no clib4)
//
// Built with -nostartfiles -nodefaultlibs -lgcc (no CRT).
// The library_glue.c provides the Resident structure, manager interface,
// and main interface vector table. Rust code provides the library logic.
//
// Global allocator backed by IExec->AllocVecTagList/FreeVec (ExecAllocator).
//
// To add library functions:
//   1. Add `#[no_mangle] pub extern "C" fn my_func(...)` here
//   2. Add `extern uint32 my_func(...)` declaration in library_glue.c
//   3. Add `(void *)my_func` to mainVectors[] in library_glue.c
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::ExecAllocator;

#[global_allocator]
static ALLOCATOR: ExecAllocator = ExecAllocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// Library lifecycle callbacks (called from library_glue.c)
// ---------------------------------------------------------------------------

/// Called once when the library is first loaded. Return 0 for success.
#[no_mangle]
pub extern "C" fn rust_lib_init() -> i32 {
    amigaos4::serial_println!("PROJNAME.library: init");
    0
}

/// Called each time a process opens the library. Return 0 for success.
#[no_mangle]
pub extern "C" fn rust_lib_open() -> i32 {
    amigaos4::serial_println!("PROJNAME.library: open");
    0
}

/// Called each time a process closes the library.
#[no_mangle]
pub extern "C" fn rust_lib_close() {
    amigaos4::serial_println!("PROJNAME.library: close");
}

// ---------------------------------------------------------------------------
// Library public API (listed in mainVectors[] in library_glue.c)
// ---------------------------------------------------------------------------

/// Add two numbers. Exposed as function index 4 in the main interface.
#[no_mangle]
pub extern "C" fn rust_lib_add(a: u32, b: u32) -> u32 {
    a + b
}

/// Return the library version. Exposed as function index 5 in the main interface.
#[no_mangle]
pub extern "C" fn rust_lib_version() -> u32 {
    1
}
