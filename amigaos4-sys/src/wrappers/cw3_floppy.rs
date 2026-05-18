//! ICW3Floppy global(s) and convenience wrappers.
//!
//! Hand-written binding for the Catweasel CW3 floppy sub-device —
//! direct register-level access for floppy-disk emulation. Just
//! 8-bit InByte / OutByte access primitives.

use crate::types::*;
use crate::interfaces::cw3_floppy::*;

// ---- ICW3Floppy (CW3FloppyIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICW3Floppy: *mut CW3FloppyIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICW3Floppy: *mut CW3FloppyIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn cw3_floppy_in_byte(register: u32) -> u8 {
    ((*ICW3Floppy).InByte)(ICW3Floppy, register)
}

#[inline]
pub unsafe fn cw3_floppy_out_byte(register: u32, value: u8) {
    ((*ICW3Floppy).OutByte)(ICW3Floppy, register, value)
}
