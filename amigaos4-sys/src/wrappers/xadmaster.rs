//! IxadMaster global(s) and convenience wrappers for xadmaster.library.

use crate::types::*;
use crate::interfaces::xadmaster::*;

#[cfg(target_arch = "powerpc")]
extern "C" { pub static IxadMaster: *mut XadMasterIFace; }
#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IxadMaster: *mut XadMasterIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn xadmaster_xad_alloc_object_a(obj_type: APTR, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadAllocObjectA)(IxadMaster, obj_type, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_free_object_a(obj: APTR, tags: *const TagItem) {
    ((*IxadMaster).xadFreeObjectA)(IxadMaster, obj, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_recog_file_a(size: APTR, mem: *const (), tags: *const TagItem) -> *mut xadClient {
    ((*IxadMaster).xadRecogFileA)(IxadMaster, size, mem, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_get_info_a(ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadGetInfoA)(IxadMaster, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_free_info(ai: *mut xadArchiveInfo) { ((*IxadMaster).xadFreeInfo)(IxadMaster, ai) }
#[inline]
pub unsafe fn xadmaster_xad_file_un_arc_a(ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadFileUnArcA)(IxadMaster, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_disk_un_arc_a(ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadDiskUnArcA)(IxadMaster, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_get_error_text(err: APTR) -> APTR { ((*IxadMaster).xadGetErrorText)(IxadMaster, err) }
#[inline]
pub unsafe fn xadmaster_xad_get_client_info() -> *mut xadClient { ((*IxadMaster).xadGetClientInfo)(IxadMaster) }
#[inline]
pub unsafe fn xadmaster_xad_hook_access(cmd: APTR, data: APTR, buffer: APTR, ai: *mut xadArchiveInfo) -> APTR {
    ((*IxadMaster).xadHookAccess)(IxadMaster, cmd, data, buffer, ai)
}
#[inline]
pub unsafe fn xadmaster_xad_convert_dates_a(tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadConvertDatesA)(IxadMaster, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_calc_crc16(id: APTR, init: APTR, len: APTR, buffer: *const APTR) -> APTR {
    ((*IxadMaster).xadCalcCRC16)(IxadMaster, id, init, len, buffer)
}
#[inline]
pub unsafe fn xadmaster_xad_calc_crc32(id: APTR, init: APTR, len: APTR, buffer: *const APTR) -> APTR {
    ((*IxadMaster).xadCalcCRC32)(IxadMaster, id, init, len, buffer)
}
#[inline]
pub unsafe fn xadmaster_xad_alloc_vec(size: APTR, flags: APTR) -> APTR {
    ((*IxadMaster).xadAllocVec)(IxadMaster, size, flags)
}
#[inline]
pub unsafe fn xadmaster_xad_copy_mem(src: *const (), dst: APTR, len: APTR) {
    ((*IxadMaster).xadCopyMem)(IxadMaster, src, dst, len)
}
#[inline]
pub unsafe fn xadmaster_xad_hook_tag_access_a(cmd: APTR, data: APTR, buffer: APTR, ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadHookTagAccessA)(IxadMaster, cmd, data, buffer, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_convert_protection_a(tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadConvertProtectionA)(IxadMaster, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_get_disk_info_a(ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadGetDiskInfoA)(IxadMaster, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_disk_file_un_arc_a(ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadDiskFileUnArcA)(IxadMaster, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_get_hook_access_a(ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadGetHookAccessA)(IxadMaster, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_free_hook_access_a(ai: *mut xadArchiveInfo, tags: *const TagItem) {
    ((*IxadMaster).xadFreeHookAccessA)(IxadMaster, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_add_file_entry_a(fi: *mut xadFileInfo, ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadAddFileEntryA)(IxadMaster, fi, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_add_disk_entry_a(di: *mut xadDiskInfo, ai: *mut xadArchiveInfo, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadAddDiskEntryA)(IxadMaster, di, ai, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_get_filename_a(size: APTR, buffer: APTR, name: *const APTR, ext: *const APTR, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadGetFilenameA)(IxadMaster, size, buffer, name, ext, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_convert_name_a(charset: APTR, tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadConvertNameA)(IxadMaster, charset, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_get_default_name_a(tags: *const TagItem) -> APTR {
    ((*IxadMaster).xadGetDefaultNameA)(IxadMaster, tags)
}
#[inline]
pub unsafe fn xadmaster_xad_get_system_info() -> *const xadSystemInfo {
    ((*IxadMaster).xadGetSystemInfo)(IxadMaster)
}
