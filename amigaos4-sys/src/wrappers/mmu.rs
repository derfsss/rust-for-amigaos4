//! IMMU global(s) and convenience wrappers.
//!
//! Hand-written binding for the MMU sub-interface of exec.library —
//! page-level memory mapping operations. The MMUIFace is declared
//! inside `exec.h` (not its own header), so the SDK audit's
//! INTERFACES table maps this back to exec.h.

use crate::types::*;
use crate::interfaces::mmu::*;

// ---- IMMU (MMUIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IMMU: *mut MMUIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IMMU: *mut MMUIFace = core::ptr::null_mut();

// ── Mapping ──────────────────────────────────────────────────

#[inline]
pub unsafe fn mmu_map_memory(virtual_addr: APTR, physical_addr: APTR, size: u32, attributes: u32) -> u32 {
    ((*IMMU).MapMemory)(IMMU, virtual_addr, physical_addr, size, attributes)
}

#[inline]
pub unsafe fn mmu_unmap_memory(virtual_addr: APTR, size: u32) {
    ((*IMMU).UnmapMemory)(IMMU, virtual_addr, size)
}

#[inline]
pub unsafe fn mmu_remap_memory(virtual_addr: APTR, new_physical_addr: APTR, size: u32, attributes: u32) {
    ((*IMMU).RemapMemory)(IMMU, virtual_addr, new_physical_addr, size, attributes)
}

// ── Memory attributes ────────────────────────────────────────

#[inline]
pub unsafe fn mmu_set_memory_attrs(virtual_addr: APTR, size: u32, attributes: u32) {
    ((*IMMU).SetMemoryAttrs)(IMMU, virtual_addr, size, attributes)
}

#[inline]
pub unsafe fn mmu_get_memory_attrs(virtual_addr: APTR, size: u32) -> u32 {
    ((*IMMU).GetMemoryAttrs)(IMMU, virtual_addr, size)
}

#[inline]
pub unsafe fn mmu_get_physical_address(virtual_addr: APTR) -> APTR {
    ((*IMMU).GetPhysicalAddress)(IMMU, virtual_addr)
}
