//! IFileSysBox global(s) and convenience wrappers.
//!
//! Hand-written binding for filesysbox.library — a FUSE-style
//! filesystem framework for AmigaOS 4. Lets userspace handlers
//! implement filesystems via callback ops rather than full DOS
//! packet handlers.

use crate::types::*;
use crate::interfaces::filesysbox::*;

// ---- IFileSysBox (FileSysBoxIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IFileSysBox: *mut FileSysBoxIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IFileSysBox: *mut FileSysBoxIFace = core::ptr::null_mut();

// ── Mount-message handling ───────────────────────────────────

#[inline]
pub unsafe fn filesysbox_fbx_query_mount_msg(msg: *mut Message, attr: i32) -> APTR {
    ((*IFileSysBox).FbxQueryMountMsg)(IFileSysBox, msg, attr)
}

#[inline]
pub unsafe fn filesysbox_fbx_return_mount_msg(msg: *mut Message, err1: i32, err2: i32) {
    ((*IFileSysBox).FbxReturnMountMsg)(IFileSysBox, msg, err1, err2)
}

// ── Filesystem lifecycle ─────────────────────────────────────

#[inline]
pub unsafe fn filesysbox_fbx_setup_fs(
    msg: *mut Message, tags: *const TagItem, ops: *const fuse_operations,
    sizeof_ops: i32, user_data: APTR,
) -> *mut FbxFS {
    ((*IFileSysBox).FbxSetupFS)(IFileSysBox, msg, tags, ops, sizeof_ops, user_data)
}

#[inline]
pub unsafe fn filesysbox_fbx_event_loop(fs: *mut FbxFS) -> i32 {
    ((*IFileSysBox).FbxEventLoop)(IFileSysBox, fs)
}

#[inline]
pub unsafe fn filesysbox_fbx_cleanup_fs(fs: *mut FbxFS) {
    ((*IFileSysBox).FbxCleanupFS)(IFileSysBox, fs)
}

// ── Version queries ──────────────────────────────────────────

#[inline]
pub unsafe fn filesysbox_fbx_fuse_version() -> i32 {
    ((*IFileSysBox).FbxFuseVersion)(IFileSysBox)
}

#[inline]
pub unsafe fn filesysbox_fbx_version() -> i32 {
    ((*IFileSysBox).FbxVersion)(IFileSysBox)
}

// ── Signal & timer callbacks ─────────────────────────────────

#[inline]
pub unsafe fn filesysbox_fbx_set_signal_callback(fs: *mut FbxFS, callback: APTR, signal_mask: u32) {
    ((*IFileSysBox).FbxSetSignalCallback)(IFileSysBox, fs, callback, signal_mask)
}

#[inline]
pub unsafe fn filesysbox_fbx_install_timer_callback(
    fs: *mut FbxFS, callback: APTR, period: u32,
) -> *mut FbxTimerCallbackData {
    ((*IFileSysBox).FbxInstallTimerCallback)(IFileSysBox, fs, callback, period)
}

#[inline]
pub unsafe fn filesysbox_fbx_uninstall_timer_callback(
    fs: *mut FbxFS, data: *mut FbxTimerCallbackData,
) {
    ((*IFileSysBox).FbxUninstallTimerCallback)(IFileSysBox, fs, data)
}

// ── Filesystem state ─────────────────────────────────────────

#[inline]
pub unsafe fn filesysbox_fbx_signal_disk_change(fs: *mut FbxFS) {
    ((*IFileSysBox).FbxSignalDiskChange)(IFileSysBox, fs)
}

#[inline]
pub unsafe fn filesysbox_fbx_query_fs(fs: *mut FbxFS, tags: *const TagItem) {
    ((*IFileSysBox).FbxQueryFS)(IFileSysBox, fs, tags)
}

#[inline]
pub unsafe fn filesysbox_fbx_get_sys_time(fs: *mut FbxFS, time: *mut TimeVal) {
    ((*IFileSysBox).FbxGetSysTime)(IFileSysBox, fs, time)
}
