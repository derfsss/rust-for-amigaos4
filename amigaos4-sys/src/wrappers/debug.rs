//! IDebug global(s) and convenience wrappers.
//!
//! Hand-written binding for debug.library — system-level debugging
//! support: PowerPC and 68k disassembly, task-context inspection,
//! stack traces, debug symbols, and the dumpdebugbuffer notification
//! API used by `dumpdebugbuffer` and similar utilities.

use crate::types::*;
use crate::interfaces::debug::*;

// ---- IDebug (DebugIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IDebug: *mut DebugIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IDebug: *mut DebugIFace = core::ptr::null_mut();

// ── Hooks and global debug state ─────────────────────────────

#[inline]
pub unsafe fn debug_add_debug_hook(task: *mut Task, hook: *mut Hook) -> u32 {
    ((*IDebug).AddDebugHook)(IDebug, task, hook)
}

#[inline]
pub unsafe fn debug_get_debug_level() -> u32 {
    ((*IDebug).GetDebugLevel)(IDebug)
}

// ── Disassembly ──────────────────────────────────────────────

#[inline]
pub unsafe fn debug_disassemble_native(address: APTR, buffer: STRPTR, args: STRPTR) -> APTR {
    ((*IDebug).DisassembleNative)(IDebug, address, buffer, args)
}

#[inline]
pub unsafe fn debug_disassemble68k(address: APTR, buffer: STRPTR, args: STRPTR) -> APTR {
    ((*IDebug).Disassemble68k)(IDebug, address, buffer, args)
}

// ── Task context and stack-trace ─────────────────────────────

#[inline]
pub unsafe fn debug_read_task_context(task: *mut Task, context: *mut ExceptionContext, flags: u32) -> u32 {
    ((*IDebug).ReadTaskContext)(IDebug, task, context, flags)
}

#[inline]
pub unsafe fn debug_write_task_context(task: *mut Task, context: *mut ExceptionContext, flags: u32) -> u32 {
    ((*IDebug).WriteTaskContext)(IDebug, task, context, flags)
}

#[inline]
pub unsafe fn debug_stack_trace(task: *mut Task, hook: *mut Hook) -> i32 {
    ((*IDebug).StackTrace)(IDebug, task, hook)
}

#[inline]
pub unsafe fn debug_is_in68k_emulator(task: *const Task) -> u32 {
    ((*IDebug).IsIn68kEmulator)(IDebug, task)
}

// ── Debug symbols ────────────────────────────────────────────

#[inline]
pub unsafe fn debug_obtain_debug_symbol(address: CONST_APTR, tag_list: *mut TagItem) -> *mut DebugSymbol {
    ((*IDebug).ObtainDebugSymbol)(IDebug, address, tag_list)
}

#[inline]
pub unsafe fn debug_release_debug_symbol(symbol: *mut DebugSymbol) {
    ((*IDebug).ReleaseDebugSymbol)(IDebug, symbol)
}

// ── Debug-output notification (dumpdebugbuffer hook) ─────────

#[inline]
pub unsafe fn debug_start_debug_output_notify(tag_list: *mut TagItem) -> *mut DebugOutputNotify {
    ((*IDebug).StartDebugOutputNotify)(IDebug, tag_list)
}

#[inline]
pub unsafe fn debug_end_debug_output_notify(handle: *mut DebugOutputNotify) {
    ((*IDebug).EndDebugOutputNotify)(IDebug, handle)
}
