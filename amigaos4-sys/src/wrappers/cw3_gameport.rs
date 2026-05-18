//! ICW3Gameport global(s) and convenience wrappers.
//!
//! Hand-written binding for the Catweasel CW3 gameport sub-device —
//! direct register-level access for joystick/paddle input.

use crate::types::*;
use crate::interfaces::cw3_gameport::*;

// ---- ICW3Gameport (CW3GameportIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICW3Gameport: *mut CW3GameportIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICW3Gameport: *mut CW3GameportIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn cw3_gameport_in_byte(register: u32) -> u8 {
    ((*ICW3Gameport).InByte)(ICW3Gameport, register)
}

#[inline]
pub unsafe fn cw3_gameport_out_byte(register: u32, value: u8) {
    ((*ICW3Gameport).OutByte)(ICW3Gameport, register, value)
}
