//! ICamd global(s) and convenience wrappers for camd.library — the
//! Commodore MIDI subsystem.

use crate::types::*;
use crate::interfaces::camd::*;

#[cfg(target_arch = "powerpc")]
extern "C" { pub static ICamd: *mut CamdIFace; }
#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut ICamd: *mut CamdIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn camd_lock_camd(mode: u32) -> APTR { ((*ICamd).LockCAMD)(ICamd, mode) }
#[inline]
pub unsafe fn camd_unlock_camd(lock: APTR) { ((*ICamd).UnlockCAMD)(ICamd, lock) }
#[inline]
pub unsafe fn camd_create_midi_a(tags: *mut TagItem) -> *mut MidiNode { ((*ICamd).CreateMidiA)(ICamd, tags) }
#[inline]
pub unsafe fn camd_delete_midi(midi: *mut MidiNode) { ((*ICamd).DeleteMidi)(ICamd, midi) }
#[inline]
pub unsafe fn camd_set_midi_attrs_a(midi: *mut MidiNode, tags: *mut TagItem) -> u32 { ((*ICamd).SetMidiAttrsA)(ICamd, midi, tags) }
#[inline]
pub unsafe fn camd_get_midi_attrs_a(midi: *mut MidiNode, tags: *mut TagItem) -> u32 { ((*ICamd).GetMidiAttrsA)(ICamd, midi, tags) }
#[inline]
pub unsafe fn camd_next_midi(midi: *mut MidiNode) -> *mut MidiNode { ((*ICamd).NextMidi)(ICamd, midi) }
#[inline]
pub unsafe fn camd_find_midi(name: STRPTR) -> *mut MidiNode { ((*ICamd).FindMidi)(ICamd, name) }
#[inline]
pub unsafe fn camd_flush_midi(midi: *mut MidiNode) { ((*ICamd).FlushMidi)(ICamd, midi) }
#[inline]
pub unsafe fn camd_add_midi_link_a(midi: *mut MidiNode, link_type: i32, tags: *mut TagItem) -> *mut MidiLink {
    ((*ICamd).AddMidiLinkA)(ICamd, midi, link_type, tags)
}
#[inline]
pub unsafe fn camd_remove_midi_link(link: *mut MidiLink) { ((*ICamd).RemoveMidiLink)(ICamd, link) }
#[inline]
pub unsafe fn camd_set_midi_link_attrs_a(link: *mut MidiLink, tags: *mut TagItem) -> u32 { ((*ICamd).SetMidiLinkAttrsA)(ICamd, link, tags) }
#[inline]
pub unsafe fn camd_get_midi_link_attrs_a(link: *mut MidiLink, tags: *mut TagItem) -> u32 { ((*ICamd).GetMidiLinkAttrsA)(ICamd, link, tags) }
#[inline]
pub unsafe fn camd_next_cluster_link(cluster: *mut MidiCluster, link: *mut MidiLink, link_type: i32) -> *mut MidiLink {
    ((*ICamd).NextClusterLink)(ICamd, cluster, link, link_type)
}
#[inline]
pub unsafe fn camd_next_midi_link(midi: *mut MidiNode, link: *mut MidiLink, link_type: i32) -> *mut MidiLink {
    ((*ICamd).NextMidiLink)(ICamd, midi, link, link_type)
}
#[inline]
pub unsafe fn camd_midi_link_connected(link: *mut MidiLink) -> u32 { ((*ICamd).MidiLinkConnected)(ICamd, link) }
#[inline]
pub unsafe fn camd_next_cluster(cluster: *mut MidiCluster) -> *mut MidiCluster { ((*ICamd).NextCluster)(ICamd, cluster) }
#[inline]
pub unsafe fn camd_find_cluster(name: STRPTR) -> *mut MidiCluster { ((*ICamd).FindCluster)(ICamd, name) }
#[inline]
pub unsafe fn camd_put_midi(link: *mut MidiLink, msg: u32) { ((*ICamd).PutMidi)(ICamd, link, msg) }
#[inline]
pub unsafe fn camd_get_midi(midi: *mut MidiNode, out: *mut APTR) -> u32 { ((*ICamd).GetMidi)(ICamd, midi, out) }
#[inline]
pub unsafe fn camd_wait_midi(midi: *mut MidiNode, out: *mut APTR) -> u32 { ((*ICamd).WaitMidi)(ICamd, midi, out) }
#[inline]
pub unsafe fn camd_put_sys_ex(link: *mut MidiLink, data: *mut u8) { ((*ICamd).PutSysEx)(ICamd, link, data) }
#[inline]
pub unsafe fn camd_get_sys_ex(midi: *mut MidiNode, buffer: *mut u8, max_len: u32) -> u32 { ((*ICamd).GetSysEx)(ICamd, midi, buffer, max_len) }
#[inline]
pub unsafe fn camd_query_sys_ex(midi: *mut MidiNode) -> u32 { ((*ICamd).QuerySysEx)(ICamd, midi) }
#[inline]
pub unsafe fn camd_skip_sys_ex(midi: *mut MidiNode) { ((*ICamd).SkipSysEx)(ICamd, midi) }
#[inline]
pub unsafe fn camd_get_midi_err(midi: *mut MidiNode) -> u8 { ((*ICamd).GetMidiErr)(ICamd, midi) }
#[inline]
pub unsafe fn camd_midi_msg_type(msg: *mut APTR) -> i16 { ((*ICamd).MidiMsgType)(ICamd, msg) }
#[inline]
pub unsafe fn camd_midi_msg_len(msg: u32) -> i16 { ((*ICamd).MidiMsgLen)(ICamd, msg) }
#[inline]
pub unsafe fn camd_parse_midi(link: *mut MidiLink, buf: *mut APTR, len: u32) { ((*ICamd).ParseMidi)(ICamd, link, buf, len) }
#[inline]
pub unsafe fn camd_open_midi_device(name: STRPTR) -> *mut MidiDeviceData { ((*ICamd).OpenMidiDevice)(ICamd, name) }
#[inline]
pub unsafe fn camd_close_midi_device(data: *mut MidiDeviceData) { ((*ICamd).CloseMidiDevice)(ICamd, data) }
#[inline]
pub unsafe fn camd_rethink_camd() -> i32 { ((*ICamd).RethinkCAMD)(ICamd) }
#[inline]
pub unsafe fn camd_start_cluster_notify(node: *mut ClusterNotifyNode) { ((*ICamd).StartClusterNotify)(ICamd, node) }
#[inline]
pub unsafe fn camd_end_cluster_notify(node: *mut ClusterNotifyNode) { ((*ICamd).EndClusterNotify)(ICamd, node) }
