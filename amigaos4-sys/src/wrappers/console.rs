//! IConsole global(s) and convenience wrappers.
//!
//! Hand-written binding for console.device. Provides raw-key
//! conversion via the active keymap, plus the system clipboard
//! ("snip") accessors used by terminal-style apps.

use crate::types::*;
use crate::interfaces::console::*;

// ---- IConsole (ConsoleIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IConsole: *mut ConsoleIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IConsole: *mut ConsoleIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn console_cdinput_handler(events: *const InputEvent, console_device: *mut Library) -> *mut InputEvent {
    ((*IConsole).CDInputHandler)(IConsole, events, console_device)
}

#[inline]
pub unsafe fn console_raw_key_convert(
    event: *const InputEvent, buffer: STRPTR, length: i32, keymap: *const KeyMap,
) -> i32 {
    ((*IConsole).RawKeyConvert)(IConsole, event, buffer, length, keymap)
}

// ── Console "snip" (clipboard) ───────────────────────────────

#[inline]
pub unsafe fn console_get_con_snip() -> STRPTR {
    ((*IConsole).GetConSnip)(IConsole)
}

#[inline]
pub unsafe fn console_set_con_snip(snip: STRPTR) -> u32 {
    ((*IConsole).SetConSnip)(IConsole, snip)
}

#[inline]
pub unsafe fn console_add_con_snip_hook(hook: *mut Hook) {
    ((*IConsole).AddConSnipHook)(IConsole, hook)
}

#[inline]
pub unsafe fn console_rem_con_snip_hook(hook: *mut Hook) {
    ((*IConsole).RemConSnipHook)(IConsole, hook)
}
