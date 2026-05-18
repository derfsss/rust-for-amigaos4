//! ICW3SID global(s) and convenience wrappers.
//!
//! Hand-written binding for the Catweasel CW3 SID sub-device —
//! direct register-level access for SID (C64 sound) chip
//! emulation.

use crate::types::*;
use crate::interfaces::cw3_sid::*;

// ---- ICW3SID (CW3SIDIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICW3SID: *mut CW3SIDIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICW3SID: *mut CW3SIDIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn cw3_sid_in_byte(register: u32) -> u8 {
    ((*ICW3SID).InByte)(ICW3SID, register)
}

#[inline]
pub unsafe fn cw3_sid_out_byte(register: u32, value: u8) {
    ((*ICW3SID).OutByte)(ICW3SID, register, value)
}
