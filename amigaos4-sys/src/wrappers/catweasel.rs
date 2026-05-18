//! ICatWeasel global(s) and convenience wrappers.
//!
//! Hand-written binding for catweasel.resource — Individual Computers
//! Catweasel I/O card (Amiga floppy controller + audio + retro
//! peripherals). The IFace just exposes presence/availability probes
//! for the various sub-functions of the card.

use crate::types::*;
use crate::interfaces::catweasel::*;

// ---- ICatWeasel (CWIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICatWeasel: *mut CWIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICatWeasel: *mut CWIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn catweasel_cw3_present() -> u32 {
    ((*ICatWeasel).CW3Present)(ICatWeasel)
}

#[inline]
pub unsafe fn catweasel_cw4_present() -> u32 {
    ((*ICatWeasel).CW4Present)(ICatWeasel)
}

#[inline]
pub unsafe fn catweasel_cw3_gameport_avail() -> u32 {
    ((*ICatWeasel).CW3GameportAvail)(ICatWeasel)
}

#[inline]
pub unsafe fn catweasel_cw3_keyboard_avail() -> u32 {
    ((*ICatWeasel).CW3KeyboardAvail)(ICatWeasel)
}

#[inline]
pub unsafe fn catweasel_cw3_sidavail() -> u32 {
    ((*ICatWeasel).CW3SIDAvail)(ICatWeasel)
}

#[inline]
pub unsafe fn catweasel_cw3_floppy_avail() -> u32 {
    ((*ICatWeasel).CW3FloppyAvail)(ICatWeasel)
}
