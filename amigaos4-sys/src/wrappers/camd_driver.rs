//! ICamdDriver global(s) and convenience wrappers.
//!
//! Hand-written binding for camddriver plugins — the per-port MIDI
//! driver interface camd.library dispatches into. SDK header is
//! `camddriver.h` (no underscore).

use crate::types::*;
use crate::interfaces::camd_driver::*;

// ---- ICamdDriver (CamdDriverIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICamdDriver: *mut CamdDriverIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICamdDriver: *mut CamdDriverIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn camd_driver_get_midi_device_data() -> *mut MidiDeviceData {
    ((*ICamdDriver).GetMidiDeviceData)(ICamdDriver)
}

#[inline]
pub unsafe fn camd_driver_open_port(
    data: *mut MidiDeviceData, port_num: i32, receive_fn: APTR, sysex_fn: APTR, user_data: APTR,
) -> u32 {
    ((*ICamdDriver).OpenPort)(ICamdDriver, data, port_num, receive_fn, sysex_fn, user_data)
}

#[inline]
pub unsafe fn camd_driver_close_port(data: *mut MidiDeviceData, port_num: i32) {
    ((*ICamdDriver).ClosePort)(ICamdDriver, data, port_num)
}

#[inline]
pub unsafe fn camd_driver_activate_xmit(port: APTR, port_num: i32) {
    ((*ICamdDriver).ActivateXmit)(ICamdDriver, port, port_num)
}
