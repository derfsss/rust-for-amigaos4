//! IBevel global(s) and convenience wrappers.
//!
//! Hand-written to match the amigaos4-bindgen convention used by the
//! other wrapper modules.

use crate::types::*;
use crate::interfaces::bevel::*;

// ---- IBevel (BevelIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IBevel: *mut BevelIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IBevel: *mut BevelIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn bevel_bevel_get_class() -> *mut APTR {
    ((*IBevel).BEVEL_GetClass)(IBevel)
}

#[inline]
pub unsafe fn bevel_new_bevel_prefs() {
    ((*IBevel).NewBevelPrefs)(IBevel)
}
