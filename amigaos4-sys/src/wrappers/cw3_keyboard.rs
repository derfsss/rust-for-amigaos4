//! ICW3Keyboard global(s) and convenience wrappers.
//!
//! Hand-written binding for the Catweasel CW3 keyboard sub-device —
//! direct register-level access for the C64 keyboard pass-through.

use crate::types::*;
use crate::interfaces::cw3_keyboard::*;

// ---- ICW3Keyboard (CW3KeyboardIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICW3Keyboard: *mut CW3KeyboardIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICW3Keyboard: *mut CW3KeyboardIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn cw3_keyboard_in_byte(register: u32) -> u8 {
    ((*ICW3Keyboard).InByte)(ICW3Keyboard, register)
}

#[inline]
pub unsafe fn cw3_keyboard_out_byte(register: u32, value: u8) {
    ((*ICW3Keyboard).OutByte)(ICW3Keyboard, register, value)
}
