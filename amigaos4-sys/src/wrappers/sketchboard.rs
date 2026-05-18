//! ISketchBoard global(s) and convenience wrappers.
//!
//! Hand-written ReAction gadget-class binding. Like the other
//! single-`GetClass` gadgets, the only user-facing method is
//! `SKETCHBOARD_GetClass`; lifecycle methods are framework-owned.

use crate::types::*;
use crate::interfaces::sketchboard::*;

// ---- ISketchBoard (SketchBoardIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ISketchBoard: *mut SketchBoardIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ISketchBoard: *mut SketchBoardIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn sketchboard_sketchboard_get_class() -> *mut APTR {
    ((*ISketchBoard).SKETCHBOARD_GetClass)(ISketchBoard)
}
