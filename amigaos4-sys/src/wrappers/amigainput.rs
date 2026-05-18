//! IAmigaInput global(s) and convenience wrappers.
//!
//! Hand-written binding for amigainput.library — the AmigaOS 4 input
//! framework. Manages device contexts, enumerates devices, queries/
//! sets device parameters, and dispatches input events. Most arguments
//! are opaque handles (`APTR`) cookies the caller obtains from
//! upstream APIs.

use crate::types::*;
use crate::interfaces::amigainput::*;

// ---- IAmigaInput (AIN_IFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IAmigaInput: *mut AIN_IFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IAmigaInput: *mut AIN_IFace = core::ptr::null_mut();

// ── Error reporting ──────────────────────────────────────────

#[inline]
pub unsafe fn amigainput_ain_get_error(context: APTR) -> u32 {
    ((*IAmigaInput).AIN_GetError)(IAmigaInput, context)
}

#[inline]
pub unsafe fn amigainput_ain_get_error_string(error_code: u32) -> STRPTR {
    ((*IAmigaInput).AIN_GetErrorString)(IAmigaInput, error_code)
}

// ── Context lifecycle ────────────────────────────────────────

#[inline]
pub unsafe fn amigainput_ain_create_context(version: u32, tag_list: *mut TagItem) -> APTR {
    ((*IAmigaInput).AIN_CreateContext)(IAmigaInput, version, tag_list)
}

#[inline]
pub unsafe fn amigainput_ain_delete_context(context: APTR) {
    ((*IAmigaInput).AIN_DeleteContext)(IAmigaInput, context)
}

// ── Device enumeration / query ───────────────────────────────

#[inline]
pub unsafe fn amigainput_ain_enum_devices(
    context: APTR, callback: APTR, user_data: APTR,
) -> u32 {
    ((*IAmigaInput).AIN_EnumDevices)(IAmigaInput, context, callback, user_data)
}

#[inline]
pub unsafe fn amigainput_ain_query(
    context: APTR, device_id: APTR, query_type: u32, query_arg: u32, buffer: APTR, buffer_len: u32,
) -> u32 {
    ((*IAmigaInput).AIN_Query)(IAmigaInput, context, device_id, query_type, query_arg, buffer, buffer_len)
}

// ── Device-handle obtain / release ───────────────────────────

#[inline]
pub unsafe fn amigainput_ain_obtain_device(context: APTR, device_id: APTR) -> *mut APTR {
    ((*IAmigaInput).AIN_ObtainDevice)(IAmigaInput, context, device_id)
}

#[inline]
pub unsafe fn amigainput_ain_release_device(context: APTR, device: *mut APTR) {
    ((*IAmigaInput).AIN_ReleaseDevice)(IAmigaInput, context, device)
}

#[inline]
pub unsafe fn amigainput_ain_set_device_parameter(
    context: APTR, device: *mut APTR, parameter: u32, value: u32,
) -> u32 {
    ((*IAmigaInput).AIN_SetDeviceParameter)(IAmigaInput, context, device, parameter, value)
}

// ── Event pump ───────────────────────────────────────────────

#[inline]
pub unsafe fn amigainput_ain_get_event(context: APTR) -> *mut APTR {
    ((*IAmigaInput).AIN_GetEvent)(IAmigaInput, context)
}

#[inline]
pub unsafe fn amigainput_ain_free_event(context: APTR, event: *mut APTR) {
    ((*IAmigaInput).AIN_FreeEvent)(IAmigaInput, context, event)
}

#[inline]
pub unsafe fn amigainput_ain_read_device(
    context: APTR, device: *mut APTR, buffer: *mut APTR,
) -> u32 {
    ((*IAmigaInput).AIN_ReadDevice)(IAmigaInput, context, device, buffer)
}

// ── Context-level set + device requester ─────────────────────

#[inline]
pub unsafe fn amigainput_ain_set(context: APTR, tag_list: *mut TagItem) -> u32 {
    ((*IAmigaInput).AIN_Set)(IAmigaInput, context, tag_list)
}

#[inline]
pub unsafe fn amigainput_ain_request_device(
    context: APTR, tag_list: *mut TagItem,
) -> *mut _AIN_RequesterResult {
    ((*IAmigaInput).AIN_RequestDevice)(IAmigaInput, context, tag_list)
}

#[inline]
pub unsafe fn amigainput_ain_free_request(context: APTR, result: *mut _AIN_RequesterResult) {
    ((*IAmigaInput).AIN_FreeRequest)(IAmigaInput, context, result)
}
