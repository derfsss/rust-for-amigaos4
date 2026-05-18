//! IACPI global(s) and convenience wrappers.
//!
//! Hand-written binding for acpi.resource — system-level Advanced
//! Configuration & Power Interface. Get/Set attributes (power states,
//! wake events, etc.) and request a clean system shutdown.

use crate::types::*;
use crate::interfaces::acpi::*;

// ---- IACPI (ACPIIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IACPI: *mut ACPIIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IACPI: *mut ACPIIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn acpi_get_attr(attr: u32, storage: APTR) {
    ((*IACPI).GetAttr)(IACPI, attr, storage)
}

#[inline]
pub unsafe fn acpi_set_attr(attr: u32, value: CONST_APTR) {
    ((*IACPI).SetAttr)(IACPI, attr, value)
}

#[inline]
pub unsafe fn acpi_shutdown() {
    ((*IACPI).Shutdown)(IACPI)
}
