//! ICyrus global(s) and convenience wrappers.
//!
//! Hand-written binding for cyrus.resource — Sam440/460ep memory
//! controller (Marvell Cyrus) resource arbiter.

use crate::types::*;
use crate::interfaces::cyrus::*;

// ---- ICyrus (CyrusIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICyrus: *mut CyrusIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICyrus: *mut CyrusIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn cyrus_alloc_resource(unit: u32) -> *mut u32 {
    ((*ICyrus).AllocResource)(ICyrus, unit)
}

#[inline]
pub unsafe fn cyrus_free_resource(unit: u32) {
    ((*ICyrus).FreeResource)(ICyrus, unit)
}
