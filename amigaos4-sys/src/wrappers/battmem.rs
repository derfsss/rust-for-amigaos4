//! IBattMem global(s) and convenience wrappers.
//!
//! Hand-written binding for battmem.resource — battery-backed NVRAM
//! access. Argument names taken from battmem.resource's AutoDocs.

use crate::types::*;
use crate::interfaces::battmem::*;

// ---- IBattMem (BattMemIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IBattMem: *mut BattMemIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IBattMem: *mut BattMemIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn battmem_obtain_batt_semaphore() {
    ((*IBattMem).ObtainBattSemaphore)(IBattMem)
}

#[inline]
pub unsafe fn battmem_release_batt_semaphore() {
    ((*IBattMem).ReleaseBattSemaphore)(IBattMem)
}

/// Read a bitstring from nonvolatile RAM. Returns error code (0=ok).
#[inline]
pub unsafe fn battmem_read_batt_mem(buffer: APTR, offset: u32, len: u32) -> u32 {
    ((*IBattMem).ReadBattMem)(IBattMem, buffer, offset, len)
}

/// Write a bitstring to nonvolatile RAM. Returns error code (0=ok).
#[inline]
pub unsafe fn battmem_write_batt_mem(buffer: CONST_APTR, offset: u32, len: u32) -> u32 {
    ((*IBattMem).WriteBattMem)(IBattMem, buffer, offset, len)
}
