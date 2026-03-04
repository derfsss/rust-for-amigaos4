//! Driver-mode allocator backed by IExec->AllocVecTagList / FreeVec.
//!
//! No clib4 dependency. Works with `-nostartfiles -nodefaultlibs -lgcc`.
//!
//! IMPORTANT: IExec must be set before any allocation occurs.
//! The C startup glue (driver_glue.c) sets IExec in _start() before
//! calling rust_handler_main(), so all Rust code sees a valid IExec.
//!
//! Uses MEMF_PRIVATE by default (CPU-only, faster).
//! For DMA buffers, allocate explicitly via AmigaVec::alloc(size, MEMF_SHARED).

use core::alloc::{GlobalAlloc, Layout};
use amigaos4_sys::{IExec, TagItem, TAG_DONE, AVT_TYPE, MEMF_PRIVATE};

/// Global allocator backed by Exec's `AllocVecTagList`/`FreeVec`.
///
/// Use this for device drivers and handlers that have no clib4.
/// IExec must be initialized before any allocation.
pub struct ExecAllocator;

unsafe impl GlobalAlloc for ExecAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let iexec = unsafe { IExec };
        if iexec.is_null() {
            return core::ptr::null_mut();
        }

        // AllocVecTagList returns memory aligned to at least 16 bytes.
        // For most Rust types this is sufficient.
        if layout.align() <= 16 {
            let tags = [
                TagItem { ti_Tag: AVT_TYPE, ti_Data: MEMF_PRIVATE },
                TagItem { ti_Tag: TAG_DONE, ti_Data: 0 },
            ];
            unsafe {
                ((*iexec).AllocVecTagList)(iexec, layout.size() as u32, tags.as_ptr()) as *mut u8
            }
        } else {
            // Over-allocate for unusual alignment (>16 bytes).
            // Store the original pointer just before the aligned region.
            let overhead = layout.align() + core::mem::size_of::<usize>();
            let total = layout.size() + overhead;
            let tags = [
                TagItem { ti_Tag: AVT_TYPE, ti_Data: MEMF_PRIVATE },
                TagItem { ti_Tag: TAG_DONE, ti_Data: 0 },
            ];
            let raw = unsafe {
                ((*iexec).AllocVecTagList)(iexec, total as u32, tags.as_ptr()) as *mut u8
            };
            if raw.is_null() {
                return raw;
            }
            // Align: skip past the space for the stored pointer
            let base = unsafe { raw.add(core::mem::size_of::<usize>()) };
            let aligned = ((base as usize + layout.align() - 1) & !(layout.align() - 1)) as *mut u8;
            // Store original pointer just before the aligned address
            unsafe {
                let store = aligned.sub(core::mem::size_of::<usize>()) as *mut usize;
                *store = raw as usize;
            }
            aligned
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let iexec = unsafe { IExec };
        if iexec.is_null() || ptr.is_null() {
            return;
        }

        if layout.align() <= 16 {
            unsafe {
                ((*iexec).FreeVec)(iexec, ptr as *mut core::ffi::c_void);
            }
        } else {
            // Recover the original pointer stored before the aligned address
            let original = unsafe {
                let store = ptr.sub(core::mem::size_of::<usize>()) as *const usize;
                *store as *mut core::ffi::c_void
            };
            unsafe {
                ((*iexec).FreeVec)(iexec, original);
            }
        }
    }
}
