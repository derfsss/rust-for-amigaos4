//
// AmigaOS 4 zlib round-trip - compress a string with z.library's
// Compress(), inflate it back with Uncompress(), verify the result
// matches the original byte-for-byte.
//
// Exercises the amigaos4-sys `z` wrapper (no example used it before).
// z.library isn't in -lauto's set, so the binary opens it manually
// via exec_open_library + exec_get_interface and provides the IZ
// global itself (see src/z_glue.c).
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use alloc::vec;
use amigaos4_sys::*;
use amigaos4_sys::wrappers::z::{z_compress, z_compress_bound, z_uncompress};
use amigaos4_alloc::Clib4Allocator;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

fn say(s: &[u8])                { unsafe { amiga_debug_str(s.as_ptr()); } }
fn say_d(p: &[u8], v: u32)      { unsafe { amiga_debug_str(p.as_ptr()); amiga_debug_fmt_u32(b"%ld\n\0".as_ptr(), v); } }

extern "C" {
    static mut ZBase: *mut Library;
    static mut IZ:    *mut amigaos4_sys::interfaces::z::ZIFace;
}

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    say(b"========================================\n\0");
    say(b"  Rust for AmigaOS 4 - zlib round-trip\n\0");
    say(b"========================================\n\0");

    // Open z.library and bind its main interface.
    let base = unsafe { exec_open_library(b"z.library\0".as_ptr() as CONST_STRPTR, 53) };
    if base.is_null() {
        say(b"FAIL: z.library not available\n\0");
        return 1;
    }
    let iface = unsafe { exec_get_interface(base, b"main\0".as_ptr() as CONST_STRPTR, 1, core::ptr::null()) };
    if iface.is_null() {
        unsafe { exec_close_library(base); }
        say(b"FAIL: z.library main IFace missing\n\0");
        return 2;
    }
    unsafe {
        ZBase = base;
        IZ = iface as *mut _;
    }
    say_d(b"  z.library opened, IZ              = \0", iface as u32);

    // Build a small test payload with deliberate redundancy so the
    // compressed form is meaningfully smaller than the input.
    let src: alloc::vec::Vec<u8> = (0..512)
        .flat_map(|_| b"Rust on AmigaOS 4! ".iter().copied())
        .collect();
    let src_len = src.len() as u32;
    say_d(b"  Input bytes                       = \0", src_len);

    // 1. Compress.
    let bound = unsafe { z_compress_bound(src_len) };
    say_d(b"  CompressBound                     = \0", bound);
    let mut dest: alloc::vec::Vec<u8> = vec![0u8; bound as usize];
    let mut dest_len: u32 = bound;
    let rc = unsafe {
        z_compress(
            dest.as_mut_ptr() as APTR,
            &mut dest_len,
            src.as_ptr() as CONST_APTR,
            src_len,
        )
    };
    say_d(b"  Compress rc                       = \0", rc as u32);
    if rc != 0 {
        say(b"FAIL: Compress returned non-zero\n\0");
        unsafe { exec_drop_interface(iface); exec_close_library(base); }
        return 3;
    }
    say_d(b"  Compressed bytes                  = \0", dest_len);
    dest.truncate(dest_len as usize);

    // 2. Decompress into a separate buffer.
    let mut back: alloc::vec::Vec<u8> = vec![0u8; src.len()];
    let mut back_len: u32 = src.len() as u32;
    let rc = unsafe {
        z_uncompress(
            back.as_mut_ptr() as APTR,
            &mut back_len,
            dest.as_ptr() as CONST_APTR,
            dest_len,
        )
    };
    say_d(b"  Uncompress rc                     = \0", rc as u32);
    if rc != 0 {
        say(b"FAIL: Uncompress returned non-zero\n\0");
        unsafe { exec_drop_interface(iface); exec_close_library(base); }
        return 4;
    }
    say_d(b"  Decompressed bytes                = \0", back_len);
    back.truncate(back_len as usize);

    if back != src {
        say(b"FAIL: round-trip mismatch\n\0");
        unsafe { exec_drop_interface(iface); exec_close_library(base); }
        return 5;
    }

    // Tidy up.
    unsafe {
        exec_drop_interface(iface);
        exec_close_library(base);
    }

    say(b"\n  => PASS: deflate -> inflate round-trip byte-identical\n\0");
    say(b"========================================\n\0");
    0
}
