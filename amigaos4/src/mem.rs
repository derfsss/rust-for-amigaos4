use crate::error::{AmigaError, Result};
use crate::tag::TagListBuilder;
use amigaos4_sys::{APTR, AVT_TYPE, AVT_CLEAR_WITH_VALUE};

/// RAII wrapper around AllocVecTagList / FreeVec.
///
/// Automatically frees the allocation on drop.
pub struct AmigaVec {
    ptr: *mut u8,
    len: usize,
}

impl AmigaVec {
    /// Allocate `size` bytes with the given memory type flags.
    pub fn alloc(size: usize, mem_type: u32) -> Result<Self> {
        let tags = TagListBuilder::new()
            .tag(AVT_TYPE, mem_type)
            .build();
        let ptr = unsafe {
            amigaos4_sys::exec_alloc_vec_tag_list(size as u32, tags.as_ptr())
        };
        if ptr.is_null() {
            Err(AmigaError::AllocationFailed)
        } else {
            Ok(Self { ptr: ptr as *mut u8, len: size })
        }
    }

    /// Allocate `size` bytes, cleared to `fill` value.
    pub fn alloc_cleared(size: usize, mem_type: u32, fill: u8) -> Result<Self> {
        let tags = TagListBuilder::new()
            .tag(AVT_TYPE, mem_type)
            .tag(AVT_CLEAR_WITH_VALUE, fill as u32)
            .build();
        let ptr = unsafe {
            amigaos4_sys::exec_alloc_vec_tag_list(size as u32, tags.as_ptr())
        };
        if ptr.is_null() {
            Err(AmigaError::AllocationFailed)
        } else {
            Ok(Self { ptr: ptr as *mut u8, len: size })
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl Drop for AmigaVec {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { amigaos4_sys::exec_free_vec(self.ptr as APTR) }
        }
    }
}
