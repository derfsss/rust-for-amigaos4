//! RAII memory allocation via AmigaOS `AllocVecTagList`/`FreeVec`.
//!
//! [`AmigaVec`] provides a safe, auto-freeing wrapper around OS memory
//! allocation. Use this for buffers that need specific memory type flags
//! (e.g. `MEMF_SHARED` for DMA) rather than the global allocator.

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
        if size > u32::MAX as usize {
            return Err(AmigaError::AllocationFailed);
        }
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
        if size > u32::MAX as usize {
            return Err(AmigaError::AllocationFailed);
        }
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

    /// Get a raw pointer to the allocated memory.
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.ptr
    }

    /// Get a mutable raw pointer to the allocated memory.
    #[inline]
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.ptr
    }

    /// Return the size of the allocation in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Return `true` if the allocation has zero length.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// View the allocation as a byte slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self.ptr, self.len) }
    }

    /// View the allocation as a mutable byte slice.
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

/// A DMA-capable buffer: `MEMF_SHARED` memory paired with the PPC cache
/// maintenance a device driver needs around DMA transfers.
///
/// Typical use in a driver:
///
/// ```ignore
/// let mut buf = DmaBuffer::alloc(4096)?;
/// buf.as_mut_slice()[..len].copy_from_slice(data);
/// buf.flush();              // CPU writes -> RAM, device may now read
/// // ... start device DMA read of buf ...
///
/// // ... device DMA wrote into buf ...
/// buf.invalidate();         // next CPU read fetches fresh RAM
/// let received = &buf.as_slice()[..len];
/// ```
pub struct DmaBuffer {
    mem: AmigaVec,
}

impl DmaBuffer {
    /// Allocate `size` bytes of zeroed `MEMF_SHARED` memory.
    pub fn alloc(size: usize) -> Result<Self> {
        Ok(Self {
            mem: AmigaVec::alloc_cleared(size, amigaos4_sys::MEMF_SHARED, 0)?,
        })
    }

    /// Flush CPU caches to RAM (`dcbst`) so the device sees the latest
    /// CPU-written data. Call before starting a device-read DMA.
    /// (No-op on non-PPC hosts.)
    pub fn flush(&self) {
        // SAFETY: ptr/len describe this allocation.
        #[cfg(target_arch = "powerpc")]
        unsafe {
            amigaos4_sys::ppc_asm::cache_flush(self.mem.as_ptr(), self.mem.len())
        }
    }

    /// Flush + invalidate CPU caches (`dcbf`) so the next CPU read
    /// fetches fresh data from RAM. Call after a device-write DMA
    /// completes, before reading the buffer. (No-op on non-PPC hosts.)
    pub fn invalidate(&self) {
        // SAFETY: ptr/len describe this allocation.
        #[cfg(target_arch = "powerpc")]
        unsafe {
            amigaos4_sys::ppc_asm::cache_invalidate(self.mem.as_ptr(), self.mem.len())
        }
    }

    /// The buffer as a byte slice.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.mem.as_slice()
    }

    /// The buffer as a mutable byte slice.
    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.mem.as_mut_slice()
    }

    /// Raw pointer for handing to the device (physical = virtual for
    /// MEMF_SHARED on supported systems; use `GetPhysicalAddress`-style
    /// APIs where the hardware needs a physical address).
    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.mem.as_ptr()
    }

    /// Buffer length in bytes.
    #[inline]
    pub fn len(&self) -> usize {
        self.mem.len()
    }

    /// True if the buffer has zero length.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.mem.is_empty()
    }
}

/// An exec memory pool (`CreatePool`/`AllocPooled`). Sub-allocations are freed
/// ATOMICALLY when the pool is dropped (`DeletePool` frees the whole pool at
/// once, even allocations you never freed individually) - useful for a unit of
/// work whose memory should all go away together (e.g. a contained subsystem).
///
/// Automatically deletes the pool on drop.
pub struct AmigaPool {
    pool: APTR,
}

impl AmigaPool {
    /// Create a pool. `mem_type` is the `MEMF_*` requirements, `puddle` the bytes
    /// allocated per puddle, and `thresh` the largest single allocation served
    /// from puddles (larger allocations get their own dedicated block). `thresh`
    /// must be <= `puddle`.
    pub fn new(mem_type: u32, puddle: u32, thresh: u32) -> Result<Self> {
        let pool = unsafe { amigaos4_sys::exec_create_pool(mem_type, puddle, thresh) };
        if pool.is_null() {
            Err(AmigaError::AllocationFailed)
        } else {
            Ok(Self { pool })
        }
    }

    /// Allocate `size` bytes from the pool. The memory is freed en masse when the
    /// pool is dropped (or individually via [`AmigaPool::free`]). Returns a null
    /// pointer on failure.
    pub fn alloc(&self, size: usize) -> *mut u8 {
        if size > u32::MAX as usize {
            return core::ptr::null_mut();
        }
        unsafe { amigaos4_sys::exec_alloc_pooled(self.pool, size as u32) as *mut u8 }
    }

    /// Free a single allocation back to the pool (optional - the pool frees
    /// everything on drop). `size` must match the original `alloc`.
    ///
    /// # Safety
    /// `ptr` must have come from this pool's `alloc` with the same `size`.
    pub unsafe fn free(&self, ptr: *mut u8, size: usize) {
        if !ptr.is_null() && size <= u32::MAX as usize {
            amigaos4_sys::exec_free_pooled(self.pool, ptr as APTR, size as u32);
        }
    }

    /// The raw pool header.
    pub fn as_ptr(&self) -> APTR {
        self.pool
    }
}

impl Drop for AmigaPool {
    fn drop(&mut self) {
        if !self.pool.is_null() {
            unsafe { amigaos4_sys::exec_delete_pool(self.pool) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::mem;

    #[test]
    fn amiga_vec_struct_size() {
        // AmigaVec contains a *mut u8 and a usize
        assert_eq!(
            mem::size_of::<AmigaVec>(),
            mem::size_of::<*mut u8>() + mem::size_of::<usize>()
        );
    }
}
