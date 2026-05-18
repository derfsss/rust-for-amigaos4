//! IUSBSys global(s) and convenience wrappers for usbsys.library.

use crate::types::*;
use crate::interfaces::usbsys::*;

#[cfg(target_arch = "powerpc")]
extern "C" { pub static IUSBSys: *mut USBSysIFace; }
#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IUSBSys: *mut USBSysIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn usbsys_usballoc_request_a(io: *mut IORequest, tags: *mut TagItem) -> *mut USBIOReq {
    ((*IUSBSys).USBAllocRequestA)(IUSBSys, io, tags)
}
#[inline]
pub unsafe fn usbsys_usbfree_request(req: *mut USBIOReq) { ((*IUSBSys).USBFreeRequest)(IUSBSys, req) }
#[inline]
pub unsafe fn usbsys_usbfind_function_a(fct: *mut UsbRawFunction, tags: *mut TagItem) -> *mut UsbRawFunction {
    ((*IUSBSys).USBFindFunctionA)(IUSBSys, fct, tags)
}
#[inline]
pub unsafe fn usbsys_usblock_function(fct: *mut UsbRawFunction) { ((*IUSBSys).USBLockFunction)(IUSBSys, fct) }
#[inline]
pub unsafe fn usbsys_usbunlock_function(fct: *mut UsbRawFunction) { ((*IUSBSys).USBUnlockFunction)(IUSBSys, fct) }
#[inline]
pub unsafe fn usbsys_usbclaim_function(fct: *mut UsbRawFunction, owner: APTR, port: *mut MsgPort) -> *mut UsbFunction {
    ((*IUSBSys).USBClaimFunction)(IUSBSys, fct, owner, port)
}
#[inline]
pub unsafe fn usbsys_usbdeclaim_function(fct: *mut UsbFunction) { ((*IUSBSys).USBDeclaimFunction)(IUSBSys, fct) }
#[inline]
pub unsafe fn usbsys_usbadd_function_a(ifc: *mut UsbInterface, tags: *mut TagItem) -> *mut UsbRawFunction {
    ((*IUSBSys).USBAddFunctionA)(IUSBSys, ifc, tags)
}
#[inline]
pub unsafe fn usbsys_usbrem_function(fct: *mut UsbRawFunction, ifc: *mut UsbInterface) {
    ((*IUSBSys).USBRemFunction)(IUSBSys, fct, ifc)
}
#[inline]
pub unsafe fn usbsys_usbget_end_point(fct: *mut UsbFunction, ifc: *mut UsbInterface, idx: i32) -> *mut UsbEndPoint {
    ((*IUSBSys).USBGetEndPoint)(IUSBSys, fct, ifc, idx)
}
#[inline]
pub unsafe fn usbsys_usbepcontrol_xfer_a(
    io: *mut IORequest, ep: *mut UsbEndPoint, rt: u32, req: u32, val: u32, idx: u32, data: APTR, len: u32, tags: *mut TagItem,
) -> i32 { ((*IUSBSys).USBEPControlXferA)(IUSBSys, io, ep, rt, req, val, idx, data, len, tags) }
#[inline]
pub unsafe fn usbsys_usbepget_descriptor_a(
    io: *mut IORequest, ep: *mut UsbEndPoint, ty: u32, idx: i32, len: i32, tags: *mut TagItem,
) -> *mut USBBusDscHead { ((*IUSBSys).USBEPGetDescriptorA)(IUSBSys, io, ep, ty, idx, len, tags) }
#[inline]
pub unsafe fn usbsys_usbfkt_get_cfg_descriptors_a(
    io: *mut IORequest, fct: *mut UsbFunction, idx: u32, tags: *mut TagItem,
) -> *mut USBBusCfgDsc { ((*IUSBSys).USBFktGetCfgDescriptorsA)(IUSBSys, io, fct, idx, tags) }
#[inline]
pub unsafe fn usbsys_usbfkt_set_configuration_a(
    io: *mut IORequest, fct: *mut UsbFunction, cfg: *mut USBBusCfgDsc, tags: *mut TagItem,
) -> i32 { ((*IUSBSys).USBFktSetConfigurationA)(IUSBSys, io, fct, cfg, tags) }
#[inline]
pub unsafe fn usbsys_usbparse_descriptors(dsc: *mut USBBusDscHead, ty: u32) -> *mut USBBusDscHead {
    ((*IUSBSys).USBParseDescriptors)(IUSBSys, dsc, ty)
}
#[inline]
pub unsafe fn usbsys_usbfree_descriptors(dsc: *mut USBBusDscHead) { ((*IUSBSys).USBFreeDescriptors)(IUSBSys, dsc) }
#[inline]
pub unsafe fn usbsys_usbnext_descriptor(dsc: *mut USBBusDscHead) -> *mut USBBusDscHead {
    ((*IUSBSys).USBNextDescriptor)(IUSBSys, dsc)
}
#[inline]
pub unsafe fn usbsys_usbprev_descriptor(dsc: *mut USBBusDscHead) -> *mut USBBusDscHead {
    ((*IUSBSys).USBPrevDescriptor)(IUSBSys, dsc)
}
#[inline]
pub unsafe fn usbsys_usbobtain_resource_a(res: u32, owner: APTR, tags: *mut TagItem) -> i32 {
    ((*IUSBSys).USBObtainResourceA)(IUSBSys, res, owner, tags)
}
#[inline]
pub unsafe fn usbsys_usbrelease_resource(res: u32, owner: APTR) { ((*IUSBSys).USBReleaseResource)(IUSBSys, res, owner) }
#[inline]
pub unsafe fn usbsys_usbfind_interface_a(ifc: *mut UsbRawInterface, tags: *mut TagItem) -> *mut UsbRawInterface {
    ((*IUSBSys).USBFindInterfaceA)(IUSBSys, ifc, tags)
}
#[inline]
pub unsafe fn usbsys_usblock_interface(ifc: *mut UsbRawInterface) { ((*IUSBSys).USBLockInterface)(IUSBSys, ifc) }
#[inline]
pub unsafe fn usbsys_usbunlock_interface(ifc: *mut UsbRawInterface) { ((*IUSBSys).USBUnlockInterface)(IUSBSys, ifc) }
#[inline]
pub unsafe fn usbsys_usbclaim_interface(ifc: *mut UsbRawInterface, owner: APTR, port: *mut MsgPort) -> *mut UsbInterface {
    ((*IUSBSys).USBClaimInterface)(IUSBSys, ifc, owner, port)
}
#[inline]
pub unsafe fn usbsys_usbdeclaim_interface(ifc: *mut UsbInterface) { ((*IUSBSys).USBDeclaimInterface)(IUSBSys, ifc) }
#[inline]
pub unsafe fn usbsys_usbget_interface(fct: *mut UsbFunction, idx: i32) -> *mut UsbInterface {
    ((*IUSBSys).USBGetInterface)(IUSBSys, fct, idx)
}
#[inline]
pub unsafe fn usbsys_usbint_get_configuration_a(io: *mut IORequest, ifc: *mut UsbInterface, tags: *mut TagItem) -> *mut USBBusDscHead {
    ((*IUSBSys).USBIntGetConfigurationA)(IUSBSys, io, ifc, tags)
}
#[inline]
pub unsafe fn usbsys_usbint_get_alt_setting_a(io: *mut IORequest, ifc: *mut UsbInterface, tags: *mut TagItem) -> *mut USBBusDscHead {
    ((*IUSBSys).USBIntGetAltSettingA)(IUSBSys, io, ifc, tags)
}
#[inline]
pub unsafe fn usbsys_usbint_set_alt_setting_a(io: *mut IORequest, ifc: *mut UsbInterface, alt: u32, tags: *mut TagItem) -> i32 {
    ((*IUSBSys).USBIntSetAltSettingA)(IUSBSys, io, ifc, alt, tags)
}
#[inline]
pub unsafe fn usbsys_usblog_puts(level: i32, src: CONST_STRPTR, msg: CONST_STRPTR) {
    ((*IUSBSys).USBLogPuts)(IUSBSys, level, src, msg)
}
#[inline]
pub unsafe fn usbsys_usblog_vprintf(level: i32, src: CONST_STRPTR, fmt: CONST_STRPTR, args: *mut u32) {
    ((*IUSBSys).USBLogVPrintf)(IUSBSys, level, src, fmt, args)
}
#[inline]
pub unsafe fn usbsys_usbset_function_attrs_a(fct: *mut UsbFunction, tags: *mut TagItem) -> i32 {
    ((*IUSBSys).USBSetFunctionAttrsA)(IUSBSys, fct, tags)
}
#[inline]
pub unsafe fn usbsys_usbset_interface_attrs_a(ifc: *mut UsbInterface, tags: *mut TagItem) -> i32 {
    ((*IUSBSys).USBSetInterfaceAttrsA)(IUSBSys, ifc, tags)
}
#[inline]
pub unsafe fn usbsys_usbget_stack_attrs_a(tags: *mut TagItem) { ((*IUSBSys).USBGetStackAttrsA)(IUSBSys, tags) }
#[inline]
pub unsafe fn usbsys_usbget_raw_function_attrs_a(fct: *mut UsbRawFunction, tags: *mut TagItem) {
    ((*IUSBSys).USBGetRawFunctionAttrsA)(IUSBSys, fct, tags)
}
#[inline]
pub unsafe fn usbsys_usbget_raw_interface_attrs_a(ifc: *mut UsbRawInterface, tags: *mut TagItem) {
    ((*IUSBSys).USBGetRawInterfaceAttrsA)(IUSBSys, ifc, tags)
}
#[inline]
pub unsafe fn usbsys_usbfkt_driver_running(fct: *mut UsbFunction) { ((*IUSBSys).USBFktDriverRunning)(IUSBSys, fct) }
#[inline]
pub unsafe fn usbsys_usbifc_driver_running(ifc: *mut UsbInterface) { ((*IUSBSys).USBIfcDriverRunning)(IUSBSys, ifc) }
#[inline]
pub unsafe fn usbsys_usbepdestall(io: *mut IORequest, ep: *mut UsbEndPoint) -> i32 {
    ((*IUSBSys).USBEPDestall)(IUSBSys, io, ep)
}
#[inline]
pub unsafe fn usbsys_usballoc_object(ty: u32, tags: *mut TagItem) -> APTR { ((*IUSBSys).USBAllocObject)(IUSBSys, ty, tags) }
#[inline]
pub unsafe fn usbsys_usbfree_object(ty: u32, obj: APTR) { ((*IUSBSys).USBFreeObject)(IUSBSys, ty, obj) }
#[inline]
pub unsafe fn usbsys_usbscan_a(bus: u32, hook: *mut Hook, tags: *mut TagItem) -> i32 {
    ((*IUSBSys).USBScanA)(IUSBSys, bus, hook, tags)
}
#[inline]
pub unsafe fn usbsys_usbget_end_point_attrs_a(ep: *mut UsbEndPoint, tags: *mut TagItem) {
    ((*IUSBSys).USBGetEndPointAttrsA)(IUSBSys, ep, tags)
}
#[inline]
pub unsafe fn usbsys_usbget_frame_number(ep: *mut UsbEndPoint, tv: *mut TimeVal) -> u32 {
    ((*IUSBSys).USBGetFrameNumber)(IUSBSys, ep, tv)
}
#[inline]
pub unsafe fn usbsys_usbset_iso_transfer_count(req: *mut USBIOReq, count: u32) -> u32 {
    ((*IUSBSys).USBSetIsoTransferCount)(IUSBSys, req, count)
}
#[inline]
pub unsafe fn usbsys_usbset_iso_transfer_setup(req: *mut USBIOReq, idx: u32, len: u32, off: u32) -> u32 {
    ((*IUSBSys).USBSetIsoTransferSetup)(IUSBSys, req, idx, len, off)
}
#[inline]
pub unsafe fn usbsys_usbget_iso_transfer_result(req: *mut USBIOReq, idx: u32, out: *mut *mut USBTransferSetup) -> *mut USBTransferResult {
    ((*IUSBSys).USBGetIsoTransferResult)(IUSBSys, req, idx, out)
}
#[inline]
pub unsafe fn usbsys_usbget_iso_transfer_setup_hcd(req: *mut USBIOReqHCD, idx: u32) -> *mut USBTransferSetup {
    ((*IUSBSys).USBGetIsoTransferSetupHcd)(IUSBSys, req, idx)
}
#[inline]
pub unsafe fn usbsys_usbset_iso_transfer_result_hcd(req: *mut USBIOReqHCD, idx: u32, count: u32, status: i32) -> u32 {
    ((*IUSBSys).USBSetIsoTransferResultHcd)(IUSBSys, req, idx, count, status)
}
