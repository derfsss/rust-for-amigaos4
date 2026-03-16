//! Application-mode allocator backed by clib4's malloc/free.
//!
//! Requires linking with `-mcrt=clib4` (normal application build).
//!
//! clib4's malloc guarantees 16-byte alignment. For rare types requiring
//! greater alignment, we over-allocate and manually align.

use core::alloc::{GlobalAlloc, Layout};

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

/// Global allocator backed by clib4's `malloc`/`free`.
///
/// Use this for normal AmigaOS applications linked with clib4.
pub struct Clib4Allocator;

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn clib4_allocator_is_zst() {
        assert_eq!(mem::size_of::<Clib4Allocator>(), 0);
    }
}

unsafe impl GlobalAlloc for Clib4Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.align() <= 16 {
            unsafe { malloc(layout.size()) }
        } else {
            // Over-allocate to guarantee alignment > 16 bytes.
            // Store original pointer just before the aligned region.
            let overhead = layout.align() + core::mem::size_of::<usize>();
            let total = match layout.size().checked_add(overhead) {
                Some(t) => t,
                None => return core::ptr::null_mut(),
            };
            let raw = unsafe { malloc(total) };
            if raw.is_null() {
                return raw;
            }
            let base = unsafe { raw.add(core::mem::size_of::<usize>()) };
            let aligned = ((base as usize + layout.align() - 1) & !(layout.align() - 1)) as *mut u8;
            unsafe {
                let store = aligned.sub(core::mem::size_of::<usize>()) as *mut usize;
                *store = raw as usize;
            }
            aligned
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.align() <= 16 {
            unsafe { free(ptr) }
        } else {
            let original = unsafe {
                let store = ptr.sub(core::mem::size_of::<usize>()) as *const usize;
                *store as *mut u8
            };
            unsafe { free(original) }
        }
    }
}
