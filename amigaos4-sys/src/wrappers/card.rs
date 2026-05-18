//! ICard global(s) and convenience wrappers.
//!
//! Hand-written binding for card.resource — Amiga PCMCIA / Credit-
//! Card slot arbitration (Rust basename `card`, SDK header `cardres.h`).

use crate::types::*;
use crate::interfaces::card::*;

// ---- ICard (CardIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static ICard: *mut CardIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICard: *mut CardIFace = core::ptr::null_mut();

// ── Ownership / access bracketing ────────────────────────────

#[inline]
pub unsafe fn card_own_card(handle: *mut CardHandle) -> *mut CardHandle {
    ((*ICard).OwnCard)(ICard, handle)
}

#[inline]
pub unsafe fn card_release_card(handle: *mut CardHandle, flags: u32) {
    ((*ICard).ReleaseCard)(ICard, handle, flags)
}

#[inline]
pub unsafe fn card_get_card_map() -> *mut CardMemoryMap {
    ((*ICard).GetCardMap)(ICard)
}

#[inline]
pub unsafe fn card_begin_card_access(handle: *mut CardHandle) -> u32 {
    ((*ICard).BeginCardAccess)(ICard, handle)
}

#[inline]
pub unsafe fn card_end_card_access(handle: *mut CardHandle) -> u32 {
    ((*ICard).EndCardAccess)(ICard, handle)
}

// ── Status / reset / control ─────────────────────────────────

#[inline]
pub unsafe fn card_read_card_status() -> u8 {
    ((*ICard).ReadCardStatus)(ICard)
}

#[inline]
pub unsafe fn card_card_reset_remove(handle: *mut CardHandle, flag: u32) -> u32 {
    ((*ICard).CardResetRemove)(ICard, handle, flag)
}

#[inline]
pub unsafe fn card_card_misc_control(handle: *mut CardHandle, control: u8) -> u8 {
    ((*ICard).CardMiscControl)(ICard, handle, control)
}

#[inline]
pub unsafe fn card_card_access_speed(handle: *mut CardHandle, nano_seconds: u32) -> u32 {
    ((*ICard).CardAccessSpeed)(ICard, handle, nano_seconds)
}

#[inline]
pub unsafe fn card_card_program_voltage(handle: *mut CardHandle, voltage: u32) -> i32 {
    ((*ICard).CardProgramVoltage)(ICard, handle, voltage)
}

#[inline]
pub unsafe fn card_card_reset_card(handle: *mut CardHandle) -> u32 {
    ((*ICard).CardResetCard)(ICard, handle)
}

// ── Tuple (config-info) parsing ──────────────────────────────

#[inline]
pub unsafe fn card_copy_tuple(
    handle: *const CardHandle, buffer: *mut u8, tuple_code: u32, size: u32,
) -> u32 {
    ((*ICard).CopyTuple)(ICard, handle, buffer, tuple_code, size)
}

#[inline]
pub unsafe fn card_device_tuple(tuple_data: *const u8, info: *mut DeviceTData) -> u32 {
    ((*ICard).DeviceTuple)(ICard, tuple_data, info)
}

#[inline]
pub unsafe fn card_if_amiga_xip(handle: *const CardHandle) -> *mut Resident {
    ((*ICard).IfAmigaXIP)(ICard, handle)
}

// ── Card-change notification ─────────────────────────────────

#[inline]
pub unsafe fn card_card_force_change() -> u32 {
    ((*ICard).CardForceChange)(ICard)
}

#[inline]
pub unsafe fn card_card_change_count() -> u32 {
    ((*ICard).CardChangeCount)(ICard)
}

#[inline]
pub unsafe fn card_card_interface() -> u32 {
    ((*ICard).CardInterface)(ICard)
}
