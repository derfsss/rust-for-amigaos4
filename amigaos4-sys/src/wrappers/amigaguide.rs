//! IAmigaGuide global(s) and convenience wrappers.
//!
//! Hand-written binding for amigaguide.library — the AmigaOS
//! hypertext viewer used by `Help` button across the Workbench UI.
//! Open/close guide documents (synchronously or asynchronously),
//! dispatch context-jumps, send commands, query/set attributes,
//! manage cross-reference databases, and register custom hosts.

use crate::types::*;
use crate::interfaces::amigaguide::*;

// ---- IAmigaGuide (AmigaGuideIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IAmigaGuide: *mut AmigaGuideIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IAmigaGuide: *mut AmigaGuideIFace = core::ptr::null_mut();

// ── Library lock (rarely needed by user code) ────────────────

#[inline]
pub unsafe fn amigaguide_lock_amiga_guide_base(reserved: APTR) -> i32 {
    ((*IAmigaGuide).LockAmigaGuideBase)(IAmigaGuide, reserved)
}

#[inline]
pub unsafe fn amigaguide_unlock_amiga_guide_base(lock: i32) {
    ((*IAmigaGuide).UnlockAmigaGuideBase)(IAmigaGuide, lock)
}

// ── Open / close guide documents ─────────────────────────────

#[inline]
pub unsafe fn amigaguide_open_amiga_guide_a(
    nag: *mut NewAmigaGuide, tag_list: *mut TagItem,
) -> APTR {
    ((*IAmigaGuide).OpenAmigaGuideA)(IAmigaGuide, nag, tag_list)
}

#[inline]
pub unsafe fn amigaguide_open_amiga_guide_async_a(
    nag: *mut NewAmigaGuide, tag_list: *mut TagItem,
) -> APTR {
    ((*IAmigaGuide).OpenAmigaGuideAsyncA)(IAmigaGuide, nag, tag_list)
}

#[inline]
pub unsafe fn amigaguide_close_amiga_guide(handle: APTR) {
    ((*IAmigaGuide).CloseAmigaGuide)(IAmigaGuide, handle)
}

// ── Message-loop integration ─────────────────────────────────

#[inline]
pub unsafe fn amigaguide_amiga_guide_signal(handle: APTR) -> u32 {
    ((*IAmigaGuide).AmigaGuideSignal)(IAmigaGuide, handle)
}

#[inline]
pub unsafe fn amigaguide_get_amiga_guide_msg(handle: APTR) -> *mut AmigaGuideMsg {
    ((*IAmigaGuide).GetAmigaGuideMsg)(IAmigaGuide, handle)
}

#[inline]
pub unsafe fn amigaguide_reply_amiga_guide_msg(msg: *mut AmigaGuideMsg) {
    ((*IAmigaGuide).ReplyAmigaGuideMsg)(IAmigaGuide, msg)
}

// ── Context jumps and commands ───────────────────────────────

#[inline]
pub unsafe fn amigaguide_set_amiga_guide_context_a(
    handle: APTR, context_id: u32, tag_list: *mut TagItem,
) -> i32 {
    ((*IAmigaGuide).SetAmigaGuideContextA)(IAmigaGuide, handle, context_id, tag_list)
}

#[inline]
pub unsafe fn amigaguide_send_amiga_guide_context_a(
    handle: APTR, tag_list: *mut TagItem,
) -> i32 {
    ((*IAmigaGuide).SendAmigaGuideContextA)(IAmigaGuide, handle, tag_list)
}

#[inline]
pub unsafe fn amigaguide_send_amiga_guide_cmd_a(
    handle: APTR, cmd: STRPTR, tag_list: *mut TagItem,
) -> i32 {
    ((*IAmigaGuide).SendAmigaGuideCmdA)(IAmigaGuide, handle, cmd, tag_list)
}

// ── Attribute access ─────────────────────────────────────────

#[inline]
pub unsafe fn amigaguide_set_amiga_guide_attrs_a(handle: APTR, tag_list: *mut TagItem) -> i32 {
    ((*IAmigaGuide).SetAmigaGuideAttrsA)(IAmigaGuide, handle, tag_list)
}

#[inline]
pub unsafe fn amigaguide_get_amiga_guide_attr(attr: u32, storage: APTR, length: *mut u32) -> i32 {
    ((*IAmigaGuide).GetAmigaGuideAttr)(IAmigaGuide, attr, storage, length)
}

// ── Cross-reference databases ────────────────────────────────

#[inline]
pub unsafe fn amigaguide_load_xref(flags: u32, name: STRPTR) -> i32 {
    ((*IAmigaGuide).LoadXRef)(IAmigaGuide, flags, name)
}

#[inline]
pub unsafe fn amigaguide_expunge_xref() {
    ((*IAmigaGuide).ExpungeXRef)(IAmigaGuide)
}

// ── Custom host registration ─────────────────────────────────

#[inline]
pub unsafe fn amigaguide_add_amiga_guide_host_a(
    hook: *mut Hook, name: STRPTR, tag_list: *mut TagItem,
) -> APTR {
    ((*IAmigaGuide).AddAmigaGuideHostA)(IAmigaGuide, hook, name, tag_list)
}

#[inline]
pub unsafe fn amigaguide_remove_amiga_guide_host_a(host: APTR, tag_list: *mut TagItem) -> i32 {
    ((*IAmigaGuide).RemoveAmigaGuideHostA)(IAmigaGuide, host, tag_list)
}

// ── String table ─────────────────────────────────────────────

#[inline]
pub unsafe fn amigaguide_get_amiga_guide_string(string_id: i32) -> CONST_STRPTR {
    ((*IAmigaGuide).GetAmigaGuideString)(IAmigaGuide, string_id)
}
