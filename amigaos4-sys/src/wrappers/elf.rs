//! IElf global(s) and convenience wrappers.
//!
//! Hand-written binding for elf.library — ELF binary loader and
//! introspection. Used by the runtime to load shared libraries and
//! resolve dynamic symbols (.so via DLOpen/DLSym/DLClose), plus
//! lower-level section/symbol access for debuggers and tools.

use crate::types::*;
use crate::interfaces::elf::*;

// ---- IElf (ElfIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IElf: *mut ElfIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IElf: *mut ElfIFace = core::ptr::null_mut();

// ── ELF handle lifecycle ─────────────────────────────────────

#[inline]
pub unsafe fn elf_open_elf(tag_list: *mut TagItem) -> APTR {
    ((*IElf).OpenElf)(IElf, tag_list)
}

#[inline]
pub unsafe fn elf_close_elf(handle: APTR, tag_list: *mut TagItem) {
    ((*IElf).CloseElf)(IElf, handle, tag_list)
}

// ── Attribute access ─────────────────────────────────────────

#[inline]
pub unsafe fn elf_get_elf_attrs(handle: APTR, tag_list: *mut TagItem) -> u32 {
    ((*IElf).GetElfAttrs)(IElf, handle, tag_list)
}

#[inline]
pub unsafe fn elf_set_elf_attrs(handle: APTR, tag_list: *mut TagItem) -> u32 {
    ((*IElf).SetElfAttrs)(IElf, handle, tag_list)
}

// ── Sections ─────────────────────────────────────────────────

#[inline]
pub unsafe fn elf_get_section_header(handle: APTR, tag_list: *mut TagItem) -> *mut APTR {
    ((*IElf).GetSectionHeader)(IElf, handle, tag_list)
}

#[inline]
pub unsafe fn elf_get_section(handle: APTR, tag_list: *mut TagItem) -> APTR {
    ((*IElf).GetSection)(IElf, handle, tag_list)
}

#[inline]
pub unsafe fn elf_elf_load_seg(handle: APTR, tag_list: *mut TagItem) -> APTR {
    ((*IElf).ElfLoadSeg)(IElf, handle, tag_list)
}

#[inline]
pub unsafe fn elf_unload_section(section: APTR, tag_list: *mut TagItem) {
    ((*IElf).UnloadSection)(IElf, section, tag_list)
}

#[inline]
pub unsafe fn elf_relocate_section(handle: APTR, tag_list: *mut TagItem) -> u32 {
    ((*IElf).RelocateSection)(IElf, handle, tag_list)
}

#[inline]
pub unsafe fn elf_address_to_section(handle: APTR, address: APTR) -> u32 {
    ((*IElf).AddressToSection)(IElf, handle, address)
}

// ── Strings and symbols ──────────────────────────────────────

#[inline]
pub unsafe fn elf_get_elf_string(
    handle: APTR, section_index: u16, string_offset: u16,
) -> CONST_STRPTR {
    ((*IElf).GetElfString)(IElf, handle, section_index, string_offset)
}

#[inline]
pub unsafe fn elf_symbol_query(handle: APTR, query_count: u32, query: *mut Elf32_SymbolQuery) -> u32 {
    ((*IElf).SymbolQuery)(IElf, handle, query_count, query)
}

#[inline]
pub unsafe fn elf_scan_symbol_table(handle: APTR, hook: *mut Hook, tag_list: *mut TagItem) {
    ((*IElf).ScanSymbolTable)(IElf, handle, hook, tag_list)
}

// ── Data-segment copies ──────────────────────────────────────

#[inline]
pub unsafe fn elf_copy_data_segment(handle: APTR, size: *mut u32) -> *mut () {
    ((*IElf).CopyDataSegment)(IElf, handle, size)
}

#[inline]
pub unsafe fn elf_free_data_segment_copy(handle: APTR, copy: *mut ()) {
    ((*IElf).FreeDataSegmentCopy)(IElf, handle, copy)
}

// ── Shared-library / dynamic-loading API ─────────────────────

#[inline]
pub unsafe fn elf_init_shlibs(handle: APTR, flags: u32) {
    ((*IElf).InitSHLibs)(IElf, handle, flags)
}

#[inline]
pub unsafe fn elf_dlopen(handle: APTR, path: CONST_STRPTR, mode: u32) -> APTR {
    ((*IElf).DLOpen)(IElf, handle, path, mode)
}

#[inline]
pub unsafe fn elf_dlclose(handle: APTR, dl_handle: APTR) -> APTR {
    ((*IElf).DLClose)(IElf, handle, dl_handle)
}

#[inline]
pub unsafe fn elf_dlsym(
    handle: APTR, dl_handle: APTR, name: CONST_STRPTR, address: *mut APTR,
) -> APTR {
    ((*IElf).DLSym)(IElf, handle, dl_handle, name, address)
}

/// Walk the shared-library handle chain via a hook callback.
/// Backfilled from current SDK (position 33) — see regression test
/// `elf_iface_has_get_so_handles` in Tests/tests/rust_regression.rs.
#[inline]
pub unsafe fn elf_get_sohandles(handle: APTR, hook: *mut Hook) -> i32 {
    ((*IElf).GetSOHandles)(IElf, handle, hook)
}
