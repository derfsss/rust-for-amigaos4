//! IAmiSSLMaster global(s) and convenience wrappers.
//!
//! Hand-written binding for amisslmaster.library — the loader for
//! amissl.library (which exposes the OpenSSL surface). Apps that
//! want OpenSSL open the master library, call `InitAmiSSLMaster`
//! to negotiate a version, then `OpenAmiSSL` to get a usable amissl
//! base.

use crate::types::*;
use crate::interfaces::amisslmaster::*;

// ---- IAmiSSLMaster (AmiSSLMasterIFace) ----

#[cfg(target_arch = "powerpc")]
extern "C" {
    pub static IAmiSSLMaster: *mut AmiSSLMasterIFace;
}

#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IAmiSSLMaster: *mut AmiSSLMasterIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn amisslmaster_init_ami_sslmaster(version: i32, flags: i32) -> i32 {
    ((*IAmiSSLMaster).InitAmiSSLMaster)(IAmiSSLMaster, version, flags)
}

#[inline]
pub unsafe fn amisslmaster_open_ami_ssl() -> *mut Library {
    ((*IAmiSSLMaster).OpenAmiSSL)(IAmiSSLMaster)
}

#[inline]
pub unsafe fn amisslmaster_close_ami_ssl() {
    ((*IAmiSSLMaster).CloseAmiSSL)(IAmiSSLMaster)
}

#[inline]
pub unsafe fn amisslmaster_open_ami_sslcipher(cipher: i32) -> *mut Library {
    ((*IAmiSSLMaster).OpenAmiSSLCipher)(IAmiSSLMaster, cipher)
}

#[inline]
pub unsafe fn amisslmaster_close_ami_sslcipher(cipher_base: *mut Library) {
    ((*IAmiSSLMaster).CloseAmiSSLCipher)(IAmiSSLMaster, cipher_base)
}

#[inline]
pub unsafe fn amisslmaster_open_ami_ssltag_list(version: i32, tag_list: *mut TagItem) -> i32 {
    ((*IAmiSSLMaster).OpenAmiSSLTagList)(IAmiSSLMaster, version, tag_list)
}
