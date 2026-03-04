//! Application-mode allocator backed by clib4's malloc/free.
//!
//! Requires linking with `-mcrt=clib4` (normal application build).

use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Global allocator backed by clib4's `malloc`/`free`.
///
/// Use this for normal AmigaOS applications linked with clib4.
pub struct Clib4Allocator;

unsafe impl GlobalAlloc for Clib4Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { malloc(layout.size()) }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        unsafe { free(ptr) }
    }
}
