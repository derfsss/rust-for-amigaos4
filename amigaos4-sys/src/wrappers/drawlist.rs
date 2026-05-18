//! IDrawList global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `DRAWLIST_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::drawlist::*;

// ---- IDrawList (DrawListIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IDrawList: *mut DrawListIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IDrawList: *mut DrawListIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn drawlist_drawlist_get_class() -> *mut APTR {
    ((*IDrawList).DRAWLIST_GetClass)(IDrawList)
}
