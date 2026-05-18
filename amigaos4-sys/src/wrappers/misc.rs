//! IMisc global(s) and convenience wrappers.
//!
//! Hand-written to match the amigaos4-bindgen convention used by the
//! other wrapper modules.

use crate::types::*;
use crate::interfaces::misc::*;

// ---- IMisc (MiscIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IMisc: *mut MiscIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IMisc: *mut MiscIFace = core::ptr::null_mut();

// ── Resource allocation ──────────────────────────────────────

#[inline]
pub unsafe fn misc_alloc_misc_resource(unit_num: u32, name: CONST_STRPTR) -> CONST_STRPTR {
    ((*IMisc).AllocMiscResource)(IMisc, unit_num, name)
}

#[inline]
pub unsafe fn misc_free_misc_resource(unit_num: u32) {
    ((*IMisc).FreeMiscResource)(IMisc, unit_num)
}
