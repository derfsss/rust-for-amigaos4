//! IAmiSSL global(s) and convenience wrappers for amissl.library.
//!
//! Auto-generated from the AmiSSLIFace vtable. Every typed method
//! slot gets a `pub unsafe fn` of the form:
//!     amissl_<snake>(args) -> ret { ((*IAmiSSL).Name)(IAmiSSL, args) }
//! Reserved/UNIMPLEMENTED slots and varargs siblings are skipped.

use crate::types::*;
use crate::interfaces::amissl::*;

#[cfg(target_arch = "powerpc")]
extern "C" { pub static IAmiSSL: *mut AmiSSLIFace; }
#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IAmiSSL: *mut AmiSSLIFace = core::ptr::null_mut();


#[inline]
pub unsafe fn amissl_internal_init_ami_ssl(a0: *mut ()) { ((*IAmiSSL).InternalInitAmiSSL)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_init_ami_ssla(a0: *mut TagItem) -> i32 { ((*IAmiSSL).InitAmiSSLA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cleanup_ami_ssla(a0: *mut TagItem) -> i32 { ((*IAmiSSL).CleanupAmiSSLA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_type_new() -> *mut APTR { ((*IAmiSSL).ASN1_TYPE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_type_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_TYPE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_type(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_TYPE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_type(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_TYPE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_any_it() -> *const APTR { ((*IAmiSSL).ASN1_ANY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_type_get(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_TYPE_get)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_type_set(a0: *mut APTR, a1: i32, a2: *mut ()) { ((*IAmiSSL).ASN1_TYPE_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_object_new() -> *mut APTR { ((*IAmiSSL).ASN1_OBJECT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_object_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_OBJECT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_asn1_object(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_OBJECT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_asn1_object(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_OBJECT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_object_it() -> *const APTR { ((*IAmiSSL).ASN1_OBJECT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_string_new() -> *mut APTR { ((*IAmiSSL).ASN1_STRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_string_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_STRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_string_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_STRING_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_string_type_new(a0: i32) -> *mut APTR { ((*IAmiSSL).ASN1_STRING_type_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_string_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_STRING_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_string_set(a0: *mut APTR, a1: *const (), a2: i32) -> i32 { ((*IAmiSSL).ASN1_STRING_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_string_length(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_STRING_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_string_length_set(a0: *mut APTR, a1: i32) { ((*IAmiSSL).ASN1_STRING_length_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_string_type(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_STRING_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_string_data(a0: *mut APTR) -> *mut u8 { ((*IAmiSSL).ASN1_STRING_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_new() -> *mut APTR { ((*IAmiSSL).ASN1_BIT_STRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_BIT_STRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_bit_string(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_BIT_STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_bit_string(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_BIT_STRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_it() -> *const APTR { ((*IAmiSSL).ASN1_BIT_STRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_set(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).ASN1_BIT_STRING_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_set_bit(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).ASN1_BIT_STRING_set_bit)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_get_bit(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).ASN1_BIT_STRING_get_bit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_name_print(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).ASN1_BIT_STRING_name_print)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_num_asc(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_BIT_STRING_num_asc)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_set_asc(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_BIT_STRING_set_asc)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_integer_new() -> *mut APTR { ((*IAmiSSL).ASN1_INTEGER_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_integer_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_INTEGER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_integer(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_INTEGER)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_integer(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_INTEGER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_integer_it() -> *const APTR { ((*IAmiSSL).ASN1_INTEGER_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_asn1_uinteger(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_UINTEGER)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_integer_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_INTEGER_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_integer_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_INTEGER_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_enumerated_new() -> *mut APTR { ((*IAmiSSL).ASN1_ENUMERATED_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_enumerated_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_ENUMERATED_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_enumerated(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_ENUMERATED)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_enumerated(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_ENUMERATED)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_enumerated_it() -> *const APTR { ((*IAmiSSL).ASN1_ENUMERATED_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_utctime_check(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_UTCTIME_check)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_utctime_set(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).ASN1_UTCTIME_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_utctime_set_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_UTCTIME_set_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_utctime_cmp_time_t(a0: *const APTR, a1: APTR) -> i32 { ((*IAmiSSL).ASN1_UTCTIME_cmp_time_t)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_check(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_GENERALIZEDTIME_check)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_set(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).ASN1_GENERALIZEDTIME_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_set_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_GENERALIZEDTIME_set_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_octet_string_new() -> *mut APTR { ((*IAmiSSL).ASN1_OCTET_STRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_octet_string_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_OCTET_STRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_octet_string(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_OCTET_STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_octet_string(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_OCTET_STRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_octet_string_it() -> *const APTR { ((*IAmiSSL).ASN1_OCTET_STRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_octet_string_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_OCTET_STRING_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_octet_string_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_OCTET_STRING_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_octet_string_set(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).ASN1_OCTET_STRING_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_visiblestring_new() -> *mut APTR { ((*IAmiSSL).ASN1_VISIBLESTRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_visiblestring_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_VISIBLESTRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_visiblestring(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_VISIBLESTRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_visiblestring(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_VISIBLESTRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_visiblestring_it() -> *const APTR { ((*IAmiSSL).ASN1_VISIBLESTRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_universalstring_new() -> *mut APTR { ((*IAmiSSL).ASN1_UNIVERSALSTRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_universalstring_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_UNIVERSALSTRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_universalstring(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_UNIVERSALSTRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_universalstring(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_UNIVERSALSTRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_universalstring_it() -> *const APTR { ((*IAmiSSL).ASN1_UNIVERSALSTRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_utf8_string_new() -> *mut APTR { ((*IAmiSSL).ASN1_UTF8STRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_utf8_string_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_UTF8STRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_utf8_string(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_UTF8STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_utf8_string(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_UTF8STRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_utf8_string_it() -> *const APTR { ((*IAmiSSL).ASN1_UTF8STRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_null_new() -> *mut APTR { ((*IAmiSSL).ASN1_NULL_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_null_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_NULL_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_null(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_NULL)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_null(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_NULL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_null_it() -> *const APTR { ((*IAmiSSL).ASN1_NULL_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_bmpstring_new() -> *mut APTR { ((*IAmiSSL).ASN1_BMPSTRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_bmpstring_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_BMPSTRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_bmpstring(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_BMPSTRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_bmpstring(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_BMPSTRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_bmpstring_it() -> *const APTR { ((*IAmiSSL).ASN1_BMPSTRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_utf8_getc(a0: *const u8, a1: i32, a2: *mut u32) -> i32 { ((*IAmiSSL).UTF8_getc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_utf8_putc(a0: *mut u8, a1: i32, a2: u32) -> i32 { ((*IAmiSSL).UTF8_putc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_printable_new() -> *mut APTR { ((*IAmiSSL).ASN1_PRINTABLE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_printable_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_PRINTABLE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_printable(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_PRINTABLE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_printable(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_PRINTABLE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_printable_it() -> *const APTR { ((*IAmiSSL).ASN1_PRINTABLE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_directorystring_new() -> *mut APTR { ((*IAmiSSL).DIRECTORYSTRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_directorystring_free(a0: *mut APTR) { ((*IAmiSSL).DIRECTORYSTRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_directorystring(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DIRECTORYSTRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_directorystring(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DIRECTORYSTRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_directorystring_it() -> *const APTR { ((*IAmiSSL).DIRECTORYSTRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_displaytext_new() -> *mut APTR { ((*IAmiSSL).DISPLAYTEXT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_displaytext_free(a0: *mut APTR) { ((*IAmiSSL).DISPLAYTEXT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_displaytext(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DISPLAYTEXT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_displaytext(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DISPLAYTEXT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_displaytext_it() -> *const APTR { ((*IAmiSSL).DISPLAYTEXT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_printablestring_new() -> *mut APTR { ((*IAmiSSL).ASN1_PRINTABLESTRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_printablestring_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_PRINTABLESTRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_printablestring(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_PRINTABLESTRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_printablestring(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_PRINTABLESTRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_printablestring_it() -> *const APTR { ((*IAmiSSL).ASN1_PRINTABLESTRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_t61_string_new() -> *mut APTR { ((*IAmiSSL).ASN1_T61STRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_t61_string_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_T61STRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_t61_string(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_T61STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_t61_string(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_T61STRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_t61_string_it() -> *const APTR { ((*IAmiSSL).ASN1_T61STRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_ia5_string_new() -> *mut APTR { ((*IAmiSSL).ASN1_IA5STRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_ia5_string_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_IA5STRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_ia5_string(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_IA5STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_ia5_string(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_IA5STRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_ia5_string_it() -> *const APTR { ((*IAmiSSL).ASN1_IA5STRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_generalstring_new() -> *mut APTR { ((*IAmiSSL).ASN1_GENERALSTRING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_generalstring_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_GENERALSTRING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_generalstring(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_GENERALSTRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_generalstring(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_GENERALSTRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_generalstring_it() -> *const APTR { ((*IAmiSSL).ASN1_GENERALSTRING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_utctime_new() -> *mut APTR { ((*IAmiSSL).ASN1_UTCTIME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_utctime_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_UTCTIME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_utctime(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_UTCTIME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_utctime(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_UTCTIME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_utctime_it() -> *const APTR { ((*IAmiSSL).ASN1_UTCTIME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_new() -> *mut APTR { ((*IAmiSSL).ASN1_GENERALIZEDTIME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_GENERALIZEDTIME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_generalizedtime(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_GENERALIZEDTIME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_generalizedtime(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_GENERALIZEDTIME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_it() -> *const APTR { ((*IAmiSSL).ASN1_GENERALIZEDTIME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_time_new() -> *mut APTR { ((*IAmiSSL).ASN1_TIME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_time_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_TIME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asn1_time(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_TIME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_time(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_TIME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_time_it() -> *const APTR { ((*IAmiSSL).ASN1_TIME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_time_set(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).ASN1_TIME_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_time_check(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_TIME_check)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_time_to_generalizedtime(a0: *const APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).ASN1_TIME_to_generalizedtime)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2a_asn1_integer(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2a_ASN1_INTEGER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_a2i_asn1_integer(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).a2i_ASN1_INTEGER)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_i2a_asn1_enumerated(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2a_ASN1_ENUMERATED)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_a2i_asn1_enumerated(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).a2i_ASN1_ENUMERATED)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_i2a_asn1_object(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2a_ASN1_OBJECT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_a2i_asn1_string(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).a2i_ASN1_STRING)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_i2a_asn1_string(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).i2a_ASN1_STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2t_asn1_object(a0: *mut APTR, a1: i32, a2: *const APTR) -> i32 { ((*IAmiSSL).i2t_ASN1_OBJECT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_a2d_asn1_object(a0: *mut u8, a1: i32, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).a2d_ASN1_OBJECT)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_object_create(a0: i32, a1: *mut u8, a2: i32, a3: *const APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_OBJECT_create)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_integer_set(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).ASN1_INTEGER_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_integer_get(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_INTEGER_get)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_to_asn1_integer(a0: *const APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_to_ASN1_INTEGER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_integer_to_bn(a0: *const APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).ASN1_INTEGER_to_BN)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_enumerated_set(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).ASN1_ENUMERATED_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_enumerated_get(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_ENUMERATED_get)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_to_asn1_enumerated(a0: *const APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_to_ASN1_ENUMERATED)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_enumerated_to_bn(a0: *const APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).ASN1_ENUMERATED_to_BN)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_printable_type(a0: *const u8, a1: i32) -> i32 { ((*IAmiSSL).ASN1_PRINTABLE_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_tag2bit(a0: i32) -> u32 { ((*IAmiSSL).ASN1_tag2bit)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_get_object(a0: *mut *mut APTR, a1: *mut i32, a2: *mut i32, a3: *mut i32, a4: i32) -> i32 { ((*IAmiSSL).ASN1_get_object)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_check_infinite_end(a0: *mut *mut u8, a1: i32) -> i32 { ((*IAmiSSL).ASN1_check_infinite_end)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_put_object(a0: *mut *mut u8, a1: i32, a2: i32, a3: i32, a4: i32) { ((*IAmiSSL).ASN1_put_object)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_object_size(a0: i32, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).ASN1_object_size)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_dup(a0: *mut APTR, a1: *mut APTR, a2: *const ()) -> *mut () { ((*IAmiSSL).ASN1_dup)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_item_dup(a0: *const APTR, a1: *const ()) -> *mut () { ((*IAmiSSL).ASN1_item_dup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_string_to_utf8(a0: *mut *mut u8, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_STRING_to_UTF8)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_d2i_bio(a0: APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut *mut ()) -> *mut () { ((*IAmiSSL).ASN1_d2i_bio)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_item_d2i_bio(a0: *const APTR, a1: *mut APTR, a2: *mut ()) -> *mut () { ((*IAmiSSL).ASN1_item_d2i_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_i2d_bio(a0: *mut APTR, a1: *mut APTR, a2: *const ()) -> i32 { ((*IAmiSSL).ASN1_i2d_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_item_i2d_bio(a0: *const APTR, a1: *mut APTR, a2: *const ()) -> i32 { ((*IAmiSSL).ASN1_item_i2d_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_utctime_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_UTCTIME_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_GENERALIZEDTIME_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_time_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_TIME_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_string_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_STRING_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_string_print_ex(a0: *mut APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).ASN1_STRING_print_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_parse(a0: *mut APTR, a1: *const u8, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).ASN1_parse)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_parse_dump(a0: *mut APTR, a1: *const u8, a2: i32, a3: i32, a4: i32) -> i32 { ((*IAmiSSL).ASN1_parse_dump)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_tag2str(a0: i32) -> *const APTR { ((*IAmiSSL).ASN1_tag2str)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_universalstring_to_string(a0: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_UNIVERSALSTRING_to_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_type_set_octetstring(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).ASN1_TYPE_set_octetstring)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_type_get_octetstring(a0: *const APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).ASN1_TYPE_get_octetstring)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_type_set_int_octetstring(a0: *mut APTR, a1: i32, a2: *mut u8, a3: i32) -> i32 { ((*IAmiSSL).ASN1_TYPE_set_int_octetstring)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_type_get_int_octetstring(a0: *const APTR, a1: *mut i32, a2: *mut u8, a3: i32) -> i32 { ((*IAmiSSL).ASN1_TYPE_get_int_octetstring)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_item_unpack(a0: *const APTR, a1: *const APTR) -> *mut () { ((*IAmiSSL).ASN1_item_unpack)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_item_pack(a0: *mut (), a1: *const APTR, a2: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).ASN1_item_pack)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_string_set_default_mask(a0: u32) { ((*IAmiSSL).ASN1_STRING_set_default_mask)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_string_set_default_mask_asc(a0: *const APTR) -> i32 { ((*IAmiSSL).ASN1_STRING_set_default_mask_asc)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_string_get_default_mask() -> u32 { ((*IAmiSSL).ASN1_STRING_get_default_mask)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_mbstring_copy(a0: *mut *mut APTR, a1: *const u8, a2: i32, a3: i32, a4: u32) -> i32 { ((*IAmiSSL).ASN1_mbstring_copy)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_mbstring_ncopy(a0: *mut *mut APTR, a1: *const u8, a2: i32, a3: i32, a4: u32, a5: i32, a6: i32) -> i32 { ((*IAmiSSL).ASN1_mbstring_ncopy)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_asn1_string_set_by_nid(a0: *mut *mut APTR, a1: *const u8, a2: i32, a3: i32, a4: i32) -> *mut APTR { ((*IAmiSSL).ASN1_STRING_set_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_string_table_get(a0: i32) -> *mut APTR { ((*IAmiSSL).ASN1_STRING_TABLE_get)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_string_table_add(a0: i32, a1: i32, a2: i32, a3: u32, a4: u32) -> i32 { ((*IAmiSSL).ASN1_STRING_TABLE_add)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_string_table_cleanup() { ((*IAmiSSL).ASN1_STRING_TABLE_cleanup)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_item_new(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_item_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_item_free(a0: *mut APTR, a1: *const APTR) { ((*IAmiSSL).ASN1_item_free)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_item_d2i(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_item_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_item_i2d(a0: *const APTR, a1: *mut *mut u8, a2: *const APTR) -> i32 { ((*IAmiSSL).ASN1_item_i2d)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_add_oid_module() { ((*IAmiSSL).ASN1_add_oid_module)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_load_asn1_strings() -> i32 { ((*IAmiSSL).ERR_load_ASN1_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_boolean_it() -> *const APTR { ((*IAmiSSL).ASN1_BOOLEAN_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_tboolean_it() -> *const APTR { ((*IAmiSSL).ASN1_TBOOLEAN_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_fboolean_it() -> *const APTR { ((*IAmiSSL).ASN1_FBOOLEAN_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_sequence_it() -> *const APTR { ((*IAmiSSL).ASN1_SEQUENCE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cbignum_it() -> *const APTR { ((*IAmiSSL).CBIGNUM_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bignum_it() -> *const APTR { ((*IAmiSSL).BIGNUM_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_long_it() -> *const APTR { ((*IAmiSSL).LONG_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_zlong_it() -> *const APTR { ((*IAmiSSL).ZLONG_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_item_ex_new(a0: *mut *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_item_ex_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_item_ex_free(a0: *mut *mut APTR, a1: *const APTR) { ((*IAmiSSL).ASN1_item_ex_free)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_item_ex_d2i(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32, a3: *const APTR, a4: i32, a5: i32, a6: APTR, a7: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_item_ex_d2i)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_asn1_item_ex_i2d(a0: *mut *mut APTR, a1: *mut *mut u8, a2: *const APTR, a3: i32, a4: i32) -> i32 { ((*IAmiSSL).ASN1_item_ex_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bio_ctrl_pending(a0: *mut APTR) -> u32 { ((*IAmiSSL).BIO_ctrl_pending)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_ctrl_wpending(a0: *mut APTR) -> u32 { ((*IAmiSSL).BIO_ctrl_wpending)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_ctrl_get_write_guarantee(a0: *mut APTR) -> u32 { ((*IAmiSSL).BIO_ctrl_get_write_guarantee)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_ctrl_get_read_request(a0: *mut APTR) -> u32 { ((*IAmiSSL).BIO_ctrl_get_read_request)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_ctrl_reset_read_request(a0: *mut APTR) -> i32 { ((*IAmiSSL).BIO_ctrl_reset_read_request)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).BIO_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).BIO_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_bio_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_BIO_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bio_number_read(a0: *mut APTR) -> APTR { ((*IAmiSSL).BIO_number_read)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_number_written(a0: *mut APTR) -> APTR { ((*IAmiSSL).BIO_number_written)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_s_file() -> *const APTR { ((*IAmiSSL).BIO_s_file)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_new_file(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new_fp_amiga(a0: u32, a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_new_fp_amiga)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_free(a0: *mut APTR) -> i32 { ((*IAmiSSL).BIO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_vfree(a0: *mut APTR) { ((*IAmiSSL).BIO_vfree)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_read(a0: *mut APTR, a1: *mut (), a2: i32) -> i32 { ((*IAmiSSL).BIO_read)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_gets(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).BIO_gets)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_write(a0: *mut APTR, a1: *const (), a2: i32) -> i32 { ((*IAmiSSL).BIO_write)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_puts(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BIO_puts)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_indent(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).BIO_indent)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).BIO_ctrl)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_callback_ctrl(a0: *mut APTR, a1: i32, a2: *mut APTR) -> i32 { ((*IAmiSSL).BIO_callback_ctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_ptr_ctrl(a0: *mut APTR, a1: i32, a2: i32) -> *mut () { ((*IAmiSSL).BIO_ptr_ctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_int_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).BIO_int_ctrl)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_push(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_push)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_pop(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_pop)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_free_all(a0: *mut APTR) { ((*IAmiSSL).BIO_free_all)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_find_type(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_find_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_next(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_next)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_get_retry_bio(a0: *mut APTR, a1: *mut i32) -> *mut APTR { ((*IAmiSSL).BIO_get_retry_BIO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_get_retry_reason(a0: *mut APTR) -> i32 { ((*IAmiSSL).BIO_get_retry_reason)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_dup_chain(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_dup_chain)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_nread0(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).BIO_nread0)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_nread(a0: *mut APTR, a1: *mut *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).BIO_nread)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_nwrite0(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).BIO_nwrite0)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_nwrite(a0: *mut APTR, a1: *mut *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).BIO_nwrite)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_debug_callback(a0: *mut APTR, a1: i32, a2: *const APTR, a3: i32, a4: i32, a5: i32) -> i32 { ((*IAmiSSL).BIO_debug_callback)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bio_s_mem() -> *const APTR { ((*IAmiSSL).BIO_s_mem)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_new_mem_buf(a0: *const (), a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_new_mem_buf)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_s_socket() -> *const APTR { ((*IAmiSSL).BIO_s_socket)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_s_connect() -> *const APTR { ((*IAmiSSL).BIO_s_connect)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_s_accept() -> *const APTR { ((*IAmiSSL).BIO_s_accept)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_s_fd() -> *const APTR { ((*IAmiSSL).BIO_s_fd)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_s_log() -> *const APTR { ((*IAmiSSL).BIO_s_log)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_s_bio() -> *const APTR { ((*IAmiSSL).BIO_s_bio)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_s_null() -> *const APTR { ((*IAmiSSL).BIO_s_null)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_null() -> *const APTR { ((*IAmiSSL).BIO_f_null)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_buffer() -> *const APTR { ((*IAmiSSL).BIO_f_buffer)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_nbio_test() -> *const APTR { ((*IAmiSSL).BIO_f_nbio_test)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_sock_should_retry(a0: i32) -> i32 { ((*IAmiSSL).BIO_sock_should_retry)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_sock_non_fatal_error(a0: i32) -> i32 { ((*IAmiSSL).BIO_sock_non_fatal_error)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_fd_should_retry(a0: i32) -> i32 { ((*IAmiSSL).BIO_fd_should_retry)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_fd_non_fatal_error(a0: i32) -> i32 { ((*IAmiSSL).BIO_fd_non_fatal_error)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_dump(a0: *mut APTR, a1: *const (), a2: i32) -> i32 { ((*IAmiSSL).BIO_dump)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_dump_indent(a0: *mut APTR, a1: *const (), a2: i32, a3: i32) -> i32 { ((*IAmiSSL).BIO_dump_indent)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_gethostbyname(a0: *const APTR) -> *mut hostent { ((*IAmiSSL).BIO_gethostbyname)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_sock_error(a0: i32) -> i32 { ((*IAmiSSL).BIO_sock_error)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_socket_ioctl(a0: i32, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).BIO_socket_ioctl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_socket_nbio(a0: i32, a1: i32) -> i32 { ((*IAmiSSL).BIO_socket_nbio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_get_port(a0: *const APTR, a1: *mut u16) -> i32 { ((*IAmiSSL).BIO_get_port)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_get_host_ip(a0: *const APTR, a1: *mut u8) -> i32 { ((*IAmiSSL).BIO_get_host_ip)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_get_accept_socket(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).BIO_get_accept_socket)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_accept(a0: i32, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).BIO_accept)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_sock_init() -> i32 { ((*IAmiSSL).BIO_sock_init)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_set_tcp_ndelay(a0: i32, a1: i32) -> i32 { ((*IAmiSSL).BIO_set_tcp_ndelay)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new_socket(a0: i32, a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_new_socket)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new_fd(a0: i32, a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_new_fd)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new_connect(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_connect)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_new_accept(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_accept)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_new_bio_pair(a0: *mut *mut APTR, a1: u32, a2: *mut *mut APTR, a3: u32) -> i32 { ((*IAmiSSL).BIO_new_bio_pair)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_copy_next_retry(a0: *mut APTR) { ((*IAmiSSL).BIO_copy_next_retry)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_vprintf(a0: *mut APTR, a1: *const APTR, a2: *mut i32) -> i32 { ((*IAmiSSL).BIO_vprintf)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_vsnprintf(a0: *mut APTR, a1: u32, a2: *const APTR, a3: *mut i32) -> i32 { ((*IAmiSSL).BIO_vsnprintf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_err_load_bio_strings() -> i32 { ((*IAmiSSL).ERR_load_BIO_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_value_one() -> *const APTR { ((*IAmiSSL).BN_value_one)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_options() -> *mut APTR { ((*IAmiSSL).BN_options)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_ctx_new() -> *mut APTR { ((*IAmiSSL).BN_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_ctx_free(a0: *mut APTR) { ((*IAmiSSL).BN_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_ctx_start(a0: *mut APTR) { ((*IAmiSSL).BN_CTX_start)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_ctx_get(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_CTX_get)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_ctx_end(a0: *mut APTR) { ((*IAmiSSL).BN_CTX_end)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_rand(a0: *mut APTR, a1: i32, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).BN_rand)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_pseudo_rand(a0: *mut APTR, a1: i32, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).BN_pseudo_rand)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_rand_range(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_rand_range)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_pseudo_rand_range(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_pseudo_rand_range)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_num_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).BN_num_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_num_bits_word(a0: APTR) -> i32 { ((*IAmiSSL).BN_num_bits_word)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_new() -> *mut APTR { ((*IAmiSSL).BN_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_clear_free(a0: *mut APTR) { ((*IAmiSSL).BN_clear_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_copy(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).BN_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_swap(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).BN_swap)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_bin2bn(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_bin2bn)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_bn2bin(a0: *const APTR, a1: *mut u8) -> i32 { ((*IAmiSSL).BN_bn2bin)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_mpi2bn(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_mpi2bn)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_bn2mpi(a0: *const APTR, a1: *mut u8) -> i32 { ((*IAmiSSL).BN_bn2mpi)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_sub(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).BN_sub)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_usub(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).BN_usub)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_uadd(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).BN_uadd)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_add(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).BN_add)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_mul(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_mul)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_sqr(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BN_sqr)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_div(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_div)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_nnmod(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_nnmod)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mod_add(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_add)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_mod_add_quick(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).BN_mod_add_quick)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mod_sub(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_sub)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_mod_sub_quick(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).BN_mod_sub_quick)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mod_mul(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_mul)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_mod_sqr(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_sqr)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mod_lshift1(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_lshift1)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mod_lshift1_quick(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).BN_mod_lshift1_quick)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_mod_lshift(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_lshift)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_mod_lshift_quick(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const APTR) -> i32 { ((*IAmiSSL).BN_mod_lshift_quick)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mod_word(a0: *const APTR, a1: APTR) -> APTR { ((*IAmiSSL).BN_mod_word)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_div_word(a0: *mut APTR, a1: APTR) -> APTR { ((*IAmiSSL).BN_div_word)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_mul_word(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BN_mul_word)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_add_word(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BN_add_word)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_sub_word(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BN_sub_word)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_set_word(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BN_set_word)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_get_word(a0: *const APTR) -> APTR { ((*IAmiSSL).BN_get_word)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_free(a0: *mut APTR) { ((*IAmiSSL).BN_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_is_bit_set(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).BN_is_bit_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_lshift(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).BN_lshift)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_lshift1(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_lshift1)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_exp(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_exp)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mod_exp(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_exp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_mod_exp_mont(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_exp_mont)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_mod_exp_mont_word(a0: *mut APTR, a1: APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_exp_mont_word)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_mod_exp2_mont(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: *const APTR, a6: *mut APTR, a7: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_exp2_mont)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_bn_mod_exp_simple(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_exp_simple)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_mask_bits(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).BN_mask_bits)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_reciprocal(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_reciprocal)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_rshift(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).BN_rshift)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_rshift1(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_rshift1)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_clear(a0: *mut APTR) { ((*IAmiSSL).BN_clear)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BN_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_ucmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_ucmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_set_bit(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).BN_set_bit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_clear_bit(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).BN_clear_bit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_bn2hex(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BN_bn2hex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_bn2dec(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BN_bn2dec)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_hex2bn(a0: *mut *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_hex2bn)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_dec2bn(a0: *mut *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_dec2bn)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_gcd(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_gcd)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_kronecker(a0: *const APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BN_kronecker)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_mod_inverse(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_mod_inverse)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mod_sqrt(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_mod_sqrt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_generate_prime(a0: *mut APTR, a1: i32, a2: i32, a3: *const APTR, a4: *const APTR, a5: APTR, a6: *mut ()) -> *mut APTR { ((*IAmiSSL).BN_generate_prime)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_bn_is_prime(a0: *const APTR, a1: i32, a2: APTR, a3: *mut APTR, a4: *mut ()) -> i32 { ((*IAmiSSL).BN_is_prime)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_is_prime_fasttest(a0: *const APTR, a1: i32, a2: APTR, a3: *mut APTR, a4: *mut (), a5: i32) -> i32 { ((*IAmiSSL).BN_is_prime_fasttest)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_mont_ctx_new() -> *mut APTR { ((*IAmiSSL).BN_MONT_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_mod_mul_montgomery(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_mul_montgomery)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_from_montgomery(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_from_montgomery)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_mont_ctx_free(a0: *mut APTR) { ((*IAmiSSL).BN_MONT_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_mont_ctx_set(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BN_MONT_CTX_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_mont_ctx_copy(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_MONT_CTX_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_blinding_new(a0: *const APTR, a1: *const APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_BLINDING_new)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_blinding_free(a0: *mut APTR) { ((*IAmiSSL).BN_BLINDING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_blinding_update(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).BN_BLINDING_update)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_blinding_convert(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BN_BLINDING_convert)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_blinding_invert(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BN_BLINDING_invert)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_set_params(a0: i32, a1: i32, a2: i32, a3: i32) { ((*IAmiSSL).BN_set_params)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_get_params(a0: i32) -> i32 { ((*IAmiSSL).BN_get_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_recp_ctx_new() -> *mut APTR { ((*IAmiSSL).BN_RECP_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_recp_ctx_free(a0: *mut APTR) { ((*IAmiSSL).BN_RECP_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_recp_ctx_set(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BN_RECP_CTX_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_mod_mul_reciprocal(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_mul_reciprocal)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_mod_exp_recp(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_exp_recp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_div_recp(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_div_recp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_bntest_rand(a0: *mut APTR, a1: i32, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).BN_bntest_rand)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_err_load_bn_strings() -> i32 { ((*IAmiSSL).ERR_load_BN_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_buf_mem_new() -> *mut APTR { ((*IAmiSSL).BUF_MEM_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_buf_mem_free(a0: *mut APTR) { ((*IAmiSSL).BUF_MEM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_buf_mem_grow(a0: *mut APTR, a1: u32) -> u32 { ((*IAmiSSL).BUF_MEM_grow)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_buf_mem_grow_clean(a0: *mut APTR, a1: u32) -> u32 { ((*IAmiSSL).BUF_MEM_grow_clean)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_buf_strdup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OBSOLETE_BUF_strdup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_buf_strlcpy(a0: *mut APTR, a1: *const APTR, a2: u32) -> u32 { ((*IAmiSSL).OBSOLETE_BUF_strlcpy)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obsolete_buf_strlcat(a0: *mut APTR, a1: *const APTR, a2: u32) -> u32 { ((*IAmiSSL).OBSOLETE_BUF_strlcat)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_err_load_buf_strings() -> i32 { ((*IAmiSSL).ERR_load_BUF_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_comp_ctx_new(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).COMP_CTX_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_comp_ctx_free(a0: *mut APTR) { ((*IAmiSSL).COMP_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_comp_compress_block(a0: *mut APTR, a1: *mut u8, a2: i32, a3: *mut u8, a4: i32) -> i32 { ((*IAmiSSL).COMP_compress_block)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_comp_expand_block(a0: *mut APTR, a1: *mut u8, a2: i32, a3: *mut u8, a4: i32) -> i32 { ((*IAmiSSL).COMP_expand_block)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_comp_zlib() -> *mut APTR { ((*IAmiSSL).COMP_zlib)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_load_comp_strings() -> i32 { ((*IAmiSSL).ERR_load_COMP_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_conf_set_default_method(a0: *mut APTR) -> i32 { ((*IAmiSSL).CONF_set_default_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_set_nconf(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).CONF_set_nconf)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_conf_load(a0: *mut APTR, a1: *const APTR, a2: *mut i32) -> *mut APTR { ((*IAmiSSL).CONF_load)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_conf_load_bio(a0: *mut APTR, a1: *mut APTR, a2: *mut i32) -> *mut APTR { ((*IAmiSSL).CONF_load_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_conf_get_section(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).CONF_get_section)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_conf_get_string(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).CONF_get_string)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_conf_get_number(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).CONF_get_number)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_conf_free(a0: *mut APTR) { ((*IAmiSSL).CONF_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_dump_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CONF_dump_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_config(a0: *const APTR) { ((*IAmiSSL).OPENSSL_config)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_openssl_no_config() { ((*IAmiSSL).OBSOLETE_OPENSSL_no_config)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_nconf_new(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).NCONF_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_nconf_default() -> *mut APTR { ((*IAmiSSL).NCONF_default)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_nconf_win32() -> *mut APTR { ((*IAmiSSL).NCONF_WIN32)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_nconf_free(a0: *mut APTR) { ((*IAmiSSL).NCONF_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_nconf_free_data(a0: *mut APTR) { ((*IAmiSSL).NCONF_free_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_nconf_load(a0: *mut APTR, a1: *const APTR, a2: *mut i32) -> i32 { ((*IAmiSSL).NCONF_load)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_nconf_load_bio(a0: *mut APTR, a1: *mut APTR, a2: *mut i32) -> i32 { ((*IAmiSSL).NCONF_load_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_nconf_get_section(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).NCONF_get_section)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_nconf_get_string(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).NCONF_get_string)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_nconf_get_number_e(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *mut i32) -> i32 { ((*IAmiSSL).NCONF_get_number_e)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_nconf_dump_bio(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).NCONF_dump_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_conf_modules_load(a0: *const APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).CONF_modules_load)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_conf_modules_load_file(a0: *const APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).CONF_modules_load_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_conf_modules_unload(a0: i32) { ((*IAmiSSL).CONF_modules_unload)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_modules_finish() { ((*IAmiSSL).CONF_modules_finish)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_conf_module_add(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).CONF_module_add)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_conf_imodule_get_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).CONF_imodule_get_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_imodule_get_value(a0: *const APTR) -> *const APTR { ((*IAmiSSL).CONF_imodule_get_value)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_imodule_get_usr_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).CONF_imodule_get_usr_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_imodule_set_usr_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).CONF_imodule_set_usr_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_conf_imodule_get_module(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).CONF_imodule_get_module)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_imodule_get_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).CONF_imodule_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_imodule_set_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).CONF_imodule_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_conf_module_get_usr_data(a0: *mut APTR) -> *mut () { ((*IAmiSSL).CONF_module_get_usr_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_conf_module_set_usr_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).CONF_module_set_usr_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_conf_get1_default_config_file() -> *mut APTR { ((*IAmiSSL).CONF_get1_default_config_file)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_conf_parse_list(a0: *const APTR, a1: i32, a2: i32, a3: APTR, a4: *mut ()) -> i32 { ((*IAmiSSL).CONF_parse_list)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_openssl_load_builtin_modules() { ((*IAmiSSL).OPENSSL_load_builtin_modules)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_load_conf_strings() -> i32 { ((*IAmiSSL).ERR_load_CONF_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_ssleay_version(a0: i32) -> *const APTR { ((*IAmiSSL).OBSOLETE_SSLeay_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_ssleay() -> u32 { ((*IAmiSSL).OBSOLETE_SSLeay)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_issetugid() -> i32 { ((*IAmiSSL).OPENSSL_issetugid)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crypto_get_ex_new_index(a0: i32, a1: i32, a2: *mut (), a3: *mut APTR, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_new_ex_data(a0: i32, a1: *mut (), a2: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_new_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_dup_ex_data(a0: i32, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).CRYPTO_dup_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_free_ex_data(a0: i32, a1: *mut (), a2: *mut APTR) { ((*IAmiSSL).CRYPTO_free_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).CRYPTO_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).CRYPTO_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_set_mem_functions(a0: APTR, a1: APTR, a2: APTR) -> i32 { ((*IAmiSSL).CRYPTO_set_mem_functions)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_get_mem_functions(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) { ((*IAmiSSL).CRYPTO_get_mem_functions)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_malloc(a0: u32, a1: *const APTR, a2: i32) -> *mut () { ((*IAmiSSL).CRYPTO_malloc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_free(a0: *mut (), a1: *const APTR, a2: i32) { ((*IAmiSSL).CRYPTO_free)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_realloc(a0: *mut (), a1: u32, a2: *const APTR, a3: i32) -> *mut () { ((*IAmiSSL).CRYPTO_realloc)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_openssl_cleanse(a0: *mut (), a1: u32) { ((*IAmiSSL).OPENSSL_cleanse)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_load_crypto_strings() -> i32 { ((*IAmiSSL).ERR_load_CRYPTO_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_gfp_simple_method() -> *const APTR { ((*IAmiSSL).EC_GFp_simple_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_gfp_mont_method() -> *const APTR { ((*IAmiSSL).EC_GFp_mont_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_group_new(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_free(a0: *mut APTR) { ((*IAmiSSL).EC_GROUP_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_clear_free(a0: *mut APTR) { ((*IAmiSSL).EC_GROUP_clear_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_method_of(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_GROUP_method_of)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_set_curve_gfp(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_set_curve_GFp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_group_get_curve_gfp(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_curve_GFp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_group_new_curve_gfp(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_new_curve_GFp)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_group_set_generator(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_set_generator)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_group_get0_generator(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_GROUP_get0_generator)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get_order(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_order)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_group_get_cofactor(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_cofactor)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_point_new(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_POINT_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_point_free(a0: *mut APTR) { ((*IAmiSSL).EC_POINT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_point_clear_free(a0: *mut APTR) { ((*IAmiSSL).EC_POINT_clear_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_point_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EC_POINT_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_point_method_of(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_POINT_method_of)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_point_set_to_infinity(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_set_to_infinity)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_point_set_jprojective_coordinates_gfp(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_set_Jprojective_coordinates_GFp)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ec_point_get_jprojective_coordinates_gfp(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_get_Jprojective_coordinates_GFp)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ec_point_set_affine_coordinates_gfp(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_set_affine_coordinates_GFp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_get_affine_coordinates_gfp(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_get_affine_coordinates_GFp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_set_compressed_coordinates_gfp(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_set_compressed_coordinates_GFp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_point2oct(a0: *const APTR, a1: *const APTR, a2: APTR, a3: *mut u8, a4: u32, a5: *mut APTR) -> u32 { ((*IAmiSSL).EC_POINT_point2oct)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ec_point_oct2point(a0: *const APTR, a1: *mut APTR, a2: *const u8, a3: u32, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_oct2point)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_add(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_add)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_dbl(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_dbl)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_point_invert(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_invert)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_point_is_at_infinity(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EC_POINT_is_at_infinity)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_point_is_on_curve(a0: *const APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_is_on_curve)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_point_cmp(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_cmp)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_point_make_affine(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_make_affine)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_points_make_affine(a0: *const APTR, a1: u32, a2: *mut *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINTs_make_affine)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_points_mul(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: u32, a4: *mut *mut APTR, a5: *mut *mut APTR, a6: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINTs_mul)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ec_point_mul(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_mul)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ec_group_precompute_mult(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_precompute_mult)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_load_ec_strings() -> i32 { ((*IAmiSSL).ERR_load_EC_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_err_put_error(a0: i32, a1: i32, a2: i32, a3: *const APTR, a4: i32) { ((*IAmiSSL).OBSOLETE_ERR_put_error)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_err_set_error_data(a0: *mut APTR, a1: i32) { ((*IAmiSSL).ERR_set_error_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_get_error() -> u32 { ((*IAmiSSL).ERR_get_error)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_get_error_line(a0: *mut *mut APTR, a1: *mut i32) -> u32 { ((*IAmiSSL).ERR_get_error_line)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_get_error_line_data(a0: *mut *mut APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut i32) -> u32 { ((*IAmiSSL).ERR_get_error_line_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_err_peek_error() -> u32 { ((*IAmiSSL).ERR_peek_error)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_peek_error_line(a0: *mut *mut APTR, a1: *mut i32) -> u32 { ((*IAmiSSL).ERR_peek_error_line)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_peek_error_line_data(a0: *mut *mut APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut i32) -> u32 { ((*IAmiSSL).ERR_peek_error_line_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_err_peek_last_error() -> u32 { ((*IAmiSSL).ERR_peek_last_error)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_peek_last_error_line(a0: *mut *mut APTR, a1: *mut i32) -> u32 { ((*IAmiSSL).ERR_peek_last_error_line)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_peek_last_error_line_data(a0: *mut *mut APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut i32) -> u32 { ((*IAmiSSL).ERR_peek_last_error_line_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_err_clear_error() { ((*IAmiSSL).ERR_clear_error)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_error_string(a0: u32, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).ERR_error_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_error_string_n(a0: u32, a1: *mut APTR, a2: u32) { ((*IAmiSSL).ERR_error_string_n)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_err_lib_error_string(a0: u32) -> *const APTR { ((*IAmiSSL).ERR_lib_error_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_func_error_string(a0: u32) -> *const APTR { ((*IAmiSSL).ERR_func_error_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_reason_error_string(a0: u32) -> *const APTR { ((*IAmiSSL).ERR_reason_error_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_print_errors_cb(a0: APTR, a1: *mut ()) { ((*IAmiSSL).ERR_print_errors_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_print_errors(a0: *mut APTR) { ((*IAmiSSL).ERR_print_errors)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_add_error_vdata(a0: i32, a1: *mut i32) { ((*IAmiSSL).ERR_add_error_vdata)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_load_strings(a0: i32, a1: *mut APTR) -> i32 { ((*IAmiSSL).ERR_load_strings)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_unload_strings(a0: i32, a1: *mut APTR) -> i32 { ((*IAmiSSL).ERR_unload_strings)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_load_err_strings() -> i32 { ((*IAmiSSL).ERR_load_ERR_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_err_load_crypto_strings() { ((*IAmiSSL).OBSOLETE_ERR_load_crypto_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_get_state() -> *mut APTR { ((*IAmiSSL).ERR_get_state)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_get_next_error_library() -> i32 { ((*IAmiSSL).ERR_get_next_error_library)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_evp_md_ctx_init(a0: *mut APTR) { ((*IAmiSSL).OBSOLETE_EVP_MD_CTX_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_evp_md_ctx_create() -> *mut APTR { ((*IAmiSSL).OBSOLETE_EVP_MD_CTX_create)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_evp_md_ctx_destroy(a0: *mut APTR) { ((*IAmiSSL).OBSOLETE_EVP_MD_CTX_destroy)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_copy_ex(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_CTX_copy_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_digest_init_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).EVP_DigestInit_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_digest_update(a0: *mut APTR, a1: *const (), a2: APTR) -> i32 { ((*IAmiSSL).EVP_DigestUpdate)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_digest_final_ex(a0: *mut APTR, a1: *mut u8, a2: *mut APTR) -> i32 { ((*IAmiSSL).EVP_DigestFinal_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_digest(a0: *const (), a1: u32, a2: *mut u8, a3: *mut APTR, a4: *const APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).EVP_Digest)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_CTX_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_digest_init(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_DigestInit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_digest_final(a0: *mut APTR, a1: *mut u8, a2: *mut APTR) -> i32 { ((*IAmiSSL).EVP_DigestFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_read_pw_string(a0: *mut APTR, a1: i32, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).EVP_read_pw_string)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_set_pw_prompt(a0: *const APTR) { ((*IAmiSSL).EVP_set_pw_prompt)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_get_pw_prompt() -> *mut APTR { ((*IAmiSSL).EVP_get_pw_prompt)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_bytes_to_key(a0: *const APTR, a1: *const APTR, a2: *const u8, a3: *const u8, a4: i32, a5: i32, a6: *mut u8, a7: *mut u8) -> i32 { ((*IAmiSSL).EVP_BytesToKey)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_evp_encrypt_init(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: *const u8) -> i32 { ((*IAmiSSL).EVP_EncryptInit)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_encrypt_init_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const u8, a4: *const u8) -> i32 { ((*IAmiSSL).EVP_EncryptInit_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_encrypt_update(a0: *mut APTR, a1: *mut u8, a2: *mut i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_EncryptUpdate)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_encrypt_final_ex(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_EncryptFinal_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_encrypt_final(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_EncryptFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_decrypt_init(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: *const u8) -> i32 { ((*IAmiSSL).EVP_DecryptInit)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_decrypt_init_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const u8, a4: *const u8) -> i32 { ((*IAmiSSL).EVP_DecryptInit_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_decrypt_update(a0: *mut APTR, a1: *mut u8, a2: *mut i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_DecryptUpdate)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_decrypt_final(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_DecryptFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_decrypt_final_ex(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_DecryptFinal_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_cipher_init(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_CipherInit)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_cipher_init_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const u8, a4: *const u8, a5: i32) -> i32 { ((*IAmiSSL).EVP_CipherInit_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_cipher_update(a0: *mut APTR, a1: *mut u8, a2: *mut i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_CipherUpdate)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_cipher_final(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_CipherFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_cipher_final_ex(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_CipherFinal_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_sign_final(a0: *mut APTR, a1: *mut u8, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_SignFinal)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_verify_final(a0: *mut APTR, a1: *const u8, a2: APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_VerifyFinal)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_open_init(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: i32, a4: *const u8, a5: *mut APTR) -> i32 { ((*IAmiSSL).EVP_OpenInit)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_open_final(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_OpenFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_seal_init(a0: *mut APTR, a1: *const APTR, a2: *mut *mut u8, a3: *mut i32, a4: *mut u8, a5: *mut *mut APTR, a6: i32) -> i32 { ((*IAmiSSL).EVP_SealInit)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_seal_final(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_SealFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_encode_init(a0: *mut APTR) { ((*IAmiSSL).EVP_EncodeInit)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_encode_update(a0: *mut APTR, a1: *mut u8, a2: *mut i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_EncodeUpdate)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_encode_final(a0: *mut APTR, a1: *mut u8, a2: *mut i32) { ((*IAmiSSL).EVP_EncodeFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_encode_block(a0: *mut u8, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_EncodeBlock)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_decode_init(a0: *mut APTR) { ((*IAmiSSL).EVP_DecodeInit)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_decode_update(a0: *mut APTR, a1: *mut u8, a2: *mut i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_DecodeUpdate)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_decode_final(a0: *mut APTR, a1: *mut u8, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_DecodeFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_decode_block(a0: *mut u8, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_DecodeBlock)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obsolete_evp_cipher_ctx_init(a0: *mut APTR) { ((*IAmiSSL).OBSOLETE_EVP_CIPHER_CTX_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_evp_cipher_ctx_cleanup(a0: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_EVP_CIPHER_CTX_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_set_key_length(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_set_key_length)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_set_padding(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_set_padding)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_ctrl)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_f_md() -> *const APTR { ((*IAmiSSL).BIO_f_md)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_base64() -> *const APTR { ((*IAmiSSL).BIO_f_base64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_cipher() -> *const APTR { ((*IAmiSSL).BIO_f_cipher)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_reliable() -> *const APTR { ((*IAmiSSL).BIO_f_reliable)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_set_cipher(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).BIO_set_cipher)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_md_null() -> *const APTR { ((*IAmiSSL).EVP_md_null)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_md2() -> *const APTR { ((*IAmiSSL).EVP_md2)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_md4() -> *const APTR { ((*IAmiSSL).EVP_md4)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_md5() -> *const APTR { ((*IAmiSSL).EVP_md5)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha1() -> *const APTR { ((*IAmiSSL).EVP_sha1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_mdc2() -> *const APTR { ((*IAmiSSL).EVP_mdc2)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_ripemd160() -> *const APTR { ((*IAmiSSL).EVP_ripemd160)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_enc_null() -> *const APTR { ((*IAmiSSL).EVP_enc_null)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ecb() -> *const APTR { ((*IAmiSSL).EVP_des_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede() -> *const APTR { ((*IAmiSSL).EVP_des_ede)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede3() -> *const APTR { ((*IAmiSSL).EVP_des_ede3)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede_ecb() -> *const APTR { ((*IAmiSSL).EVP_des_ede_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede3_ecb() -> *const APTR { ((*IAmiSSL).EVP_des_ede3_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_cfb64() -> *const APTR { ((*IAmiSSL).EVP_des_cfb64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_cfb1() -> *const APTR { ((*IAmiSSL).EVP_des_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_cfb8() -> *const APTR { ((*IAmiSSL).EVP_des_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede_cfb64() -> *const APTR { ((*IAmiSSL).EVP_des_ede_cfb64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede3_cfb64() -> *const APTR { ((*IAmiSSL).EVP_des_ede3_cfb64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede3_cfb1() -> *const APTR { ((*IAmiSSL).EVP_des_ede3_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede3_cfb8() -> *const APTR { ((*IAmiSSL).EVP_des_ede3_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ofb() -> *const APTR { ((*IAmiSSL).EVP_des_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede_ofb() -> *const APTR { ((*IAmiSSL).EVP_des_ede_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede3_ofb() -> *const APTR { ((*IAmiSSL).EVP_des_ede3_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_cbc() -> *const APTR { ((*IAmiSSL).EVP_des_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede_cbc() -> *const APTR { ((*IAmiSSL).EVP_des_ede_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede3_cbc() -> *const APTR { ((*IAmiSSL).EVP_des_ede3_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_desx_cbc() -> *const APTR { ((*IAmiSSL).EVP_desx_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc4() -> *const APTR { ((*IAmiSSL).EVP_rc4)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc4_40() -> *const APTR { ((*IAmiSSL).EVP_rc4_40)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_idea_ecb() -> *const APTR { ((*IAmiSSL).EVP_idea_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_idea_cfb64() -> *const APTR { ((*IAmiSSL).EVP_idea_cfb64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_idea_ofb() -> *const APTR { ((*IAmiSSL).EVP_idea_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_idea_cbc() -> *const APTR { ((*IAmiSSL).EVP_idea_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc2_ecb() -> *const APTR { ((*IAmiSSL).EVP_rc2_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc2_cbc() -> *const APTR { ((*IAmiSSL).EVP_rc2_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc2_40_cbc() -> *const APTR { ((*IAmiSSL).EVP_rc2_40_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc2_64_cbc() -> *const APTR { ((*IAmiSSL).EVP_rc2_64_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc2_cfb64() -> *const APTR { ((*IAmiSSL).EVP_rc2_cfb64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc2_ofb() -> *const APTR { ((*IAmiSSL).EVP_rc2_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_bf_ecb() -> *const APTR { ((*IAmiSSL).EVP_bf_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_bf_cbc() -> *const APTR { ((*IAmiSSL).EVP_bf_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_bf_cfb64() -> *const APTR { ((*IAmiSSL).EVP_bf_cfb64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_bf_ofb() -> *const APTR { ((*IAmiSSL).EVP_bf_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cast5_ecb() -> *const APTR { ((*IAmiSSL).EVP_cast5_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cast5_cbc() -> *const APTR { ((*IAmiSSL).EVP_cast5_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cast5_cfb64() -> *const APTR { ((*IAmiSSL).EVP_cast5_cfb64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cast5_ofb() -> *const APTR { ((*IAmiSSL).EVP_cast5_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc5_32_12_16_cbc() -> *const APTR { ((*IAmiSSL).EVP_rc5_32_12_16_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc5_32_12_16_ecb() -> *const APTR { ((*IAmiSSL).EVP_rc5_32_12_16_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc5_32_12_16_cfb64() -> *const APTR { ((*IAmiSSL).EVP_rc5_32_12_16_cfb64)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc5_32_12_16_ofb() -> *const APTR { ((*IAmiSSL).EVP_rc5_32_12_16_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_ecb() -> *const APTR { ((*IAmiSSL).EVP_aes_128_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_cbc() -> *const APTR { ((*IAmiSSL).EVP_aes_128_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_cfb1() -> *const APTR { ((*IAmiSSL).EVP_aes_128_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_cfb8() -> *const APTR { ((*IAmiSSL).EVP_aes_128_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_cfb128() -> *const APTR { ((*IAmiSSL).EVP_aes_128_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_ofb() -> *const APTR { ((*IAmiSSL).EVP_aes_128_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_ecb() -> *const APTR { ((*IAmiSSL).EVP_aes_192_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_cbc() -> *const APTR { ((*IAmiSSL).EVP_aes_192_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_cfb1() -> *const APTR { ((*IAmiSSL).EVP_aes_192_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_cfb8() -> *const APTR { ((*IAmiSSL).EVP_aes_192_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_cfb128() -> *const APTR { ((*IAmiSSL).EVP_aes_192_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_ofb() -> *const APTR { ((*IAmiSSL).EVP_aes_192_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_ecb() -> *const APTR { ((*IAmiSSL).EVP_aes_256_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_cbc() -> *const APTR { ((*IAmiSSL).EVP_aes_256_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_cfb1() -> *const APTR { ((*IAmiSSL).EVP_aes_256_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_cfb8() -> *const APTR { ((*IAmiSSL).EVP_aes_256_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_cfb128() -> *const APTR { ((*IAmiSSL).EVP_aes_256_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_ofb() -> *const APTR { ((*IAmiSSL).EVP_aes_256_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_openssl_add_all_algorithms_noconf() { ((*IAmiSSL).OBSOLETE_OPENSSL_add_all_algorithms_noconf)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_openssl_add_all_algorithms_conf() { ((*IAmiSSL).OBSOLETE_OPENSSL_add_all_algorithms_conf)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_open_ssl_add_all_ciphers() { ((*IAmiSSL).OBSOLETE_OpenSSL_add_all_ciphers)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_open_ssl_add_all_digests() { ((*IAmiSSL).OBSOLETE_OpenSSL_add_all_digests)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_add_cipher(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_add_cipher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_add_digest(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_add_digest)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_get_cipherbyname(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_get_cipherbyname)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_get_digestbyname(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_get_digestbyname)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_decrypt(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_decrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_encrypt(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_encrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_type(a0: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_size(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_assign(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_PKEY_assign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_set1_rsa(a0: *mut APTR, a1: *mut rsa_st) -> i32 { ((*IAmiSSL).EVP_PKEY_set1_RSA)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get1_rsa(a0: *mut APTR) -> *mut rsa_st { ((*IAmiSSL).EVP_PKEY_get1_RSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_set1_dsa(a0: *mut APTR, a1: *mut dsa_st) -> i32 { ((*IAmiSSL).EVP_PKEY_set1_DSA)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get1_dsa(a0: *mut APTR) -> *mut dsa_st { ((*IAmiSSL).EVP_PKEY_get1_DSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_set1_dh(a0: *mut APTR, a1: *mut dh_st) -> i32 { ((*IAmiSSL).EVP_PKEY_set1_DH)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get1_dh(a0: *mut APTR) -> *mut dh_st { ((*IAmiSSL).EVP_PKEY_get1_DH)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_new() -> *mut APTR { ((*IAmiSSL).EVP_PKEY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkey_free(a0: *mut APTR) { ((*IAmiSSL).EVP_PKEY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_public_key(a0: i32, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).d2i_PublicKey)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_i2d_public_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PublicKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_private_key(a0: i32, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).d2i_PrivateKey)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_d2i_auto_private_key(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_AutoPrivateKey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_private_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_copy_parameters(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_copy_parameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_missing_parameters(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_missing_parameters)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_save_parameters(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_save_parameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_cmp_parameters(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_cmp_parameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_type(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_get_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_param_to_asn1(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_param_to_asn1)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_asn1_to_param(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_asn1_to_param)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_set_asn1_iv(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_set_asn1_iv)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_asn1_iv(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_get_asn1_iv)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe_keyivgen(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *const APTR, a6: i32) -> i32 { ((*IAmiSSL).PKCS5_PBE_keyivgen)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs5_pbkdf2_hmac_sha1(a0: *const APTR, a1: i32, a2: *const u8, a3: i32, a4: i32, a5: i32, a6: *mut u8) -> i32 { ((*IAmiSSL).PKCS5_PBKDF2_HMAC_SHA1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs5_v2_pbe_keyivgen(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *const APTR, a6: i32) -> i32 { ((*IAmiSSL).PKCS5_v2_PBE_keyivgen)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe_add() { ((*IAmiSSL).PKCS5_PBE_add)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pbe_cipher_init(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: i32) -> i32 { ((*IAmiSSL).EVP_PBE_CipherInit)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_pbe_alg_add(a0: i32, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PBE_alg_add)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pbe_cleanup() { ((*IAmiSSL).EVP_PBE_cleanup)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_load_evp_strings() -> i32 { ((*IAmiSSL).ERR_load_EVP_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_hmac_init(a0: *mut APTR, a1: *const (), a2: i32, a3: *const APTR) -> i32 { ((*IAmiSSL).HMAC_Init)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_hmac_init_ex(a0: *mut APTR, a1: *const (), a2: i32, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).HMAC_Init_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_hmac_update(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).HMAC_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_hmac_final(a0: *mut APTR, a1: *mut u8, a2: *mut APTR) -> i32 { ((*IAmiSSL).HMAC_Final)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_hmac(a0: *const APTR, a1: *const (), a2: i32, a3: *const u8, a4: u32, a5: *mut u8, a6: *mut APTR) -> *mut u8 { ((*IAmiSSL).HMAC)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_openssl_lh_new(a0: APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).OPENSSL_LH_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_lh_free(a0: *mut APTR) { ((*IAmiSSL).OPENSSL_LH_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_lh_insert(a0: *mut APTR, a1: *mut ()) -> *mut () { ((*IAmiSSL).OPENSSL_LH_insert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_lh_delete(a0: *mut APTR, a1: *const ()) -> *mut () { ((*IAmiSSL).OPENSSL_LH_delete)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_lh_retrieve(a0: *mut APTR, a1: *const ()) -> *mut () { ((*IAmiSSL).OPENSSL_LH_retrieve)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_lh_doall(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).OPENSSL_LH_doall)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_lh_doall_arg(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).OPENSSL_LH_doall_arg)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_lh_strhash(a0: *const APTR) -> u32 { ((*IAmiSSL).OPENSSL_LH_strhash)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_lh_num_items(a0: *const APTR) -> u32 { ((*IAmiSSL).OPENSSL_LH_num_items)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_lh_stats_bio(a0: *const APTR, a1: *mut APTR) { ((*IAmiSSL).OPENSSL_LH_stats_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_lh_node_stats_bio(a0: *const APTR, a1: *mut APTR) { ((*IAmiSSL).OPENSSL_LH_node_stats_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_lh_node_usage_stats_bio(a0: *const APTR, a1: *mut APTR) { ((*IAmiSSL).OPENSSL_LH_node_usage_stats_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obj_name_init() -> i32 { ((*IAmiSSL).OBJ_NAME_init)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obj_name_new_index(a0: APTR, a1: APTR, a2: APTR) -> i32 { ((*IAmiSSL).OBJ_NAME_new_index)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obj_name_get(a0: *const APTR, a1: i32) -> *const APTR { ((*IAmiSSL).OBJ_NAME_get)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obj_name_add(a0: *const APTR, a1: i32, a2: *const APTR) -> i32 { ((*IAmiSSL).OBJ_NAME_add)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obj_name_remove(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).OBJ_NAME_remove)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obj_name_cleanup(a0: i32) { ((*IAmiSSL).OBJ_NAME_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_name_do_all(a0: i32, a1: APTR, a2: *mut ()) { ((*IAmiSSL).OBJ_NAME_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obj_name_do_all_sorted(a0: i32, a1: APTR, a2: *mut ()) { ((*IAmiSSL).OBJ_NAME_do_all_sorted)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obj_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OBJ_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_nid2obj(a0: i32) -> *mut APTR { ((*IAmiSSL).OBJ_nid2obj)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_nid2ln(a0: i32) -> *const APTR { ((*IAmiSSL).OBJ_nid2ln)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_nid2sn(a0: i32) -> *const APTR { ((*IAmiSSL).OBJ_nid2sn)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_obj2nid(a0: *const APTR) -> i32 { ((*IAmiSSL).OBJ_obj2nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_txt2obj(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OBJ_txt2obj)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obj_obj2txt(a0: *mut APTR, a1: i32, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).OBJ_obj2txt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_obj_txt2nid(a0: *const APTR) -> i32 { ((*IAmiSSL).OBJ_txt2nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_ln2nid(a0: *const APTR) -> i32 { ((*IAmiSSL).OBJ_ln2nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_sn2nid(a0: *const APTR) -> i32 { ((*IAmiSSL).OBJ_sn2nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OBJ_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obj_new_nid(a0: i32) -> i32 { ((*IAmiSSL).OBJ_new_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_add_object(a0: *const APTR) -> i32 { ((*IAmiSSL).OBJ_add_object)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_create(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OBJ_create)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obj_create_objects(a0: *mut APTR) -> i32 { ((*IAmiSSL).OBJ_create_objects)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_obj_strings() -> i32 { ((*IAmiSSL).ERR_load_OBJ_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_sendreq_bio(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_sendreq_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_cert_to_id(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OCSP_cert_to_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_cert_id_new(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).OCSP_cert_id_new)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ocsp_request_add0_id(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_request_add0_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_add1_nonce(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).OCSP_request_add1_nonce)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_basic_add1_nonce(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).OCSP_basic_add1_nonce)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_check_nonce(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_check_nonce)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_copy_nonce(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_copy_nonce)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_set1_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OCSP_request_set1_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_add1_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_request_add1_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_sign(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR, a4: *mut APTR, a5: u32) -> i32 { ((*IAmiSSL).OCSP_request_sign)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ocsp_response_status(a0: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_response_status)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_response_get1_basic(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_response_get1_basic)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_resp_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_resp_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get0(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_resp_get0)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_resp_find(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_resp_find)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_single_get0_status(a0: *mut APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR) -> i32 { ((*IAmiSSL).OCSP_single_get0_status)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ocsp_resp_find_status(a0: *mut APTR, a1: *mut APTR, a2: *mut i32, a3: *mut i32, a4: *mut *mut APTR, a5: *mut *mut APTR, a6: *mut *mut APTR) -> i32 { ((*IAmiSSL).OCSP_resp_find_status)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ocsp_check_validity(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).OCSP_check_validity)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ocsp_request_verify(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: u32) -> i32 { ((*IAmiSSL).OCSP_request_verify)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_obsolete_ocsp_parse_url(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut i32) -> i32 { ((*IAmiSSL).OBSOLETE_OCSP_parse_url)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ocsp_id_issuer_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OCSP_id_issuer_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_id_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OCSP_id_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_onereq_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_request_onereq_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_request_onereq_get0(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_request_onereq_get0)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_get0_id(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_onereq_get0_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_id_get0_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_id_get0_info)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ocsp_request_is_signed(a0: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_request_is_signed)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_response_create(a0: i32, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_response_create)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_basic_add1_status(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: i32, a4: *mut APTR, a5: *mut APTR, a6: *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_basic_add1_status)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ocsp_basic_add1_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_basic_add1_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_basic_sign(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR, a4: *mut APTR, a5: u32) -> i32 { ((*IAmiSSL).OCSP_basic_sign)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ocsp_crl_id_new(a0: *const APTR, a1: *mut i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_crlID_new)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_accept_responses_new(a0: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_accept_responses_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_archive_cutoff_new(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_archive_cutoff_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_url_svcloc_new(a0: *const APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).OCSP_url_svcloc_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_get_ext_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_REQUEST_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_request_get_ext_by_nid(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OCSP_REQUEST_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_request_get_ext_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_REQUEST_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_request_get_ext_by_critical(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OCSP_REQUEST_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_request_get_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_REQUEST_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_REQUEST_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_get1_ext_d2i(a0: *mut APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).OCSP_REQUEST_get1_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ocsp_request_add1_ext_i2d(a0: *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).OCSP_REQUEST_add1_ext_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ocsp_request_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_REQUEST_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_get_ext_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_ONEREQ_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_get_ext_by_nid(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OCSP_ONEREQ_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_get_ext_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_ONEREQ_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_get_ext_by_critical(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OCSP_ONEREQ_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_get_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_ONEREQ_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_ONEREQ_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_get1_ext_d2i(a0: *mut APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).OCSP_ONEREQ_get1_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_add1_ext_i2d(a0: *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).OCSP_ONEREQ_add1_ext_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_ONEREQ_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_get_ext_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_BASICRESP_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_get_ext_by_nid(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OCSP_BASICRESP_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_get_ext_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_BASICRESP_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_get_ext_by_critical(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OCSP_BASICRESP_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_get_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_BASICRESP_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_BASICRESP_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_get1_ext_d2i(a0: *mut APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).OCSP_BASICRESP_get1_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_add1_ext_i2d(a0: *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).OCSP_BASICRESP_add1_ext_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_BASICRESP_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_get_ext_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_SINGLERESP_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_get_ext_by_nid(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OCSP_SINGLERESP_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_get_ext_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_SINGLERESP_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_get_ext_by_critical(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OCSP_SINGLERESP_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_get_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_SINGLERESP_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OCSP_SINGLERESP_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_get1_ext_d2i(a0: *mut APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).OCSP_SINGLERESP_get1_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_add1_ext_i2d(a0: *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).OCSP_SINGLERESP_add1_ext_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).OCSP_SINGLERESP_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_new() -> *mut APTR { ((*IAmiSSL).OCSP_SINGLERESP_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_SINGLERESP_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_singleresp(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_SINGLERESP)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_singleresp(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_SINGLERESP)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_it() -> *const APTR { ((*IAmiSSL).OCSP_SINGLERESP_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_certstatus_new() -> *mut APTR { ((*IAmiSSL).OCSP_CERTSTATUS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_certstatus_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_CERTSTATUS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_certstatus(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_CERTSTATUS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_certstatus(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_CERTSTATUS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_certstatus_it() -> *const APTR { ((*IAmiSSL).OCSP_CERTSTATUS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_revokedinfo_new() -> *mut APTR { ((*IAmiSSL).OCSP_REVOKEDINFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_revokedinfo_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_REVOKEDINFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_revokedinfo(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_REVOKEDINFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_revokedinfo(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_REVOKEDINFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_revokedinfo_it() -> *const APTR { ((*IAmiSSL).OCSP_REVOKEDINFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_new() -> *mut APTR { ((*IAmiSSL).OCSP_BASICRESP_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_BASICRESP_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_basicresp(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_BASICRESP)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_basicresp(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_BASICRESP)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_basicresp_it() -> *const APTR { ((*IAmiSSL).OCSP_BASICRESP_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_respdata_new() -> *mut APTR { ((*IAmiSSL).OCSP_RESPDATA_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_respdata_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_RESPDATA_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_respdata(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_RESPDATA)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_respdata(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_RESPDATA)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_respdata_it() -> *const APTR { ((*IAmiSSL).OCSP_RESPDATA_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_respid_new() -> *mut APTR { ((*IAmiSSL).OCSP_RESPID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_respid_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_RESPID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_respid(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_RESPID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_respid(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_RESPID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_respid_it() -> *const APTR { ((*IAmiSSL).OCSP_RESPID_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_response_new() -> *mut APTR { ((*IAmiSSL).OCSP_RESPONSE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_response_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_RESPONSE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_response(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_RESPONSE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_response(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_RESPONSE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_response_it() -> *const APTR { ((*IAmiSSL).OCSP_RESPONSE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_respbytes_new() -> *mut APTR { ((*IAmiSSL).OCSP_RESPBYTES_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_respbytes_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_RESPBYTES_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_respbytes(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_RESPBYTES)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_respbytes(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_RESPBYTES)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_respbytes_it() -> *const APTR { ((*IAmiSSL).OCSP_RESPBYTES_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_new() -> *mut APTR { ((*IAmiSSL).OCSP_ONEREQ_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_ONEREQ_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_onereq(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_ONEREQ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_onereq(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_ONEREQ)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_onereq_it() -> *const APTR { ((*IAmiSSL).OCSP_ONEREQ_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_certid_new() -> *mut APTR { ((*IAmiSSL).OCSP_CERTID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_certid_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_CERTID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_certid(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_CERTID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_certid(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_CERTID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_certid_it() -> *const APTR { ((*IAmiSSL).OCSP_CERTID_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_request_new() -> *mut APTR { ((*IAmiSSL).OCSP_REQUEST_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_request_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_REQUEST_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_request(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_REQUEST)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_request(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_REQUEST)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_request_it() -> *const APTR { ((*IAmiSSL).OCSP_REQUEST_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_signature_new() -> *mut APTR { ((*IAmiSSL).OCSP_SIGNATURE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_signature_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_SIGNATURE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_signature(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_SIGNATURE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_signature(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_SIGNATURE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_signature_it() -> *const APTR { ((*IAmiSSL).OCSP_SIGNATURE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_reqinfo_new() -> *mut APTR { ((*IAmiSSL).OCSP_REQINFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_reqinfo_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_REQINFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_reqinfo(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_REQINFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_reqinfo(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_REQINFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_reqinfo_it() -> *const APTR { ((*IAmiSSL).OCSP_REQINFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_crlid_new() -> *mut APTR { ((*IAmiSSL).OCSP_CRLID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_crlid_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_CRLID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_crlid(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_CRLID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_crlid(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_CRLID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_crlid_it() -> *const APTR { ((*IAmiSSL).OCSP_CRLID_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_serviceloc_new() -> *mut APTR { ((*IAmiSSL).OCSP_SERVICELOC_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_serviceloc_free(a0: *mut APTR) { ((*IAmiSSL).OCSP_SERVICELOC_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ocsp_serviceloc(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OCSP_SERVICELOC)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ocsp_serviceloc(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OCSP_SERVICELOC)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_serviceloc_it() -> *const APTR { ((*IAmiSSL).OCSP_SERVICELOC_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_response_status_str(a0: i32) -> *const APTR { ((*IAmiSSL).OCSP_response_status_str)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_cert_status_str(a0: i32) -> *const APTR { ((*IAmiSSL).OCSP_cert_status_str)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_crl_reason_str(a0: i32) -> *const APTR { ((*IAmiSSL).OCSP_crl_reason_str)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_request_print(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).OCSP_REQUEST_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_response_print(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).OCSP_RESPONSE_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ocsp_basic_verify(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: u32) -> i32 { ((*IAmiSSL).OCSP_basic_verify)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_err_load_ocsp_strings() -> i32 { ((*IAmiSSL).ERR_load_OCSP_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pem_get_evp_cipher_info(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PEM_get_EVP_CIPHER_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_do_header(a0: *mut APTR, a1: *mut u8, a2: *mut i32, a3: *mut APTR, a4: *mut ()) -> i32 { ((*IAmiSSL).PEM_do_header)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pem_read_bio(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut u8, a4: *mut i32) -> i32 { ((*IAmiSSL).PEM_read_bio)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pem_write_bio(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).PEM_write_bio)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pem_bytes_read_bio(a0: *mut *mut u8, a1: *mut i32, a2: *mut *mut APTR, a3: *const APTR, a4: *mut APTR, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_bytes_read_bio)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pem_asn1_read_bio(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *mut *mut (), a4: *mut APTR, a5: *mut ()) -> *mut () { ((*IAmiSSL).PEM_ASN1_read_bio)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pem_asn1_write_bio(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const (), a4: *const APTR, a5: *const u8, a6: i32, a7: *mut APTR, a8: *mut ()) -> i32 { ((*IAmiSSL).PEM_ASN1_write_bio)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_pem_x509_info_read_bio(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_X509_INFO_read_bio)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_x509_info_write_bio(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const u8, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_X509_INFO_write_bio)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pem_sign_init(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PEM_SignInit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_sign_update(a0: *mut APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).PEM_SignUpdate)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pem_sign_final(a0: *mut APTR, a1: *mut u8, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).PEM_SignFinal)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_def_callback(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).PEM_def_callback)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_proc_type(a0: *mut APTR, a1: i32) { ((*IAmiSSL).PEM_proc_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_dek_info(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const APTR) { ((*IAmiSSL).PEM_dek_info)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_read_bio_x509(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_X509)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_x509(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_X509)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_x509_aux(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_X509_AUX)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_x509_aux(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_X509_AUX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_x509_req(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_X509_REQ)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_x509_req(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_X509_REQ)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_write_bio_x509_req_new(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_X509_REQ_NEW)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_x509_crl(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_X509_CRL)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_x509_crl(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_X509_CRL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_pkcs7(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_PKCS7)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_pkcs7(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_PKCS7)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_netscape_cert_sequence(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_NETSCAPE_CERT_SEQUENCE)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_netscape_cert_sequence(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_NETSCAPE_CERT_SEQUENCE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_pkcs8(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_PKCS8)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_pkcs8(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_PKCS8)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_pkcs8_priv_key_info(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_PKCS8_PRIV_KEY_INFO)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_pkcs8_priv_key_info(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_PKCS8_PRIV_KEY_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_rsaprivate_key(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_RSAPrivateKey)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_rsaprivate_key(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_write_bio_RSAPrivateKey)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pem_read_bio_rsapublic_key(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_RSAPublicKey)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_rsapublic_key(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_RSAPublicKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_rsa_pubkey(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_RSA_PUBKEY)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_rsa_pubkey(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_RSA_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_dsaprivate_key(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_DSAPrivateKey)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_dsaprivate_key(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_write_bio_DSAPrivateKey)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pem_read_bio_dsa_pubkey(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_DSA_PUBKEY)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_dsa_pubkey(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_DSA_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_dsaparams(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_DSAparams)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_dsaparams(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_DSAparams)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_dhparams(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_DHparams)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_dhparams(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_DHparams)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_private_key(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_PrivateKey)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_private_key(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_write_bio_PrivateKey)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pem_read_bio_pubkey(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_PUBKEY)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_pubkey(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_write_bio_pkcs8_private_key_nid(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const APTR, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_write_bio_PKCS8PrivateKey_nid)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pem_write_bio_pkcs8_private_key(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_write_bio_PKCS8PrivateKey)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_i2d_pkcs8_private_key_bio(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).i2d_PKCS8PrivateKey_bio)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_i2d_pkcs8_private_key_nid_bio(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const APTR, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).i2d_PKCS8PrivateKey_nid_bio)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_d2i_pkcs8_private_key_bio(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).d2i_PKCS8PrivateKey_bio)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_err_load_pem_strings() -> i32 { ((*IAmiSSL).ERR_load_PEM_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_pkcs12_x5092certbag(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OBSOLETE_PKCS12_x5092certbag)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_pkcs12_x509crl2certbag(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OBSOLETE_PKCS12_x509crl2certbag)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_pkcs12_certbag2x509(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OBSOLETE_PKCS12_certbag2x509)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_item_pack_safebag(a0: *mut (), a1: *const APTR, a2: i32, a3: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_item_pack_safebag)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_obsolete_pkcs12_make_keybag(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OBSOLETE_PKCS12_MAKE_KEYBAG)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs8_decrypt(a0: *const APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).PKCS8_decrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_decrypt_skey(a0: *const APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_decrypt_skey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs8_encrypt(a0: i32, a1: *const APTR, a2: *const APTR, a3: i32, a4: *mut u8, a5: i32, a6: i32, a7: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS8_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_obsolete_pkcs12_make_shkeybag(a0: i32, a1: *const APTR, a2: i32, a3: *mut u8, a4: i32, a5: i32, a6: *mut APTR) -> *mut APTR { ((*IAmiSSL).OBSOLETE_PKCS12_MAKE_SHKEYBAG)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs12_pack_p7data(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_pack_p7data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_unpack_p7data(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_unpack_p7data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_pack_p7encdata(a0: i32, a1: *const APTR, a2: i32, a3: *mut u8, a4: i32, a5: i32, a6: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_pack_p7encdata)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs12_unpack_p7encdata(a0: *mut APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_unpack_p7encdata)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_pack_authsafes(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS12_pack_authsafes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_unpack_authsafes(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_unpack_authsafes)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_add_localkeyid(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).PKCS12_add_localkeyid)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_add_friendlyname_asc(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).PKCS12_add_friendlyname_asc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_add_cspname_asc(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).PKCS12_add_CSPName_asc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_add_friendlyname_uni(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).PKCS12_add_friendlyname_uni)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs8_add_keyusage(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).PKCS8_add_keyusage)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_get_attr_gen(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_get_attr_gen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_get_friendlyname(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_get_friendlyname)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_pbe_crypt(a0: *const APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32, a5: *mut *mut u8, a6: *mut i32, a7: i32) -> *mut u8 { ((*IAmiSSL).PKCS12_pbe_crypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_pkcs12_item_decrypt_d2i(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: i32, a4: *const APTR, a5: i32) -> *mut () { ((*IAmiSSL).PKCS12_item_decrypt_d2i)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pkcs12_item_i2d_encrypt(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: i32, a4: *mut (), a5: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_item_i2d_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pkcs12_init(a0: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_key_gen_asc(a0: *const APTR, a1: i32, a2: *mut u8, a3: i32, a4: i32, a5: i32, a6: i32, a7: *mut u8, a8: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_key_gen_asc)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_pkcs12_key_gen_uni(a0: *mut u8, a1: i32, a2: *mut u8, a3: i32, a4: i32, a5: i32, a6: i32, a7: *mut u8, a8: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_key_gen_uni)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_pkcs12_pbe_keyivgen(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *const APTR, a6: i32) -> i32 { ((*IAmiSSL).PKCS12_PBE_keyivgen)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs12_gen_mac(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut u8, a4: *mut APTR) -> i32 { ((*IAmiSSL).PKCS12_gen_mac)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs12_verify_mac(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).PKCS12_verify_mac)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_set_mac(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut u8, a4: i32, a5: i32, a6: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_set_mac)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs12_setup_mac(a0: *mut APTR, a1: i32, a2: *mut u8, a3: i32, a4: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_setup_mac)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_openssl_asc2uni(a0: *const APTR, a1: i32, a2: *mut *mut u8, a3: *mut i32) -> *mut u8 { ((*IAmiSSL).OPENSSL_asc2uni)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_openssl_uni2asc(a0: *const u8, a1: i32) -> *mut APTR { ((*IAmiSSL).OPENSSL_uni2asc)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_new() -> *mut APTR { ((*IAmiSSL).PKCS12_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_free(a0: *mut APTR) { ((*IAmiSSL).PKCS12_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs12(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS12)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs12(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS12)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_it() -> *const APTR { ((*IAmiSSL).PKCS12_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_mac_data_new() -> *mut APTR { ((*IAmiSSL).PKCS12_MAC_DATA_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_mac_data_free(a0: *mut APTR) { ((*IAmiSSL).PKCS12_MAC_DATA_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs12_mac_data(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS12_MAC_DATA)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs12_mac_data(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS12_MAC_DATA)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_mac_data_it() -> *const APTR { ((*IAmiSSL).PKCS12_MAC_DATA_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_new() -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_free(a0: *mut APTR) { ((*IAmiSSL).PKCS12_SAFEBAG_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs12_safebag(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS12_SAFEBAG)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs12_safebag(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS12_SAFEBAG)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_it() -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_bags_new() -> *mut APTR { ((*IAmiSSL).PKCS12_BAGS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_bags_free(a0: *mut APTR) { ((*IAmiSSL).PKCS12_BAGS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs12_bags(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS12_BAGS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs12_bags(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS12_BAGS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_bags_it() -> *const APTR { ((*IAmiSSL).PKCS12_BAGS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_safebags_it() -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAGS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_authsafes_it() -> *const APTR { ((*IAmiSSL).PKCS12_AUTHSAFES_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_pbe_add() { ((*IAmiSSL).PKCS12_PBE_add)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_parse(a0: *mut APTR, a1: *const APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR) -> i32 { ((*IAmiSSL).PKCS12_parse)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs12_create(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_create)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_i2d_pkcs12_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_PKCS12_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_pkcs12_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_PKCS12_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_newpass(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_newpass)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_err_load_pkcs12_strings() -> i32 { ((*IAmiSSL).ERR_load_PKCS12_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_issuer_and_serial_digest(a0: *mut APTR, a1: *const APTR, a2: *mut u8, a3: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_ISSUER_AND_SERIAL_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_PKCS7_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_issuer_and_serial_new() -> *mut APTR { ((*IAmiSSL).PKCS7_ISSUER_AND_SERIAL_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_issuer_and_serial_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_ISSUER_AND_SERIAL_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_issuer_and_serial(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_ISSUER_AND_SERIAL)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_issuer_and_serial(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_ISSUER_AND_SERIAL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_issuer_and_serial_it() -> *const APTR { ((*IAmiSSL).PKCS7_ISSUER_AND_SERIAL_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_signer_info_new() -> *mut APTR { ((*IAmiSSL).PKCS7_SIGNER_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_signer_info_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_SIGNER_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_signer_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_SIGNER_INFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_signer_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_SIGNER_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_signer_info_it() -> *const APTR { ((*IAmiSSL).PKCS7_SIGNER_INFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_recip_info_new() -> *mut APTR { ((*IAmiSSL).PKCS7_RECIP_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_recip_info_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_RECIP_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_recip_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_RECIP_INFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_recip_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_RECIP_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_recip_info_it() -> *const APTR { ((*IAmiSSL).PKCS7_RECIP_INFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_signed_new() -> *mut APTR { ((*IAmiSSL).PKCS7_SIGNED_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_signed_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_SIGNED_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_signed(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_SIGNED)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_signed(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_SIGNED)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_signed_it() -> *const APTR { ((*IAmiSSL).PKCS7_SIGNED_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_enc_content_new() -> *mut APTR { ((*IAmiSSL).PKCS7_ENC_CONTENT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_enc_content_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_ENC_CONTENT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_enc_content(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_ENC_CONTENT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_enc_content(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_ENC_CONTENT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_enc_content_it() -> *const APTR { ((*IAmiSSL).PKCS7_ENC_CONTENT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_envelope_new() -> *mut APTR { ((*IAmiSSL).PKCS7_ENVELOPE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_envelope_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_ENVELOPE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_envelope(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_ENVELOPE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_envelope(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_ENVELOPE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_envelope_it() -> *const APTR { ((*IAmiSSL).PKCS7_ENVELOPE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_sign_envelope_new() -> *mut APTR { ((*IAmiSSL).PKCS7_SIGN_ENVELOPE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_sign_envelope_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_SIGN_ENVELOPE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_sign_envelope(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_SIGN_ENVELOPE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_sign_envelope(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_SIGN_ENVELOPE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_sign_envelope_it() -> *const APTR { ((*IAmiSSL).PKCS7_SIGN_ENVELOPE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_digest_new() -> *mut APTR { ((*IAmiSSL).PKCS7_DIGEST_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_digest_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_DIGEST_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_digest(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_DIGEST)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_digest(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_DIGEST)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_digest_it() -> *const APTR { ((*IAmiSSL).PKCS7_DIGEST_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_encrypt_new() -> *mut APTR { ((*IAmiSSL).PKCS7_ENCRYPT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_encrypt_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_ENCRYPT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7_encrypt(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7_ENCRYPT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_encrypt(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_ENCRYPT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_encrypt_it() -> *const APTR { ((*IAmiSSL).PKCS7_ENCRYPT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_new() -> *mut APTR { ((*IAmiSSL).PKCS7_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_free(a0: *mut APTR) { ((*IAmiSSL).PKCS7_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs7(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS7)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_it() -> *const APTR { ((*IAmiSSL).PKCS7_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_attr_sign_it() -> *const APTR { ((*IAmiSSL).PKCS7_ATTR_SIGN_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_attr_verify_it() -> *const APTR { ((*IAmiSSL).PKCS7_ATTR_VERIFY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_ctrl)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_set_type(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).PKCS7_set_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_set_content(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_set_content)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_signer_info_set(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).PKCS7_SIGNER_INFO_set)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_add_signer(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_add_signer)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_add_certificate(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_add_certificate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_add_crl(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_add_crl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_content_new(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).PKCS7_content_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_data_verify(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_dataVerify)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs7_signature_verify(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_signatureVerify)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_data_init(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_dataInit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_data_final(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_dataFinal)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_data_decode(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_dataDecode)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_add_signature(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_add_signature)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_cert_from_signer_info(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_cert_from_signer_info)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_get_signer_info(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_get_signer_info)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_add_recipient(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_add_recipient)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_add_recipient_info(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_add_recipient_info)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_recip_info_set(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_RECIP_INFO_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_set_cipher(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PKCS7_set_cipher)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_get_issuer_and_serial(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).PKCS7_get_issuer_and_serial)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_digest_from_attributes(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_digest_from_attributes)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_add_signed_attribute(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).PKCS7_add_signed_attribute)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_add_attribute(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).PKCS7_add_attribute)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_get_attribute(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).PKCS7_get_attribute)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_get_signed_attribute(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).PKCS7_get_signed_attribute)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_set_signed_attributes(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_set_signed_attributes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_set_attributes(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_set_attributes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_sign(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: i32) -> *mut APTR { ((*IAmiSSL).PKCS7_sign)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs7_verify(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: i32) -> i32 { ((*IAmiSSL).PKCS7_verify)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pkcs7_get0_signers(a0: *mut APTR, a1: *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).PKCS7_get0_signers)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs7_encrypt(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).PKCS7_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_decrypt(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: i32) -> i32 { ((*IAmiSSL).PKCS7_decrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs7_add_attrib_smimecap(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_add_attrib_smimecap)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_get_smimecap(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_get_smimecap)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_simple_smimecap(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).PKCS7_simple_smimecap)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_smime_write_pkcs7(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).SMIME_write_PKCS7)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_smime_read_pkcs7(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).SMIME_read_PKCS7)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_smime_crlf_copy(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).SMIME_crlf_copy)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_smime_text(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SMIME_text)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_load_pkcs7_strings() -> i32 { ((*IAmiSSL).ERR_load_PKCS7_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rand_set_rand_method(a0: *const APTR) -> i32 { ((*IAmiSSL).RAND_set_rand_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_get_rand_method() -> *const APTR { ((*IAmiSSL).RAND_get_rand_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rand_bytes(a0: *mut u8, a1: i32) -> i32 { ((*IAmiSSL).RAND_bytes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rand_pseudo_bytes(a0: *mut u8, a1: i32) -> i32 { ((*IAmiSSL).RAND_pseudo_bytes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rand_seed(a0: *const (), a1: i32) { ((*IAmiSSL).RAND_seed)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rand_add(a0: *const (), a1: i32, a2: APTR) { ((*IAmiSSL).RAND_add)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rand_load_file(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).RAND_load_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rand_write_file(a0: *const APTR) -> i32 { ((*IAmiSSL).RAND_write_file)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_file_name(a0: *mut APTR, a1: u32) -> *const APTR { ((*IAmiSSL).RAND_file_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rand_status() -> i32 { ((*IAmiSSL).RAND_status)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rand_poll() -> i32 { ((*IAmiSSL).RAND_poll)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_load_rand_strings() -> i32 { ((*IAmiSSL).ERR_load_RAND_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_msg_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_msg_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_msg_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_msg_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_sessions(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_sessions)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_finished(a0: *const APTR, a1: *mut (), a2: u32) -> u32 { ((*IAmiSSL).SSL_get_finished)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_peer_finished(a0: *const APTR, a1: *mut (), a2: u32) -> u32 { ((*IAmiSSL).SSL_get_peer_finished)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_f_ssl() -> *const APTR { ((*IAmiSSL).BIO_f_ssl)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_new_ssl(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_new_ssl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new_ssl_connect(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_ssl_connect)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_new_buffer_ssl_connect(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_buffer_ssl_connect)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_ssl_copy_session_id(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).BIO_ssl_copy_session_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_ssl_shutdown(a0: *mut APTR) { ((*IAmiSSL).BIO_ssl_shutdown)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_cipher_list(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_cipher_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_new(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_free(a0: *mut APTR) { ((*IAmiSSL).SSL_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_timeout(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_CTX_set_timeout)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_timeout(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_timeout)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_cert_store(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_get_cert_store)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_cert_store(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_CTX_set_cert_store)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_want(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_want)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_clear(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_clear)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_flush_sessions(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_CTX_flush_sessions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_current_cipher(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get_current_cipher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_bits(a0: *const APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).SSL_CIPHER_get_bits)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_version(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_get_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_mac(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_get_mac)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_encryption(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_get_encryption)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_authentication(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_get_authentication)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_key_exchange(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_get_key_exchange)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_fd(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_fd)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_rfd(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_rfd)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_wfd(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_wfd)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_cipher_list(a0: *const APTR, a1: i32) -> *const APTR { ((*IAmiSSL).SSL_get_cipher_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_shared_ciphers(a0: *const APTR, a1: *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).SSL_get_shared_ciphers)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_read_ahead(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_read_ahead)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_pending(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_pending)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_fd(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_set_fd)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_rfd(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_set_rfd)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_wfd(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_set_wfd)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_bio(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) { ((*IAmiSSL).SSL_set_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_rbio(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_rbio)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_wbio(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_wbio)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_cipher_list(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_set_cipher_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_read_ahead(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_set_read_ahead)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_verify_mode(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_verify_mode)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_verify_depth(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_verify_depth)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_verify_callback(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_verify_callback)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_verify(a0: *mut APTR, a1: i32, a2: APTR) { ((*IAmiSSL).SSL_set_verify)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_verify_depth(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_set_verify_depth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_use_rsaprivate_key(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_use_RSAPrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_use_rsaprivate_key_asn1(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).SSL_use_RSAPrivateKey_ASN1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_use_private_key(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_use_PrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_use_private_key_asn1(a0: i32, a1: *mut APTR, a2: *const u8, a3: i32) -> i32 { ((*IAmiSSL).SSL_use_PrivateKey_ASN1)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_use_certificate(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_use_certificate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_use_certificate_asn1(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).SSL_use_certificate_ASN1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_use_rsaprivate_key_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).SSL_use_RSAPrivateKey_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_use_private_key_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).SSL_use_PrivateKey_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_use_certificate_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).SSL_use_certificate_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_rsaprivate_key_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).SSL_CTX_use_RSAPrivateKey_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_private_key_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).SSL_CTX_use_PrivateKey_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_certificate_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).SSL_CTX_use_certificate_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_certificate_chain_file(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_use_certificate_chain_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_load_client_ca_file(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_load_client_CA_file)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_add_file_cert_subjects_to_stack(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_add_file_cert_subjects_to_stack)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_add_dir_cert_subjects_to_stack(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_add_dir_cert_subjects_to_stack)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_ssl_load_error_strings() { ((*IAmiSSL).OBSOLETE_SSL_load_error_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_state_string(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_state_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_rstate_string(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_rstate_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_state_string_long(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_state_string_long)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_rstate_string_long(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_rstate_string_long)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_get_time(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_get_time)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_set_time(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_SESSION_set_time)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_get_timeout(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_get_timeout)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_set_timeout(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_SESSION_set_timeout)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_copy_session_id(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_copy_session_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_new() -> *mut APTR { ((*IAmiSSL).SSL_SESSION_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_session_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_free(a0: *mut APTR) { ((*IAmiSSL).SSL_SESSION_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_ssl_session(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_SSL_SESSION)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_session(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_set_session)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_add_session(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_add_session)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_remove_session(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_remove_session)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_generate_session_id(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_generate_session_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_generate_session_id(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_set_generate_session_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_has_matching_session_id(a0: *const APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).SSL_has_matching_session_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ssl_session(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_SSL_SESSION)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get1_peer_certificate(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get1_peer_certificate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_peer_cert_chain(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_peer_cert_chain)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_verify_mode(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_verify_mode)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_verify_depth(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_verify_depth)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_verify_callback(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_verify_callback)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_verify(a0: *mut APTR, a1: i32, a2: APTR) { ((*IAmiSSL).SSL_CTX_set_verify)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_verify_depth(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_CTX_set_verify_depth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_cert_verify_callback(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_CTX_set_cert_verify_callback)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_rsaprivate_key(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_use_RSAPrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_rsaprivate_key_asn1(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).SSL_CTX_use_RSAPrivateKey_ASN1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_private_key(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_use_PrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_private_key_asn1(a0: i32, a1: *mut APTR, a2: *const u8, a3: i32) -> i32 { ((*IAmiSSL).SSL_CTX_use_PrivateKey_ASN1)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_certificate(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_use_certificate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_certificate_asn1(a0: *mut APTR, a1: i32, a2: *const u8) -> i32 { ((*IAmiSSL).SSL_CTX_use_certificate_ASN1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_default_passwd_cb(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_CTX_set_default_passwd_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_default_passwd_cb_userdata(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).SSL_CTX_set_default_passwd_cb_userdata)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_check_private_key(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_check_private_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_check_private_key(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_check_private_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_session_id_context(a0: *mut APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_session_id_context)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_new(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_session_id_context(a0: *mut APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).SSL_set_session_id_context)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_purpose(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_CTX_set_purpose)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_purpose(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_set_purpose)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_trust(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_CTX_set_trust)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_trust(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_set_trust)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_free(a0: *mut APTR) { ((*IAmiSSL).SSL_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_accept(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_accept)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_connect(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_connect)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_read(a0: *mut APTR, a1: *mut (), a2: i32) -> i32 { ((*IAmiSSL).SSL_read)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_peek(a0: *mut APTR, a1: *mut (), a2: i32) -> i32 { ((*IAmiSSL).SSL_peek)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_write(a0: *mut APTR, a1: *const (), a2: i32) -> i32 { ((*IAmiSSL).SSL_write)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).SSL_ctrl)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_callback_ctrl(a0: *mut APTR, a1: i32, a2: APTR) -> i32 { ((*IAmiSSL).SSL_callback_ctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_ctrl)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_ctx_callback_ctrl(a0: *mut APTR, a1: i32, a2: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_callback_ctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_error(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_get_error)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_version(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_ssl_version(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_ssl_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_tlsv1_method() -> *const APTR { ((*IAmiSSL).TLSv1_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_tlsv1_server_method() -> *const APTR { ((*IAmiSSL).TLSv1_server_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_tlsv1_client_method() -> *const APTR { ((*IAmiSSL).TLSv1_client_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_get_ciphers(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_ciphers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_do_handshake(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_do_handshake)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_renegotiate(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_renegotiate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_renegotiate_pending(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_renegotiate_pending)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_shutdown(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_shutdown)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_ssl_method(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get_ssl_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_ssl_method(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_set_ssl_method)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_alert_type_string_long(a0: i32) -> *const APTR { ((*IAmiSSL).SSL_alert_type_string_long)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_alert_type_string(a0: i32) -> *const APTR { ((*IAmiSSL).SSL_alert_type_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_alert_desc_string_long(a0: i32) -> *const APTR { ((*IAmiSSL).SSL_alert_desc_string_long)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_alert_desc_string(a0: i32) -> *const APTR { ((*IAmiSSL).SSL_alert_desc_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_client_ca_list(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_set_client_CA_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_client_ca_list(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_CTX_set_client_CA_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_client_ca_list(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_client_CA_list)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_client_ca_list(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_get_client_CA_list)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_add_client_ca(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_add_client_CA)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_add_client_ca(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_add_client_CA)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_connect_state(a0: *mut APTR) { ((*IAmiSSL).SSL_set_connect_state)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_accept_state(a0: *mut APTR) { ((*IAmiSSL).SSL_set_accept_state)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_default_timeout(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_default_timeout)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_ssl_library_init() -> i32 { ((*IAmiSSL).OBSOLETE_SSL_library_init)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_cipher_description(a0: *const APTR, a1: *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).SSL_CIPHER_description)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_dup_ca_list(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_dup_CA_list)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_dup(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_certificate(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_certificate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_privatekey(a0: *const APTR) -> *mut evp_pkey_st { ((*IAmiSSL).SSL_get_privatekey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_quiet_shutdown(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_CTX_set_quiet_shutdown)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_quiet_shutdown(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_quiet_shutdown)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_quiet_shutdown(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_set_quiet_shutdown)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_quiet_shutdown(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_quiet_shutdown)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_shutdown(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_set_shutdown)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_shutdown(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_shutdown)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_version(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_default_verify_paths(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_default_verify_paths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_load_verify_locations(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_load_verify_locations)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_session(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_session)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get1_session(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get1_session)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_ssl_ctx(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_SSL_CTX)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_info_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_info_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_info_callback(a0: *const APTR) { ((*IAmiSSL).SSL_get_info_callback)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_verify_result(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_set_verify_result)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_verify_result(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_verify_result)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).SSL_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).SSL_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_ssl_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_SSL_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_session_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).SSL_SESSION_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_session_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).SSL_SESSION_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_ssl_session_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_SSL_SESSION_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).SSL_CTX_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_ssl_ctx_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_SSL_CTX_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_get_ex_data_x509_store_ctx_idx() -> i32 { ((*IAmiSSL).SSL_get_ex_data_X509_STORE_CTX_idx)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_tmp_dh_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_tmp_dh_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_tmp_dh_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_tmp_dh_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_comp_add_compression_method(a0: i32, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_COMP_add_compression_method)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_load_ssl_strings() -> i32 { ((*IAmiSSL).ERR_load_SSL_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_sk_num(a0: *const APTR) -> i32 { ((*IAmiSSL).OPENSSL_sk_num)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_value(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).OPENSSL_sk_value)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_set(a0: *mut APTR, a1: i32, a2: *const ()) -> *mut () { ((*IAmiSSL).OPENSSL_sk_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_sk_new(a0: APTR) -> *mut APTR { ((*IAmiSSL).OPENSSL_sk_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_new_null() -> *mut APTR { ((*IAmiSSL).OPENSSL_sk_new_null)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_sk_free(a0: *mut APTR) { ((*IAmiSSL).OPENSSL_sk_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_pop_free(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).OPENSSL_sk_pop_free)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_insert(a0: *mut APTR, a1: *const (), a2: i32) -> i32 { ((*IAmiSSL).OPENSSL_sk_insert)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_sk_delete(a0: *mut APTR, a1: i32) -> *mut () { ((*IAmiSSL).OPENSSL_sk_delete)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_delete_ptr(a0: *mut APTR, a1: *const ()) -> *mut () { ((*IAmiSSL).OPENSSL_sk_delete_ptr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_find(a0: *mut APTR, a1: *const ()) -> i32 { ((*IAmiSSL).OPENSSL_sk_find)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_push(a0: *mut APTR, a1: *const ()) -> i32 { ((*IAmiSSL).OPENSSL_sk_push)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_unshift(a0: *mut APTR, a1: *const ()) -> i32 { ((*IAmiSSL).OPENSSL_sk_unshift)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_shift(a0: *mut APTR) -> *mut () { ((*IAmiSSL).OPENSSL_sk_shift)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_pop(a0: *mut APTR) -> *mut () { ((*IAmiSSL).OPENSSL_sk_pop)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_zero(a0: *mut APTR) { ((*IAmiSSL).OPENSSL_sk_zero)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_set_cmp_func(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OPENSSL_sk_set_cmp_func)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OPENSSL_sk_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_sort(a0: *mut APTR) { ((*IAmiSSL).OPENSSL_sk_sort)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_is_sorted(a0: *const APTR) -> i32 { ((*IAmiSSL).OPENSSL_sk_is_sorted)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_txt_db_read(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).TXT_DB_read)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_txt_db_write(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TXT_DB_write)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_txt_db_create_index(a0: *mut APTR, a1: i32, a2: APTR, a3: APTR, a4: APTR) -> i32 { ((*IAmiSSL).TXT_DB_create_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_txt_db_free(a0: *mut APTR) { ((*IAmiSSL).TXT_DB_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_txt_db_get_by_index(a0: *mut APTR, a1: i32, a2: *mut *mut APTR) -> *mut *mut APTR { ((*IAmiSSL).TXT_DB_get_by_index)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_txt_db_insert(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).TXT_DB_insert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_new() -> *mut APTR { ((*IAmiSSL).UI_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ui_new_method(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).UI_new_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_free(a0: *mut APTR) { ((*IAmiSSL).UI_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_add_input_string(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: i32, a5: i32) -> i32 { ((*IAmiSSL).UI_add_input_string)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ui_dup_input_string(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: i32, a5: i32) -> i32 { ((*IAmiSSL).UI_dup_input_string)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ui_add_verify_string(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: i32, a5: i32, a6: *const APTR) -> i32 { ((*IAmiSSL).UI_add_verify_string)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ui_dup_verify_string(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: i32, a5: i32, a6: *const APTR) -> i32 { ((*IAmiSSL).UI_dup_verify_string)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ui_add_input_boolean(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: i32, a6: *mut APTR) -> i32 { ((*IAmiSSL).UI_add_input_boolean)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ui_dup_input_boolean(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: i32, a6: *mut APTR) -> i32 { ((*IAmiSSL).UI_dup_input_boolean)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ui_add_info_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).UI_add_info_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_dup_info_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).UI_dup_info_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_add_error_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).UI_add_error_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_dup_error_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).UI_dup_error_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_construct_prompt(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).UI_construct_prompt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ui_add_user_data(a0: *mut APTR, a1: *mut ()) -> *mut () { ((*IAmiSSL).UI_add_user_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_get0_user_data(a0: *mut APTR) -> *mut () { ((*IAmiSSL).UI_get0_user_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get0_result(a0: *mut APTR, a1: i32) -> *const APTR { ((*IAmiSSL).UI_get0_result)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_process(a0: *mut APTR) -> i32 { ((*IAmiSSL).UI_process)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: *mut (), a4: APTR) -> i32 { ((*IAmiSSL).UI_ctrl)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_obsolete_ui_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_UI_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ui_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).UI_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ui_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).UI_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_set_default_method(a0: *const APTR) { ((*IAmiSSL).UI_set_default_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get_default_method() -> *const APTR { ((*IAmiSSL).UI_get_default_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ui_get_method(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).UI_get_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_set_method(a0: *mut APTR, a1: *const APTR) -> *const APTR { ((*IAmiSSL).UI_set_method)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_open_ssl() -> *mut APTR { ((*IAmiSSL).UI_OpenSSL)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ui_create_method(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).UI_create_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_destroy_method(a0: *mut APTR) { ((*IAmiSSL).UI_destroy_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_method_set_opener(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).UI_method_set_opener)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_method_set_writer(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).UI_method_set_writer)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_method_set_flusher(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).UI_method_set_flusher)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_method_set_reader(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).UI_method_set_reader)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_method_set_closer(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).UI_method_set_closer)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_method_get_opener(a0: *const APTR) -> i32 { ((*IAmiSSL).UI_method_get_opener)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_method_get_writer(a0: *const APTR) -> i32 { ((*IAmiSSL).UI_method_get_writer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_method_get_flusher(a0: *const APTR) -> i32 { ((*IAmiSSL).UI_method_get_flusher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_method_get_reader(a0: *const APTR) -> i32 { ((*IAmiSSL).UI_method_get_reader)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_method_get_closer(a0: *const APTR) -> i32 { ((*IAmiSSL).UI_method_get_closer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get_string_type(a0: *mut APTR) -> APTR { ((*IAmiSSL).UI_get_string_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get_input_flags(a0: *mut APTR) -> i32 { ((*IAmiSSL).UI_get_input_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get0_output_string(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).UI_get0_output_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get0_action_string(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).UI_get0_action_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get0_result_string(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).UI_get0_result_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get0_test_string(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).UI_get0_test_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get_result_minsize(a0: *mut APTR) -> i32 { ((*IAmiSSL).UI_get_result_minsize)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_get_result_maxsize(a0: *mut APTR) -> i32 { ((*IAmiSSL).UI_get_result_maxsize)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_set_result(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).UI_set_result)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ui_util_read_pw_string(a0: *mut APTR, a1: i32, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).UI_UTIL_read_pw_string)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ui_util_read_pw(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: *const APTR, a4: i32) -> i32 { ((*IAmiSSL).UI_UTIL_read_pw)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_err_load_ui_strings() -> i32 { ((*IAmiSSL).ERR_load_UI_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_verify_cert_error_string(a0: i32) -> *const APTR { ((*IAmiSSL).X509_verify_cert_error_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_verify(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REQ_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_verify(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_CRL_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_netscape_spki_verify(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).NETSCAPE_SPKI_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_netscape_spki_b64_decode(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).NETSCAPE_SPKI_b64_decode)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_netscape_spki_b64_encode(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).NETSCAPE_SPKI_b64_encode)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_netscape_spki_get_pubkey(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).NETSCAPE_SPKI_get_pubkey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_netscape_spki_set_pubkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).NETSCAPE_SPKI_set_pubkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_netscape_spki_print(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).NETSCAPE_SPKI_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_signature_print(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_signature_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_sign(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_req_sign(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_REQ_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_sign(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_netscape_spki_sign(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).NETSCAPE_SPKI_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_pubkey_digest(a0: *const APTR, a1: *const APTR, a2: *mut u8, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509_pubkey_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_digest(a0: *const APTR, a1: *const APTR, a2: *mut u8, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_crl_digest(a0: *const APTR, a1: *const APTR, a2: *mut u8, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509_CRL_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_req_digest(a0: *const APTR, a1: *const APTR, a2: *mut u8, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509_REQ_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_name_digest(a0: *const APTR, a1: *const APTR, a2: *mut u8, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509_NAME_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_d2i_x509_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_X509_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_x509_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_X509_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_x509_crl_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_X509_CRL_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_x509_crl_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_X509_CRL_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_x509_req_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_X509_REQ_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_x509_req_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_X509_REQ_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_rsaprivate_key_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_RSAPrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_rsaprivate_key_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_RSAPrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_rsapublic_key_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_RSAPublicKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_rsapublic_key_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_RSAPublicKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_rsa_pubkey_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_RSA_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_rsa_pubkey_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_RSA_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_dsa_pubkey_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_DSA_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_dsa_pubkey_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_DSA_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_dsaprivate_key_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_DSAPrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_dsaprivate_key_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_DSAPrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_pkcs8_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_PKCS8_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_pkcs8_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_PKCS8_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_pkcs8_priv_key_info_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_PKCS8_PRIV_KEY_INFO_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_pkcs8_priv_key_info_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_PKCS8_PRIV_KEY_INFO_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_pkcs8_private_key_info_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_PKCS8PrivateKeyInfo_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_private_key_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_PrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_private_key_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_PrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_pubkey_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_pubkey_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_attribute_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_ATTRIBUTE_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_extension_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_EXTENSION_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_CRL_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_algor_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_ALGOR_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_name_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_NAME_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_name_entry_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_NAME_ENTRY_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_cmp_time(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_cmp_time)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_cmp_current_time(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_cmp_current_time)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_time_adj(a0: *mut APTR, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_time_adj)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_gmtime_adj(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_gmtime_adj)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_get_default_cert_area() -> *const APTR { ((*IAmiSSL).X509_get_default_cert_area)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_get_default_cert_dir() -> *const APTR { ((*IAmiSSL).X509_get_default_cert_dir)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_get_default_cert_file() -> *const APTR { ((*IAmiSSL).X509_get_default_cert_file)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_get_default_cert_dir_env() -> *const APTR { ((*IAmiSSL).X509_get_default_cert_dir_env)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_get_default_cert_file_env() -> *const APTR { ((*IAmiSSL).X509_get_default_cert_file_env)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_get_default_private_dir() -> *const APTR { ((*IAmiSSL).X509_get_default_private_dir)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_to_x509_req(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_to_X509_REQ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_req_to_x509(a0: *mut APTR, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_to_X509)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_algor_new() -> *mut APTR { ((*IAmiSSL).X509_ALGOR_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_algor_free(a0: *mut APTR) { ((*IAmiSSL).X509_ALGOR_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_algor(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_ALGOR)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_algor(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_ALGOR)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_algor_it() -> *const APTR { ((*IAmiSSL).X509_ALGOR_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_val_new() -> *mut APTR { ((*IAmiSSL).X509_VAL_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_val_free(a0: *mut APTR) { ((*IAmiSSL).X509_VAL_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_val(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_VAL)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_val(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_VAL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_val_it() -> *const APTR { ((*IAmiSSL).X509_VAL_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_pubkey_new() -> *mut APTR { ((*IAmiSSL).X509_PUBKEY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_pubkey_free(a0: *mut APTR) { ((*IAmiSSL).X509_PUBKEY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_pubkey(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_PUBKEY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_pubkey(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_pubkey_it() -> *const APTR { ((*IAmiSSL).X509_PUBKEY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_pubkey_set(a0: *mut *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_PUBKEY_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_pubkey_get(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_PUBKEY_get)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_pubkey_parameters(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_get_pubkey_parameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_pubkey(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_pubkey(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PUBKEY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_rsa_pubkey(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_RSA_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_rsa_pubkey(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_RSA_PUBKEY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_dsa_pubkey(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DSA_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_dsa_pubkey(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DSA_PUBKEY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_sig_new() -> *mut APTR { ((*IAmiSSL).X509_SIG_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_sig_free(a0: *mut APTR) { ((*IAmiSSL).X509_SIG_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_sig(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_SIG)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_sig(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_SIG)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_sig_it() -> *const APTR { ((*IAmiSSL).X509_SIG_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_req_info_new() -> *mut APTR { ((*IAmiSSL).X509_REQ_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_req_info_free(a0: *mut APTR) { ((*IAmiSSL).X509_REQ_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_req_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_REQ_INFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_req_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_REQ_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_info_it() -> *const APTR { ((*IAmiSSL).X509_REQ_INFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_req_new() -> *mut APTR { ((*IAmiSSL).X509_REQ_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_req_free(a0: *mut APTR) { ((*IAmiSSL).X509_REQ_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_req(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_REQ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_req(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_REQ)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_it() -> *const APTR { ((*IAmiSSL).X509_REQ_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_attribute_new() -> *mut APTR { ((*IAmiSSL).X509_ATTRIBUTE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_attribute_free(a0: *mut APTR) { ((*IAmiSSL).X509_ATTRIBUTE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_attribute(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_ATTRIBUTE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_attribute(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_ATTRIBUTE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_attribute_it() -> *const APTR { ((*IAmiSSL).X509_ATTRIBUTE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_attribute_create(a0: i32, a1: i32, a2: *mut ()) -> *mut APTR { ((*IAmiSSL).X509_ATTRIBUTE_create)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_extension_new() -> *mut APTR { ((*IAmiSSL).X509_EXTENSION_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_extension_free(a0: *mut APTR) { ((*IAmiSSL).X509_EXTENSION_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_extension(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_EXTENSION)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_extension(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_EXTENSION)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_extension_it() -> *const APTR { ((*IAmiSSL).X509_EXTENSION_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_name_entry_new() -> *mut APTR { ((*IAmiSSL).X509_NAME_ENTRY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_name_entry_free(a0: *mut APTR) { ((*IAmiSSL).X509_NAME_ENTRY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_name_entry(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_NAME_ENTRY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_name_entry(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_NAME_ENTRY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_name_entry_it() -> *const APTR { ((*IAmiSSL).X509_NAME_ENTRY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_name_new() -> *mut APTR { ((*IAmiSSL).X509_NAME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_name_free(a0: *mut APTR) { ((*IAmiSSL).X509_NAME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_name(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_NAME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_name(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_NAME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_name_it() -> *const APTR { ((*IAmiSSL).X509_NAME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_name_set(a0: *mut *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_NAME_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_cinf_new() -> *mut APTR { ((*IAmiSSL).X509_CINF_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_cinf_free(a0: *mut APTR) { ((*IAmiSSL).X509_CINF_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_cinf(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_CINF)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_cinf(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_CINF)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_cinf_it() -> *const APTR { ((*IAmiSSL).X509_CINF_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_new() -> *mut APTR { ((*IAmiSSL).X509_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_free(a0: *mut APTR) { ((*IAmiSSL).X509_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_it() -> *const APTR { ((*IAmiSSL).X509_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_cert_aux_new() -> *mut APTR { ((*IAmiSSL).X509_CERT_AUX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_cert_aux_free(a0: *mut APTR) { ((*IAmiSSL).X509_CERT_AUX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_cert_aux(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_CERT_AUX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_cert_aux(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_CERT_AUX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_cert_aux_it() -> *const APTR { ((*IAmiSSL).X509_CERT_AUX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_x509_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_X509_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).X509_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).X509_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_x509_aux(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_AUX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_x509_aux(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_AUX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_alias_set1(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).X509_alias_set1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_keyid_set1(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).X509_keyid_set1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_alias_get0(a0: *mut APTR, a1: *mut i32) -> *mut u8 { ((*IAmiSSL).X509_alias_get0)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_trust_set_default(a0: APTR) -> i32 { ((*IAmiSSL).X509_TRUST_set_default)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_trust_set(a0: *mut i32, a1: i32) -> i32 { ((*IAmiSSL).X509_TRUST_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_add1_trust_object(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_add1_trust_object)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_add1_reject_object(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_add1_reject_object)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_trust_clear(a0: *mut APTR) { ((*IAmiSSL).X509_trust_clear)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_reject_clear(a0: *mut APTR) { ((*IAmiSSL).X509_reject_clear)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_revoked_new() -> *mut APTR { ((*IAmiSSL).X509_REVOKED_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_revoked_free(a0: *mut APTR) { ((*IAmiSSL).X509_REVOKED_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_revoked(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_REVOKED)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_revoked(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_REVOKED)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_revoked_it() -> *const APTR { ((*IAmiSSL).X509_REVOKED_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_crl_info_new() -> *mut APTR { ((*IAmiSSL).X509_CRL_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_crl_info_free(a0: *mut APTR) { ((*IAmiSSL).X509_CRL_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_crl_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_CRL_INFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_crl_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_CRL_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_info_it() -> *const APTR { ((*IAmiSSL).X509_CRL_INFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_crl_new() -> *mut APTR { ((*IAmiSSL).X509_CRL_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_crl_free(a0: *mut APTR) { ((*IAmiSSL).X509_CRL_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_x509_crl(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_CRL)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_crl(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_CRL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_it() -> *const APTR { ((*IAmiSSL).X509_CRL_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_crl_add0_revoked(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_CRL_add0_revoked)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_pkey_new() -> *mut APTR { ((*IAmiSSL).X509_PKEY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_pkey_free(a0: *mut APTR) { ((*IAmiSSL).X509_PKEY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_netscape_spki_new() -> *mut APTR { ((*IAmiSSL).NETSCAPE_SPKI_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_netscape_spki_free(a0: *mut APTR) { ((*IAmiSSL).NETSCAPE_SPKI_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_netscape_spki(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_NETSCAPE_SPKI)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_netscape_spki(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_NETSCAPE_SPKI)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_netscape_spki_it() -> *const APTR { ((*IAmiSSL).NETSCAPE_SPKI_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_netscape_spkac_new() -> *mut APTR { ((*IAmiSSL).NETSCAPE_SPKAC_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_netscape_spkac_free(a0: *mut APTR) { ((*IAmiSSL).NETSCAPE_SPKAC_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_netscape_spkac(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_NETSCAPE_SPKAC)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_netscape_spkac(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_NETSCAPE_SPKAC)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_netscape_spkac_it() -> *const APTR { ((*IAmiSSL).NETSCAPE_SPKAC_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_netscape_cert_sequence_new() -> *mut APTR { ((*IAmiSSL).NETSCAPE_CERT_SEQUENCE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_netscape_cert_sequence_free(a0: *mut APTR) { ((*IAmiSSL).NETSCAPE_CERT_SEQUENCE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_netscape_cert_sequence(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_NETSCAPE_CERT_SEQUENCE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_netscape_cert_sequence(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_NETSCAPE_CERT_SEQUENCE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_netscape_cert_sequence_it() -> *const APTR { ((*IAmiSSL).NETSCAPE_CERT_SEQUENCE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_info_new() -> *mut APTR { ((*IAmiSSL).X509_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_info_free(a0: *mut APTR) { ((*IAmiSSL).X509_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_name_oneline(a0: *const APTR, a1: *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).X509_NAME_oneline)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_verify(a0: APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_verify)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_digest(a0: APTR, a1: *const APTR, a2: *mut APTR, a3: *mut u8, a4: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_digest)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_sign(a0: APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR, a6: *const APTR) -> i32 { ((*IAmiSSL).ASN1_sign)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_asn1_item_digest(a0: *const APTR, a1: *const APTR, a2: *mut (), a3: *mut u8, a4: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_item_digest)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_item_verify(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const (), a4: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_item_verify)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_item_sign(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *const (), a5: *mut APTR, a6: *const APTR) -> i32 { ((*IAmiSSL).ASN1_item_sign)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_x509_set_version(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_set_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_set_serial_number(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_set_serialNumber)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_get_serial_number(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_get_serialNumber)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_set_issuer_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_set_issuer_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_get_issuer_name(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_get_issuer_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_set_subject_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_set_subject_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_get_subject_name(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_get_subject_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_set1_not_before(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_set1_notBefore)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_set1_not_after(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_set1_notAfter)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_set_pubkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_set_pubkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_get_pubkey(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_get_pubkey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_pubkey_bitstr(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_get0_pubkey_bitstr)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_certificate_type(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_certificate_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_set_version(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_REQ_set_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_set_subject_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_REQ_set_subject_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_set_pubkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REQ_set_pubkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_get_pubkey(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_get_pubkey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_extension_nid(a0: i32) -> i32 { ((*IAmiSSL).X509_REQ_extension_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get_extension_nids() -> *mut i32 { ((*IAmiSSL).X509_REQ_get_extension_nids)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_req_set_extension_nids(a0: *mut i32) { ((*IAmiSSL).X509_REQ_set_extension_nids)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get_extensions(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_get_extensions)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_add_extensions_nid(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_REQ_add_extensions_nid)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_req_add_extensions(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_REQ_add_extensions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_get_attr_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_REQ_get_attr_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get_attr_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_REQ_get_attr_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_req_get_attr_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_REQ_get_attr_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_req_get_attr(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_REQ_get_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_delete_attr(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_REQ_delete_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_add1_attr(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REQ_add1_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_add1_attr_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).X509_REQ_add1_attr_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_req_add1_attr_by_nid(a0: *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).X509_REQ_add1_attr_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_req_add1_attr_by_txt(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).X509_REQ_add1_attr_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_crl_set_version(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_CRL_set_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_set_issuer_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_set_issuer_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_set1_last_update(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_set1_lastUpdate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_set1_next_update(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_set1_nextUpdate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_sort(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_CRL_sort)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_revoked_set_serial_number(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REVOKED_set_serialNumber)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_revoked_set_revocation_date(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REVOKED_set_revocationDate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_check_private_key(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_check_private_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_issuer_and_serial_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_issuer_and_serial_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_issuer_and_serial_hash(a0: *mut APTR) -> u32 { ((*IAmiSSL).X509_issuer_and_serial_hash)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_issuer_name_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_issuer_name_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_issuer_name_hash(a0: *mut APTR) -> u32 { ((*IAmiSSL).X509_issuer_name_hash)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_subject_name_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_subject_name_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_subject_name_hash(a0: *mut APTR) -> u32 { ((*IAmiSSL).X509_subject_name_hash)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_name_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_NAME_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_x509_name_hash(a0: *mut APTR) -> u32 { ((*IAmiSSL).OBSOLETE_X509_NAME_hash)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_name_print(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_NAME_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_name_print_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: u32) -> i32 { ((*IAmiSSL).X509_NAME_print_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_print_ex(a0: *mut APTR, a1: *mut APTR, a2: u32, a3: u32) -> i32 { ((*IAmiSSL).X509_print_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_print(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_ocspid_print(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_ocspid_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_print(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_CRL_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_print_ex(a0: *mut APTR, a1: *mut APTR, a2: u32, a3: u32) -> i32 { ((*IAmiSSL).X509_REQ_print_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_req_print(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REQ_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_name_entry_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_NAME_entry_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_name_get_text_by_nid(a0: *const APTR, a1: i32, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).X509_NAME_get_text_by_NID)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_name_get_text_by_obj(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).X509_NAME_get_text_by_OBJ)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_name_get_index_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_NAME_get_index_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_name_get_index_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_NAME_get_index_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_name_get_entry(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_NAME_get_entry)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_name_delete_entry(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_NAME_delete_entry)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_name_add_entry(a0: *mut APTR, a1: *const APTR, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).X509_NAME_add_entry)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_name_add_entry_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32, a5: i32, a6: i32) -> i32 { ((*IAmiSSL).X509_NAME_add_entry_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_x509_name_add_entry_by_nid(a0: *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32, a5: i32, a6: i32) -> i32 { ((*IAmiSSL).X509_NAME_add_entry_by_NID)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_x509_name_entry_create_by_txt(a0: *mut *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> *mut APTR { ((*IAmiSSL).X509_NAME_ENTRY_create_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_name_entry_create_by_nid(a0: *mut *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32) -> *mut APTR { ((*IAmiSSL).X509_NAME_ENTRY_create_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_name_add_entry_by_txt(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32, a5: i32, a6: i32) -> i32 { ((*IAmiSSL).X509_NAME_add_entry_by_txt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_x509_name_entry_create_by_obj(a0: *mut *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> *mut APTR { ((*IAmiSSL).X509_NAME_ENTRY_create_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_name_entry_set_object(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_NAME_ENTRY_set_object)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_name_entry_set_data(a0: *mut APTR, a1: i32, a2: *const u8, a3: i32) -> i32 { ((*IAmiSSL).X509_NAME_ENTRY_set_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_name_entry_get_object(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_NAME_ENTRY_get_object)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_name_entry_get_data(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_NAME_ENTRY_get_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_get_ext_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509v3_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_get_ext_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509v3_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509v3_get_ext_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509v3_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509v3_get_ext_by_critical(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509v3_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509v3_get_ext(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509v3_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509v3_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509v3_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509v3_add_ext(a0: *mut *mut APTR, a1: *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).X509v3_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get_ext_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_ext_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get_ext_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get_ext_by_critical(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get_ext(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get_ext_d2i(a0: *const APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).X509_get_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_add1_ext_i2d(a0: *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).X509_add1_ext_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_crl_get_ext_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get_ext_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_CRL_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_get_ext_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_CRL_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_get_ext_by_critical(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_CRL_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_get_ext(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_CRL_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_CRL_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_CRL_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_get_ext_d2i(a0: *const APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).X509_CRL_get_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_crl_add1_ext_i2d(a0: *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).X509_CRL_add1_ext_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_revoked_get_ext_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_REVOKED_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_revoked_get_ext_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_REVOKED_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_revoked_get_ext_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_REVOKED_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_revoked_get_ext_by_critical(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_REVOKED_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_revoked_get_ext(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_REVOKED_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_revoked_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_REVOKED_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_revoked_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_REVOKED_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_revoked_get_ext_d2i(a0: *const APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).X509_REVOKED_get_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_revoked_add1_ext_i2d(a0: *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).X509_REVOKED_add1_ext_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_extension_create_by_nid(a0: *mut *mut APTR, a1: i32, a2: i32, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_EXTENSION_create_by_NID)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_extension_create_by_obj(a0: *mut *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_EXTENSION_create_by_OBJ)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_extension_set_object(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_EXTENSION_set_object)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_extension_set_critical(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_EXTENSION_set_critical)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_extension_set_data(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_EXTENSION_set_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_extension_get_object(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_EXTENSION_get_object)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_extension_get_data(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_EXTENSION_get_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_extension_get_critical(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_EXTENSION_get_critical)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509at_get_attr_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509at_get_attr_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509at_get_attr_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509at_get_attr_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509at_get_attr_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509at_get_attr_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509at_get_attr(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509at_get_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509at_delete_attr(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509at_delete_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509at_add1_attr(a0: *mut *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509at_add1_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509at_add1_attr_by_obj(a0: *mut *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> *mut APTR { ((*IAmiSSL).X509at_add1_attr_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509at_add1_attr_by_nid(a0: *mut *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32) -> *mut APTR { ((*IAmiSSL).X509at_add1_attr_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509at_add1_attr_by_txt(a0: *mut *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> *mut APTR { ((*IAmiSSL).X509at_add1_attr_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_attribute_create_by_nid(a0: *mut *mut APTR, a1: i32, a2: i32, a3: *const (), a4: i32) -> *mut APTR { ((*IAmiSSL).X509_ATTRIBUTE_create_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_attribute_create_by_obj(a0: *mut *mut APTR, a1: *const APTR, a2: i32, a3: *const (), a4: i32) -> *mut APTR { ((*IAmiSSL).X509_ATTRIBUTE_create_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_attribute_create_by_txt(a0: *mut *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> *mut APTR { ((*IAmiSSL).X509_ATTRIBUTE_create_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_attribute_set1_object(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_ATTRIBUTE_set1_object)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_attribute_set1_data(a0: *mut APTR, a1: i32, a2: *const (), a3: i32) -> i32 { ((*IAmiSSL).X509_ATTRIBUTE_set1_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_attribute_get0_data(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> *mut () { ((*IAmiSSL).X509_ATTRIBUTE_get0_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_attribute_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_ATTRIBUTE_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_attribute_get0_object(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_ATTRIBUTE_get0_object)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_attribute_get0_type(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_ATTRIBUTE_get0_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_cert(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_verify_cert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_find_by_issuer_and_serial(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_find_by_issuer_and_serial)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_find_by_subject(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_find_by_subject)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pbeparam_new() -> *mut APTR { ((*IAmiSSL).PBEPARAM_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pbeparam_free(a0: *mut APTR) { ((*IAmiSSL).PBEPARAM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pbeparam(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PBEPARAM)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pbeparam(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PBEPARAM)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pbeparam_it() -> *const APTR { ((*IAmiSSL).PBEPARAM_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pbe2_param_new() -> *mut APTR { ((*IAmiSSL).PBE2PARAM_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pbe2_param_free(a0: *mut APTR) { ((*IAmiSSL).PBE2PARAM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pbe2_param(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PBE2PARAM)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pbe2_param(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PBE2PARAM)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pbe2_param_it() -> *const APTR { ((*IAmiSSL).PBE2PARAM_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pbkdf2_param_new() -> *mut APTR { ((*IAmiSSL).PBKDF2PARAM_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pbkdf2_param_free(a0: *mut APTR) { ((*IAmiSSL).PBKDF2PARAM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pbkdf2_param(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PBKDF2PARAM)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pbkdf2_param(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PBKDF2PARAM)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pbkdf2_param_it() -> *const APTR { ((*IAmiSSL).PBKDF2PARAM_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe_set(a0: i32, a1: i32, a2: *const u8, a3: i32) -> *mut APTR { ((*IAmiSSL).PKCS5_pbe_set)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe2_set(a0: *const APTR, a1: i32, a2: *mut u8, a3: i32) -> *mut APTR { ((*IAmiSSL).PKCS5_pbe2_set)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs8_priv_key_info_new() -> *mut APTR { ((*IAmiSSL).PKCS8_PRIV_KEY_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs8_priv_key_info_free(a0: *mut APTR) { ((*IAmiSSL).PKCS8_PRIV_KEY_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkcs8_priv_key_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKCS8_PRIV_KEY_INFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs8_priv_key_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS8_PRIV_KEY_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs8_priv_key_info_it() -> *const APTR { ((*IAmiSSL).PKCS8_PRIV_KEY_INFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkcs82_pkey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKCS82PKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey2_pkcs8(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY2PKCS8)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_check_trust(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_check_trust)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_trust_get_count() -> i32 { ((*IAmiSSL).X509_TRUST_get_count)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_trust_get0(a0: i32) -> *mut APTR { ((*IAmiSSL).X509_TRUST_get0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_trust_get_by_id(a0: i32) -> i32 { ((*IAmiSSL).X509_TRUST_get_by_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_trust_add(a0: i32, a1: i32, a2: APTR, a3: *const APTR, a4: i32, a5: *mut ()) -> i32 { ((*IAmiSSL).X509_TRUST_add)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_x509_trust_cleanup() { ((*IAmiSSL).X509_TRUST_cleanup)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_trust_get_flags(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_TRUST_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_trust_get0_name(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_TRUST_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_trust_get_trust(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_TRUST_get_trust)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_x509_strings() -> i32 { ((*IAmiSSL).ERR_load_X509_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_object_idx_by_subject(a0: *mut APTR, a1: APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_OBJECT_idx_by_subject)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_object_retrieve_by_subject(a0: *mut APTR, a1: APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_OBJECT_retrieve_by_subject)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_object_retrieve_match(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_OBJECT_retrieve_match)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_object_up_ref_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_OBJECT_up_ref_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_new() -> *mut APTR { ((*IAmiSSL).X509_STORE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_store_free(a0: *mut APTR) { ((*IAmiSSL).X509_STORE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_flags(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).X509_STORE_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_set_purpose(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_STORE_set_purpose)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_set_trust(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_STORE_set_trust)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_new() -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get1_issuer(a0: *mut *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_get1_issuer)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_free(a0: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_init(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_init)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set0_trusted_stack(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set0_trusted_stack)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_cleanup(a0: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_add_lookup(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_add_lookup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_hash_dir() -> *mut APTR { ((*IAmiSSL).X509_LOOKUP_hash_dir)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_lookup_file() -> *mut APTR { ((*IAmiSSL).X509_LOOKUP_file)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_store_add_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_add_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_add_crl(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_add_crl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_by_subject(a0: *const APTR, a1: APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_get_by_subject)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_lookup_ctrl(a0: *mut APTR, a1: i32, a2: *const APTR, a3: i32, a4: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_ctrl)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_load_cert_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_load_cert_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_load_crl_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_load_crl_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_load_cert_crl_file(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_load_cert_crl_file)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_lookup_new(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_LOOKUP_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_free(a0: *mut APTR) { ((*IAmiSSL).X509_LOOKUP_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_by_subject(a0: *mut APTR, a1: APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_by_subject)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_lookup_by_issuer_serial(a0: *mut APTR, a1: APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_by_issuer_serial)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_lookup_by_fingerprint(a0: *mut APTR, a1: APTR, a2: *const u8, a3: i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_by_fingerprint)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_lookup_by_alias(a0: *mut APTR, a1: APTR, a2: *const APTR, a3: i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_by_alias)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_lookup_shutdown(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_shutdown)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_load_locations(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_load_locations)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_set_default_paths(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_set_default_paths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_x509_store_ctx_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_X509_STORE_CTX_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).X509_STORE_CTX_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).X509_STORE_CTX_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_error(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_get_error)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_error(a0: *mut APTR, a1: i32) { ((*IAmiSSL).X509_STORE_CTX_set_error)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_error_depth(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_get_error_depth)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_current_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get_current_cert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_chain(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_chain)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get1_chain(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get1_chain)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_cert(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_purpose(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_STORE_CTX_set_purpose)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_trust(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_STORE_CTX_set_trust)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_purpose_inherit(a0: *mut APTR, a1: i32, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).X509_STORE_CTX_purpose_inherit)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).X509_STORE_CTX_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_time(a0: *mut APTR, a1: u32, a2: APTR) { ((*IAmiSSL).X509_STORE_CTX_set_time)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_verify_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_CTX_set_verify_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_basic_constraints_new() -> *mut APTR { ((*IAmiSSL).BASIC_CONSTRAINTS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_basic_constraints_free(a0: *mut APTR) { ((*IAmiSSL).BASIC_CONSTRAINTS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_basic_constraints(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_BASIC_CONSTRAINTS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_basic_constraints(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_BASIC_CONSTRAINTS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_basic_constraints_it() -> *const APTR { ((*IAmiSSL).BASIC_CONSTRAINTS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_sxnet_new() -> *mut APTR { ((*IAmiSSL).SXNET_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_sxnet_free(a0: *mut APTR) { ((*IAmiSSL).SXNET_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_sxnet(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_SXNET)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_sxnet(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_SXNET)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sxnet_it() -> *const APTR { ((*IAmiSSL).SXNET_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_sxnetid_new() -> *mut APTR { ((*IAmiSSL).SXNETID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_sxnetid_free(a0: *mut APTR) { ((*IAmiSSL).SXNETID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_sxnetid(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_SXNETID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_sxnetid(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_SXNETID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sxnetid_it() -> *const APTR { ((*IAmiSSL).SXNETID_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_sxnet_add_id_asc(a0: *mut *mut APTR, a1: *const APTR, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).SXNET_add_id_asc)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_sxnet_add_id_ulong(a0: *mut *mut APTR, a1: u32, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).SXNET_add_id_ulong)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_sxnet_add_id_integer(a0: *mut *mut APTR, a1: *mut APTR, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).SXNET_add_id_INTEGER)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_sxnet_get_id_asc(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).SXNET_get_id_asc)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sxnet_get_id_ulong(a0: *mut APTR, a1: u32) -> *mut APTR { ((*IAmiSSL).SXNET_get_id_ulong)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sxnet_get_id_integer(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).SXNET_get_id_INTEGER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_authority_keyid_new() -> *mut APTR { ((*IAmiSSL).AUTHORITY_KEYID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_authority_keyid_free(a0: *mut APTR) { ((*IAmiSSL).AUTHORITY_KEYID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_authority_keyid(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_AUTHORITY_KEYID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_authority_keyid(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_AUTHORITY_KEYID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_authority_keyid_it() -> *const APTR { ((*IAmiSSL).AUTHORITY_KEYID_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkey_usage_period_new() -> *mut APTR { ((*IAmiSSL).PKEY_USAGE_PERIOD_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkey_usage_period_free(a0: *mut APTR) { ((*IAmiSSL).PKEY_USAGE_PERIOD_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pkey_usage_period(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PKEY_USAGE_PERIOD)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkey_usage_period(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKEY_USAGE_PERIOD)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkey_usage_period_it() -> *const APTR { ((*IAmiSSL).PKEY_USAGE_PERIOD_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_general_name_new() -> *mut APTR { ((*IAmiSSL).GENERAL_NAME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_general_name_free(a0: *mut APTR) { ((*IAmiSSL).GENERAL_NAME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_general_name(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_GENERAL_NAME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_general_name(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_GENERAL_NAME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_general_name_it() -> *const APTR { ((*IAmiSSL).GENERAL_NAME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_i2v_general_name(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).i2v_GENERAL_NAME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_general_name_print(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).GENERAL_NAME_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_general_names_new() -> *mut APTR { ((*IAmiSSL).GENERAL_NAMES_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_general_names_free(a0: *mut APTR) { ((*IAmiSSL).GENERAL_NAMES_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_general_names(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_GENERAL_NAMES)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_general_names(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_GENERAL_NAMES)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_general_names_it() -> *const APTR { ((*IAmiSSL).GENERAL_NAMES_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_i2v_general_names(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).i2v_GENERAL_NAMES)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_v2i_general_names(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).v2i_GENERAL_NAMES)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_othername_new() -> *mut APTR { ((*IAmiSSL).OTHERNAME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_othername_free(a0: *mut APTR) { ((*IAmiSSL).OTHERNAME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_othername(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OTHERNAME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_othername(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OTHERNAME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_othername_it() -> *const APTR { ((*IAmiSSL).OTHERNAME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_edipartyname_new() -> *mut APTR { ((*IAmiSSL).EDIPARTYNAME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_edipartyname_free(a0: *mut APTR) { ((*IAmiSSL).EDIPARTYNAME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_edipartyname(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_EDIPARTYNAME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_edipartyname(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_EDIPARTYNAME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_edipartyname_it() -> *const APTR { ((*IAmiSSL).EDIPARTYNAME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_i2s_asn1_octet_string(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).i2s_ASN1_OCTET_STRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_s2i_asn1_octet_string(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).s2i_ASN1_OCTET_STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_extended_key_usage_new() -> *mut APTR { ((*IAmiSSL).EXTENDED_KEY_USAGE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_extended_key_usage_free(a0: *mut APTR) { ((*IAmiSSL).EXTENDED_KEY_USAGE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_extended_key_usage(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_EXTENDED_KEY_USAGE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_extended_key_usage(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_EXTENDED_KEY_USAGE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_extended_key_usage_it() -> *const APTR { ((*IAmiSSL).EXTENDED_KEY_USAGE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_i2a_access_description(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2a_ACCESS_DESCRIPTION)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_certificatepolicies_new() -> *mut APTR { ((*IAmiSSL).CERTIFICATEPOLICIES_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_certificatepolicies_free(a0: *mut APTR) { ((*IAmiSSL).CERTIFICATEPOLICIES_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_certificatepolicies(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_CERTIFICATEPOLICIES)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_certificatepolicies(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_CERTIFICATEPOLICIES)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_certificatepolicies_it() -> *const APTR { ((*IAmiSSL).CERTIFICATEPOLICIES_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_policyinfo_new() -> *mut APTR { ((*IAmiSSL).POLICYINFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_policyinfo_free(a0: *mut APTR) { ((*IAmiSSL).POLICYINFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_policyinfo(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_POLICYINFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_policyinfo(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_POLICYINFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_policyinfo_it() -> *const APTR { ((*IAmiSSL).POLICYINFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_policyqualinfo_new() -> *mut APTR { ((*IAmiSSL).POLICYQUALINFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_policyqualinfo_free(a0: *mut APTR) { ((*IAmiSSL).POLICYQUALINFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_policyqualinfo(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_POLICYQUALINFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_policyqualinfo(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_POLICYQUALINFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_policyqualinfo_it() -> *const APTR { ((*IAmiSSL).POLICYQUALINFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_usernotice_new() -> *mut APTR { ((*IAmiSSL).USERNOTICE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_usernotice_free(a0: *mut APTR) { ((*IAmiSSL).USERNOTICE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_usernotice(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_USERNOTICE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_usernotice(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_USERNOTICE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_usernotice_it() -> *const APTR { ((*IAmiSSL).USERNOTICE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_noticeref_new() -> *mut APTR { ((*IAmiSSL).NOTICEREF_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_noticeref_free(a0: *mut APTR) { ((*IAmiSSL).NOTICEREF_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_noticeref(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_NOTICEREF)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_noticeref(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_NOTICEREF)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_noticeref_it() -> *const APTR { ((*IAmiSSL).NOTICEREF_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crl_dist_points_new() -> *mut APTR { ((*IAmiSSL).CRL_DIST_POINTS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crl_dist_points_free(a0: *mut APTR) { ((*IAmiSSL).CRL_DIST_POINTS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_crl_dist_points(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_CRL_DIST_POINTS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_crl_dist_points(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_CRL_DIST_POINTS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crl_dist_points_it() -> *const APTR { ((*IAmiSSL).CRL_DIST_POINTS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dist_point_new() -> *mut APTR { ((*IAmiSSL).DIST_POINT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dist_point_free(a0: *mut APTR) { ((*IAmiSSL).DIST_POINT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_dist_point(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DIST_POINT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_dist_point(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DIST_POINT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dist_point_it() -> *const APTR { ((*IAmiSSL).DIST_POINT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dist_point_name_new() -> *mut APTR { ((*IAmiSSL).DIST_POINT_NAME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dist_point_name_free(a0: *mut APTR) { ((*IAmiSSL).DIST_POINT_NAME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_dist_point_name(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DIST_POINT_NAME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_dist_point_name(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DIST_POINT_NAME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dist_point_name_it() -> *const APTR { ((*IAmiSSL).DIST_POINT_NAME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_access_description_new() -> *mut APTR { ((*IAmiSSL).ACCESS_DESCRIPTION_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_access_description_free(a0: *mut APTR) { ((*IAmiSSL).ACCESS_DESCRIPTION_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_access_description(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ACCESS_DESCRIPTION)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_access_description(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ACCESS_DESCRIPTION)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_access_description_it() -> *const APTR { ((*IAmiSSL).ACCESS_DESCRIPTION_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_authority_info_access_new() -> *mut APTR { ((*IAmiSSL).AUTHORITY_INFO_ACCESS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_authority_info_access_free(a0: *mut APTR) { ((*IAmiSSL).AUTHORITY_INFO_ACCESS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_authority_info_access(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_AUTHORITY_INFO_ACCESS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_authority_info_access(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_AUTHORITY_INFO_ACCESS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_authority_info_access_it() -> *const APTR { ((*IAmiSSL).AUTHORITY_INFO_ACCESS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_v2i_general_name(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).v2i_GENERAL_NAME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_v3_conf_free(a0: *mut APTR) { ((*IAmiSSL).X509V3_conf_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_nconf_nid(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).X509V3_EXT_nconf_nid)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_nconf(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).X509V3_EXT_nconf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_add_nconf_sk(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_add_nconf_sk)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_add_nconf(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_add_nconf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_req_add_nconf(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_REQ_add_nconf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_crl_add_nconf(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_CRL_add_nconf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_conf_nid(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).X509V3_EXT_conf_nid)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_conf(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).X509V3_EXT_conf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_add_conf(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_add_conf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_req_add_conf(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_REQ_add_conf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_crl_add_conf(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_CRL_add_conf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_add_value_bool_nf(a0: *const APTR, a1: i32, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509V3_add_value_bool_nf)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_v3_get_value_bool(a0: *const APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).X509V3_get_value_bool)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_get_value_int(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509V3_get_value_int)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_set_nconf(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509V3_set_nconf)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_set_conf_lhash(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509V3_set_conf_lhash)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_get_string(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).X509V3_get_string)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_v3_get_section(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509V3_get_section)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_string_free(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509V3_string_free)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_section_free(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509V3_section_free)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_set_ctx(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: i32) { ((*IAmiSSL).X509V3_set_ctx)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_x509_v3_add_value(a0: *const APTR, a1: *const APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509V3_add_value)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_v3_add_value_uchar(a0: *const APTR, a1: *const u8, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509V3_add_value_uchar)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_v3_add_value_bool(a0: *const APTR, a1: i32, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509V3_add_value_bool)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_v3_add_value_int(a0: *const APTR, a1: *const APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509V3_add_value_int)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2s_asn1_integer(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).i2s_ASN1_INTEGER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_s2i_asn1_integer(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).s2i_ASN1_INTEGER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2s_asn1_enumerated(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).i2s_ASN1_ENUMERATED)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2s_asn1_enumerated_table(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).i2s_ASN1_ENUMERATED_TABLE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_add(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_add)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_add_list(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_EXT_add_list)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_add_alias(a0: i32, a1: i32) -> i32 { ((*IAmiSSL).X509V3_EXT_add_alias)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_cleanup() { ((*IAmiSSL).X509V3_EXT_cleanup)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_get(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).X509V3_EXT_get)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_get_nid(a0: i32) -> *const APTR { ((*IAmiSSL).X509V3_EXT_get_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_v3_add_standard_extensions() -> i32 { ((*IAmiSSL).X509V3_add_standard_extensions)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_v3_parse_list(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509V3_parse_list)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_d2i(a0: *mut APTR) -> *mut () { ((*IAmiSSL).X509V3_EXT_d2i)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_v3_get_d2i(a0: *const APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).X509V3_get_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_i2d(a0: i32, a1: i32, a2: *mut ()) -> *mut APTR { ((*IAmiSSL).X509V3_EXT_i2d)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_v3_add1_i2d(a0: *mut *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).X509V3_add1_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_openssl_buf2hexstr(a0: *const u8, a1: i32) -> *mut APTR { ((*IAmiSSL).OPENSSL_buf2hexstr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_hexstr2buf(a0: *const APTR, a1: *mut i32) -> *mut u8 { ((*IAmiSSL).OPENSSL_hexstr2buf)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_val_prn(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: i32) { ((*IAmiSSL).X509V3_EXT_val_prn)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_ext_print(a0: *mut APTR, a1: *mut APTR, a2: u32, a3: i32) -> i32 { ((*IAmiSSL).X509V3_EXT_print)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_v3_extensions_print(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: u32, a4: i32) -> i32 { ((*IAmiSSL).X509V3_extensions_print)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_check_purpose(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_check_purpose)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_supported_extension(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_supported_extension)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_purpose_set(a0: *mut i32, a1: i32) -> i32 { ((*IAmiSSL).X509_PURPOSE_set)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_check_issued(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_check_issued)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_purpose_get_count() -> i32 { ((*IAmiSSL).X509_PURPOSE_get_count)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_purpose_get0(a0: i32) -> *mut APTR { ((*IAmiSSL).X509_PURPOSE_get0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_purpose_get_by_sname(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_PURPOSE_get_by_sname)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_purpose_get_by_id(a0: i32) -> i32 { ((*IAmiSSL).X509_PURPOSE_get_by_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_purpose_add(a0: i32, a1: i32, a2: i32, a3: APTR, a4: *const APTR, a5: *const APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).X509_PURPOSE_add)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_x509_purpose_get0_name(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_PURPOSE_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_purpose_get0_sname(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_PURPOSE_get0_sname)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_purpose_get_trust(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_PURPOSE_get_trust)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_purpose_cleanup() { ((*IAmiSSL).X509_PURPOSE_cleanup)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_purpose_get_id(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_PURPOSE_get_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get1_email(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_get1_email)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get1_email(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_get1_email)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_email_free(a0: *mut APTR) { ((*IAmiSSL).X509_email_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_x509_v3_strings() -> i32 { ((*IAmiSSL).ERR_load_X509V3_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_aes_options() -> *const APTR { ((*IAmiSSL).AES_options)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_aes_set_encrypt_key(a0: *const u8, a1: APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).AES_set_encrypt_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_aes_set_decrypt_key(a0: *const u8, a1: APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).AES_set_decrypt_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_aes_encrypt(a0: *const u8, a1: *mut u8, a2: *const APTR) { ((*IAmiSSL).AES_encrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_aes_decrypt(a0: *const u8, a1: *mut u8, a2: *const APTR) { ((*IAmiSSL).AES_decrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_aes_ecb_encrypt(a0: *const u8, a1: *mut u8, a2: *const APTR, a3: APTR) { ((*IAmiSSL).AES_ecb_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_aes_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: APTR) { ((*IAmiSSL).AES_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_aes_cfb128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: APTR) { ((*IAmiSSL).AES_cfb128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_aes_cfb1_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: APTR) { ((*IAmiSSL).AES_cfb1_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_aes_cfb8_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: APTR) { ((*IAmiSSL).AES_cfb8_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_aes_ofb128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32) { ((*IAmiSSL).AES_ofb128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bf_set_key(a0: *mut APTR, a1: i32, a2: *const u8) { ((*IAmiSSL).BF_set_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bf_encrypt(a0: *mut APTR, a1: *const APTR) { ((*IAmiSSL).BF_encrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bf_decrypt(a0: *mut APTR, a1: *const APTR) { ((*IAmiSSL).BF_decrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bf_ecb_encrypt(a0: *const u8, a1: *mut u8, a2: *const APTR, a3: i32) { ((*IAmiSSL).BF_ecb_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bf_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *const APTR, a4: *mut u8, a5: i32) { ((*IAmiSSL).BF_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bf_cfb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: i32) { ((*IAmiSSL).BF_cfb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_bf_ofb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *const APTR, a4: *mut u8, a5: *mut i32) { ((*IAmiSSL).BF_ofb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bf_options() -> *const APTR { ((*IAmiSSL).BF_options)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cast_set_key(a0: *mut APTR, a1: i32, a2: *const u8) { ((*IAmiSSL).CAST_set_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cast_ecb_encrypt(a0: *const u8, a1: *mut u8, a2: *const APTR, a3: i32) { ((*IAmiSSL).CAST_ecb_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cast_encrypt(a0: *mut APTR, a1: *const APTR) { ((*IAmiSSL).CAST_encrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cast_decrypt(a0: *mut APTR, a1: *const APTR) { ((*IAmiSSL).CAST_decrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cast_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *const APTR, a4: *mut u8, a5: i32) { ((*IAmiSSL).CAST_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_cast_cfb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: i32) { ((*IAmiSSL).CAST_cfb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_cast_ofb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *const APTR, a4: *mut u8, a5: *mut i32) { ((*IAmiSSL).CAST_ofb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_des_options() -> *const APTR { ((*IAmiSSL).DES_options)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_des_ecb3_encrypt(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: i32) { ((*IAmiSSL).DES_ecb3_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_des_cbc_cksum(a0: *const u8, a1: *mut APTR, a2: i32, a3: *mut APTR, a4: *mut APTR) -> APTR { ((*IAmiSSL).DES_cbc_cksum)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_des_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: i32) { ((*IAmiSSL).DES_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_des_ncbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: i32) { ((*IAmiSSL).DES_ncbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_des_xcbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR, a6: *mut APTR, a7: i32) { ((*IAmiSSL).DES_xcbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_des_cfb_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: i32, a4: *mut APTR, a5: *mut APTR, a6: i32) { ((*IAmiSSL).DES_cfb_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_des_ecb_encrypt(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) { ((*IAmiSSL).DES_ecb_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_des_encrypt1(a0: *mut APTR, a1: *mut APTR, a2: i32) { ((*IAmiSSL).DES_encrypt1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_des_encrypt2(a0: *mut APTR, a1: *mut APTR, a2: i32) { ((*IAmiSSL).DES_encrypt2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_des_encrypt3(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) { ((*IAmiSSL).DES_encrypt3)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_des_decrypt3(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) { ((*IAmiSSL).DES_decrypt3)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_des_ede3_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR, a6: *mut APTR, a7: i32) { ((*IAmiSSL).DES_ede3_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_des_ede3_cfb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR, a6: *mut APTR, a7: *mut i32, a8: i32) { ((*IAmiSSL).DES_ede3_cfb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_des_ede3_cfb_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: i32, a4: *mut APTR, a5: *mut APTR, a6: *mut APTR, a7: *mut APTR, a8: i32) { ((*IAmiSSL).DES_ede3_cfb_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_des_ede3_ofb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR, a6: *mut APTR, a7: *mut i32) { ((*IAmiSSL).DES_ede3_ofb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_des_fcrypt(a0: *const APTR, a1: *const APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).DES_fcrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_des_crypt(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).DES_crypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_des_ofb_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: i32, a4: *mut APTR, a5: *mut APTR) { ((*IAmiSSL).DES_ofb_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_des_pcbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: i32) { ((*IAmiSSL).DES_pcbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_des_quad_cksum(a0: *const u8, a1: *mut APTR, a2: i32, a3: i32, a4: *mut APTR) -> APTR { ((*IAmiSSL).DES_quad_cksum)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_des_random_key(a0: *mut APTR) -> i32 { ((*IAmiSSL).DES_random_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_des_set_odd_parity(a0: *mut APTR) { ((*IAmiSSL).DES_set_odd_parity)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_des_check_key_parity(a0: *mut APTR) -> i32 { ((*IAmiSSL).DES_check_key_parity)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_des_is_weak_key(a0: *mut APTR) -> i32 { ((*IAmiSSL).DES_is_weak_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_des_set_key(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).DES_set_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_des_key_sched(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).DES_key_sched)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_des_set_key_checked(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).DES_set_key_checked)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_des_set_key_unchecked(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).DES_set_key_unchecked)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_des_string_to_key(a0: *const APTR, a1: *mut APTR) { ((*IAmiSSL).DES_string_to_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_des_string_to_2keys(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) { ((*IAmiSSL).DES_string_to_2keys)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_des_cfb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: *mut i32, a6: i32) { ((*IAmiSSL).DES_cfb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_des_ofb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: *mut i32) { ((*IAmiSSL).DES_ofb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_dh_open_ssl() -> *const APTR { ((*IAmiSSL).DH_OpenSSL)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dh_set_default_method(a0: *const APTR) { ((*IAmiSSL).DH_set_default_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_get_default_method() -> *const APTR { ((*IAmiSSL).DH_get_default_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dh_set_method(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).DH_set_method)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_new_method(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).DH_new_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_new() -> *mut APTR { ((*IAmiSSL).DH_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dh_free(a0: *mut APTR) { ((*IAmiSSL).DH_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).DH_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_size(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_dh_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_DH_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_dh_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).DH_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dh_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).DH_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_generate_parameters(a0: i32, a1: i32, a2: APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).DH_generate_parameters)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dh_check(a0: *const APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).DH_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_generate_key(a0: *mut APTR) -> i32 { ((*IAmiSSL).DH_generate_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_compute_key(a0: *mut u8, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).DH_compute_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_dhparams(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DHparams)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_dhparams(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DHparams)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dhparams_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).DHparams_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_load_dh_strings() -> i32 { ((*IAmiSSL).ERR_load_DH_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dsa_sig_new() -> *mut APTR { ((*IAmiSSL).DSA_SIG_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dsa_sig_free(a0: *mut APTR) { ((*IAmiSSL).DSA_SIG_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_dsa_sig(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DSA_SIG)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_dsa_sig(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DSA_SIG)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dsa_do_sign(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).DSA_do_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dsa_do_verify(a0: *const u8, a1: i32, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).DSA_do_verify)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dsa_open_ssl() -> *const APTR { ((*IAmiSSL).DSA_OpenSSL)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dsa_set_default_method(a0: *const APTR) { ((*IAmiSSL).DSA_set_default_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_get_default_method() -> *const APTR { ((*IAmiSSL).DSA_get_default_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dsa_set_method(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).DSA_set_method)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_new() -> *mut APTR { ((*IAmiSSL).DSA_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dsa_new_method(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).DSA_new_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_free(a0: *mut APTR) { ((*IAmiSSL).DSA_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).DSA_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_size(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_sign_setup(a0: *mut APTR, a1: *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).DSA_sign_setup)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dsa_sign(a0: i32, a1: *const u8, a2: i32, a3: *mut u8, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).DSA_sign)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_dsa_verify(a0: i32, a1: *const u8, a2: i32, a3: *const u8, a4: i32, a5: *mut APTR) -> i32 { ((*IAmiSSL).DSA_verify)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_obsolete_dsa_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_DSA_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_dsa_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).DSA_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dsa_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).DSA_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_dsapublic_key(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DSAPublicKey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_dsaprivate_key(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DSAPrivateKey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_dsaparams(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DSAparams)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dsa_generate_parameters(a0: i32, a1: *mut u8, a2: i32, a3: *mut i32, a4: *mut u32, a5: APTR, a6: *mut ()) -> *mut APTR { ((*IAmiSSL).DSA_generate_parameters)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_dsa_generate_key(a0: *mut APTR) -> i32 { ((*IAmiSSL).DSA_generate_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_dsapublic_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DSAPublicKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_dsaprivate_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DSAPrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_dsaparams(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DSAparams)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsaparams_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).DSAparams_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_print(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).DSA_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dsa_dup_dh(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).DSA_dup_DH)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_dsa_strings() -> i32 { ((*IAmiSSL).ERR_load_DSA_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_idea_options() -> *const APTR { ((*IAmiSSL).IDEA_options)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_idea_ecb_encrypt(a0: *const u8, a1: *mut u8, a2: *mut APTR) { ((*IAmiSSL).IDEA_ecb_encrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_idea_set_encrypt_key(a0: *const u8, a1: *mut APTR) { ((*IAmiSSL).IDEA_set_encrypt_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_idea_set_decrypt_key(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).IDEA_set_decrypt_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_idea_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: i32) { ((*IAmiSSL).IDEA_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_idea_cfb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: *mut i32, a6: i32) { ((*IAmiSSL).IDEA_cfb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_idea_ofb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: *mut i32) { ((*IAmiSSL).IDEA_ofb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_idea_encrypt(a0: *mut u32, a1: *mut APTR) { ((*IAmiSSL).IDEA_encrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_md2_options() -> *const APTR { ((*IAmiSSL).MD2_options)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_md2_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).MD2_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_md2_update(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).MD2_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_md2_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).MD2_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_md2(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).MD2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_md4_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).MD4_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_md4_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).MD4_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_md4_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).MD4_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_md4(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).MD4)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_md4_transform(a0: *mut APTR, a1: *const u8) { ((*IAmiSSL).MD4_Transform)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_md5_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).MD5_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_md5_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).MD5_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_md5_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).MD5_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_md5(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).MD5)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_md5_transform(a0: *mut APTR, a1: *const u8) { ((*IAmiSSL).MD5_Transform)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_mdc2_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).MDC2_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_mdc2_update(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).MDC2_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_mdc2_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).MDC2_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_mdc2(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).MDC2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rc2_set_key(a0: *mut APTR, a1: i32, a2: *const u8, a3: i32) { ((*IAmiSSL).RC2_set_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rc2_ecb_encrypt(a0: *const u8, a1: *mut u8, a2: *mut APTR, a3: i32) { ((*IAmiSSL).RC2_ecb_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rc2_encrypt(a0: *mut u32, a1: *mut APTR) { ((*IAmiSSL).RC2_encrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rc2_decrypt(a0: *mut u32, a1: *mut APTR) { ((*IAmiSSL).RC2_decrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rc2_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: i32) { ((*IAmiSSL).RC2_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rc2_cfb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: *mut i32, a6: i32) { ((*IAmiSSL).RC2_cfb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_rc2_ofb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: *mut i32) { ((*IAmiSSL).RC2_ofb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rc4_options() -> *const APTR { ((*IAmiSSL).RC4_options)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rc4_set_key(a0: *mut APTR, a1: i32, a2: *const u8) { ((*IAmiSSL).RC4_set_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rc4(a0: *mut APTR, a1: u32, a2: *const u8, a3: *mut u8) { ((*IAmiSSL).RC4)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rc5_32_set_key(a0: *mut APTR, a1: i32, a2: *const u8, a3: i32) -> i32 { ((*IAmiSSL).RC5_32_set_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rc5_32_ecb_encrypt(a0: *const u8, a1: *mut u8, a2: *mut APTR, a3: i32) { ((*IAmiSSL).RC5_32_ecb_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rc5_32_encrypt(a0: *mut u32, a1: *mut APTR) { ((*IAmiSSL).RC5_32_encrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rc5_32_decrypt(a0: *mut u32, a1: *mut APTR) { ((*IAmiSSL).RC5_32_decrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rc5_32_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: i32) { ((*IAmiSSL).RC5_32_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rc5_32_cfb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: *mut i32, a6: i32) { ((*IAmiSSL).RC5_32_cfb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_rc5_32_ofb64_encrypt(a0: *const u8, a1: *mut u8, a2: i32, a3: *mut APTR, a4: *mut u8, a5: *mut i32) { ((*IAmiSSL).RC5_32_ofb64_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ripemd160_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).RIPEMD160_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ripemd160_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).RIPEMD160_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ripemd160_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).RIPEMD160_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ripemd160(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).RIPEMD160)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ripemd160_transform(a0: *mut APTR, a1: *const u8) { ((*IAmiSSL).RIPEMD160_Transform)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_new() -> *mut APTR { ((*IAmiSSL).RSA_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_new_method(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).RSA_new_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_size(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_generate_key(a0: i32, a1: u32, a2: APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).RSA_generate_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_check_key(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_check_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_public_encrypt(a0: i32, a1: *const u8, a2: *mut u8, a3: *mut APTR, a4: i32) -> i32 { ((*IAmiSSL).RSA_public_encrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_private_encrypt(a0: i32, a1: *const u8, a2: *mut u8, a3: *mut APTR, a4: i32) -> i32 { ((*IAmiSSL).RSA_private_encrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_public_decrypt(a0: i32, a1: *const u8, a2: *mut u8, a3: *mut APTR, a4: i32) -> i32 { ((*IAmiSSL).RSA_public_decrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_private_decrypt(a0: i32, a1: *const u8, a2: *mut u8, a3: *mut APTR, a4: i32) -> i32 { ((*IAmiSSL).RSA_private_decrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_free(a0: *mut APTR) { ((*IAmiSSL).RSA_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).RSA_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_flags(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_set_default_method(a0: *const APTR) { ((*IAmiSSL).RSA_set_default_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get_default_method() -> *const APTR { ((*IAmiSSL).RSA_get_default_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_get_method(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_set_method(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).RSA_set_method)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_null_method() -> *const APTR { ((*IAmiSSL).RSA_null_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_rsapublic_key(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_RSAPublicKey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_rsapublic_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_RSAPublicKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsapublic_key_it() -> *const APTR { ((*IAmiSSL).RSAPublicKey_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_rsaprivate_key(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_RSAPrivateKey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_rsaprivate_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_RSAPrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsaprivate_key_it() -> *const APTR { ((*IAmiSSL).RSAPrivateKey_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_print(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).RSA_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rsa_sign(a0: i32, a1: *const u8, a2: APTR, a3: *mut u8, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).RSA_sign)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rsa_verify(a0: i32, a1: *const u8, a2: APTR, a3: *const u8, a4: APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).RSA_verify)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rsa_sign_asn1_octet_string(a0: i32, a1: *const u8, a2: APTR, a3: *mut u8, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).RSA_sign_ASN1_OCTET_STRING)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rsa_verify_asn1_octet_string(a0: i32, a1: *const u8, a2: APTR, a3: *mut u8, a4: APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).RSA_verify_ASN1_OCTET_STRING)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rsa_blinding_on(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).RSA_blinding_on)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_blinding_off(a0: *mut APTR) { ((*IAmiSSL).RSA_blinding_off)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_padding_add_pkcs1_type_1(a0: *mut u8, a1: i32, a2: *const u8, a3: i32) -> i32 { ((*IAmiSSL).RSA_padding_add_PKCS1_type_1)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_padding_check_pkcs1_type_1(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: i32) -> i32 { ((*IAmiSSL).RSA_padding_check_PKCS1_type_1)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_padding_add_pkcs1_type_2(a0: *mut u8, a1: i32, a2: *const u8, a3: i32) -> i32 { ((*IAmiSSL).RSA_padding_add_PKCS1_type_2)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_padding_check_pkcs1_type_2(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: i32) -> i32 { ((*IAmiSSL).RSA_padding_check_PKCS1_type_2)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_padding_add_pkcs1_oaep(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: *const u8, a5: i32) -> i32 { ((*IAmiSSL).RSA_padding_add_PKCS1_OAEP)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rsa_padding_check_pkcs1_oaep(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: i32, a5: *const u8, a6: i32) -> i32 { ((*IAmiSSL).RSA_padding_check_PKCS1_OAEP)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_rsa_padding_add_none(a0: *mut u8, a1: i32, a2: *const u8, a3: i32) -> i32 { ((*IAmiSSL).RSA_padding_add_none)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_padding_check_none(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: i32) -> i32 { ((*IAmiSSL).RSA_padding_check_none)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_obsolete_rsa_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_RSA_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).RSA_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rsa_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).RSA_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsapublic_key_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).RSAPublicKey_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsaprivate_key_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).RSAPrivateKey_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_rsa_strings() -> i32 { ((*IAmiSSL).ERR_load_RSA_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_sha1_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).SHA1_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sha1_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).SHA1_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sha1_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).SHA1_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sha1(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).SHA1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sha1_transform(a0: *mut APTR, a1: *const u8) { ((*IAmiSSL).SHA1_Transform)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_read_string_lib(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).UI_read_string_lib)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_write_string_lib(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).UI_write_string_lib)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_hmac_ctx_set_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).HMAC_CTX_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_check_ca(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_check_ca)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_proxy_policy_new() -> *mut APTR { ((*IAmiSSL).PROXY_POLICY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_proxy_policy_free(a0: *mut APTR) { ((*IAmiSSL).PROXY_POLICY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_proxy_policy(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PROXY_POLICY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_proxy_policy(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PROXY_POLICY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_proxy_policy_it() -> *const APTR { ((*IAmiSSL).PROXY_POLICY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_proxy_cert_info_extension_new() -> *mut APTR { ((*IAmiSSL).PROXY_CERT_INFO_EXTENSION_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_proxy_cert_info_extension_free(a0: *mut APTR) { ((*IAmiSSL).PROXY_CERT_INFO_EXTENSION_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_proxy_cert_info_extension(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PROXY_CERT_INFO_EXTENSION)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_proxy_cert_info_extension(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PROXY_CERT_INFO_EXTENSION)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_proxy_cert_info_extension_it() -> *const APTR { ((*IAmiSSL).PROXY_CERT_INFO_EXTENSION_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_mod_exp_mont_consttime(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_exp_mont_consttime)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_mont_ctx_set_locked(a0: *mut *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_MONT_CTX_set_locked)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs1_mgf1(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: *const APTR) -> i32 { ((*IAmiSSL).PKCS1_MGF1)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_padding_add_x931(a0: *mut u8, a1: i32, a2: *const u8, a3: i32) -> i32 { ((*IAmiSSL).RSA_padding_add_X931)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_padding_check_x931(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: i32) -> i32 { ((*IAmiSSL).RSA_padding_check_X931)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_x931_hash_id(a0: i32) -> i32 { ((*IAmiSSL).RSA_X931_hash_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_verify_pkcs1_pss(a0: *mut APTR, a1: *const u8, a2: *const APTR, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).RSA_verify_PKCS1_PSS)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_padding_add_pkcs1_pss(a0: *mut APTR, a1: *mut u8, a2: *const u8, a3: *const APTR, a4: i32) -> i32 { ((*IAmiSSL).RSA_padding_add_PKCS1_PSS)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_sha224() -> *const APTR { ((*IAmiSSL).EVP_sha224)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha256() -> *const APTR { ((*IAmiSSL).EVP_sha256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha384() -> *const APTR { ((*IAmiSSL).EVP_sha384)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha512() -> *const APTR { ((*IAmiSSL).EVP_sha512)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_gf2m_add(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).BN_GF2m_add)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_gf2m_arr2poly(a0: *const i32, a1: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_arr2poly)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_mul(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_mul)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_sqr(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_sqr)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_inv(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_inv)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_div(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_div)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_exp(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_exp)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_sqrt(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_sqrt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_solve_quad(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_solve_quad)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_generate_prime_ex(a0: *mut APTR, a1: i32, a2: i32, a3: *const APTR, a4: *const APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).BN_generate_prime_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_is_prime_ex(a0: *const APTR, a1: i32, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_is_prime_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_set_negative(a0: *mut APTR, a1: i32) { ((*IAmiSSL).BN_set_negative)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_get_degree(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_degree)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_gf2m_simple_method() -> *const APTR { ((*IAmiSSL).EC_GF2m_simple_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_group_set_curve_gf2m(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_set_curve_GF2m)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_group_get_curve_gf2m(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_curve_GF2m)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_set_affine_coordinates_gf2m(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_set_affine_coordinates_GF2m)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_get_affine_coordinates_gf2m(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_get_affine_coordinates_GF2m)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_get_builtin_curves(a0: *mut APTR, a1: u32) -> u32 { ((*IAmiSSL).EC_get_builtin_curves)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_new_by_curve_name(a0: i32) -> *mut APTR { ((*IAmiSSL).EC_GROUP_new_by_curve_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_check(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_generate_parameters_ex(a0: *mut APTR, a1: i32, a2: *const u8, a3: i32, a4: *mut i32, a5: *mut u32, a6: *mut APTR) -> i32 { ((*IAmiSSL).DSA_generate_parameters_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ec_key_new_by_curve_name(a0: i32) -> *mut APTR { ((*IAmiSSL).EC_KEY_new_by_curve_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_generate_key(a0: *mut APTR) -> i32 { ((*IAmiSSL).EC_KEY_generate_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_free(a0: *mut APTR) { ((*IAmiSSL).EC_KEY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdsa_sig_free(a0: *mut APTR) { ((*IAmiSSL).ECDSA_SIG_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdsa_do_sign(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).ECDSA_do_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ecdsa_do_verify(a0: *const u8, a1: i32, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).ECDSA_do_verify)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_key_new() -> *mut APTR { ((*IAmiSSL).EC_KEY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_key_set_group(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EC_KEY_set_group)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_get0_group(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_KEY_get0_group)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_check_key(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_KEY_check_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdsa_size(a0: *const APTR) -> i32 { ((*IAmiSSL).ECDSA_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdsa_sign(a0: i32, a1: *const u8, a2: i32, a3: *mut u8, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).ECDSA_sign)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ecdsa_verify(a0: i32, a1: *const u8, a2: i32, a3: *const u8, a4: i32, a5: *mut APTR) -> i32 { ((*IAmiSSL).ECDSA_verify)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_d2i_ecdsa_sig(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ECDSA_SIG)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ecdsa_sig(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ECDSA_SIG)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_generate_parameters_ex(a0: *mut APTR, a1: i32, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).DH_generate_parameters_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_method_get_field_type(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_METHOD_get_field_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_get0_public_key(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_KEY_get0_public_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdh_compute_key(a0: *mut (), a1: u32, a2: *const APTR, a3: *const APTR, a4: APTR) -> i32 { ((*IAmiSSL).ECDH_compute_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_key_get0_private_key(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_KEY_get0_private_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_first() -> *mut APTR { ((*IAmiSSL).ENGINE_get_first)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_get_id(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ENGINE_get_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ENGINE_get_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_next(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).ENGINE_get_next)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_free(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_new() -> *mut APTR { ((*IAmiSSL).ENGINE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_set_id(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_add(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_add)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_remove(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_remove)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_load_builtin_engines() { ((*IAmiSSL).ENGINE_load_builtin_engines)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_register_all_ciphers() { ((*IAmiSSL).ENGINE_register_all_ciphers)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_get_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_get_type(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_get_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_all_digests() { ((*IAmiSSL).ENGINE_register_all_digests)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_test_flags(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).BIO_test_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_comp_get_compression_methods() -> *mut APTR { ((*IAmiSSL).SSL_COMP_get_compression_methods)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_generate_key_ex(a0: *mut APTR, a1: i32, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).RSA_generate_key_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_verify_param_free(a0: *mut APTR) { ((*IAmiSSL).X509_VERIFY_PARAM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set1_param(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_set1_param)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_generate_nconf(a0: *const APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).ASN1_generate_nconf)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_ecpkparameters(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_ECPKParameters)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_key_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_KEY_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_set_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).BIO_set_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_set_callback_arg(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).BIO_set_callback_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_set_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EVP_MD_CTX_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_set_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).BIO_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_set_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EVP_CIPHER_CTX_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_iv_length(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_get_iv_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_check_private_key(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REQ_check_private_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_ec_pubkey(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_EC_PUBKEY)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_read_bio_ecprivate_key(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_ECPrivateKey)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_ec_pubkey(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_EC_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ec_pubkey_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_EC_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_set_asn1_flag(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EC_KEY_set_asn1_flag)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_set_conv_form(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EC_KEY_set_conv_form)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_print(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).EC_KEY_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ecprivate_key_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_ECPrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ec_pubkey_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_EC_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_write_bio_ecprivate_key(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_write_bio_ECPrivateKey)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pem_write_bio_ecpkparameters(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_ECPKParameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ecprivate_key_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_ECPrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ecpkparameters(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ECPKParameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_set_asn1_flag(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EC_GROUP_set_asn1_flag)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_set_point_conversion_form(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EC_GROUP_set_point_conversion_form)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_get_point_conversion_form(a0: *const APTR) -> APTR { ((*IAmiSSL).EC_GROUP_get_point_conversion_form)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecpkparameters_print(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).ECPKParameters_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_group_set_seed(a0: *mut APTR, a1: *const u8, a2: u32) -> u32 { ((*IAmiSSL).EC_GROUP_set_seed)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ecpkparameters(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ECPKParameters)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_point_point2bn(a0: *const APTR, a1: *const APTR, a2: APTR, a3: *mut APTR, a4: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_POINT_point2bn)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_get1_ocsp(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_get1_ocsp)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_servername(a0: *const APTR, a1: APTR) -> *const APTR { ((*IAmiSSL).SSL_get_servername)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_ssl_ctx(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_set_SSL_CTX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_cookie_generate_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_cookie_generate_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_cookie_verify_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_cookie_verify_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_info_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_info_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new_dgram(a0: i32, a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_new_dgram)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_servername_type(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_servername_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_current_compression(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get_current_compression)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_current_expansion(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get_current_expansion)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_comp_get_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_COMP_get_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_by_id(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ENGINE_by_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_client_cert_engine(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_client_cert_engine)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sha256(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).SHA256)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sha512(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).SHA512)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_aes_ige_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: APTR) { ((*IAmiSSL).AES_ige_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ec_key_precompute_mult(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EC_KEY_precompute_mult)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_load_private_key(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).ENGINE_load_private_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_engine_load_public_key(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).ENGINE_load_public_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_engine_ctrl_cmd(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut (), a4: APTR, a5: i32) -> i32 { ((*IAmiSSL).ENGINE_ctrl_cmd)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_engine_set_default(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: *mut (), a4: APTR) -> i32 { ((*IAmiSSL).ENGINE_ctrl)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_engine_register_all_complete() -> i32 { ((*IAmiSSL).ENGINE_register_all_complete)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_ctrl_cmd_string(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).ENGINE_ctrl_cmd_string)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_verify_param_add0_policy(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_add0_policy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set_purpose(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set_purpose)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set_flags(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_new() -> *mut APTR { ((*IAmiSSL).X509_VERIFY_PARAM_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_policy_node_print(a0: *mut APTR, a1: *mut APTR, a2: i32) { ((*IAmiSSL).X509_POLICY_NODE_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_policy_tree(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_policy_tree)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_explicit_policy(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_get_explicit_policy)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_policy_tree_get0_policies(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_policy_tree_get0_policies)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_policy_tree_get0_user_policies(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_policy_tree_get0_user_policies)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_get_callback_arg(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_get_callback_arg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_add1_attr_by_nid(a0: *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_add1_attr_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_engine_get_rsa(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ENGINE_get_RSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_dsa(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ENGINE_get_DSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_dh(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ENGINE_get_DH)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_rand(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ENGINE_get_RAND)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_ciphers(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_ciphers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_digests(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_digests)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_finish(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_finish)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_sendreq_new(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).OCSP_sendreq_new)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_obsolete_ocsp_sendreq_nbio(a0: *mut *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_OCSP_sendreq_nbio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_psk_identity_hint(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_use_psk_identity_hint)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_psk_client_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_psk_client_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_psk_identity_hint(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get_psk_identity_hint)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_psk_server_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_psk_server_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_use_psk_identity_hint(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_use_psk_identity_hint)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_psk_client_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_psk_client_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_psk_server_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_psk_server_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_psk_identity(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get_psk_identity)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_session_ticket_ext(a0: *mut APTR, a1: *mut (), a2: i32) -> i32 { ((*IAmiSSL).SSL_set_session_ticket_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_session_secret_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).SSL_set_session_secret_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_session_ticket_ext_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).SSL_set_session_ticket_ext_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set1_param(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_set1_param)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set1_param(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set1_param)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_renegotiate_abbreviated(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_renegotiate_abbreviated)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_srp_client_pwd_callback(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_srp_client_pwd_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_srp_g(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_srp_g)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_srp_username_callback(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_srp_username_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_srp_userinfo(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_srp_userinfo)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_srp_server_param(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).SSL_set_srp_server_param)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ssl_set_srp_server_param_pw(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).SSL_set_srp_server_param_pw)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_get_srp_n(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_srp_N)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_srp_username(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_srp_username)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_srp_password(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_srp_password)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_srp_strength(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_CTX_set_srp_strength)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_srp_verify_param_callback(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_srp_verify_param_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_srp_cb_arg(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_set_srp_cb_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_srp_username(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_srp_username)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_srp_ctx_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_SRP_CTX_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_srp_calc_a_param(a0: *mut APTR) -> i32 { ((*IAmiSSL).SRP_Calc_A_param)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_srp_ctx_free(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_SRP_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_srp_server_param_with_username(a0: *mut APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).SSL_srp_server_param_with_username)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_srp_ctx_free(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_SRP_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_debug(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_set_debug)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_get0_peer(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_SESSION_get0_peer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_set1_id_context(a0: *mut APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_set1_id_context)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obsolete_ssl_cache_hit(a0: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_SSL_cache_hit)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_id(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_CIPHER_get_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_export_keying_material(a0: *mut APTR, a1: *mut u8, a2: u32, a3: *const APTR, a4: u32, a5: *const u8, a6: u32, a7: i32) -> i32 { ((*IAmiSSL).SSL_export_keying_material)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ssl_set_tlsext_use_srtp(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_set_tlsext_use_srtp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_next_protos_advertised_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_CTX_set_next_protos_advertised_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get0_next_proto_negotiated(a0: *const APTR, a1: *mut *mut APTR, a2: *mut APTR) { ((*IAmiSSL).SSL_get0_next_proto_negotiated)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_selected_srtp_profile(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_selected_srtp_profile)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_tlsext_use_srtp(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_tlsext_use_srtp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_select_next_proto(a0: *mut *mut u8, a1: *mut u8, a2: *const u8, a3: APTR, a4: *const u8, a5: APTR) -> i32 { ((*IAmiSSL).SSL_select_next_proto)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ssl_get_srtp_profiles(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_srtp_profiles)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_next_proto_select_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_CTX_set_next_proto_select_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_session_get_compress_id(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_SESSION_get_compress_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_srp_ctx_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_SRP_CTX_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_engine_strings() -> i32 { ((*IAmiSSL).ERR_load_ENGINE_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_set_dsa(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_DSA)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_finish_function(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_finish_function)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_default_rsa() -> *mut APTR { ((*IAmiSSL).ENGINE_get_default_RSA)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_set_dh(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_DH)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_init_function(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_init_function)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_init_function(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_init_function)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_default_dsa(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_DSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_last() -> *mut APTR { ((*IAmiSSL).ENGINE_get_last)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_get_prev(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).ENGINE_get_prev)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_default_dh() -> *mut APTR { ((*IAmiSSL).ENGINE_get_default_DH)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_set_finish_function(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_finish_function)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_rsa(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_RSA)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_default_rand(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_RAND)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_default_dsa() -> *mut APTR { ((*IAmiSSL).ENGINE_get_default_DSA)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_set_default_rsa(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_RSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_default_rand() -> *mut APTR { ((*IAmiSSL).ENGINE_get_default_RAND)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_set_rand(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_RAND)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_default_dh(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_DH)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_ctrl_function(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_ctrl_function)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_ctrl_function(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_ctrl_function)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_unregister_ciphers(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_ciphers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_unregister_rsa(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_RSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_all_rand() { ((*IAmiSSL).ENGINE_register_all_RAND)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_engine_load_dynamic() { ((*IAmiSSL).OBSOLETE_ENGINE_load_dynamic)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_get_digest_engine(a0: i32) -> *mut APTR { ((*IAmiSSL).ENGINE_get_digest_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_dh(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_DH)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_rand(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_RAND)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_engine_load_cryptodev() { ((*IAmiSSL).OBSOLETE_ENGINE_load_cryptodev)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_register_ciphers(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_ciphers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_engine_load_openssl() { ((*IAmiSSL).OBSOLETE_ENGINE_load_openssl)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_get_cmd_defns(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ENGINE_get_cmd_defns)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_load_privkey_function(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_load_privkey_function)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_default_digests(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_digests)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_rsa(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_RSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_unregister_dsa(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_DSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_ciphers(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_ciphers)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rand_set_rand_engine(a0: *mut APTR) -> i32 { ((*IAmiSSL).RAND_set_rand_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_digest(a0: *mut APTR, a1: i32) -> *const APTR { ((*IAmiSSL).ENGINE_get_digest)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_cipher(a0: *mut APTR, a1: i32) -> *const APTR { ((*IAmiSSL).ENGINE_get_cipher)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_cmd_is_executable(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).ENGINE_cmd_is_executable)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_register_dsa(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_DSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_load_pubkey_function(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_load_pubkey_function)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_load_pubkey_function(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_load_pubkey_function)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_all_rsa() { ((*IAmiSSL).ENGINE_register_all_RSA)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_unregister_digests(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_digests)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_engine_get_ex_new_index(a0: i32, a1: *mut (), a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_ENGINE_get_ex_new_index)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_engine_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).ENGINE_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_cmd_defns(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_cmd_defns)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_register_digests(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_digests)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_all_dh() { ((*IAmiSSL).ENGINE_register_all_DH)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_get_flags(a0: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_unregister_dh(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_DH)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_all_dsa() { ((*IAmiSSL).ENGINE_register_all_DSA)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_set_digests(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_digests)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_register_complete(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_complete)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).ENGINE_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_engine_set_destroy_function(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_destroy_function)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_cipher_engine(a0: i32) -> *mut APTR { ((*IAmiSSL).ENGINE_get_cipher_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_default_ciphers(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_ciphers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_unregister_rand(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_RAND)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_table_flags(a0: APTR) { ((*IAmiSSL).ENGINE_set_table_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_destroy_function(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_destroy_function)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_table_flags() -> APTR { ((*IAmiSSL).ENGINE_get_table_flags)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_set_flags(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).ENGINE_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_load_privkey_function(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_load_privkey_function)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_default_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_add_conf_module() { ((*IAmiSSL).ENGINE_add_conf_module)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get_trinomial_basis(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_trinomial_basis)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sha512_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).SHA512_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ecprivate_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ECPrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_exp_arr(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_exp_arr)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_mul_arr(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_mul_arr)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_o2i_ecpublic_key(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).o2i_ECPublicKey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_key_copy(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_KEY_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_check_discriminant(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_check_discriminant)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2o_ecpublic_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2o_ECPublicKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_new_curve_gf2m(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_new_curve_GF2m)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_get1_ec_key(a0: *mut APTR) -> *mut ec_key_st { ((*IAmiSSL).EVP_PKEY_get1_EC_KEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_get_conv_form(a0: *const APTR) -> APTR { ((*IAmiSSL).EC_KEY_get_conv_form)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_static_state() -> *mut () { ((*IAmiSSL).ENGINE_get_static_state)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ecdsa_sig_new() -> *mut APTR { ((*IAmiSSL).ECDSA_SIG_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_point_bn2point(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_POINT_bn2point)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ecdsa_sign_ex(a0: i32, a1: *const u8, a2: i32, a3: *mut u8, a4: *mut APTR, a5: *const APTR, a6: *const APTR, a7: *mut APTR) -> i32 { ((*IAmiSSL).ECDSA_sign_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ec_group_get_pentanomial_basis(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_pentanomial_basis)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ecdsa_sign_setup(a0: *mut APTR, a1: *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).ECDSA_sign_setup)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_solve_quad_arr(a0: *mut APTR, a1: *const APTR, a2: *const i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_solve_quad_arr)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_key_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EC_KEY_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ec_pubkey(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_EC_PUBKEY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_group_have_precompute_mult(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_have_precompute_mult)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_arr(a0: *mut APTR, a1: *const APTR, a2: *const i32) -> i32 { ((*IAmiSSL).BN_GF2m_mod_arr)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_point_dup(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_POINT_dup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_set1_ec_key(a0: *mut APTR, a1: *mut ec_key_st) -> i32 { ((*IAmiSSL).EVP_PKEY_set1_EC_KEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_sqrt_arr(a0: *mut APTR, a1: *const APTR, a2: *const i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_sqrt_arr)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_key_set_private_key(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EC_KEY_set_private_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_gf2m_poly2arr(a0: *const APTR, a1: *mut i32, a2: i32) -> i32 { ((*IAmiSSL).BN_GF2m_poly2arr)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ecparameters(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ECParameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sha256_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).SHA256_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sha224(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).SHA224)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_group_get_seed_len(a0: *const APTR) -> u32 { ((*IAmiSSL).EC_GROUP_get_seed_len)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_ec_pubkey(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_EC_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_gfp_nist_method() -> *const APTR { ((*IAmiSSL).EC_GFp_nist_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_engine_load_padlock() { ((*IAmiSSL).OBSOLETE_ENGINE_load_padlock)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_group_set_curve_name(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EC_GROUP_set_curve_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_sqr_arr(a0: *mut APTR, a1: *const APTR, a2: *const i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_sqr_arr)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_s_datagram() -> *const APTR { ((*IAmiSSL).BIO_s_datagram)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_sha384_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).SHA384_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sha224_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).SHA224_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sha224_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).SHA224_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ecprivate_key(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ECPrivateKey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sha512_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).SHA512_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_get_asn1_flag(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_asn1_flag)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get0_seed(a0: *const APTR) -> *mut u8 { ((*IAmiSSL).EC_GROUP_get0_seed)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_div_arr(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_div_arr)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_key_get_enc_flags(a0: *const APTR) -> APTR { ((*IAmiSSL).EC_KEY_get_enc_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_point_set_compressed_coordinates_gf2m(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_set_compressed_coordinates_GF2m)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_group_cmp(a0: *const APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_cmp)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sha224_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).SHA224_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sha512_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).SHA512_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get_basis_type(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_basis_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sha256_transform(a0: *mut APTR, a1: *const u8) { ((*IAmiSSL).SHA256_Transform)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_set_enc_flags(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EC_KEY_set_enc_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_point_point2hex(a0: *const APTR, a1: *const APTR, a2: APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_POINT_point2hex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ecdsa_do_sign_ex(a0: *const u8, a1: i32, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> *mut APTR { ((*IAmiSSL).ECDSA_do_sign_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_sha512_transform(a0: *mut APTR, a1: *const u8) { ((*IAmiSSL).SHA512_Transform)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_set_public_key(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EC_KEY_set_public_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_get_curve_name(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_curve_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sha256_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).SHA256_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ecparameters(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ECParameters)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sha384_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).SHA384_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sha384_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).SHA384_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sha384(a0: *const u8, a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).SHA384)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_point_hex2point(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_POINT_hex2point)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_sha256_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).SHA256_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_gf2m_mod_inv_arr(a0: *mut APTR, a1: *const APTR, a2: *const i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_GF2m_mod_inv_arr)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dh_check_pub_key(a0: *const APTR, a1: *const APTR, a2: *mut i32) -> i32 { ((*IAmiSSL).DH_check_pub_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_camellia_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: APTR) { ((*IAmiSSL).Camellia_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_camellia_cfb128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: APTR) { ((*IAmiSSL).Camellia_cfb128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_camellia_cfb1_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: APTR) { ((*IAmiSSL).Camellia_cfb1_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_camellia_cfb8_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: APTR) { ((*IAmiSSL).Camellia_cfb8_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_camellia_ctr128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut u8, a6: *mut APTR) { ((*IAmiSSL).Camellia_ctr128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_camellia_decrypt(a0: *const u8, a1: *mut u8, a2: *const APTR) { ((*IAmiSSL).Camellia_decrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_camellia_ecb_encrypt(a0: *const u8, a1: *mut u8, a2: *const APTR, a3: APTR) { ((*IAmiSSL).Camellia_ecb_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_camellia_encrypt(a0: *const u8, a1: *mut u8, a2: *const APTR) { ((*IAmiSSL).Camellia_encrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_camellia_ofb128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32) { ((*IAmiSSL).Camellia_ofb128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_camellia_set_key(a0: *const u8, a1: APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).Camellia_set_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_camellia_128_cbc() -> *const APTR { ((*IAmiSSL).EVP_camellia_128_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_128_cfb128() -> *const APTR { ((*IAmiSSL).EVP_camellia_128_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_128_cfb1() -> *const APTR { ((*IAmiSSL).EVP_camellia_128_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_128_cfb8() -> *const APTR { ((*IAmiSSL).EVP_camellia_128_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_128_ecb() -> *const APTR { ((*IAmiSSL).EVP_camellia_128_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_128_ofb() -> *const APTR { ((*IAmiSSL).EVP_camellia_128_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_192_cbc() -> *const APTR { ((*IAmiSSL).EVP_camellia_192_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_192_cfb128() -> *const APTR { ((*IAmiSSL).EVP_camellia_192_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_192_cfb1() -> *const APTR { ((*IAmiSSL).EVP_camellia_192_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_192_cfb8() -> *const APTR { ((*IAmiSSL).EVP_camellia_192_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_192_ecb() -> *const APTR { ((*IAmiSSL).EVP_camellia_192_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_192_ofb() -> *const APTR { ((*IAmiSSL).EVP_camellia_192_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_256_cbc() -> *const APTR { ((*IAmiSSL).EVP_camellia_256_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_256_cfb128() -> *const APTR { ((*IAmiSSL).EVP_camellia_256_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_256_cfb1() -> *const APTR { ((*IAmiSSL).EVP_camellia_256_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_256_cfb8() -> *const APTR { ((*IAmiSSL).EVP_camellia_256_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_256_ecb() -> *const APTR { ((*IAmiSSL).EVP_camellia_256_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_256_ofb() -> *const APTR { ((*IAmiSSL).EVP_camellia_256_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_aes_bi_ige_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *const APTR, a5: *const u8, a6: APTR) { ((*IAmiSSL).AES_bi_ige_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_seed_decrypt(a0: *const u8, a1: *mut u8, a2: *const APTR) { ((*IAmiSSL).SEED_decrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_seed_encrypt(a0: *const u8, a1: *mut u8, a2: *const APTR) { ((*IAmiSSL).SEED_encrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_seed_cbc_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: i32) { ((*IAmiSSL).SEED_cbc_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_seed_ofb() -> *const APTR { ((*IAmiSSL).EVP_seed_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_seed_cfb128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32, a6: i32) { ((*IAmiSSL).SEED_cfb128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_seed_ofb128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const APTR, a4: *mut u8, a5: *mut i32) { ((*IAmiSSL).SEED_ofb128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_seed_cbc() -> *const APTR { ((*IAmiSSL).EVP_seed_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_seed_ecb_encrypt(a0: *const u8, a1: *mut u8, a2: *const APTR, a3: i32) { ((*IAmiSSL).SEED_ecb_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_seed_ecb() -> *const APTR { ((*IAmiSSL).EVP_seed_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_seed_set_key(a0: *const u8, a1: *mut APTR) { ((*IAmiSSL).SEED_set_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_seed_cfb128() -> *const APTR { ((*IAmiSSL).EVP_seed_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_aes_unwrap_key(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: *const u8, a4: APTR) -> i32 { ((*IAmiSSL).AES_unwrap_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_aes_wrap_key(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: *const u8, a4: APTR) -> i32 { ((*IAmiSSL).AES_wrap_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_receipt_request_free(a0: *mut APTR) { ((*IAmiSSL).CMS_ReceiptRequest_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_add0_certificate_choices(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_add0_CertificateChoices)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_unsigned_add1_attr_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).CMS_unsigned_add1_attr_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_err_load_cms_strings() -> i32 { ((*IAmiSSL).ERR_load_CMS_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_sign_receipt(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: APTR) -> *mut APTR { ((*IAmiSSL).CMS_sign_receipt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_i2d_cms_content_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_CMS_ContentInfo)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signed_delete_attr(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).CMS_signed_delete_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_cms_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_CMS_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_unsigned_get_attr_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).CMS_unsigned_get_attr_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_verify(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: APTR) -> i32 { ((*IAmiSSL).CMS_verify)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_smime_read_cms(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).SMIME_read_CMS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_decrypt_set1_key(a0: *mut APTR, a1: *mut u8, a2: u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).CMS_decrypt_set1_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_signer_info_get0_algs(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR) { ((*IAmiSSL).CMS_SignerInfo_get0_algs)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_add1_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_add1_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_set_detached(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).CMS_set_detached)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_encrypt(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: APTR) -> *mut APTR { ((*IAmiSSL).CMS_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_enveloped_data_create(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_EnvelopedData_create)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_uncompress(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: APTR) -> i32 { ((*IAmiSSL).CMS_uncompress)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_add0_crl(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_add0_crl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signer_info_verify_content(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_SignerInfo_verify_content)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_unsigned_get0_data_by_obj(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: i32) -> *mut () { ((*IAmiSSL).CMS_unsigned_get0_data_by_OBJ)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_cms(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_CMS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_unsigned_get_attr(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).CMS_unsigned_get_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_ktri_cert_cmp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_ktri_cert_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_ktri_get0_algs(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_ktri_get0_algs)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_content_info_free(a0: *mut APTR) { ((*IAmiSSL).CMS_ContentInfo_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_final(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: APTR) -> i32 { ((*IAmiSSL).CMS_final)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_add_simple_smimecap(a0: *mut *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).CMS_add_simple_smimecap)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_signer_info_verify(a0: *mut APTR) -> i32 { ((*IAmiSSL).CMS_SignerInfo_verify)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_data(a0: *mut APTR, a1: *mut APTR, a2: APTR) -> i32 { ((*IAmiSSL).CMS_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_cms_receipt_request(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_CMS_ReceiptRequest)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_compress(a0: *mut APTR, a1: i32, a2: APTR) -> *mut APTR { ((*IAmiSSL).CMS_compress)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_digest_create(a0: *mut APTR, a1: *const APTR, a2: APTR) -> *mut APTR { ((*IAmiSSL).CMS_digest_create)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_signer_info_cert_cmp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_SignerInfo_cert_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signer_info_sign(a0: *mut APTR) -> i32 { ((*IAmiSSL).CMS_SignerInfo_sign)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_data_create(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).CMS_data_create)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_cms_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).i2d_CMS_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_encrypted_data_set1_key(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: u32) -> i32 { ((*IAmiSSL).CMS_EncryptedData_set1_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_decrypt(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: APTR) -> i32 { ((*IAmiSSL).CMS_decrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_cms_unsigned_delete_attr(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).CMS_unsigned_delete_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_unsigned_get_attr_count(a0: *const APTR) -> i32 { ((*IAmiSSL).CMS_unsigned_get_attr_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_add_smimecap(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_add_smimecap)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signed_get_attr_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).CMS_signed_get_attr_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_cms_content_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_CMS_ContentInfo)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_add_standard_smimecap(a0: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_add_standard_smimecap)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_content_info_new() -> *mut APTR { ((*IAmiSSL).CMS_ContentInfo_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_type(a0: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_get0_type(a0: *const APTR) -> *const APTR { ((*IAmiSSL).CMS_get0_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_is_detached(a0: *mut APTR) -> i32 { ((*IAmiSSL).CMS_is_detached)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_sign(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: APTR) -> *mut APTR { ((*IAmiSSL).CMS_sign)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_signed_add1_attr(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_signed_add1_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_unsigned_get_attr_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).CMS_unsigned_get_attr_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_smime_write_cms(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).SMIME_write_CMS)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_encrypted_data_decrypt(a0: *mut APTR, a1: *const u8, a2: u32, a3: *mut APTR, a4: *mut APTR, a5: APTR) -> i32 { ((*IAmiSSL).CMS_EncryptedData_decrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_cms_get0_recipient_infos(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_get0_RecipientInfos)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_add0_revocation_info_choice(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_add0_RevocationInfoChoice)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_decrypt_set1_pkey(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).CMS_decrypt_set1_pkey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_signer_info_set1_signer_cert(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).CMS_SignerInfo_set1_signer_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_get0_signers(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_get0_signers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_receipt_request_get0_values(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut i32, a3: *mut *mut APTR, a4: *mut *mut APTR) { ((*IAmiSSL).CMS_ReceiptRequest_get0_values)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_signed_get0_data_by_obj(a0: *const APTR, a1: *const APTR, a2: i32, a3: i32) -> *mut () { ((*IAmiSSL).CMS_signed_get0_data_by_OBJ)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_get0_signer_infos(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_get0_SignerInfos)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_add0_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_add0_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_encrypted_data_encrypt(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: u32, a4: APTR) -> *mut APTR { ((*IAmiSSL).CMS_EncryptedData_encrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_digest_verify(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: APTR) -> i32 { ((*IAmiSSL).CMS_digest_verify)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_set1_signers_certs(a0: *mut APTR, a1: *mut APTR, a2: APTR) -> i32 { ((*IAmiSSL).CMS_set1_signers_certs)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_signed_get_attr(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).CMS_signed_get_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_set0_key(a0: *mut APTR, a1: *mut u8, a2: u32) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_set0_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_signed_data_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).CMS_SignedData_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kekri_get0_id(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR, a5: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kekri_get0_id)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_cms_verify_receipt(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: APTR) -> i32 { ((*IAmiSSL).CMS_verify_receipt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pem_read_bio_cms(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_CMS)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_get1_crls(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_get1_crls)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_add0_recipient_key(a0: *mut APTR, a1: i32, a2: *mut u8, a3: u32, a4: *mut u8, a5: u32, a6: *mut APTR, a7: *mut APTR, a8: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_add0_recipient_key)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_cms_receipt_request_new() -> *mut APTR { ((*IAmiSSL).CMS_ReceiptRequest_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_get0_content(a0: *mut APTR) -> *mut *mut APTR { ((*IAmiSSL).CMS_get0_content)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_get1_receipt_request(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_get1_ReceiptRequest)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signed_add1_attr_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).CMS_signed_add1_attr_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kekri_id_cmp(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kekri_id_cmp)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_add1_receipt_request(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_add1_ReceiptRequest)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signer_info_get0_signer_id(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_SignerInfo_get0_signer_id)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_unsigned_add1_attr_by_nid(a0: *mut APTR, a1: i32, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).CMS_unsigned_add1_attr_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_unsigned_add1_attr(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_unsigned_add1_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signed_get_attr_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).CMS_signed_get_attr_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_get1_certs(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_get1_certs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_signed_add1_attr_by_nid(a0: *mut APTR, a1: i32, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).CMS_signed_add1_attr_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_unsigned_add1_attr_by_txt(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).CMS_unsigned_add1_attr_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_data_final(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_dataFinal)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_ktri_get0_signer_id(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_ktri_get0_signer_id)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_i2d_cms_receipt_request(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_CMS_ReceiptRequest)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_add1_recipient_cert(a0: *mut APTR, a1: *mut APTR, a2: APTR) -> *mut APTR { ((*IAmiSSL).CMS_add1_recipient_cert)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_data_init(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_dataInit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signed_add1_attr_by_txt(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).CMS_signed_add1_attr_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_decrypt(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_decrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_signed_get_attr_count(a0: *const APTR) -> i32 { ((*IAmiSSL).CMS_signed_get_attr_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_get0_e_content_type(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).CMS_get0_eContentType)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_set1_e_content_type(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).CMS_set1_eContentType)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_receipt_request_create0(a0: *mut u8, a1: i32, a2: i32, a3: *mut APTR, a4: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_ReceiptRequest_create0)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_add1_signer(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR, a4: APTR) -> *mut APTR { ((*IAmiSSL).CMS_add1_signer)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_set0_pkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_set0_pkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_load_ssl_client_cert_function(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_load_ssl_client_cert_function)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_ssl_client_cert_function(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_ssl_client_cert_function)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_load_ssl_client_cert(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR, a5: *mut *mut APTR, a6: *mut APTR, a7: *mut ()) -> i32 { ((*IAmiSSL).ENGINE_load_ssl_client_cert)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_obsolete_engine_load_capi() { ((*IAmiSSL).OBSOLETE_ENGINE_load_capi)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_register_pkey_meths(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_pkey_meths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_get_pkey_asn1_meth_engine(a0: i32) -> *mut APTR { ((*IAmiSSL).ENGINE_get_pkey_asn1_meth_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_whirlpool_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).WHIRLPOOL_Init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_whirlpool(a0: *const (), a1: u32, a2: *mut u8) -> *mut u8 { ((*IAmiSSL).WHIRLPOOL)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_engine_get_pkey_asn1_meth(a0: *mut APTR, a1: i32) -> *const APTR { ((*IAmiSSL).ENGINE_get_pkey_asn1_meth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_pkey_meth(a0: *mut APTR, a1: i32) -> *const APTR { ((*IAmiSSL).ENGINE_get_pkey_meth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new_cms(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_CMS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_default_pkey_asn1_meths(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_pkey_asn1_meths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_whirlpool_bit_update(a0: *mut APTR, a1: *const (), a2: u32) { ((*IAmiSSL).WHIRLPOOL_BitUpdate)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_cms_bio_stream(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).i2d_CMS_bio_stream)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_stream(a0: *mut *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_stream)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_pkey_asn1_find_str(a0: *mut *mut APTR, a1: *const APTR, a2: i32) -> *const APTR { ((*IAmiSSL).ENGINE_pkey_asn1_find_str)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_engine_get_pkey_meths(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_pkey_meths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_pkey_asn1_meths(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_pkey_asn1_meths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_default_pkey_meths(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_pkey_meths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_hmac_ctx_copy(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).HMAC_CTX_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_pkey_asn1_meths(a0: *const APTR) -> APTR { ((*IAmiSSL).ENGINE_get_pkey_asn1_meths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_whirlpool() -> *const APTR { ((*IAmiSSL).EVP_whirlpool)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_register_all_pkey_meths() { ((*IAmiSSL).ENGINE_register_all_pkey_meths)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_whirlpool_final(a0: *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).WHIRLPOOL_Final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_pkey_asn1_meth_str(a0: *mut APTR, a1: *const APTR, a2: i32) -> *const APTR { ((*IAmiSSL).ENGINE_get_pkey_asn1_meth_str)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_engine_register_all_pkey_asn1_meths() { ((*IAmiSSL).ENGINE_register_all_pkey_asn1_meths)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_content_info_print_ctx(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const APTR) -> i32 { ((*IAmiSSL).CMS_ContentInfo_print_ctx)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_add1_crl(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_add1_crl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_set_pkey_asn1_meths(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_pkey_asn1_meths)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_get_pkey_meth_engine(a0: i32) -> *mut APTR { ((*IAmiSSL).ENGINE_get_pkey_meth_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_whirlpool_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).WHIRLPOOL_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pem_write_bio_cms_stream(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).PEM_write_bio_CMS_stream)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_engine_unregister_pkey_meths(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_pkey_meths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_unregister_pkey_asn1_meths(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_pkey_asn1_meths)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_pkey_meths(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ENGINE_set_pkey_meths)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsaparams_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).DSAparams_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dhparams_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).DHparams_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_issuer_name_hash_old(a0: *mut APTR) -> u32 { ((*IAmiSSL).X509_issuer_name_hash_old)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_subject_name_hash_old(a0: *mut APTR) -> u32 { ((*IAmiSSL).X509_subject_name_hash_old)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_srp_vbase_get_by_user(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).SRP_VBASE_get_by_user)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_srp_calc_server_key(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_server_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_srp_create_verifier(a0: *const APTR, a1: *const APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *const APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_create_verifier)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_srp_create_verifier_bn(a0: *const APTR, a1: *const APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *const APTR, a5: *const APTR) -> i32 { ((*IAmiSSL).SRP_create_verifier_BN)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_srp_calc_u(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_u)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_srp_vbase_free(a0: *mut APTR) { ((*IAmiSSL).SRP_VBASE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_srp_calc_client_key(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_client_key)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_srp_get_default_g_n(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_get_default_gN)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_srp_calc_x(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_x)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_srp_calc_b(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_B)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_srp_vbase_new(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SRP_VBASE_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_srp_check_known_g_n_param(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_check_known_gN_param)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_srp_calc_a(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_A)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_srp_verify_a_mod_n(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SRP_Verify_A_mod_N)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_srp_vbase_init(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SRP_VBASE_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_srp_verify_b_mod_n(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SRP_Verify_B_mod_N)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_set_public_key_affine_coordinates(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).EC_KEY_set_public_key_affine_coordinates)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_aes_192_ctr() -> *const APTR { ((*IAmiSSL).EVP_aes_192_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_ctr() -> *const APTR { ((*IAmiSSL).EVP_aes_128_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_ctr() -> *const APTR { ((*IAmiSSL).EVP_aes_256_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_key_get_flags(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_KEY_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_aes_128_xts() -> *const APTR { ((*IAmiSSL).EVP_aes_128_xts)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_xts() -> *const APTR { ((*IAmiSSL).EVP_aes_256_xts)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_gcm() -> *const APTR { ((*IAmiSSL).EVP_aes_128_gcm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_key_clear_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EC_KEY_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_set_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EC_KEY_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_aes_256_ccm() -> *const APTR { ((*IAmiSSL).EVP_aes_256_ccm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_ccm() -> *const APTR { ((*IAmiSSL).EVP_aes_128_ccm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_gcm() -> *const APTR { ((*IAmiSSL).EVP_aes_192_gcm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_gcm() -> *const APTR { ((*IAmiSSL).EVP_aes_256_gcm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_ccm() -> *const APTR { ((*IAmiSSL).EVP_aes_192_ccm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_rc4_hmac_md5() -> *const APTR { ((*IAmiSSL).EVP_rc4_hmac_md5)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_cbc_hmac_sha1() -> *const APTR { ((*IAmiSSL).EVP_aes_128_cbc_hmac_sha1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_obsolete_engine_load_rdrand() { ((*IAmiSSL).OBSOLETE_ENGINE_load_rdrand)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_cbc_hmac_sha1() -> *const APTR { ((*IAmiSSL).EVP_aes_256_cbc_hmac_sha1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_add0_recipient_password(a0: *mut APTR, a1: i32, a2: i32, a3: i32, a4: *mut u8, a5: APTR, a6: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_add0_recipient_password)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_cms_decrypt_set1_password(a0: *mut APTR, a1: *mut u8, a2: APTR) -> i32 { ((*IAmiSSL).CMS_decrypt_set1_password)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_set0_password(a0: *mut APTR, a1: *mut u8, a2: APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_set0_password)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_req_sign_ctx(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REQ_sign_ctx)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_sign_ctx(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_CRL_sign_ctx)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_signature_dump(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_signature_dump)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_sign_ctx(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_sign_ctx)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_item_sign_ctx(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *const (), a5: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_item_sign_ctx)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set0_crls(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set0_crls)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_get_id(a0: *const APTR, a1: *mut APTR) -> *const u8 { ((*IAmiSSL).SSL_SESSION_get_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_sess_set_new_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_sess_set_new_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_sess_get_get_cb(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_sess_get_get_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_sess_set_get_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_sess_set_get_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_info_callback(a0: *mut APTR) { ((*IAmiSSL).SSL_CTX_get_info_callback)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_client_cert_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_client_cert_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_sess_set_remove_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_sess_set_remove_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_sess_get_new_cb(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_sess_get_new_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_client_cert_cb(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_client_cert_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_sess_get_remove_cb(a0: *mut APTR) { ((*IAmiSSL).SSL_CTX_sess_get_remove_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pem_write_bio_ssl_session(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_SSL_SESSION)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_ssl_session(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_SSL_SESSION)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_x931_generate_xpq(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_X931_generate_Xpq)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_get0_nist_prime_384() -> *const APTR { ((*IAmiSSL).BN_get0_nist_prime_384)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_set_mark() -> i32 { ((*IAmiSSL).ERR_set_mark)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_blinding_invert_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_BLINDING_invert_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_name_constraints_free(a0: *mut APTR) { ((*IAmiSSL).NAME_CONSTRAINTS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set0_param(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set0_param)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_policy_constraints_free(a0: *mut APTR) { ((*IAmiSSL).POLICY_CONSTRAINTS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_nist_mod_192(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_nist_mod_192)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_general_subtree_free(a0: *mut APTR) { ((*IAmiSSL).GENERAL_SUBTREE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_add_safe(a0: *mut *mut APTR, a1: *mut APTR, a2: i32, a3: i32, a4: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_add_safe)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_get0_nist_prime_192() -> *const APTR { ((*IAmiSSL).BN_get0_nist_prime_192)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_keyid_get0(a0: *mut APTR, a1: *mut i32) -> *mut u8 { ((*IAmiSSL).X509_keyid_get0)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_policy_node_get0_parent(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_policy_node_get0_parent)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_a2i_ipaddress(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).a2i_IPADDRESS)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_depth(a0: *mut APTR, a1: i32) { ((*IAmiSSL).X509_STORE_CTX_set_depth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_inherit(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_inherit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set_depth(a0: *mut APTR, a1: i32) { ((*IAmiSSL).X509_VERIFY_PARAM_set_depth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_add1_attr_by_txt(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_add1_attr_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_blinding_set_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).BN_BLINDING_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set1_policies(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set1_policies)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set1_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set1_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_policy_mapping_free(a0: *mut APTR) { ((*IAmiSSL).POLICY_MAPPING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_policy_level_node_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_policy_level_node_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_attr(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_get_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_general_subtree_new() -> *mut APTR { ((*IAmiSSL).GENERAL_SUBTREE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_policy_node_get0_qualifiers(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_policy_node_get0_qualifiers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_policy_node_get0_policy(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_policy_node_get0_policy)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_add_safes(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_add_safes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_blinding_convert_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_BLINDING_convert_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_policy_tree_free(a0: *mut APTR) { ((*IAmiSSL).X509_policy_tree_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_get0_nist_prime_224() -> *const APTR { ((*IAmiSSL).BN_get0_nist_prime_224)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_gencb_call(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).BN_GENCB_call)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_name_constraints_new() -> *mut APTR { ((*IAmiSSL).NAME_CONSTRAINTS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ecparameters_print(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ECParameters_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_buf_memdup(a0: *const (), a1: u32) -> *mut () { ((*IAmiSSL).OBSOLETE_BUF_memdup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set_trust(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set_trust)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_attr_count(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_attr_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_param(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_param)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_depth(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_STORE_set_depth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obsolete_buf_strndup(a0: *const APTR, a1: u32) -> *mut APTR { ((*IAmiSSL).OBSOLETE_BUF_strndup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_put_eoc(a0: *mut *mut u8) -> i32 { ((*IAmiSSL).ASN1_put_eoc)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_table_cleanup() { ((*IAmiSSL).X509_VERIFY_PARAM_table_cleanup)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_setup_blinding(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).RSA_setup_blinding)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_find_ex(a0: *mut APTR, a1: *const ()) -> i32 { ((*IAmiSSL).OPENSSL_sk_find_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_policy_constraints_new() -> *mut APTR { ((*IAmiSSL).POLICY_CONSTRAINTS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get_depth(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_get_depth)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_item_ndef_i2d(a0: *const APTR, a1: *mut *mut u8, a2: *const APTR) -> i32 { ((*IAmiSSL).ASN1_item_ndef_i2d)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_err_pop_to_mark() -> i32 { ((*IAmiSSL).ERR_pop_to_mark)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_policy_level_get0_node(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_policy_level_get0_node)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_ndef(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS7_NDEF)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_generate_v3(a0: *const APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).ASN1_generate_v3)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_policy_tree_level_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_policy_tree_level_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_nist_mod_224(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_nist_mod_224)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_dgram_non_fatal_error(a0: i32) -> i32 { ((*IAmiSSL).BIO_dgram_non_fatal_error)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_v2i_asn1_bit_string(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).v2i_ASN1_BIT_STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_default(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_set_default)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set1(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set1)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_v2i_general_name_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: i32) -> *mut APTR { ((*IAmiSSL).v2i_GENERAL_NAME_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_nist_mod_521(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_nist_mod_521)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_policy_tree_get0_level(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_policy_tree_get0_level)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_const_check_infinite_end(a0: *mut *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).ASN1_const_check_infinite_end)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_delete_attr(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_delete_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_get0_nist_prime_256() -> *const APTR { ((*IAmiSSL).BN_get0_nist_prime_256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_i2v_asn1_bit_string(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).i2v_ASN1_BIT_STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_nist_mod_384(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_nist_mod_384)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_attr_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_attr_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_verify_param_lookup(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_VERIFY_PARAM_lookup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_v3_name_from_section(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).X509V3_NAME_from_section)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_add1_attr(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_add1_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_dump_indent_cb(a0: APTR, a1: *mut (), a2: *const (), a3: i32, a4: i32) -> i32 { ((*IAmiSSL).BIO_dump_indent_cb)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_nist_mod_256(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_nist_mod_256)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_verify_param_add0_table(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_add0_table)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_blinding_create_param(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: APTR, a5: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_BLINDING_create_param)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_get0_nist_prime_521() -> *const APTR { ((*IAmiSSL).BN_get0_nist_prime_521)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_is_prime_fasttest_ex(a0: *const APTR, a1: i32, a2: *mut APTR, a3: i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).BN_is_prime_fasttest_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_policy_check(a0: *mut *mut APTR, a1: *mut i32, a2: *mut APTR, a3: *mut APTR, a4: APTR) -> i32 { ((*IAmiSSL).X509_policy_check)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_attr_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_attr_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_blinding_get_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).BN_BLINDING_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_add_cert(a0: *mut *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_add_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_rand_key(a0: *mut APTR, a1: *mut u8) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_rand_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_a2i_ipaddress_nc(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).a2i_IPADDRESS_NC)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_set_digest(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PKCS7_set_digest)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_policy_mapping_new() -> *mut APTR { ((*IAmiSSL).POLICY_MAPPING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_set0_type_other(a0: *mut APTR, a1: i32, a2: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_set0_type_other)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_add1_attr_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_add1_attr_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set_time(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_VERIFY_PARAM_set_time)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_add_key(a0: *mut *mut APTR, a1: *mut APTR, a2: i32, a3: i32, a4: i32, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_add_key)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bio_dump_cb(a0: APTR, a1: *mut (), a2: *const (), a3: i32) -> i32 { ((*IAmiSSL).BIO_dump_cb)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_get_rfc3526_prime_8192(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_get_rfc3526_prime_8192)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_clear_flags(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_get_rfc2409_prime_1024(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_get_rfc2409_prime_1024)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_get_rfc3526_prime_2048(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_get_rfc3526_prime_2048)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_get_rfc3526_prime_6144(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_get_rfc3526_prime_6144)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_get_rfc3526_prime_1536(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_get_rfc3526_prime_1536)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_get_rfc3526_prime_3072(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_get_rfc3526_prime_3072)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_get_rfc3526_prime_4096(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_get_rfc3526_prime_4096)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_get_rfc2409_prime_768(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_get_rfc2409_prime_768)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).X509_VERIFY_PARAM_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_new() -> *mut APTR { ((*IAmiSSL).EVP_CIPHER_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_free(a0: *mut APTR) { ((*IAmiSSL).EVP_CIPHER_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_block_size(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_get_block_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_set_app_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).EVP_CIPHER_CTX_set_app_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_method_type(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_method_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_key_length(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_key_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_get_size(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_get_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_test_flags(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_MD_CTX_test_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_clear_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).BIO_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_get_pkey_type(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_get_pkey_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_clear_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EVP_MD_CTX_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).EVP_CIPHER_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_get_callback(a0: *const APTR) -> APTR { ((*IAmiSSL).BIO_get_callback)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_key_length(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_get_key_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher(a0: *mut APTR, a1: *mut u8, a2: *const u8, a3: APTR) -> i32 { ((*IAmiSSL).EVP_Cipher)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_block_size(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_block_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_cipher(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_CTX_cipher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_app_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).EVP_CIPHER_CTX_get_app_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_get_block_size(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_get_block_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_evp_cipher_ctx_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).OBSOLETE_EVP_CIPHER_CTX_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_md(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_CTX_md)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_method_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).BIO_method_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_iv_length(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_iv_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_memcmp(a0: *const (), a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).CRYPTO_memcmp)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_consttime_swap(a0: APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) { ((*IAmiSSL).BN_consttime_swap)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_i2d_x509_extensions(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_EXTENSIONS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_x509_extensions(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_EXTENSIONS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_algor_get0(a0: *mut *mut APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *const APTR) { ((*IAmiSSL).X509_ALGOR_get0)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_algor_set0(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).X509_ALGOR_set0)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509at_get0_data_by_obj(a0: *const APTR, a1: *const APTR, a2: i32, a3: i32) -> *mut () { ((*IAmiSSL).X509at_get0_data_by_OBJ)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_type_set1(a0: *mut APTR, a1: i32, a2: *const ()) -> i32 { ((*IAmiSSL).ASN1_TYPE_set1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_string_set0(a0: *mut APTR, a1: *mut (), a2: i32) { ((*IAmiSSL).ASN1_STRING_set0)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_algors(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_ALGORS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_x509_algors(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_ALGORS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_smime_read_asn1(a0: *mut APTR, a1: *mut *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).SMIME_read_ASN1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_isservice() -> i32 { ((*IAmiSSL).OPENSSL_isservice)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_clear_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).EVP_CIPHER_CTX_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_x931_generate_prime_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *const APTR, a6: *const APTR, a7: *mut APTR, a8: *mut APTR) -> i32 { ((*IAmiSSL).BN_X931_generate_prime_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_test_flags(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_test_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_add_alg_module() { ((*IAmiSSL).EVP_add_alg_module)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_x931_derive_prime_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR, a4: *const APTR, a5: *const APTR, a6: *const APTR, a7: *mut APTR, a8: *mut APTR) -> i32 { ((*IAmiSSL).BN_X931_derive_prime_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_openssl_init() { ((*IAmiSSL).OPENSSL_init)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crypto_strdup(a0: *const APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).CRYPTO_strdup)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ts_accuracy(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_TS_ACCURACY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ts_msg_imprint(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_TS_MSG_IMPRINT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_print_public(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_print_public)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_new(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_CTX_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ts_tst_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_TS_TST_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_find(a0: *mut *mut APTR, a1: i32) -> *const APTR { ((*IAmiSSL).EVP_PKEY_asn1_find)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_load_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_CONF_load_cert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_get_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).TS_REQ_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_sign_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_sign_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_item_print(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).ASN1_item_print)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_nonce(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_nonce)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_RESP_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_add0(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_asn1_add0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_add0_attrib_signing_time(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_add0_attrib_signing_time)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_asn1_get_prefix(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).BIO_asn1_get_prefix)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_time(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_time)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_decrypt(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_decrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_type_str(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_set_type_str)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_keygen_info(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_keygen_info)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_req_set_policy_id(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_REQ_set_policy_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_set_status_info(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_RESP_set_status_info)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_keygen(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_keygen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_digest_sign_init(a0: *mut APTR, a1: *mut *mut APTR, a2: *const APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EVP_DigestSignInit)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_accuracy_set_millis(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_ACCURACY_set_millis)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_req_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_REQ_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_general_name_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).GENERAL_NAME_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get1_crls(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get1_crls)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_new(a0: i32, a1: i32, a2: *const APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_asn1_new)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_new_ndef(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_NDEF)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_msg_imprint_set_algo(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_MSG_IMPRINT_set_algo)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ts_tst_info_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_TS_TST_INFO_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_ordering(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_ordering)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_ext_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).TS_TST_INFO_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_conf_get_tsa_section(a0: *mut APTR, a1: *const APTR) -> *const APTR { ((*IAmiSSL).TS_CONF_get_tsa_section)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_smime_write_asn1(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32, a4: i32, a5: i32, a6: *mut APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).SMIME_write_ASN1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_signer_key(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_signer_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_encrypt_old(a0: *mut u8, a1: *const u8, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_encrypt_old)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_encrypt_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_encrypt_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_pctx_get_cert_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).ASN1_PCTX_get_cert_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_ess_signing_cert(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ESS_SIGNING_CERT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_load_key(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_CONF_load_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_asn1_sequence_any(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_SEQUENCE_ANY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ts_msg_imprint_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_TS_MSG_IMPRINT_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_public(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR, a4: APTR, a5: APTR, a6: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_public)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_b2i_public_key_bio(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).b2i_PublicKey_bio)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_asn1_set_prefix(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BIO_asn1_set_prefix)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_new_mac_key(a0: i32, a1: *mut APTR, a2: *const u8, a3: i32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_new_mac_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_req_ext_free(a0: *mut APTR) { ((*IAmiSSL).TS_REQ_ext_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_free(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_free)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_asn1(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_get0_asn1)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_recover_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_recover_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).EVP_PKEY_CTX_set_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_keygen_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_keygen_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_status_info(a0: *mut APTR, a1: i32, a2: *const APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_status_info)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_msg_imprint_get_algo(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_MSG_IMPRINT_get_algo)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_print_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_REQ_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_ctrl_str(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_ctrl_str)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_default_digest_nid(a0: *mut APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_default_digest_nid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_write_bio_pkcs7_stream(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).PEM_write_bio_PKCS7_stream)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_msg_imprint_print_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_MSG_IMPRINT_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_asc2bn(a0: *mut *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_asc2bn)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_req_get_policy_id(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_REQ_get_policy_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ts_accuracy(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_TS_ACCURACY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dso_global_lookup(a0: *const APTR) -> *mut () { ((*IAmiSSL).DSO_global_lookup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_conf_set_tsa_name(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_tsa_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asn1_set_any(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASN1_SET_ANY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_pctx_get_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).ASN1_PCTX_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_ext_by_nid(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).TS_TST_INFO_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_resp_new() -> *mut APTR { ((*IAmiSSL).TS_RESP_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ess_cert_id_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ESS_CERT_ID_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_status_info_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_STATUS_INFO_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).TS_REQ_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_digest_verify_final(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_DigestVerifyFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_print_params(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_print_params)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_req_get_msg_imprint(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_REQ_get_msg_imprint)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_find_sigid_by_algs(a0: *mut i32, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OBJ_find_sigid_by_algs)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_serial(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_TST_INFO_get_serial)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_get_nonce(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_REQ_get_nonce)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_pubkey_set0_param(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: *mut (), a4: *mut u8, a5: i32) -> i32 { ((*IAmiSSL).X509_PUBKEY_set0_param)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set0_keygen_info(a0: *mut APTR, a1: *mut i32, a2: i32) { ((*IAmiSSL).EVP_PKEY_CTX_set0_keygen_info)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dist_point_set_dpname(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).DIST_POINT_set_dpname)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_issuing_dist_point(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ISSUING_DIST_POINT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).EVP_PKEY_CTX_get_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_status_info_print_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_STATUS_INFO_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_derive_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_derive_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ts_tst_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_TS_TST_INFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_add_alias(a0: i32, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_asn1_add_alias)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ts_resp_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_TS_RESP_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_othername_cmp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OTHERNAME_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_general_name_set0_value(a0: *mut APTR, a1: i32, a2: *mut ()) { ((*IAmiSSL).GENERAL_NAME_set0_value)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs7_recip_info_get0_alg(a0: *mut APTR, a1: *mut *mut APTR) { ((*IAmiSSL).PKCS7_RECIP_INFO_get0_alg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_new() -> *mut APTR { ((*IAmiSSL).TS_RESP_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ts_resp_set_tst_info(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) { ((*IAmiSSL).TS_RESP_set_tst_info)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs7_final(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).PKCS7_final)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_base_id(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_base_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_signer_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_signer_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_req_set_msg_imprint(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_REQ_set_msg_imprint)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: i32, a4: i32, a5: *mut ()) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_ctrl)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ts_conf_set_digests(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_digests)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ts_msg_imprint(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_TS_MSG_IMPRINT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_ctrl(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_ctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_req_get_ext_by_nid(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).TS_REQ_get_ext_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe_set0_algor(a0: *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).PKCS5_pbe_set0_algor)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_accuracy_new() -> *mut APTR { ((*IAmiSSL).TS_ACCURACY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_crl_method_free(a0: *mut APTR) { ((*IAmiSSL).X509_CRL_METHOD_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_pctx_get_nm_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).ASN1_PCTX_get_nm_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_sign(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_decrypt_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_decrypt_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2b_pvk_bio(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *mut ()) -> i32 { ((*IAmiSSL).i2b_PVK_bio)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_print_private(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_print_private)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_general_name_get0_value(a0: *const APTR, a1: *mut i32) -> *mut () { ((*IAmiSSL).GENERAL_NAME_get0_value)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_b2i_pvk_bio(a0: *mut APTR, a1: *mut APTR, a2: *mut ()) -> *mut APTR { ((*IAmiSSL).b2i_PVK_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_utctime_adj(a0: *mut APTR, a1: APTR, a2: i32, a3: i32) -> *mut APTR { ((*IAmiSSL).ASN1_UTCTIME_adj)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_tst_info_new() -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_md_do_all_sorted(a0: APTR, a1: *mut ()) { ((*IAmiSSL).EVP_MD_do_all_sorted)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_default_engine(a0: *const APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_default_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_accuracy_set_seconds(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_ACCURACY_set_seconds)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_time(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_TST_INFO_get_time)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs8_pkey_get0(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: *mut i32, a3: *mut *mut APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).PKCS8_pkey_get0)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_get0(a0: i32) -> *const APTR { ((*IAmiSSL).EVP_PKEY_asn1_get0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_add_sigid(a0: i32, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OBJ_add_sigid)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs7_signer_info_sign(a0: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_SIGNER_INFO_sign)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_paramgen_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_paramgen_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_sign(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_sign)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_obj_sigid_free() { ((*IAmiSSL).OBJ_sigid_free)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_init(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ess_issuer_serial(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ESS_ISSUER_SERIAL)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_issuing_dist_point_new() -> *mut APTR { ((*IAmiSSL).ISSUING_DIST_POINT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_time_adj(a0: *mut APTR, a1: APTR, a2: i32, a3: i32) -> *mut APTR { ((*IAmiSSL).ASN1_TIME_adj)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_obj_print_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_OBJ_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_verify_recover(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_verify_recover)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_resp_get_status_info(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_RESP_get_status_info)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_cb(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).EVP_PKEY_CTX_set_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_to_ts_tst_info(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_to_TS_TST_INFO)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_pctx_get_oid_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).ASN1_PCTX_get_oid_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_tst_info_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).TS_TST_INFO_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_derive(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_derive)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ts_msg_imprint_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_TS_MSG_IMPRINT_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_accuracy(a0: *mut APTR, a1: i32, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_accuracy)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_req_set_nonce(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_REQ_set_nonce)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ess_cert_id_new() -> *mut APTR { ((*IAmiSSL).ESS_CERT_ID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ts_req_get_ext_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).TS_REQ_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_buf_reverse(a0: *mut u8, a1: *const u8, a2: u32) { ((*IAmiSSL).BUF_reverse)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_tst_info_print_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_issuing_dist_point(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ISSUING_DIST_POINT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2b_private_key_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2b_PrivateKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ts_resp(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_TS_RESP)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_b2i_public_key(a0: *mut *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).b2i_PublicKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_cleanup(a0: *mut APTR) { ((*IAmiSSL).TS_VERIFY_CTX_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_status_info_free(a0: *mut APTR) { ((*IAmiSSL).TS_STATUS_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_verify_token(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_RESP_verify_token)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obj_bsearch_ex_(a0: *const (), a1: *const (), a2: i32, a3: i32, a4: APTR, a5: i32) -> *const () { ((*IAmiSSL).OBJ_bsearch_ex_)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_asn1_bn_print(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut u8, a4: i32) -> i32 { ((*IAmiSSL).ASN1_bn_print)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_get_count() -> i32 { ((*IAmiSSL).EVP_PKEY_asn1_get_count)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_pctx_set_nm_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).ASN1_PCTX_set_nm_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_digest_verify_init(a0: *mut APTR, a1: *mut *mut APTR, a2: *const APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EVP_DigestVerifyInit)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_policy_id(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_get_policy_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_get_cert_req(a0: *const APTR) -> i32 { ((*IAmiSSL).TS_REQ_get_cert_req)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_set_meth_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).X509_CRL_set_meth_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs8_pkey_set0(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: i32, a4: *mut (), a5: *mut u8, a6: i32) -> i32 { ((*IAmiSSL).PKCS8_pkey_set0)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_asn1_string_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_STRING_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_match(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_match)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_private(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_private)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_ext_d2i(a0: *mut APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).TS_TST_INFO_get_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_add_policy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_add_policy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ts_resp(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_TS_RESP)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_conf_load_certs(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_CONF_load_certs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_msg_imprint(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_get_msg_imprint)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_ts_strings() -> i32 { ((*IAmiSSL).ERR_load_TS_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_version(a0: *const APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_CTX_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_verify(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_verify)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2b_public_key_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2b_PublicKey_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_certs(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_certs)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_get0_info(a0: *mut i32, a1: *mut i32, a2: *mut i32, a3: *mut *mut APTR, a4: *mut *mut APTR, a5: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_asn1_get0_info)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_free(a0: *mut APTR) { ((*IAmiSSL).TS_VERIFY_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_get_ext_by_critical(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).TS_REQ_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_serial_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).TS_RESP_CTX_set_serial_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_get_meth_data(a0: *mut APTR) -> *mut () { ((*IAmiSSL).X509_CRL_get_meth_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_time_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).TS_RESP_CTX_set_time_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_msg_imprint_get_msg(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_MSG_IMPRINT_get_msg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_tst_info_ext_free(a0: *mut APTR) { ((*IAmiSSL).TS_TST_INFO_ext_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_get_version(a0: *const APTR) -> i32 { ((*IAmiSSL).TS_REQ_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_add_ext(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).TS_REQ_add_ext)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_app_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).EVP_PKEY_CTX_set_app_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_obj_bsearch_(a0: *const (), a1: *const (), a2: i32, a3: i32, a4: APTR) -> *const () { ((*IAmiSSL).OBJ_bsearch_)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_verifyctx(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_verifyctx)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pkcs7_bio_stream(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).i2d_PKCS7_bio_stream)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs7_sign_add_signer(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR, a4: i32) -> *mut APTR { ((*IAmiSSL).PKCS7_sign_add_signer)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_d2i_ts_tst_info_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_TS_TST_INFO_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_ordering(a0: *const APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_get_ordering)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_print_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_RESP_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_exts(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_get_exts)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe2_set_iv(a0: *const APTR, a1: i32, a2: *mut u8, a3: i32, a4: *mut u8, a5: i32) -> *mut APTR { ((*IAmiSSL).PKCS5_pbe2_set_iv)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_b2i_private_key(a0: *mut *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).b2i_PrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_app_data(a0: *mut APTR) -> *mut () { ((*IAmiSSL).EVP_PKEY_CTX_get_app_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_set_cert_req(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).TS_REQ_set_cert_req)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_serial(a0: *mut APTR, a1: *const APTR, a2: APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_serial)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_tst_info_free(a0: *mut APTR) { ((*IAmiSSL).TS_TST_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_verify_response(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_RESP_verify_response)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ess_issuer_serial(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ESS_ISSUER_SERIAL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_accuracy_get_seconds(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_ACCURACY_get_seconds)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_do_all(a0: APTR, a1: *mut ()) { ((*IAmiSSL).EVP_CIPHER_do_all)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_b2i_private_key_bio(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).b2i_PrivateKey_bio)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_certid_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OCSP_CERTID_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_pubkey_get0_param(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: *mut i32, a3: *mut *mut APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).X509_PUBKEY_get0_param)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_msg_imprint_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_MSG_IMPRINT_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_print_ctx(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const APTR) -> i32 { ((*IAmiSSL).PKCS7_print_ctx)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_i2d_ts_req_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_TS_REQ_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_param(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR, a4: APTR, a5: APTR, a6: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_param)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_encrypt(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_encrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_pctx_set_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).ASN1_PCTX_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ess_cert_id(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ESS_CERT_ID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_new() -> *mut APTR { ((*IAmiSSL).TS_VERIFY_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_extension_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).TS_RESP_CTX_set_extension_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_status_info_cond(a0: *mut APTR, a1: i32, a2: *const APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_status_info_cond)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify(a0: *mut APTR, a1: *const u8, a2: u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_verify)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_crl_method_new(a0: APTR, a1: APTR, a2: APTR, a3: APTR) -> *mut APTR { ((*IAmiSSL).X509_CRL_METHOD_new)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_digest_sign_final(a0: *mut APTR, a1: *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).EVP_DigestSignFinal)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_def_policy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_def_policy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_create_response(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_RESP_create_response)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_signer_info_get0_algs(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) { ((*IAmiSSL).PKCS7_SIGNER_INFO_get0_algs)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_nonce(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_TST_INFO_get_nonce)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_decrypt_old(a0: *mut u8, a1: *const u8, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_decrypt_old)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_policy_id(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_policy_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_ess_cert_id_chain(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_ess_cert_id_chain)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_pkey(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_CTX_get0_pkey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ts_req(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_TS_REQ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_find_str(a0: *mut *mut APTR, a1: *const APTR, a2: i32) -> *const APTR { ((*IAmiSSL).EVP_PKEY_asn1_find_str)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_f_asn1() -> *const APTR { ((*IAmiSSL).BIO_f_asn1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ess_signing_cert_new() -> *mut APTR { ((*IAmiSSL).ESS_SIGNING_CERT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pbe_find(a0: i32, a1: i32, a2: *mut i32, a3: *mut i32, a4: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PBE_find)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_crl_get0_by_cert(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).X509_CRL_get0_by_cert)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_derive(a0: *mut APTR, a1: *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_derive)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ts_req(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_TS_REQ)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_delete_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_delete_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ess_issuer_serial_free(a0: *mut APTR) { ((*IAmiSSL).ESS_ISSUER_SERIAL_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_pctx_set_str_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).ASN1_PCTX_set_str_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_signer_key(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_signer_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_accuracy_get_millis(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_ACCURACY_get_millis)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_get_token(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_RESP_get_token)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_accuracy_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_ACCURACY_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_set_default_method(a0: *const APTR) { ((*IAmiSSL).X509_CRL_set_default_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_free(a0: *mut APTR) { ((*IAmiSSL).TS_RESP_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_issuing_dist_point_free(a0: *mut APTR) { ((*IAmiSSL).ISSUING_DIST_POINT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ess_issuer_serial_new() -> *mut APTR { ((*IAmiSSL).ESS_ISSUER_SERIAL_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs7_add1_attrib_digest(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).PKCS7_add1_attrib_digest)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_add_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_add_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pem_write_bio_parameters(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_Parameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_accuracy(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_get_accuracy)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get0_by_serial(a0: *mut APTR, a1: *mut *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_get0_by_serial)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_version(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_get_tst_info(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_RESP_CTX_get_tst_info)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_verify_signature(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).TS_RESP_verify_signature)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_tsa(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_get_tsa)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_status_info_new() -> *mut APTR { ((*IAmiSSL).TS_STATUS_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_cb(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_get_ext_d2i(a0: *mut APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).TS_REQ_get_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_general_name_set0_othername(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).GENERAL_NAME_set0_othername)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_ext_count(a0: *mut APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_get_ext_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_get_request(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_RESP_CTX_get_request)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_signctx(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_signctx)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_copy(a0: *mut APTR, a1: *const APTR) { ((*IAmiSSL).EVP_PKEY_asn1_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_type_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_TYPE_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_do_all_sorted(a0: APTR, a1: *mut ()) { ((*IAmiSSL).EVP_CIPHER_do_all_sorted)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_free(a0: *mut APTR) { ((*IAmiSSL).EVP_PKEY_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get1_certs(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get1_certs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_operation(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_operation)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ess_signing_cert(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ESS_SIGNING_CERT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_conf_set_ordering(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_ordering)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pbe_alg_add_type(a0: i32, a1: i32, a2: i32, a3: i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PBE_alg_add_type)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_req_set_version(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).TS_REQ_set_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0(a0: *const APTR) -> *mut () { ((*IAmiSSL).EVP_PKEY_get0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_asn1_set_suffix(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BIO_asn1_set_suffix)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ts_status_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_TS_STATUS_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_do_all(a0: APTR, a1: *mut ()) { ((*IAmiSSL).EVP_MD_do_all)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_accuracy(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_accuracy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_add_attrib_content_type(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_add_attrib_content_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_add0(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_meth_add0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_tsa(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_tsa)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_new(a0: i32, a1: i32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_meth_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_accuracy(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_accuracy)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_pctx_set_oid_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).ASN1_PCTX_set_oid_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ess_signing_cert_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ESS_SIGNING_CERT_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ts_req_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_TS_REQ_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_time_adj_ex(a0: *mut APTR, a1: i32, a2: i32, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_time_adj_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_add_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).TS_RESP_CTX_add_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ts_status_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_TS_STATUS_INFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_msg_imprint_set_msg(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).TS_MSG_IMPRINT_set_msg)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_asn1_get_suffix(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).BIO_asn1_get_suffix)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_req_free(a0: *mut APTR) { ((*IAmiSSL).TS_REQ_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_free(a0: *mut APTR) { ((*IAmiSSL).EVP_PKEY_meth_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_get_exts(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_REQ_get_exts)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_clock_precision_digits(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_clock_precision_digits)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_add_failure_info(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).TS_RESP_CTX_add_failure_info)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ts_resp_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_TS_RESP_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_peerkey(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_CTX_get0_peerkey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_req_new() -> *mut APTR { ((*IAmiSSL).TS_REQ_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ts_msg_imprint_new() -> *mut APTR { ((*IAmiSSL).TS_MSG_IMPRINT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_find(a0: i32) -> *const APTR { ((*IAmiSSL).EVP_PKEY_meth_find)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_id(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_serial(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_serial)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_a2i_general_name(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: i32, a4: *const APTR, a5: i32) -> *mut APTR { ((*IAmiSSL).a2i_GENERAL_NAME)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ts_conf_set_crypto_device(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_crypto_device)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_conf_set_policies(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_policies)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_pctx_new() -> *mut APTR { ((*IAmiSSL).ASN1_PCTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ess_cert_id_free(a0: *mut APTR) { ((*IAmiSSL).ESS_CERT_ID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_msg_imprint_free(a0: *mut APTR) { ((*IAmiSSL).TS_MSG_IMPRINT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_init(a0: *mut APTR) { ((*IAmiSSL).TS_VERIFY_CTX_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_stream(a0: *mut *mut u8, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_stream)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_certs(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_certs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_def_policy(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_def_policy)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_adj(a0: *mut APTR, a1: APTR, a2: i32, a3: i32) -> *mut APTR { ((*IAmiSSL).ASN1_GENERALIZEDTIME_adj)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_accuracy_free(a0: *mut APTR) { ((*IAmiSSL).TS_ACCURACY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_get_tst_info(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_RESP_get_tst_info)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_derive_set_peer(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_derive_set_peer)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pem_read_bio_parameters(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_Parameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_clock_precision_digits(a0: *const APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_clock_precision_digits)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ess_issuer_serial_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ESS_ISSUER_SERIAL_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_accuracy_get_micros(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_ACCURACY_get_micros)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_pctx_get_str_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).ASN1_PCTX_get_str_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_name_constraints_check(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).NAME_CONSTRAINTS_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_bit_string_check(a0: *const APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).ASN1_BIT_STRING_check)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_check_akid(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_check_akid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_pctx_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_PCTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pem_write_bio_asn1_stream(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32, a4: *const APTR, a5: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_ASN1_stream)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_i2d_asn1_bio_stream(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32, a4: *const APTR) -> i32 { ((*IAmiSSL).i2d_ASN1_bio_stream)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_x509_algor_print_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_X509_ALGOR_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_cleanup(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_cleanup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_free(a0: *mut APTR) { ((*IAmiSSL).EVP_PKEY_asn1_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ess_signing_cert_free(a0: *mut APTR) { ((*IAmiSSL).ESS_SIGNING_CERT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_tst_info_set_msg_imprint(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_TST_INFO_set_msg_imprint)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_general_name_cmp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).GENERAL_NAME_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_asn1_set_any(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_SET_ANY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_asn1_sequence_any(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASN1_SEQUENCE_ANY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_general_name_get0_other_name(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).GENERAL_NAME_get0_otherName)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ess_cert_id(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ESS_CERT_ID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obj_find_sigid_algs(a0: i32, a1: *mut i32, a2: *mut i32) -> i32 { ((*IAmiSSL).OBJ_find_sigid_algs)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_keygen(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_keygen)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs5_pbkdf2_hmac(a0: *const APTR, a1: i32, a2: *const u8, a3: i32, a4: i32, a5: *const APTR, a6: i32, a7: *mut u8) -> i32 { ((*IAmiSSL).PKCS5_PBKDF2_HMAC)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_evp_pkey_paramgen(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_paramgen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_paramgen(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_paramgen)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_new_pkcs7(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_PKCS7)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_recover(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_recover)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ts_ext_print_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_ext_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_asn1_integer_print_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_ASN1_INTEGER_print_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_type(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_set_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_accuracy_set_micros(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_ACCURACY_set_micros)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_req_to_ts_verify_ctx(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_REQ_to_TS_VERIFY_CTX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_copy(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_pctx_set_cert_flags(a0: *mut APTR, a1: u32) { ((*IAmiSSL).ASN1_PCTX_set_cert_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_ext(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).TS_TST_INFO_get_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_ctrl(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_ctrl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_tst_info_get_ext_by_critical(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).TS_TST_INFO_get_ext_by_critical)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_new_id(a0: i32, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_CTX_new_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_req_get_ext_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).TS_REQ_get_ext_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_conf_set_signer_cert(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_signer_cert)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_name_hash_old(a0: *const APTR) -> u32 { ((*IAmiSSL).X509_NAME_hash_old)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_time_set_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_TIME_set_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_get_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).EVP_MD_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_free(a0: *mut APTR) { ((*IAmiSSL).TS_RESP_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_add1_header(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_add1_header)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obsolete_ocsp_req_ctx_set1_req(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_OCSP_REQ_CTX_set1_req)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_set_verify_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_verify_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_current_crl(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_current_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_parent_ctx(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_parent_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_current_issuer(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_current_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_method_get_prompt_constructor(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).UI_method_get_prompt_constructor)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_method_set_prompt_constructor(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).UI_method_set_prompt_constructor)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_read_pw_string_min(a0: *mut APTR, a1: i32, a2: i32, a3: *const APTR, a4: i32) -> i32 { ((*IAmiSSL).EVP_read_pw_string_min)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_cts128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_cts128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_cts128_decrypt_block(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_cts128_decrypt_block)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_cfb128_1_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: *mut i32, a6: i32, a7: APTR) { ((*IAmiSSL).CRYPTO_cfb128_1_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_crypto_cbc128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) { ((*IAmiSSL).CRYPTO_cbc128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_ctr128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: *mut u8, a6: *mut APTR, a7: APTR) { ((*IAmiSSL).CRYPTO_ctr128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_crypto_ofb128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: *mut i32, a6: APTR) { ((*IAmiSSL).CRYPTO_ofb128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_crypto_cts128_decrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_cts128_decrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_cts128_encrypt_block(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_cts128_encrypt_block)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_cbc128_decrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) { ((*IAmiSSL).CRYPTO_cbc128_decrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_cfb128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: *mut i32, a6: i32, a7: APTR) { ((*IAmiSSL).CRYPTO_cfb128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_crypto_cfb128_8_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: *mut i32, a6: i32, a7: APTR) { ((*IAmiSSL).CRYPTO_cfb128_8_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_openssl_gmtime(a0: *const APTR, a1: *mut tm) -> *mut tm { ((*IAmiSSL).OPENSSL_gmtime)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_gmtime_adj(a0: *mut tm, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OPENSSL_gmtime_adj)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_gmtime_diff(a0: *mut i32, a1: *mut i32, a2: *const tm, a3: *const tm) -> i32 { ((*IAmiSSL).OPENSSL_gmtime_diff)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get0_info(a0: *mut i32, a1: *mut i32, a2: *const APTR) { ((*IAmiSSL).EVP_PKEY_meth_get0_info)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_copy(a0: *mut APTR, a1: *const APTR) { ((*IAmiSSL).EVP_PKEY_meth_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_padding_add_pkcs1_pss_mgf1(a0: *mut APTR, a1: *mut u8, a2: *const u8, a3: *const APTR, a4: *const APTR, a5: i32) -> i32 { ((*IAmiSSL).RSA_padding_add_PKCS1_PSS_mgf1)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_rsa_verify_pkcs1_pss_mgf1(a0: *mut APTR, a1: *const u8, a2: *const APTR, a3: *const APTR, a4: *const u8, a5: i32) -> i32 { ((*IAmiSSL).RSA_verify_PKCS1_PSS_mgf1)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_x509_algor_set_md(a0: *mut APTR, a1: *const APTR) { ((*IAmiSSL).X509_ALGOR_set_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cmac_ctx_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).CMAC_CTX_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cmac_ctx_free(a0: *mut APTR) { ((*IAmiSSL).CMAC_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cmac_ctx_get0_cipher_ctx(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMAC_CTX_get0_cipher_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cmac_ctx_cleanup(a0: *mut APTR) { ((*IAmiSSL).CMAC_CTX_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cmac_init(a0: *mut APTR, a1: *const (), a2: u32, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).CMAC_Init)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cmac_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).CMAC_Update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cmac_resume(a0: *mut APTR) -> i32 { ((*IAmiSSL).CMAC_resume)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cmac_ctx_new() -> *mut APTR { ((*IAmiSSL).CMAC_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cmac_final(a0: *mut APTR, a1: *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).CMAC_Final)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_ctr128_encrypt_ctr32(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: *mut u8, a6: *mut APTR, a7: APTR) { ((*IAmiSSL).CRYPTO_ctr128_encrypt_ctr32)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_release(a0: *mut APTR) { ((*IAmiSSL).CRYPTO_gcm128_release)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_ccm128_decrypt_ccm64(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32, a4: APTR) -> i32 { ((*IAmiSSL).CRYPTO_ccm128_decrypt_ccm64)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_ccm128_encrypt(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32) -> i32 { ((*IAmiSSL).CRYPTO_ccm128_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_encrypt(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32) -> i32 { ((*IAmiSSL).CRYPTO_gcm128_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_xts128_encrypt(a0: *const APTR, a1: *const u8, a2: *const u8, a3: *mut u8, a4: u32, a5: i32) -> i32 { ((*IAmiSSL).CRYPTO_xts128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_nistcts128_decrypt_block(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_nistcts128_decrypt_block)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_setiv(a0: *mut APTR, a1: *const u8, a2: u32) { ((*IAmiSSL).CRYPTO_gcm128_setiv)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_nistcts128_encrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_nistcts128_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_tag(a0: *mut APTR, a1: *mut u8, a2: u32) { ((*IAmiSSL).CRYPTO_gcm128_tag)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_ccm128_encrypt_ccm64(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32, a4: APTR) -> i32 { ((*IAmiSSL).CRYPTO_ccm128_encrypt_ccm64)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_ccm128_setiv(a0: *mut APTR, a1: *const u8, a2: u32, a3: u32) -> i32 { ((*IAmiSSL).CRYPTO_ccm128_setiv)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_nistcts128_encrypt_block(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_nistcts128_encrypt_block)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_aad(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).CRYPTO_gcm128_aad)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_ccm128_init(a0: *mut APTR, a1: APTR, a2: APTR, a3: *mut (), a4: APTR) { ((*IAmiSSL).CRYPTO_ccm128_init)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_nistcts128_decrypt(a0: *const u8, a1: *mut u8, a2: u32, a3: *const (), a4: *mut u8, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_nistcts128_decrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_new(a0: *mut (), a1: APTR) -> *mut APTR { ((*IAmiSSL).CRYPTO_gcm128_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_ccm128_tag(a0: *mut APTR, a1: *mut u8, a2: u32) -> u32 { ((*IAmiSSL).CRYPTO_ccm128_tag)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_ccm128_decrypt(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32) -> i32 { ((*IAmiSSL).CRYPTO_ccm128_decrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_ccm128_aad(a0: *mut APTR, a1: *const u8, a2: u32) { ((*IAmiSSL).CRYPTO_ccm128_aad)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_init(a0: *mut APTR, a1: *mut (), a2: APTR) { ((*IAmiSSL).CRYPTO_gcm128_init)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_decrypt(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32) -> i32 { ((*IAmiSSL).CRYPTO_gcm128_decrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_decrypt_ctr32(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32, a4: APTR) -> i32 { ((*IAmiSSL).CRYPTO_gcm128_decrypt_ctr32)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_encrypt_ctr32(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32, a4: APTR) -> i32 { ((*IAmiSSL).CRYPTO_gcm128_encrypt_ctr32)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_gcm128_finish(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).CRYPTO_gcm128_finish)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs5_pbkdf2_set(a0: i32, a1: *mut u8, a2: i32, a3: i32, a4: i32) -> *mut APTR { ((*IAmiSSL).PKCS5_pbkdf2_set)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_pss_params_new() -> *mut APTR { ((*IAmiSSL).RSA_PSS_PARAMS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_rsa_pss_params(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_RSA_PSS_PARAMS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rsa_pss_params_free(a0: *mut APTR) { ((*IAmiSSL).RSA_PSS_PARAMS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_rsa_pss_params(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_RSA_PSS_PARAMS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_policy_mapping_it() -> *const APTR { ((*IAmiSSL).POLICY_MAPPING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_name_constraints_it() -> *const APTR { ((*IAmiSSL).NAME_CONSTRAINTS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_octet_string_ndef_it() -> *const APTR { ((*IAmiSSL).ASN1_OCTET_STRING_NDEF_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_policy_constraints_it() -> *const APTR { ((*IAmiSSL).POLICY_CONSTRAINTS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_policy_mappings_it() -> *const APTR { ((*IAmiSSL).POLICY_MAPPINGS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_general_subtree_it() -> *const APTR { ((*IAmiSSL).GENERAL_SUBTREE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_extensions_it() -> *const APTR { ((*IAmiSSL).X509_EXTENSIONS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_algors_it() -> *const APTR { ((*IAmiSSL).X509_ALGORS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_content_info_it() -> *const APTR { ((*IAmiSSL).CMS_ContentInfo_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_receipt_request_it() -> *const APTR { ((*IAmiSSL).CMS_ReceiptRequest_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_sequence_any_it() -> *const APTR { ((*IAmiSSL).ASN1_SEQUENCE_ANY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_set_any_it() -> *const APTR { ((*IAmiSSL).ASN1_SET_ANY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_issuing_dist_point_it() -> *const APTR { ((*IAmiSSL).ISSUING_DIST_POINT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_pss_params_it() -> *const APTR { ((*IAmiSSL).RSA_PSS_PARAMS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pem_write_bio_dhxparams(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_DHxparams)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_algor_cmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_ALGOR_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_string_clear_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_STRING_clear_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_time_diff(a0: *mut i32, a1: *mut i32, a2: *const APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).ASN1_TIME_diff)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_hex_string(a0: *mut APTR, a1: i32, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).BIO_hex_string)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_obsolete_buf_strnlen(a0: *const APTR, a1: u32) -> u32 { ((*IAmiSSL).OBSOLETE_BUF_strnlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_encrypted_key_cert_cmp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientEncryptedKey_cert_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_encrypted_key_get0_id(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR, a5: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientEncryptedKey_get0_id)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_encrypt(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_encrypt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_get0_pkey_ctx(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_RecipientInfo_get0_pkey_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kari_decrypt(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kari_decrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kari_get0_alg(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kari_get0_alg)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kari_get0_ctx(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_RecipientInfo_kari_get0_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kari_get0_orig_id(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR, a5: *mut *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kari_get0_orig_id)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kari_get0_reks(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_RecipientInfo_kari_get0_reks)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kari_orig_id_cmp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kari_orig_id_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kari_set0_pkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kari_set0_pkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_shared_info_encode(a0: *mut *mut u8, a1: *mut APTR, a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).CMS_SharedInfo_encode)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_signer_info_get0_md_ctx(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_SignerInfo_get0_md_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_signer_info_get0_pkey_ctx(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_SignerInfo_get0_pkey_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_signer_info_get0_signature(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_SignerInfo_get0_signature)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_dhxparams(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_DHxparams)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_dhxparams(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_DHxparams)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_compute_key_padded(a0: *mut u8, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).DH_compute_key_padded)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dh_get_1024_160() -> *mut APTR { ((*IAmiSSL).DH_get_1024_160)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dh_get_2048_224() -> *mut APTR { ((*IAmiSSL).DH_get_2048_224)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dh_get_2048_256() -> *mut APTR { ((*IAmiSSL).DH_get_2048_256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dh_kdf_x9_42(a0: *mut u8, a1: u32, a2: *const u8, a3: u32, a4: *mut APTR, a5: *const u8, a6: u32, a7: *const APTR) -> i32 { ((*IAmiSSL).DH_KDF_X9_42)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ec_curve_nid2nist(a0: i32) -> *const APTR { ((*IAmiSSL).EC_curve_nid2nist)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_curve_nist2nid(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_curve_nist2nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get_mont_data(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_get_mont_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdh_kdf_x9_62(a0: *mut u8, a1: u32, a2: *const u8, a3: u32, a4: *const u8, a5: u32, a6: *const APTR) -> i32 { ((*IAmiSSL).ECDH_KDF_X9_62)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_aes_128_cbc_hmac_sha256() -> *const APTR { ((*IAmiSSL).EVP_aes_128_cbc_hmac_sha256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_wrap() -> *const APTR { ((*IAmiSSL).EVP_aes_128_wrap)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_wrap() -> *const APTR { ((*IAmiSSL).EVP_aes_192_wrap)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_cbc_hmac_sha256() -> *const APTR { ((*IAmiSSL).EVP_aes_256_cbc_hmac_sha256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_wrap() -> *const APTR { ((*IAmiSSL).EVP_aes_256_wrap)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_des_ede3_wrap() -> *const APTR { ((*IAmiSSL).EVP_des_ede3_wrap)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crypto_128_unwrap(a0: *mut (), a1: *const u8, a2: *mut u8, a3: *const u8, a4: u32, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_128_unwrap)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_128_wrap(a0: *mut (), a1: *const u8, a2: *mut u8, a3: *const u8, a4: u32, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_128_wrap)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_get0_mem_bio(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_get0_mem_bio)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_ocsp_req_ctx_http(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OBSOLETE_OCSP_REQ_CTX_http)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obsolete_ocsp_req_ctx_i2d(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_OCSP_REQ_CTX_i2d)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_nbio(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_nbio)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_nbio_d2i(a0: *mut APTR, a1: *mut *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_nbio_d2i)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obsolete_ocsp_req_ctx_new(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OBSOLETE_OCSP_REQ_CTX_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_set_max_response_length(a0: *mut APTR, a1: u32) { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_set_max_response_length)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_rsa_oaep_params(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_RSA_OAEP_PARAMS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rsa_oaep_params_free(a0: *mut APTR) { ((*IAmiSSL).RSA_OAEP_PARAMS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_oaep_params_it() -> *const APTR { ((*IAmiSSL).RSA_OAEP_PARAMS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_oaep_params_new() -> *mut APTR { ((*IAmiSSL).RSA_OAEP_PARAMS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_padding_add_pkcs1_oaep_mgf1(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: *const u8, a5: i32, a6: *const APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).RSA_padding_add_PKCS1_OAEP_mgf1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_rsa_padding_check_pkcs1_oaep_mgf1(a0: *mut u8, a1: i32, a2: *const u8, a3: i32, a4: i32, a5: *const u8, a6: i32, a7: *const APTR, a8: *const APTR) -> i32 { ((*IAmiSSL).RSA_padding_check_PKCS1_OAEP_mgf1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_openssl_sk_deep_copy(a0: *const APTR, a1: APTR, a2: APTR) -> *mut APTR { ((*IAmiSSL).OPENSSL_sk_deep_copy)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_re_x509_tbs(a0: *mut APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_re_X509_tbs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_chain_check_suiteb(a0: *mut i32, a1: *mut APTR, a2: *mut APTR, a3: u32) -> i32 { ((*IAmiSSL).X509_chain_check_suiteb)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_chain_up_ref(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_chain_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_check_suiteb(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).X509_CRL_check_suiteb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_diff(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR, a4: APTR) -> *mut APTR { ((*IAmiSSL).X509_CRL_diff)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_obsolete_x509_crl_http_nbio(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_X509_CRL_http_nbio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_get0_signature(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: *const APTR) { ((*IAmiSSL).X509_get0_signature)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get_signature_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_get_signature_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_x509_http_nbio(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_X509_http_nbio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_revoked_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_REVOKED_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_store(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_store)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_add1_host(a0: *mut APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_add1_host)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get0(a0: i32) -> *const APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get0_peername(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get0_peername)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get_count() -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_get_count)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set1_email(a0: *mut APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set1_email)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set1_host(a0: *mut APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set1_host)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set1_ip(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set1_ip)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set1_ip_asc(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set1_ip_asc)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set_hostflags(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_VERIFY_PARAM_set_hostflags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_check_email(a0: *mut APTR, a1: *const APTR, a2: u32, a3: APTR) -> i32 { ((*IAmiSSL).X509_check_email)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_check_host(a0: *mut APTR, a1: *const APTR, a2: u32, a3: APTR, a4: *mut *mut APTR) -> i32 { ((*IAmiSSL).X509_check_host)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_check_ip(a0: *mut APTR, a1: *const u8, a2: u32, a3: APTR) -> i32 { ((*IAmiSSL).X509_check_ip)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_check_ip_asc(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).X509_check_ip_asc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_item(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_item)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dtls_client_method() -> *const APTR { ((*IAmiSSL).DTLS_client_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dtls_method() -> *const APTR { ((*IAmiSSL).DTLS_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dtls_server_method() -> *const APTR { ((*IAmiSSL).DTLS_server_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_certs_clear(a0: *mut APTR) { ((*IAmiSSL).SSL_certs_clear)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_find(a0: *mut APTR, a1: *const u8) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_find)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_conf_cmd(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).SSL_CONF_cmd)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_conf_cmd_argv(a0: *mut APTR, a1: *mut i32, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).SSL_CONF_cmd_argv)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_conf_cmd_value_type(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CONF_cmd_value_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_conf_ctx_clear_flags(a0: *mut APTR, a1: APTR) -> APTR { ((*IAmiSSL).SSL_CONF_CTX_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_conf_ctx_finish(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CONF_CTX_finish)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_conf_ctx_free(a0: *mut APTR) { ((*IAmiSSL).SSL_CONF_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_conf_ctx_new() -> *mut APTR { ((*IAmiSSL).SSL_CONF_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_conf_ctx_set1_prefix(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CONF_CTX_set1_prefix)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_conf_ctx_set_flags(a0: *mut APTR, a1: APTR) -> APTR { ((*IAmiSSL).SSL_CONF_CTX_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_conf_ctx_set_ssl(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_CONF_CTX_set_ssl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_conf_ctx_set_ssl_ctx(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_CONF_CTX_set_ssl_ctx)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_add_client_custom_ext(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR, a4: *mut (), a5: APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_add_client_custom_ext)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ssl_ctx_add_server_custom_ext(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR, a4: *mut (), a5: APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_add_server_custom_ext)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get0_certificate(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_get0_certificate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get0_param(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_get0_param)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get0_privatekey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_get0_privatekey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_ssl_method(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CTX_get_ssl_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_alpn_protos(a0: *mut APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_alpn_protos)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_alpn_select_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_CTX_set_alpn_select_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_cert_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_CTX_set_cert_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_serverinfo(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SSL_CTX_use_serverinfo)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_serverinfo_file(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_use_serverinfo_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_extension_supported(a0: APTR) -> i32 { ((*IAmiSSL).SSL_extension_supported)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_alpn_selected(a0: *const APTR, a1: *mut *mut APTR, a2: *mut APTR) { ((*IAmiSSL).SSL_get0_alpn_selected)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get0_param(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get0_param)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_is_server(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_is_server)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_alpn_protos(a0: *mut APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).SSL_set_alpn_protos)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_cert_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_set_cert_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_check_chain(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).SSL_check_chain)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_get_shared_sigalgs(a0: *mut APTR, a1: i32, a2: *mut i32, a3: *mut i32, a4: *mut i32, a5: *mut u8, a6: *mut u8) -> i32 { ((*IAmiSSL).SSL_get_shared_sigalgs)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ssl_get_sigalgs(a0: *mut APTR, a1: i32, a2: *mut i32, a3: *mut i32, a4: *mut i32, a5: *mut u8, a6: *mut u8) -> i32 { ((*IAmiSSL).SSL_get_sigalgs)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_asn1_add_stable_module() { ((*IAmiSSL).ASN1_add_stable_module)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_buf_print(a0: *mut APTR, a1: *const u8, a2: u32, a3: i32) -> i32 { ((*IAmiSSL).ASN1_buf_print)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_asn1_enumerated_get_int64(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_ENUMERATED_get_int64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_enumerated_set_int64(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ASN1_ENUMERATED_set_int64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_integer_get_int64(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_INTEGER_get_int64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_integer_get_uint64(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_INTEGER_get_uint64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_integer_set_int64(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ASN1_INTEGER_set_int64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_integer_set_uint64(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).ASN1_INTEGER_set_uint64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_sctx_free(a0: *mut APTR) { ((*IAmiSSL).ASN1_SCTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_sctx_get_app_data(a0: *mut APTR) -> *mut () { ((*IAmiSSL).ASN1_SCTX_get_app_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_sctx_get_flags(a0: *mut APTR) -> u32 { ((*IAmiSSL).ASN1_SCTX_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_sctx_get_item(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).ASN1_SCTX_get_item)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_sctx_get_template(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).ASN1_SCTX_get_template)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_sctx_new(a0: APTR) -> *mut APTR { ((*IAmiSSL).ASN1_SCTX_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_sctx_set_app_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).ASN1_SCTX_set_app_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_str2mask(a0: *const APTR, a1: *mut u32) -> i32 { ((*IAmiSSL).ASN1_str2mask)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_type_pack_sequence(a0: *const APTR, a1: *mut (), a2: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).ASN1_TYPE_pack_sequence)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_type_unpack_sequence(a0: *const APTR, a1: *const APTR) -> *mut () { ((*IAmiSSL).ASN1_TYPE_unpack_sequence)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_async_block_pause() { ((*IAmiSSL).ASYNC_block_pause)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_async_cleanup_thread() { ((*IAmiSSL).ASYNC_cleanup_thread)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_async_get_current_job() -> *mut APTR { ((*IAmiSSL).ASYNC_get_current_job)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_async_init_thread(a0: u32, a1: u32) -> i32 { ((*IAmiSSL).ASYNC_init_thread)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_async_pause_job() -> i32 { ((*IAmiSSL).ASYNC_pause_job)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_async_start_job(a0: *mut *mut APTR, a1: *mut APTR, a2: *mut i32, a3: APTR, a4: *mut (), a5: u32) -> i32 { ((*IAmiSSL).ASYNC_start_job)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_async_unblock_pause() { ((*IAmiSSL).ASYNC_unblock_pause)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_load_async_strings() -> i32 { ((*IAmiSSL).ERR_load_ASYNC_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_accept_ex(a0: i32, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).BIO_accept_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_addr_clear(a0: *mut APTR) { ((*IAmiSSL).BIO_ADDR_clear)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addr_family(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_ADDR_family)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addr_free(a0: *mut APTR) { ((*IAmiSSL).BIO_ADDR_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addr_hostname_string(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_ADDR_hostname_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_addr_new() -> *mut APTR { ((*IAmiSSL).BIO_ADDR_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_addr_path_string(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_ADDR_path_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addr_rawaddress(a0: *const APTR, a1: *mut (), a2: *mut u32) -> i32 { ((*IAmiSSL).BIO_ADDR_rawaddress)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_addr_rawmake(a0: *mut APTR, a1: i32, a2: *const (), a3: u32, a4: u16) -> i32 { ((*IAmiSSL).BIO_ADDR_rawmake)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bio_addr_rawport(a0: *const APTR) -> u16 { ((*IAmiSSL).BIO_ADDR_rawport)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addr_service_string(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).BIO_ADDR_service_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_addrinfo_address(a0: *const APTR) -> *const APTR { ((*IAmiSSL).BIO_ADDRINFO_address)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addrinfo_family(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_ADDRINFO_family)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addrinfo_free(a0: *mut APTR) { ((*IAmiSSL).BIO_ADDRINFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addrinfo_next(a0: *const APTR) -> *const APTR { ((*IAmiSSL).BIO_ADDRINFO_next)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addrinfo_protocol(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_ADDRINFO_protocol)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_addrinfo_socktype(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_ADDRINFO_socktype)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_closesocket(a0: i32) -> i32 { ((*IAmiSSL).BIO_closesocket)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_connect(a0: i32, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).BIO_connect)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_listen(a0: i32, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).BIO_listen)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_lookup(a0: *const APTR, a1: *const APTR, a2: APTR, a3: i32, a4: i32, a5: *mut *mut APTR) -> i32 { ((*IAmiSSL).BIO_lookup)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bio_parse_hostserv(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: APTR) -> i32 { ((*IAmiSSL).BIO_parse_hostserv)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_s_secmem() -> *const APTR { ((*IAmiSSL).BIO_s_secmem)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_sock_info(a0: i32, a1: APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BIO_sock_info)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_socket(a0: i32, a1: i32, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).BIO_socket)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_abs_is_word(a0: *const APTR, a1: APTR) -> i32 { ((*IAmiSSL).BN_abs_is_word)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_bn2binpad(a0: *const APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).BN_bn2binpad)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_bn2lebinpad(a0: *const APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).BN_bn2lebinpad)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_ctx_secure_new() -> *mut APTR { ((*IAmiSSL).BN_CTX_secure_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_gencb_free(a0: *mut APTR) { ((*IAmiSSL).BN_GENCB_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_gencb_get_arg(a0: *mut APTR) -> *mut () { ((*IAmiSSL).BN_GENCB_get_arg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_gencb_new() -> *mut APTR { ((*IAmiSSL).BN_GENCB_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_gencb_set(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).BN_GENCB_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_gencb_set_old(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).BN_GENCB_set_old)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_generate_dsa_nonce(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: u32, a5: *mut APTR) -> i32 { ((*IAmiSSL).BN_generate_dsa_nonce)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_get_flags(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).BN_get_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_is_negative(a0: *const APTR) -> i32 { ((*IAmiSSL).BN_is_negative)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_is_odd(a0: *const APTR) -> i32 { ((*IAmiSSL).BN_is_odd)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_is_one(a0: *const APTR) -> i32 { ((*IAmiSSL).BN_is_one)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_is_word(a0: *const APTR, a1: APTR) -> i32 { ((*IAmiSSL).BN_is_word)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_is_zero(a0: *const APTR) -> i32 { ((*IAmiSSL).BN_is_zero)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_lebin2bn(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_lebin2bn)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_nist_mod_func(a0: *const APTR) -> i32 { ((*IAmiSSL).BN_nist_mod_func)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_secure_new() -> *mut APTR { ((*IAmiSSL).BN_secure_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bn_security_bits(a0: i32, a1: i32) -> i32 { ((*IAmiSSL).BN_security_bits)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_set_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).BN_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_to_montgomery(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_to_montgomery)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_with_flags(a0: *mut APTR, a1: *const APTR, a2: i32) { ((*IAmiSSL).BN_with_flags)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_zero_ex(a0: *mut APTR) { ((*IAmiSSL).BN_zero_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_buf_mem_new_ex(a0: u32) -> *mut APTR { ((*IAmiSSL).BUF_MEM_new_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_comp_ctx_get_method(a0: *const APTR) -> *const APTR { ((*IAmiSSL).COMP_CTX_get_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_comp_ctx_get_type(a0: *const APTR) -> i32 { ((*IAmiSSL).COMP_CTX_get_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_comp_get_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).COMP_get_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_comp_get_type(a0: *const APTR) -> i32 { ((*IAmiSSL).COMP_get_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_clear_free(a0: *mut (), a1: u32, a2: *const APTR, a3: i32) { ((*IAmiSSL).CRYPTO_clear_free)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_clear_realloc(a0: *mut (), a1: u32, a2: u32, a3: *const APTR, a4: i32) -> *mut () { ((*IAmiSSL).CRYPTO_clear_realloc)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_free_ex_index(a0: i32, a1: i32) -> i32 { ((*IAmiSSL).CRYPTO_free_ex_index)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_memdup(a0: *const (), a1: u32, a2: *const APTR, a3: i32) -> *mut () { ((*IAmiSSL).CRYPTO_memdup)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_secure_actual_size(a0: *mut ()) -> u32 { ((*IAmiSSL).CRYPTO_secure_actual_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_secure_allocated(a0: *const ()) -> i32 { ((*IAmiSSL).CRYPTO_secure_allocated)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_secure_free(a0: *mut (), a1: *const APTR, a2: i32) { ((*IAmiSSL).CRYPTO_secure_free)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_secure_malloc(a0: u32, a1: *const APTR, a2: i32) -> *mut () { ((*IAmiSSL).CRYPTO_secure_malloc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_secure_malloc_done() -> i32 { ((*IAmiSSL).CRYPTO_secure_malloc_done)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crypto_secure_malloc_init(a0: u32, a1: u32) -> i32 { ((*IAmiSSL).CRYPTO_secure_malloc_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_secure_malloc_initialized() -> i32 { ((*IAmiSSL).CRYPTO_secure_malloc_initialized)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crypto_secure_used() -> u32 { ((*IAmiSSL).CRYPTO_secure_used)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crypto_secure_zalloc(a0: u32, a1: *const APTR, a2: i32) -> *mut () { ((*IAmiSSL).CRYPTO_secure_zalloc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_strndup(a0: *const APTR, a1: u32, a2: *const APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).CRYPTO_strndup)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_zalloc(a0: u32, a1: *const APTR, a2: i32) -> *mut () { ((*IAmiSSL).CRYPTO_zalloc)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_atexit(a0: APTR) -> i32 { ((*IAmiSSL).OPENSSL_atexit)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_cleanup() { ((*IAmiSSL).OPENSSL_cleanup)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_init_crypto(a0: APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OPENSSL_init_crypto)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_init_free(a0: *mut APTR) { ((*IAmiSSL).OPENSSL_INIT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_init_new() -> *mut APTR { ((*IAmiSSL).OPENSSL_INIT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_strlcat(a0: *mut APTR, a1: *const APTR, a2: u32) -> u32 { ((*IAmiSSL).OPENSSL_strlcat)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_strlcpy(a0: *mut APTR, a1: *const APTR, a2: u32) -> u32 { ((*IAmiSSL).OPENSSL_strlcpy)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_strnlen(a0: *const APTR, a1: u32) -> u32 { ((*IAmiSSL).OPENSSL_strnlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_thread_stop() { ((*IAmiSSL).OPENSSL_thread_stop)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_open_ssl_version(a0: i32) -> *const APTR { ((*IAmiSSL).OpenSSL_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_open_ssl_version_num() -> u32 { ((*IAmiSSL).OpenSSL_version_num)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dh_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_security_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_security_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_security_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_security_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get0_cofactor(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_GROUP_get0_cofactor)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get0_order(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_GROUP_get0_order)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_order_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_order_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_get_default_method() -> *const APTR { ((*IAmiSSL).EC_KEY_get_default_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_key_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).EC_KEY_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_get_method(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_KEY_get_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_key2buf(a0: *const APTR, a1: APTR, a2: *mut *mut u8, a3: *mut APTR) -> u32 { ((*IAmiSSL).EC_KEY_key2buf)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_key_method_free(a0: *mut APTR) { ((*IAmiSSL).EC_KEY_METHOD_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_method_get_compute_key(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EC_KEY_METHOD_get_compute_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_method_get_init(a0: *const APTR, a1: APTR, a2: APTR, a3: APTR, a4: APTR, a5: APTR, a6: APTR) { ((*IAmiSSL).EC_KEY_METHOD_get_init)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ec_key_method_get_keygen(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EC_KEY_METHOD_get_keygen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_method_get_sign(a0: *const APTR, a1: APTR, a2: APTR, a3: APTR) { ((*IAmiSSL).EC_KEY_METHOD_get_sign)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_key_method_get_verify(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EC_KEY_METHOD_get_verify)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_key_method_new(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_KEY_METHOD_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_method_set_compute_key(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EC_KEY_METHOD_set_compute_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_method_set_init(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR, a4: APTR, a5: APTR, a6: APTR) { ((*IAmiSSL).EC_KEY_METHOD_set_init)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ec_key_method_set_keygen(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EC_KEY_METHOD_set_keygen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_method_set_sign(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR) { ((*IAmiSSL).EC_KEY_METHOD_set_sign)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_key_method_set_verify(a0: *mut APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EC_KEY_METHOD_set_verify)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_key_new_method(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_KEY_new_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_oct2key(a0: *mut APTR, a1: *const u8, a2: u32, a3: *mut APTR) -> i32 { ((*IAmiSSL).EC_KEY_oct2key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_key_oct2priv(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EC_KEY_oct2priv)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_key_open_ssl() -> *const APTR { ((*IAmiSSL).EC_KEY_OpenSSL)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_key_priv2buf(a0: *const APTR, a1: *mut *mut u8) -> u32 { ((*IAmiSSL).EC_KEY_priv2buf)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_priv2oct(a0: *const APTR, a1: *mut u8, a2: u32) -> u32 { ((*IAmiSSL).EC_KEY_priv2oct)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_key_set_default_method(a0: *const APTR) { ((*IAmiSSL).EC_KEY_set_default_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_key_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).EC_KEY_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_key_set_method(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EC_KEY_set_method)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_point_point2buf(a0: *const APTR, a1: *const APTR, a2: APTR, a3: *mut *mut u8, a4: *mut APTR) -> u32 { ((*IAmiSSL).EC_POINT_point2buf)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ecdsa_sig_get0(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).ECDSA_SIG_get0)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_engine_get_default_ec() -> *mut APTR { ((*IAmiSSL).ENGINE_get_default_EC)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_get_ec(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ENGINE_get_EC)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_register_all_ec() { ((*IAmiSSL).ENGINE_register_all_EC)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_engine_register_ec(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_register_EC)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_default_ec(a0: *mut APTR) -> i32 { ((*IAmiSSL).ENGINE_set_default_EC)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_engine_set_ec(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ENGINE_set_EC)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_engine_unregister_ec(a0: *mut APTR) { ((*IAmiSSL).ENGINE_unregister_EC)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_aes_128_ocb() -> *const APTR { ((*IAmiSSL).EVP_aes_128_ocb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_128_wrap_pad() -> *const APTR { ((*IAmiSSL).EVP_aes_128_wrap_pad)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_ocb() -> *const APTR { ((*IAmiSSL).EVP_aes_192_ocb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_192_wrap_pad() -> *const APTR { ((*IAmiSSL).EVP_aes_192_wrap_pad)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_ocb() -> *const APTR { ((*IAmiSSL).EVP_aes_256_ocb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aes_256_wrap_pad() -> *const APTR { ((*IAmiSSL).EVP_aes_256_wrap_pad)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_128_ctr() -> *const APTR { ((*IAmiSSL).EVP_camellia_128_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_192_ctr() -> *const APTR { ((*IAmiSSL).EVP_camellia_192_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_camellia_256_ctr() -> *const APTR { ((*IAmiSSL).EVP_camellia_256_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_chacha20() -> *const APTR { ((*IAmiSSL).EVP_chacha20)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_chacha20_poly1305() -> *const APTR { ((*IAmiSSL).EVP_chacha20_poly1305)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_buf_noconst(a0: *mut APTR) -> *mut u8 { ((*IAmiSSL).EVP_CIPHER_CTX_buf_noconst)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_cipher_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).EVP_CIPHER_CTX_get_cipher_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_set_cipher_data(a0: *mut APTR, a1: *mut ()) -> *mut () { ((*IAmiSSL).EVP_CIPHER_CTX_set_cipher_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_is_encrypting(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_is_encrypting)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_iv(a0: *const APTR) -> *const u8 { ((*IAmiSSL).EVP_CIPHER_CTX_iv)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_iv_noconst(a0: *mut APTR) -> *mut u8 { ((*IAmiSSL).EVP_CIPHER_CTX_iv_noconst)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_num(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_num)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_original_iv(a0: *const APTR) -> *const u8 { ((*IAmiSSL).EVP_CIPHER_CTX_original_iv)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_reset(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_reset)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_set_num(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_set_num)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_impl_ctx_size(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_impl_ctx_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_CIPHER_meth_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_free(a0: *mut APTR) { ((*IAmiSSL).EVP_CIPHER_meth_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_get_cleanup(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_get_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_get_ctrl(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_get_ctrl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_get_do_cipher(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_get_do_cipher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_get_get_asn1_params(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_get_get_asn1_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_get_init(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_get_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_get_set_asn1_params(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_get_set_asn1_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_new(a0: i32, a1: i32, a2: i32) -> *mut APTR { ((*IAmiSSL).EVP_CIPHER_meth_new)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_cleanup(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_cleanup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_ctrl(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_ctrl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_do_cipher(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_do_cipher)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_flags(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_get_asn1_params(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_get_asn1_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_impl_ctx_size(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_impl_ctx_size)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_init(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_iv_length(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_iv_length)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_meth_set_set_asn1_params(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_meth_set_set_asn1_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_encode_ctx_free(a0: *mut APTR) { ((*IAmiSSL).EVP_ENCODE_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_encode_ctx_new() -> *mut APTR { ((*IAmiSSL).EVP_ENCODE_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_encode_ctx_num(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_ENCODE_CTX_num)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md5_sha1() -> *const APTR { ((*IAmiSSL).EVP_md5_sha1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: *mut ()) -> i32 { ((*IAmiSSL).EVP_MD_CTX_ctrl)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_free(a0: *mut APTR) { ((*IAmiSSL).EVP_MD_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_get0_md_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).EVP_MD_CTX_get0_md_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_new() -> *mut APTR { ((*IAmiSSL).EVP_MD_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_get_pkey_ctx(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_MD_CTX_get_pkey_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_reset(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_MD_CTX_reset)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_set_update_fn(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_MD_CTX_set_update_fn)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_update_fn(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_MD_CTX_update_fn)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_MD_meth_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_free(a0: *mut APTR) { ((*IAmiSSL).EVP_MD_meth_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_app_datasize(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_app_datasize)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_cleanup(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_copy(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_copy)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_ctrl(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_ctrl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_final(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_final)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_flags(a0: *const APTR) -> u32 { ((*IAmiSSL).EVP_MD_meth_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_init(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_input_blocksize(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_input_blocksize)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_result_size(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_result_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_get_update(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_get_update)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_meth_new(a0: i32, a1: i32) -> *mut APTR { ((*IAmiSSL).EVP_MD_meth_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_app_datasize(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_app_datasize)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_cleanup(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_cleanup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_copy(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_ctrl(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_ctrl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_final(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_final)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_flags(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_init(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_input_blocksize(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_input_blocksize)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_result_size(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_result_size)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_meth_set_update(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_MD_meth_set_update)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pbe_get(a0: *mut i32, a1: *mut i32, a2: u32) -> i32 { ((*IAmiSSL).EVP_PBE_get)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obsolete_evp_pbe_scrypt(a0: *const APTR, a1: u32, a2: *const u8, a3: u32, a4: u32, a5: u32, a6: u32, a7: APTR, a8: *mut u8, a9: u32) -> i32 { ((*IAmiSSL).OBSOLETE_EVP_PBE_scrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_security_bits(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_security_bits)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_dh(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_get0_DH)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_dsa(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_get0_DSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_ec_key(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_get0_EC_KEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_rsa(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_get0_RSA)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_cleanup(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_cleanup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_copy(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_ctrl(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_ctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_decrypt(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_decrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_derive(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_derive)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_encrypt(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_encrypt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_init(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_keygen(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_keygen)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_paramgen(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_paramgen)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_sign(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_signctx(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_signctx)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_verify(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_verify)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_verify_recover(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_verify_recover)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_verifyctx(a0: *const APTR, a1: APTR, a2: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_verifyctx)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_security_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_security_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs5_v2_scrypt_keyivgen(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *const APTR, a6: i32) -> i32 { ((*IAmiSSL).PKCS5_v2_scrypt_keyivgen)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_hmac_ctx_free(a0: *mut APTR) { ((*IAmiSSL).HMAC_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_hmac_ctx_new() -> *mut APTR { ((*IAmiSSL).HMAC_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_hmac_ctx_reset(a0: *mut APTR) -> i32 { ((*IAmiSSL).HMAC_CTX_reset)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_hmac_size(a0: *const APTR) -> u32 { ((*IAmiSSL).HMAC_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_lh_error(a0: *mut APTR) -> i32 { ((*IAmiSSL).OPENSSL_LH_error)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_lh_get_down_load(a0: *const APTR) -> u32 { ((*IAmiSSL).OPENSSL_LH_get_down_load)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_lh_set_down_load(a0: *mut APTR, a1: u32) { ((*IAmiSSL).OPENSSL_LH_set_down_load)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_128_unwrap_pad(a0: *mut (), a1: *const u8, a2: *mut u8, a3: *const u8, a4: u32, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_128_unwrap_pad)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_128_wrap_pad(a0: *mut (), a1: *const u8, a2: *mut u8, a3: *const u8, a4: u32, a5: APTR) -> u32 { ((*IAmiSSL).CRYPTO_128_wrap_pad)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_aad(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).CRYPTO_ocb128_aad)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_cleanup(a0: *mut APTR) { ((*IAmiSSL).CRYPTO_ocb128_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_copy_ctx(a0: *mut APTR, a1: *mut APTR, a2: *mut (), a3: *mut ()) -> i32 { ((*IAmiSSL).CRYPTO_ocb128_copy_ctx)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_decrypt(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32) -> i32 { ((*IAmiSSL).CRYPTO_ocb128_decrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_encrypt(a0: *mut APTR, a1: *const u8, a2: *mut u8, a3: u32) -> i32 { ((*IAmiSSL).CRYPTO_ocb128_encrypt)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_finish(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).CRYPTO_ocb128_finish)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_init(a0: *mut APTR, a1: *mut (), a2: *mut (), a3: APTR, a4: APTR, a5: APTR) -> i32 { ((*IAmiSSL).CRYPTO_ocb128_init)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_new(a0: *mut (), a1: *mut (), a2: APTR, a3: APTR, a4: APTR) -> *mut APTR { ((*IAmiSSL).CRYPTO_ocb128_new)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_setiv(a0: *mut APTR, a1: *const u8, a2: u32, a3: u32) -> i32 { ((*IAmiSSL).CRYPTO_ocb128_setiv)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_ocb128_tag(a0: *mut APTR, a1: *mut u8, a2: u32) -> i32 { ((*IAmiSSL).CRYPTO_ocb128_tag)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_obj_get0_data(a0: *const APTR) -> *const u8 { ((*IAmiSSL).OBJ_get0_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obj_length(a0: *const APTR) -> u32 { ((*IAmiSSL).OBJ_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get0_produced_at(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OCSP_resp_get0_produced_at)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get0_signature(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OCSP_resp_get0_signature)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_singleresp_get0_id(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OCSP_SINGLERESP_get0_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_get0_mac(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *const APTR) { ((*IAmiSSL).PKCS12_get0_mac)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs12_mac_present(a0: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_mac_present)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_create0_p8inf(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_create0_p8inf)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_create0_pkcs8(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_create0_pkcs8)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_create_cert(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_create_cert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_create_crl(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_create_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_create_pkcs8_encrypt(a0: i32, a1: *const APTR, a2: i32, a3: *mut u8, a4: i32, a5: i32, a6: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_create_pkcs8_encrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get0_attr(a0: *const APTR, a1: i32) -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get0_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get0_attrs(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get0_attrs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get0_p8inf(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get0_p8inf)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get0_pkcs8(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get0_pkcs8)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get0_safes(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get0_safes)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get0_type(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get0_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get1_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get1_cert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get1_crl(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get1_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get_bag_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_SAFEBAG_get_bag_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_SAFEBAG_get_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs8_get_attr(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).PKCS8_get_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs8_set0_pbe(a0: *const APTR, a1: i32, a2: *mut APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS8_set0_pbe)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rand_open_ssl() -> *mut APTR { ((*IAmiSSL).RAND_OpenSSL)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_check_key_ex(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).RSA_check_key_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_pkcs1_open_ssl() -> *const APTR { ((*IAmiSSL).RSA_PKCS1_OpenSSL)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_security_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_security_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_obsolete_rsa_x931_derive_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *const APTR, a6: *const APTR, a7: *const APTR, a8: *const APTR, a9: *const APTR, a10: *const APTR, a11: *const APTR, a12: *mut APTR) -> i32 { ((*IAmiSSL).OBSOLETE_RSA_X931_derive_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11, a12) }
#[inline]
pub unsafe fn amissl_rsa_x931_generate_key_ex(a0: *mut APTR, a1: i32, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).RSA_X931_generate_key_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_conf_set_signer_digest(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_signer_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_signer_digest(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_signer_digest)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_status_info_get0_failure_info(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_STATUS_INFO_get0_failure_info)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_status_info_get0_status(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_STATUS_INFO_get0_status)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_status_info_set_status(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).TS_STATUS_INFO_set_status)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_add_flags(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).TS_VERIFY_CTX_add_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set_data(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_VERIFY_CTX_set_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set_flags(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).TS_VERIFY_CTX_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set_imprint(a0: *mut APTR, a1: *mut u8, a2: i32) -> *mut u8 { ((*IAmiSSL).TS_VERIFY_CTX_set_imprint)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set_store(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_VERIFY_CTX_set_store)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set_certs(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).TS_VERIFY_CTX_set_certs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_re_x509_crl_tbs(a0: *mut APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_re_X509_CRL_tbs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_re_x509_req_tbs(a0: *mut APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_re_X509_REQ_tbs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe2_set_scrypt(a0: *const APTR, a1: *const u8, a2: i32, a3: *mut u8, a4: APTR, a5: APTR, a6: APTR) -> *mut APTR { ((*IAmiSSL).PKCS5_pbe2_set_scrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_x509_crl_get0_extensions(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_CRL_get0_extensions)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get0_signature(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).X509_CRL_get0_signature)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_get_issuer(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_CRL_get_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get_last_update(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_CRL_get_lastUpdate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get_next_update(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_CRL_get_nextUpdate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get_revoked(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_CRL_get_REVOKED)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get_signature_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_get_signature_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get_version(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_CRL_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_CRL_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_extensions(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_get0_extensions)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_pubkey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_get0_pubkey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_reject_objects(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_get0_reject_objects)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_tbs_sigalg(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_get0_tbs_sigalg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_trust_objects(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_get0_trust_objects)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_uids(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).X509_get0_uids)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_getm_not_after(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_getm_notAfter)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_getm_not_before(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_getm_notBefore)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_signature_type(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_get_signature_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_version(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_x509_pubkey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_get_X509_PUBKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_name_entry_set(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_NAME_ENTRY_set)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_name_get0_der(a0: *const APTR, a1: *mut *mut APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).X509_NAME_get0_der)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_pubkey_get0(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_PUBKEY_get0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get0_signature(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).X509_REQ_get0_signature)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_req_get_signature_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_REQ_get_signature_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get_subject_name(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_get_subject_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get_version(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_REQ_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get_x509_pubkey(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_get_X509_PUBKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_revoked_get0_extensions(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_REVOKED_get0_extensions)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_revoked_get0_revocation_date(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_REVOKED_get0_revocationDate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_revoked_get0_serial_number(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_REVOKED_get0_serialNumber)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_trusted(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_trusted)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_num_untrusted(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_get_num_untrusted)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set0_dane(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set0_dane)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_move_peername(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_VERIFY_PARAM_move_peername)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2s_asn1_ia5_string(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).i2s_ASN1_IA5STRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_s2i_asn1_ia5_string(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).s2i_ASN1_IA5STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509v3_addr_add_inherit(a0: *mut APTR, a1: APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509v3_addr_add_inherit)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509v3_addr_add_prefix(a0: *mut APTR, a1: APTR, a2: *const APTR, a3: *mut u8, a4: APTR) -> i32 { ((*IAmiSSL).X509v3_addr_add_prefix)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509v3_addr_add_range(a0: *mut APTR, a1: APTR, a2: *const APTR, a3: *mut u8, a4: *mut u8) -> i32 { ((*IAmiSSL).X509v3_addr_add_range)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509v3_addr_canonize(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_addr_canonize)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_addr_get_afi(a0: *const APTR) -> APTR { ((*IAmiSSL).X509v3_addr_get_afi)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_addr_get_range(a0: *mut APTR, a1: APTR, a2: *mut u8, a3: *mut u8, a4: APTR) -> i32 { ((*IAmiSSL).X509v3_addr_get_range)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509v3_addr_inherits(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_addr_inherits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_addr_is_canonical(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_addr_is_canonical)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_addr_subset(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_addr_subset)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509v3_addr_validate_path(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_addr_validate_path)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_addr_validate_resource_set(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).X509v3_addr_validate_resource_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509v3_asid_add_inherit(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509v3_asid_add_inherit)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509v3_asid_add_id_or_range(a0: *mut APTR, a1: i32, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_asid_add_id_or_range)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509v3_asid_canonize(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_asid_canonize)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_asid_inherits(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_asid_inherits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_asid_is_canonical(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_asid_is_canonical)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_asid_subset(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_asid_subset)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509v3_asid_validate_path(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509v3_asid_validate_path)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509v3_asid_validate_resource_set(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).X509v3_asid_validate_resource_set)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get0_subject_key_id(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).X509_get0_subject_key_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_extended_key_usage(a0: *mut APTR) -> APTR { ((*IAmiSSL).X509_get_extended_key_usage)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_extension_flags(a0: *mut APTR) -> APTR { ((*IAmiSSL).X509_get_extension_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_key_usage(a0: *mut APTR) -> APTR { ((*IAmiSSL).X509_get_key_usage)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dtlsv1_listen(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).DTLSv1_listen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_init_ssl(a0: APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OPENSSL_init_ssl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_add1_host(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_add1_host)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_add_ssl_module() { ((*IAmiSSL).SSL_add_ssl_module)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_cipher_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CIPHER_get_cipher_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_digest_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CIPHER_get_digest_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_clear_options(a0: *mut APTR, a1: APTR) -> APTR { ((*IAmiSSL).SSL_clear_options)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_config(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_config)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_clear_options(a0: *mut APTR, a1: APTR) -> APTR { ((*IAmiSSL).SSL_CTX_clear_options)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_config(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_config)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_dane_enable(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_dane_enable)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_dane_mtype_set(a0: *mut APTR, a1: *const APTR, a2: APTR, a3: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_dane_mtype_set)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get0_security_ex_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).SSL_CTX_get0_security_ex_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_options(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_CTX_get_options)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_security_callback(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_security_callback)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_security_level(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_security_level)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set0_security_ex_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).SSL_CTX_set0_security_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_default_verify_dir(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_default_verify_dir)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_default_verify_file(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_default_verify_file)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_not_resumable_session_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_not_resumable_session_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_options(a0: *mut APTR, a1: APTR) -> APTR { ((*IAmiSSL).SSL_CTX_set_options)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_security_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_security_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_security_level(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_CTX_set_security_level)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_dane_enable(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_dane_enable)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_dane_tlsa_add(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR, a4: *mut APTR, a5: u32) -> i32 { ((*IAmiSSL).SSL_dane_tlsa_add)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ssl_get0_dane(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get0_dane)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_dane_authority(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).SSL_get0_dane_authority)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get0_dane_tlsa(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut *mut APTR, a5: *mut u32) -> i32 { ((*IAmiSSL).SSL_get0_dane_tlsa)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ssl_get0_peername(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).SSL_get0_peername)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_security_ex_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).SSL_get0_security_ex_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get1_supported_ciphers(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get1_supported_ciphers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_client_random(a0: *const APTR, a1: *mut u8, a2: u32) -> u32 { ((*IAmiSSL).SSL_get_client_random)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_options(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_get_options)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_security_callback(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_security_callback)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_security_level(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_security_level)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_server_random(a0: *const APTR, a1: *mut u8, a2: u32) -> u32 { ((*IAmiSSL).SSL_get_server_random)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_state(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_get_state)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_in_before(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_in_before)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_in_init(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_in_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_is_init_finished(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_is_init_finished)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_get0_ticket(a0: *const APTR, a1: *mut *mut APTR, a2: *mut u32) { ((*IAmiSSL).SSL_SESSION_get0_ticket)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_session_get_master_key(a0: *const APTR, a1: *mut u8, a2: u32) -> u32 { ((*IAmiSSL).SSL_SESSION_get_master_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_session_get_ticket_lifetime_hint(a0: *const APTR) -> u32 { ((*IAmiSSL).SSL_SESSION_get_ticket_lifetime_hint)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_has_ticket(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_has_ticket)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_print_keylog(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_print_keylog)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_reused(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_session_reused)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set0_security_ex_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).SSL_set0_security_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set1_host(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_set1_host)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_default_passwd_cb(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_set_default_passwd_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_default_passwd_cb_userdata(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).SSL_set_default_passwd_cb_userdata)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_hostflags(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_hostflags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_not_resumable_session_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_not_resumable_session_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_options(a0: *mut APTR, a1: APTR) -> APTR { ((*IAmiSSL).SSL_set_options)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set0_rbio(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_set0_rbio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_security_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_security_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_security_level(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_set_security_level)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set0_wbio(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_set0_wbio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_use_certificate_chain_file(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_use_certificate_chain_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_waiting_for_async(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_waiting_for_async)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_tls_client_method() -> *const APTR { ((*IAmiSSL).TLS_client_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_tls_method() -> *const APTR { ((*IAmiSSL).TLS_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_tls_server_method() -> *const APTR { ((*IAmiSSL).TLS_server_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_new() -> *mut APTR { ((*IAmiSSL).ASYNC_WAIT_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_free(a0: *mut APTR) { ((*IAmiSSL).ASYNC_WAIT_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_set_wait_fd(a0: *mut APTR, a1: *const (), a2: APTR, a3: *mut (), a4: APTR) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_set_wait_fd)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_get_fd(a0: *mut APTR, a1: *const (), a2: *mut APTR, a3: *mut *mut ()) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_get_fd)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_get_all_fds(a0: *mut APTR, a1: *mut APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_get_all_fds)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_get_changed_fds(a0: *mut APTR, a1: *mut APTR, a2: *mut u32, a3: *mut APTR, a4: *mut u32) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_get_changed_fds)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_clear_fd(a0: *mut APTR, a1: *const ()) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_clear_fd)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_async_is_capable() -> i32 { ((*IAmiSSL).ASYNC_is_capable)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_async_get_wait_ctx(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).ASYNC_get_wait_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).BIO_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_blinding_is_current_thread(a0: *mut APTR) -> i32 { ((*IAmiSSL).BN_BLINDING_is_current_thread)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_blinding_set_current_thread(a0: *mut APTR) { ((*IAmiSSL).BN_BLINDING_set_current_thread)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_blinding_lock(a0: *mut APTR) -> i32 { ((*IAmiSSL).BN_BLINDING_lock)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_blinding_unlock(a0: *mut APTR) -> i32 { ((*IAmiSSL).BN_BLINDING_unlock)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_thread_run_once(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).CRYPTO_THREAD_run_once)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_thread_init_local(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).CRYPTO_THREAD_init_local)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_thread_get_local(a0: *mut APTR) -> *mut () { ((*IAmiSSL).CRYPTO_THREAD_get_local)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_thread_set_local(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).CRYPTO_THREAD_set_local)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_thread_cleanup_local(a0: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_THREAD_cleanup_local)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_thread_get_current_id() -> APTR { ((*IAmiSSL).CRYPTO_THREAD_get_current_id)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crypto_thread_compare_id(a0: APTR, a1: APTR) -> i32 { ((*IAmiSSL).CRYPTO_THREAD_compare_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_thread_lock_new() -> *mut APTR { ((*IAmiSSL).CRYPTO_THREAD_lock_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_crypto_thread_read_lock(a0: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_THREAD_read_lock)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_thread_write_lock(a0: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_THREAD_write_lock)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_thread_unlock(a0: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_THREAD_unlock)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_thread_lock_free(a0: *mut APTR) { ((*IAmiSSL).CRYPTO_THREAD_lock_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_atomic_add(a0: *mut i32, a1: i32, a2: *mut i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_atomic_add)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_openssl_die(a0: *const APTR, a1: *const APTR, a2: i32) { ((*IAmiSSL).OPENSSL_die)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_new() -> *mut APTR { ((*IAmiSSL).CT_POLICY_EVAL_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_free(a0: *mut APTR) { ((*IAmiSSL).CT_POLICY_EVAL_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_get0_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).CT_POLICY_EVAL_CTX_get0_cert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_get0_issuer(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).CT_POLICY_EVAL_CTX_get0_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_get0_log_store(a0: *const APTR) -> *const APTR { ((*IAmiSSL).CT_POLICY_EVAL_CTX_get0_log_store)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_new() -> *mut APTR { ((*IAmiSSL).SCT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_sct_new_from_base64(a0: u8, a1: *const APTR, a2: APTR, a3: APTR, a4: *const APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).SCT_new_from_base64)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_sct_free(a0: *mut APTR) { ((*IAmiSSL).SCT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_list_free(a0: *mut APTR) { ((*IAmiSSL).SCT_LIST_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_get_version(a0: *const APTR) -> APTR { ((*IAmiSSL).SCT_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_set_version(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SCT_set_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_get_log_entry_type(a0: *const APTR) -> APTR { ((*IAmiSSL).SCT_get_log_entry_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_set_log_entry_type(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SCT_set_log_entry_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_get0_log_id(a0: *const APTR, a1: *mut *mut u8) -> u32 { ((*IAmiSSL).SCT_get0_log_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_set0_log_id(a0: *mut APTR, a1: *mut u8, a2: u32) -> i32 { ((*IAmiSSL).SCT_set0_log_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sct_set1_log_id(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SCT_set1_log_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sct_get_timestamp(a0: *const APTR) -> APTR { ((*IAmiSSL).SCT_get_timestamp)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_set_timestamp(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SCT_set_timestamp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_get_signature_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).SCT_get_signature_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_set_signature_nid(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SCT_set_signature_nid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_get0_extensions(a0: *const APTR, a1: *mut *mut u8) -> u32 { ((*IAmiSSL).SCT_get0_extensions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_set0_extensions(a0: *mut APTR, a1: *mut u8, a2: u32) { ((*IAmiSSL).SCT_set0_extensions)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sct_set1_extensions(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SCT_set1_extensions)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sct_get0_signature(a0: *const APTR, a1: *mut *mut u8) -> u32 { ((*IAmiSSL).SCT_get0_signature)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_set0_signature(a0: *mut APTR, a1: *mut u8, a2: u32) { ((*IAmiSSL).SCT_set0_signature)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sct_set1_signature(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SCT_set1_signature)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_sct_get_source(a0: *const APTR) -> APTR { ((*IAmiSSL).SCT_get_source)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_set_source(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SCT_set_source)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_print(a0: *const APTR, a1: *mut APTR, a2: i32, a3: *const APTR) { ((*IAmiSSL).SCT_print)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_sct_list_print(a0: *const APTR, a1: *mut APTR, a2: i32, a3: *const APTR, a4: *const APTR) { ((*IAmiSSL).SCT_LIST_print)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_sct_get_validation_status(a0: *const APTR) -> APTR { ((*IAmiSSL).SCT_get_validation_status)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_validate(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SCT_validate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_sct_list_validate(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SCT_LIST_validate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2o_sct_list(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2o_SCT_LIST)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_o2i_sct_list(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: u32) -> *mut APTR { ((*IAmiSSL).o2i_SCT_LIST)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_sct_list(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_SCT_LIST)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_sct_list(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_SCT_LIST)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2o_sct(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2o_SCT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_o2i_sct(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: u32) -> *mut APTR { ((*IAmiSSL).o2i_SCT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ctlog_new(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).CTLOG_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ctlog_new_from_base64(a0: *mut *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).CTLOG_new_from_base64)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ctlog_free(a0: *mut APTR) { ((*IAmiSSL).CTLOG_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ctlog_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).CTLOG_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ctlog_get0_log_id(a0: *const APTR, a1: *mut *mut APTR, a2: *mut u32) { ((*IAmiSSL).CTLOG_get0_log_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ctlog_get0_public_key(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).CTLOG_get0_public_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ctlog_store_new() -> *mut APTR { ((*IAmiSSL).CTLOG_STORE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ctlog_store_free(a0: *mut APTR) { ((*IAmiSSL).CTLOG_STORE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ctlog_store_get0_log_by_id(a0: *const APTR, a1: *const APTR, a2: u32) -> *const APTR { ((*IAmiSSL).CTLOG_STORE_get0_log_by_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ctlog_store_load_file(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).CTLOG_STORE_load_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ctlog_store_load_default_file(a0: *mut APTR) -> i32 { ((*IAmiSSL).CTLOG_STORE_load_default_file)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_ct_strings() -> i32 { ((*IAmiSSL).ERR_load_CT_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dhparams_it() -> *const APTR { ((*IAmiSSL).DHparams_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_dsa_sig_get0(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).DSA_SIG_get0)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_group_new_from_ecparameters(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_new_from_ecparameters)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get_ecparameters(a0: *const APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_get_ecparameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_group_new_from_ecpkparameters(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_new_from_ecpkparameters)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get_ecpkparameters(a0: *const APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_get_ecpkparameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ecpkparameters_it() -> *const APTR { ((*IAmiSSL).ECPKPARAMETERS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ecparameters_it() -> *const APTR { ((*IAmiSSL).ECPARAMETERS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_key_can_sign(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_KEY_can_sign)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_blake2b512() -> *const APTR { ((*IAmiSSL).EVP_blake2b512)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_blake2s256() -> *const APTR { ((*IAmiSSL).EVP_blake2s256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_str2ctrl(a0: *mut APTR, a1: i32, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_str2ctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_hex2ctrl(a0: *mut APTR, a1: i32, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_hex2ctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_err_load_kdf_strings() -> i32 { ((*IAmiSSL).ERR_load_KDF_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_srp_user_pwd_free(a0: *mut APTR) { ((*IAmiSSL).SRP_user_pwd_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_srp_vbase_get1_by_user(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).SRP_VBASE_get1_by_user)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_has_client_custom_ext(a0: *const APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_has_client_custom_ext)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_kx_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CIPHER_get_kx_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_auth_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CIPHER_get_auth_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_is_aead(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CIPHER_is_aead)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_has_pending(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_has_pending)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_default_passwd_cb(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_default_passwd_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_default_passwd_cb_userdata(a0: *mut APTR) -> *mut () { ((*IAmiSSL).SSL_CTX_get_default_passwd_cb_userdata)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_default_passwd_cb(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_default_passwd_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_default_passwd_cb_userdata(a0: *mut APTR) -> *mut () { ((*IAmiSSL).SSL_get_default_passwd_cb_userdata)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_all_async_fds(a0: *mut APTR, a1: *mut APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).SSL_get_all_async_fds)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_changed_async_fds(a0: *mut APTR, a1: *mut APTR, a2: *mut u32, a3: *mut APTR, a4: *mut u32) -> i32 { ((*IAmiSSL).SSL_get_changed_async_fds)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_default_read_buffer_len(a0: *mut APTR, a1: u32) { ((*IAmiSSL).SSL_CTX_set_default_read_buffer_len)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_default_read_buffer_len(a0: *mut APTR, a1: u32) { ((*IAmiSSL).SSL_set_default_read_buffer_len)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_ct_validation_callback(a0: *mut APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).SSL_set_ct_validation_callback)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_ct_validation_callback(a0: *mut APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_set_ct_validation_callback)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ct_is_enabled(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_ct_is_enabled)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_ct_is_enabled(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_ct_is_enabled)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_peer_scts(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).SSL_get0_peer_scts)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_default_ctlog_list_file(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_default_ctlog_list_file)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_ctlog_list_file(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_ctlog_list_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set0_ctlog_store(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_CTX_set0_ctlog_store)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get0_ctlog_store(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CTX_get0_ctlog_store)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_sig_get0(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).X509_SIG_get0)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs8_pkey_get0_attrs(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PKCS8_pkey_get0_attrs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs8_pkey_add1_attr_by_nid(a0: *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).PKCS8_pkey_add1_attr_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_store_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_enable_ct(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_enable_ct)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_enable_ct(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_CTX_enable_ct)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_ciphers(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_get_ciphers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_get0_hostname(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_SESSION_get0_hostname)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_sct_validation_status_string(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SCT_validation_status_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_new(a0: i32, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_meth_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_get_puts(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_puts)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_get_ctrl(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_ctrl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_get_gets(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_gets)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_get_data(a0: *mut APTR) -> *mut () { ((*IAmiSSL).BIO_get_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_set_init(a0: *mut APTR, a1: i32) { ((*IAmiSSL).BIO_set_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_set_puts(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_puts)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_get_shutdown(a0: *mut APTR) -> i32 { ((*IAmiSSL).BIO_get_shutdown)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_get_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).BIO_get_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_set_ctrl(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_ctrl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_set_read(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_read)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_set_shutdown(a0: *mut APTR, a1: i32) { ((*IAmiSSL).BIO_set_shutdown)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_set_create(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_create)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_get_write(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_write)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_set_callback_ctrl(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_callback_ctrl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_get_create(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_create)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_set_next(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).BIO_set_next)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_set_data(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).BIO_set_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_set_write(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_write)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_set_destroy(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_destroy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_set_gets(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_gets)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_get_callback_ctrl(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_callback_ctrl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_get_destroy(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_destroy)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_get_read(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_read)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_set_retry_reason(a0: *mut APTR, a1: i32) { ((*IAmiSSL).BIO_set_retry_reason)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_free(a0: *mut APTR) { ((*IAmiSSL).BIO_meth_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_bn_mod_exp(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_bn_mod_exp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_init(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_free(a0: *mut APTR) { ((*IAmiSSL).DSA_meth_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_mod_exp(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_mod_exp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_sign(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_sign)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_finish(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_finish)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_set_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).DSA_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_get0_pqg(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) { ((*IAmiSSL).DSA_get0_pqg)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dsa_meth_get0_app_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).DSA_meth_get0_app_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_keygen(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_keygen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_clear_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).DSA_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DSA_meth_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_paramgen(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_paramgen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_sign(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).DSA_meth_get_sign)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_paramgen(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_paramgen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_test_flags(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).DSA_test_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_set0_app_data(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).DSA_meth_set0_app_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_set1_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_set1_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_get0_key(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).DSA_get0_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_mod_exp(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_mod_exp)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_set0_pqg(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).DSA_set0_pqg)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_flags(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_verify(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_verify)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_verify(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_finish(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_finish)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_keygen(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_keygen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).DSA_meth_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_set0_key(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).DSA_set0_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_init(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_sign_setup(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DSA_meth_set_sign_setup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_bn_mod_exp(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_bn_mod_exp)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_get_method(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).DSA_get_method)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_meth_new(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).DSA_meth_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_set_flags(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).DSA_meth_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dsa_meth_get_sign_setup(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_meth_get_sign_setup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_get0_engine(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).DSA_get0_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set_auth_level(a0: *mut APTR, a1: i32) { ((*IAmiSSL).X509_VERIFY_PARAM_set_auth_level)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get_auth_level(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_get_auth_level)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_req_get0_pubkey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_get0_pubkey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_set0_key(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).RSA_set0_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_flags(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_finish(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_finish)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_priv_dec(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_priv_dec)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_sign(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_sign)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_bn_mod_exp(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_bn_mod_exp)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_test_flags(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).RSA_test_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_new(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).RSA_meth_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_get0_app_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).RSA_meth_get0_app_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).RSA_meth_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set1_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_set1_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_set0_app_data(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).RSA_meth_set0_app_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_set_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).RSA_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_sign(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_sign)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_clear_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).RSA_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_keygen(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_keygen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_keygen(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_keygen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_pub_dec(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_pub_dec)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_finish(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_finish)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_key(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) { ((*IAmiSSL).RSA_get0_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_get0_engine(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).RSA_get0_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_priv_enc(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_priv_enc)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_verify(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_get0_factors(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).RSA_get0_factors)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rsa_meth_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_meth_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_mod_exp(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_mod_exp)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_flags(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).RSA_meth_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_pub_dec(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_pub_dec)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_bn_mod_exp(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_bn_mod_exp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_init(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_free(a0: *mut APTR) { ((*IAmiSSL).RSA_meth_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_pub_enc(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_pub_enc)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_mod_exp(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_mod_exp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_set0_factors(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).RSA_set0_factors)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_pub_enc(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_pub_enc)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_priv_dec(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_priv_dec)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_verify(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_verify)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_init(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_priv_enc(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_priv_enc)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_set0_crt_params(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).RSA_set0_crt_params)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_get0_crt_params(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) { ((*IAmiSSL).RSA_get0_crt_params)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dh_set0_pqg(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).DH_set0_pqg)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dh_clear_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).DH_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_get0_key(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).DH_get0_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dh_get0_engine(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).DH_get0_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_set0_key(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).DH_set0_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dh_set_length(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).DH_set_length)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_test_flags(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).DH_test_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_get_length(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_get_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_get0_pqg(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) { ((*IAmiSSL).DH_get0_pqg)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_dh_meth_get_compute_key(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_meth_get_compute_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_set1_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).DH_meth_set1_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_set_init(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DH_meth_set_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_get_finish(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_meth_get_finish)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DH_meth_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_set_generate_params(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DH_meth_set_generate_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_set_compute_key(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DH_meth_set_compute_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_set_flags(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).DH_meth_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_get_generate_params(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_meth_get_generate_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_get_flags(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_meth_get_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_set_finish(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DH_meth_set_finish)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_get0_app_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).DH_meth_get0_app_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_set0_app_data(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).DH_meth_set0_app_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_get_init(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_meth_get_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_get_bn_mod_exp(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_meth_get_bn_mod_exp)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_new(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).DH_meth_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).DH_meth_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_set_bn_mod_exp(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DH_meth_set_bn_mod_exp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_set_generate_key(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).DH_meth_set_generate_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_meth_free(a0: *mut APTR) { ((*IAmiSSL).DH_meth_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_meth_get_generate_key(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_meth_get_generate_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_set_flags(a0: *mut APTR, a1: i32) { ((*IAmiSSL).DH_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_obj_by_subject(a0: *mut APTR, a1: APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get_obj_by_subject)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_object_free(a0: *mut APTR) { ((*IAmiSSL).X509_OBJECT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_object_get0_x509(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_OBJECT_get0_X509)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_untrusted(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_untrusted)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_cert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_verify(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_CTX_set_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_verify(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_verify)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_verify_cb(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_verify_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set0_verified_chain(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set0_verified_chain)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set0_untrusted(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set0_untrusted)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_hexchar2int(a0: u8) -> i32 { ((*IAmiSSL).OPENSSL_hexchar2int)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_sig_set0(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).DSA_SIG_set0)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dsa_bits(a0: *const APTR) -> i32 { ((*IAmiSSL).DSA_bits)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdsa_sig_set0(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).ECDSA_SIG_set0)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_encode_ctx_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_ENCODE_CTX_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_hmac(a0: *const APTR, a1: *mut u32) -> *const u8 { ((*IAmiSSL).EVP_PKEY_get0_hmac)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_hmac_ctx_get_md(a0: *const APTR) -> *const APTR { ((*IAmiSSL).HMAC_CTX_get_md)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get0_certs(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OCSP_resp_get0_certs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get0_id(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).OCSP_resp_get0_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pem_write_bio_private_key_traditional(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: i32, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_write_bio_PrivateKey_traditional)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ssl_session_get_protocol_version(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_get_protocol_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_is_dtls(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_is_dtls)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_dane_set_flags(a0: *mut APTR, a1: u32) -> u32 { ((*IAmiSSL).SSL_CTX_dane_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_dane_clear_flags(a0: *mut APTR, a1: u32) -> u32 { ((*IAmiSSL).SSL_CTX_dane_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_dane_set_flags(a0: *mut APTR, a1: u32) -> u32 { ((*IAmiSSL).SSL_dane_set_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_dane_clear_flags(a0: *mut APTR, a1: u32) -> u32 { ((*IAmiSSL).SSL_dane_clear_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_client_version(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_client_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get_pathlen(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_get_pathlen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_object_get_type(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_OBJECT_get_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_object_get0_x509_crl(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_OBJECT_get0_X509_CRL)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_lock(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_lock)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_unlock(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_unlock)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_get0_objects(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_get0_objects)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_get0_param(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_get0_param)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_verify(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_verify(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_verify)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_get_verify_cb(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_verify_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_get_issuer(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_get_issuer)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_get_issuer(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_get_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_check_issued(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_check_issued)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_check_issued(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_check_issued)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_check_revocation(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_check_revocation)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_check_revocation(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_check_revocation)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_get_crl(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_get_crl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_get_crl(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_get_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_check_crl(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_check_crl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_check_crl(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_check_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_cert_crl(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_cert_crl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_cert_crl(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_cert_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_check_policy(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_check_policy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_check_policy(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_check_policy)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_lookup_certs(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_lookup_certs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_lookup_certs(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_lookup_certs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_lookup_crls(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_lookup_crls)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_lookup_crls(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_lookup_crls)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_cleanup(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_set_cleanup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get_cleanup(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_get_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).X509_STORE_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).X509_STORE_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_get_issuer(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_get_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_check_issued(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_check_issued)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_check_revocation(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_check_revocation)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_get_crl(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_get_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_check_crl(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_check_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_cert_crl(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_cert_crl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_check_policy(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_check_policy)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_lookup_certs(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_lookup_certs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_lookup_crls(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_lookup_crls)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get_cleanup(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_STORE_CTX_get_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_error_depth(a0: *mut APTR, a1: i32) { ((*IAmiSSL).X509_STORE_CTX_set_error_depth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_current_cert(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set_current_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_name_constraints_check_cn(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).NAME_CONSTRAINTS_check_CN)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_set_proxy_flag(a0: *mut APTR) { ((*IAmiSSL).X509_set_proxy_flag)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_set_proxy_pathlen(a0: *mut APTR, a1: i32) { ((*IAmiSSL).X509_set_proxy_pathlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_get_proxy_pathlen(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_get_proxy_pathlen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asidentifier_choice_new() -> *mut APTR { ((*IAmiSSL).ASIdentifierChoice_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asidentifier_choice_free(a0: *mut APTR) { ((*IAmiSSL).ASIdentifierChoice_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asidentifier_choice(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASIdentifierChoice)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asidentifier_choice(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASIdentifierChoice)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asidentifier_choice_it() -> *const APTR { ((*IAmiSSL).ASIdentifierChoice_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asidentifiers_new() -> *mut APTR { ((*IAmiSSL).ASIdentifiers_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asidentifiers_free(a0: *mut APTR) { ((*IAmiSSL).ASIdentifiers_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asidentifiers(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASIdentifiers)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asidentifiers(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASIdentifiers)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asidentifiers_it() -> *const APTR { ((*IAmiSSL).ASIdentifiers_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asid_or_range_new() -> *mut APTR { ((*IAmiSSL).ASIdOrRange_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asid_or_range_free(a0: *mut APTR) { ((*IAmiSSL).ASIdOrRange_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asid_or_range(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASIdOrRange)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asid_or_range(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASIdOrRange)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asid_or_range_it() -> *const APTR { ((*IAmiSSL).ASIdOrRange_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asrange_new() -> *mut APTR { ((*IAmiSSL).ASRange_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asrange_free(a0: *mut APTR) { ((*IAmiSSL).ASRange_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_asrange(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ASRange)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_asrange(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ASRange)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asrange_it() -> *const APTR { ((*IAmiSSL).ASRange_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ipaddress_choice_new() -> *mut APTR { ((*IAmiSSL).IPAddressChoice_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ipaddress_choice_free(a0: *mut APTR) { ((*IAmiSSL).IPAddressChoice_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ipaddress_choice(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_IPAddressChoice)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ipaddress_choice(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_IPAddressChoice)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ipaddress_choice_it() -> *const APTR { ((*IAmiSSL).IPAddressChoice_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ipaddress_family_new() -> *mut APTR { ((*IAmiSSL).IPAddressFamily_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ipaddress_family_free(a0: *mut APTR) { ((*IAmiSSL).IPAddressFamily_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ipaddress_family(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_IPAddressFamily)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ipaddress_family(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_IPAddressFamily)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ipaddress_family_it() -> *const APTR { ((*IAmiSSL).IPAddressFamily_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ipaddress_or_range_new() -> *mut APTR { ((*IAmiSSL).IPAddressOrRange_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ipaddress_or_range_free(a0: *mut APTR) { ((*IAmiSSL).IPAddressOrRange_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ipaddress_or_range(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_IPAddressOrRange)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ipaddress_or_range(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_IPAddressOrRange)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ipaddress_or_range_it() -> *const APTR { ((*IAmiSSL).IPAddressOrRange_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_string_get0_data(a0: *const APTR) -> *const u8 { ((*IAmiSSL).ASN1_STRING_get0_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_not_before(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_get0_notBefore)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_not_after(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_get0_notAfter)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get0_last_update(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_CRL_get0_lastUpdate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get0_next_update(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_CRL_get0_nextUpdate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_uni2utf8(a0: *const u8, a1: i32) -> *mut APTR { ((*IAmiSSL).OPENSSL_uni2utf8)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_set_shared_ctlog_store(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).CT_POLICY_EVAL_CTX_set_shared_CTLOG_STORE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_set1_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CT_POLICY_EVAL_CTX_set1_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_set1_issuer(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CT_POLICY_EVAL_CTX_set1_issuer)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ecparameters_new() -> *mut APTR { ((*IAmiSSL).ECPARAMETERS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ecparameters_free(a0: *mut APTR) { ((*IAmiSSL).ECPARAMETERS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_get_new_index() -> i32 { ((*IAmiSSL).BIO_get_new_index)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkey_set1_encoded_public_key(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_set1_encoded_public_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get1_encoded_public_key(a0: *mut APTR, a1: *mut *mut u8) -> u32 { ((*IAmiSSL).EVP_PKEY_get1_encoded_public_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_respid_set_by_name(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_RESPID_set_by_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_respid_set_by_key(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_RESPID_set_by_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_respid_match(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_RESPID_match)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_add_friendlyname_utf8(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).PKCS12_add_friendlyname_utf8)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_key_gen_utf8(a0: *const APTR, a1: i32, a2: *mut u8, a3: i32, a4: i32, a5: i32, a6: i32, a7: *mut u8, a8: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_key_gen_utf8)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_openssl_utf82uni(a0: *const APTR, a1: i32, a2: *mut *mut u8, a3: *mut i32) -> *mut u8 { ((*IAmiSSL).OPENSSL_utf82uni)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_session_get0_cipher(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_SESSION_get0_cipher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_set1_id(a0: *mut APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_set1_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_session_get0_id_context(a0: *const APTR, a1: *mut APTR) -> *const u8 { ((*IAmiSSL).SSL_SESSION_get0_id_context)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_status_info_get0_text(a0: *const APTR) -> *const APTR { ((*IAmiSSL).TS_STATUS_INFO_get0_text)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_sig_getm(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).X509_SIG_getm)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_get0_serial_number(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_get0_serialNumber)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_get_time(a0: *const APTR) -> APTR { ((*IAmiSSL).CT_POLICY_EVAL_CTX_get_time)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_set_time(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).CT_POLICY_EVAL_CTX_set_time)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_check_params(a0: *const APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).DH_check_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_comp_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_COMP_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_comp_get_id(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_COMP_get_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get_time(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get_time)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_set_inh_flags(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_set_inh_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get_inh_flags(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get_inh_flags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_secure_clear_free(a0: *mut (), a1: u32, a2: *const APTR, a3: i32) { ((*IAmiSSL).CRYPTO_secure_clear_free)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_set1_engine(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_set1_engine)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_client_ciphers(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get_client_ciphers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_standard_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_standard_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_verified_chain(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get0_verified_chain)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_comp_set0_compression_methods(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_COMP_set0_compression_methods)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set1_cert_store(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_CTX_set1_cert_store)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dtls_get_data_mtu(a0: *const APTR) -> u32 { ((*IAmiSSL).DTLS_get_data_mtu)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_read_ex(a0: *mut APTR, a1: *mut (), a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).SSL_read_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_peek_ex(a0: *mut APTR, a1: *mut (), a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).SSL_peek_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_write_ex(a0: *mut APTR, a1: *const (), a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).SSL_write_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_keylog_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_keylog_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_keylog_callback(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_CTX_get_keylog_callback)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_peer_signature_type_nid(a0: *const APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).SSL_get_peer_signature_type_nid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_key_update(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_key_update)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_key_update_type(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_key_update_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_bytes_to_cipher_list(a0: *mut APTR, a1: *const u8, a2: u32, a3: i32, a4: *mut *mut APTR, a5: *mut *mut APTR) -> i32 { ((*IAmiSSL).SSL_bytes_to_cipher_list)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_get0_compression_methods(a0: *mut APTR, a1: *mut *mut APTR) -> u32 { ((*IAmiSSL).SSL_client_hello_get0_compression_methods)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_get0_ciphers(a0: *mut APTR, a1: *mut *mut APTR) -> u32 { ((*IAmiSSL).SSL_client_hello_get0_ciphers)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_get0_ext(a0: *mut APTR, a1: APTR, a2: *mut *mut APTR, a3: *mut u32) -> i32 { ((*IAmiSSL).SSL_client_hello_get0_ext)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_get0_session_id(a0: *mut APTR, a1: *mut *mut APTR) -> u32 { ((*IAmiSSL).SSL_client_hello_get0_session_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_get0_random(a0: *mut APTR, a1: *mut *mut APTR) -> u32 { ((*IAmiSSL).SSL_client_hello_get0_random)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_client_hello_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_CTX_set_client_hello_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_get0_legacy_version(a0: *mut APTR) -> APTR { ((*IAmiSSL).SSL_client_hello_get0_legacy_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_isv2(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_client_hello_isv2)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_max_early_data(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_set_max_early_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_max_early_data(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_max_early_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_max_early_data(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_get_max_early_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_max_early_data(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_CTX_get_max_early_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_write_early_data(a0: *mut APTR, a1: *const (), a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).SSL_write_early_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_read_early_data(a0: *mut APTR, a1: *mut (), a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).SSL_read_early_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_get_early_data_status(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_early_data_status)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_get_max_early_data(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_SESSION_get_max_early_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_add1_to_ca_list(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_add1_to_CA_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set0_ca_list(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_set0_CA_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set0_ca_list(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).SSL_CTX_set0_CA_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get0_ca_list(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get0_CA_list)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_peer_ca_list(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get0_peer_CA_list)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_add1_to_ca_list(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_add1_to_CA_list)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get0_ca_list(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CTX_get0_CA_list)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_add_custom_ext(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR, a4: APTR, a5: *mut (), a6: APTR, a7: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_add_custom_ext)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ssl_session_is_resumable(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_is_resumable)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_record_padding_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_record_padding_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_record_padding_callback(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_set_record_padding_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_block_padding(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).SSL_CTX_set_block_padding)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_record_padding_callback_arg(a0: *const APTR) -> *mut () { ((*IAmiSSL).SSL_CTX_get_record_padding_callback_arg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_record_padding_callback_arg(a0: *const APTR) -> *mut () { ((*IAmiSSL).SSL_get_record_padding_callback_arg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_block_padding(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).SSL_set_block_padding)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_record_padding_callback_arg(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).SSL_set_record_padding_callback_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_record_padding_callback_arg(a0: *mut APTR, a1: *mut ()) { ((*IAmiSSL).SSL_CTX_set_record_padding_callback_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_serverinfo_ex(a0: *mut APTR, a1: APTR, a2: *const u8, a3: u32) -> i32 { ((*IAmiSSL).SSL_CTX_use_serverinfo_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_get1_extensions_present(a0: *mut APTR, a1: *mut *mut i32, a2: *mut u32) -> i32 { ((*IAmiSSL).SSL_client_hello_get1_extensions_present)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_psk_find_session_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_psk_find_session_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_psk_use_session_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_set_psk_use_session_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_psk_use_session_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_psk_use_session_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_psk_find_session_callback(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_psk_find_session_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_handshake_digest(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_CIPHER_get_handshake_digest)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_set1_master_key(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SSL_SESSION_set1_master_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_session_set_cipher(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_set_cipher)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_set_protocol_version(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_SESSION_set_protocol_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_cipher_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OPENSSL_cipher_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_alloc_buffers(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_alloc_buffers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_free_buffers(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_free_buffers)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_SESSION_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_pending_cipher(a0: *const APTR) -> *const APTR { ((*IAmiSSL).SSL_get_pending_cipher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_cipher_get_protocol_id(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_CIPHER_get_protocol_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_set_max_early_data(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_set_max_early_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_set1_alpn_selected(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SSL_SESSION_set1_alpn_selected)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_session_set1_hostname(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_SESSION_set1_hostname)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_get0_alpn_selected(a0: *const APTR, a1: *mut *mut APTR, a2: *mut u32) { ((*IAmiSSL).SSL_SESSION_get0_alpn_selected)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_dtls_set_timer_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).DTLS_set_timer_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_tlsext_max_fragment_length(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_tlsext_max_fragment_length)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_tlsext_max_fragment_length(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_set_tlsext_max_fragment_length)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_session_get_max_fragment_length(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_SESSION_get_max_fragment_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_stateless(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_stateless)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_verify_client_post_handshake(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_verify_client_post_handshake)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_post_handshake_auth(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_set_post_handshake_auth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_export_keying_material_early(a0: *mut APTR, a1: *mut u8, a2: u32, a3: *const APTR, a4: u32, a5: *const u8, a6: u32) -> i32 { ((*IAmiSSL).SSL_export_keying_material_early)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ssl_ctx_use_cert_and_key(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: i32) -> i32 { ((*IAmiSSL).SSL_CTX_use_cert_and_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_use_cert_and_key(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: i32) -> i32 { ((*IAmiSSL).SSL_use_cert_and_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_session_get0_ticket_appdata(a0: *mut APTR, a1: *mut *mut (), a2: *mut u32) -> i32 { ((*IAmiSSL).SSL_SESSION_get0_ticket_appdata)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_session_set1_ticket_appdata(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).SSL_SESSION_set1_ticket_appdata)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_session_ticket_cb(a0: *mut APTR, a1: APTR, a2: APTR, a3: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_set_session_ticket_cb)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_stateless_cookie_generate_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_stateless_cookie_generate_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_stateless_cookie_verify_cb(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_set_stateless_cookie_verify_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_ciphersuites(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_ciphersuites)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_ciphersuites(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_set_ciphersuites)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_num_tickets(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).SSL_set_num_tickets)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_num_tickets(a0: *const APTR) -> u32 { ((*IAmiSSL).SSL_CTX_get_num_tickets)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_num_tickets(a0: *const APTR) -> u32 { ((*IAmiSSL).SSL_get_num_tickets)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_num_tickets(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).SSL_CTX_set_num_tickets)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_allow_early_data_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_CTX_set_allow_early_data_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_allow_early_data_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_set_allow_early_data_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_recv_max_early_data(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_set_recv_max_early_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_recv_max_early_data(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_get_recv_max_early_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_recv_max_early_data(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_CTX_get_recv_max_early_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_recv_max_early_data(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_recv_max_early_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_post_handshake_auth(a0: *mut APTR, a1: i32) { ((*IAmiSSL).SSL_CTX_set_post_handshake_auth)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_signature_type_nid(a0: *const APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).SSL_get_signature_type_nid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ipaddress_range_free(a0: *mut APTR) { ((*IAmiSSL).IPAddressRange_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_tls_feature_new() -> *mut APTR { ((*IAmiSSL).TLS_FEATURE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_linebuffer() -> *const APTR { ((*IAmiSSL).BIO_f_linebuffer)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ipaddress_range_new() -> *mut APTR { ((*IAmiSSL).IPAddressRange_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ipaddress_range_it() -> *const APTR { ((*IAmiSSL).IPAddressRange_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ipaddress_range(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_IPAddressRange)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_tls_feature_free(a0: *mut APTR) { ((*IAmiSSL).TLS_FEATURE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_rsa_oaep_params(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_RSA_OAEP_PARAMS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ipaddress_range(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_IPAddressRange)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_object_new() -> *mut APTR { ((*IAmiSSL).X509_OBJECT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pkcs12_get_attr(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_get_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ecpkparameters_free(a0: *mut APTR) { ((*IAmiSSL).ECPKPARAMETERS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecpkparameters_new() -> *mut APTR { ((*IAmiSSL).ECPKPARAMETERS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_asn1_item_lookup(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ASN1_ITEM_lookup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_item_get(a0: u32) -> *const APTR { ((*IAmiSSL).ASN1_ITEM_get)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_read_ex(a0: *mut APTR, a1: *mut (), a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).BIO_read_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_set_callback_ex(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).BIO_set_callback_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_get_callback_ex(a0: *const APTR) -> APTR { ((*IAmiSSL).BIO_get_callback_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_set_read_ex(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_read_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_get_read_ex(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_read_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_write_ex(a0: *mut APTR, a1: *const (), a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).BIO_write_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bio_meth_get_write_ex(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_write_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_set_write_ex(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_write_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_md(a0: *mut APTR, a1: i32, a2: i32, a3: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_md)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rsa_pkey_ctx_ctrl(a0: *mut APTR, a1: i32, a2: i32, a3: i32, a4: *mut ()) -> i32 { ((*IAmiSSL).RSA_pkey_ctx_ctrl)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ui_method_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).UI_method_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ui_method_get_ex_data(a0: *const APTR, a1: i32) -> *const () { ((*IAmiSSL).UI_method_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_util_wrap_read_pem_callback(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).UI_UTIL_wrap_read_pem_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_poly1305(a0: *const APTR, a1: *mut u32) -> *const u8 { ((*IAmiSSL).EVP_PKEY_get0_poly1305)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_siphash(a0: *const APTR, a1: *mut u32) -> *const u8 { ((*IAmiSSL).EVP_PKEY_get0_siphash)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_aria_256_ofb() -> *const APTR { ((*IAmiSSL).EVP_aria_256_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_256_cfb128() -> *const APTR { ((*IAmiSSL).EVP_aria_256_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_cfb1() -> *const APTR { ((*IAmiSSL).EVP_aria_128_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_ecb() -> *const APTR { ((*IAmiSSL).EVP_aria_128_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_cfb128() -> *const APTR { ((*IAmiSSL).EVP_aria_128_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_ecb() -> *const APTR { ((*IAmiSSL).EVP_aria_192_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_cbc() -> *const APTR { ((*IAmiSSL).EVP_aria_128_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_ofb() -> *const APTR { ((*IAmiSSL).EVP_aria_192_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_cbc() -> *const APTR { ((*IAmiSSL).EVP_aria_192_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_cfb1() -> *const APTR { ((*IAmiSSL).EVP_aria_192_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_cfb8() -> *const APTR { ((*IAmiSSL).EVP_aria_128_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_256_cfb1() -> *const APTR { ((*IAmiSSL).EVP_aria_256_cfb1)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_cfb8() -> *const APTR { ((*IAmiSSL).EVP_aria_192_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_256_cfb8() -> *const APTR { ((*IAmiSSL).EVP_aria_256_cfb8)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_256_cbc() -> *const APTR { ((*IAmiSSL).EVP_aria_256_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_ofb() -> *const APTR { ((*IAmiSSL).EVP_aria_128_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_cfb128() -> *const APTR { ((*IAmiSSL).EVP_aria_192_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_256_ecb() -> *const APTR { ((*IAmiSSL).EVP_aria_256_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_256_ctr() -> *const APTR { ((*IAmiSSL).EVP_aria_256_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_ctr() -> *const APTR { ((*IAmiSSL).EVP_aria_128_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_ctr() -> *const APTR { ((*IAmiSSL).EVP_aria_192_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ui_null() -> *const APTR { ((*IAmiSSL).UI_null)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ec_key_get0_engine(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_KEY_get0_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_int32_it() -> *const APTR { ((*IAmiSSL).INT32_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_uint64_it() -> *const APTR { ((*IAmiSSL).UINT64_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_zint32_it() -> *const APTR { ((*IAmiSSL).ZINT32_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_zuint64_it() -> *const APTR { ((*IAmiSSL).ZUINT64_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_int64_it() -> *const APTR { ((*IAmiSSL).INT64_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_zuint32_it() -> *const APTR { ((*IAmiSSL).ZUINT32_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_uint32_it() -> *const APTR { ((*IAmiSSL).UINT32_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_zint64_it() -> *const APTR { ((*IAmiSSL).ZINT64_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_lookup_ex(a0: *const APTR, a1: *const APTR, a2: i32, a3: i32, a4: i32, a5: i32, a6: *mut *mut APTR) -> i32 { ((*IAmiSSL).BIO_lookup_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_x509_crl_print_ex(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).X509_CRL_print_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_sig_info_get(a0: *const APTR, a1: *mut i32, a2: *mut i32, a3: *mut i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).X509_SIG_INFO_get)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_get_signature_info(a0: *mut APTR, a1: *mut i32, a2: *mut i32, a3: *mut i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).X509_get_signature_info)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_sig_info_set(a0: *mut APTR, a1: i32, a2: i32, a3: i32, a4: APTR) { ((*IAmiSSL).X509_SIG_INFO_set)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ess_cert_id_v2_free(a0: *mut APTR) { ((*IAmiSSL).ESS_CERT_ID_V2_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ess_signing_cert_v2_new() -> *mut APTR { ((*IAmiSSL).ESS_SIGNING_CERT_V2_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ess_signing_cert_v2(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ESS_SIGNING_CERT_V2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ess_cert_id_v2(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ESS_CERT_ID_V2)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ess_cert_id_v2_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ESS_CERT_ID_V2_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_set_ess_cert_id_digest(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).TS_RESP_CTX_set_ess_cert_id_digest)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ess_cert_id_v2(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ESS_CERT_ID_V2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ess_signing_cert_v2(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ESS_SIGNING_CERT_V2)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_conf_set_ess_cert_id_digest(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).TS_CONF_set_ess_cert_id_digest)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ess_signing_cert_v2_free(a0: *mut APTR) { ((*IAmiSSL).ESS_SIGNING_CERT_V2_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ess_signing_cert_v2_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ESS_SIGNING_CERT_V2_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ess_cert_id_v2_new() -> *mut APTR { ((*IAmiSSL).ESS_CERT_ID_V2_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pem_read_bio_ex(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut u8, a4: *mut i32, a5: APTR) -> i32 { ((*IAmiSSL).PEM_read_bio_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pem_bytes_read_bio_secmem(a0: *mut *mut u8, a1: *mut i32, a2: *mut *mut APTR, a3: *const APTR, a4: *mut APTR, a5: *mut APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).PEM_bytes_read_bio_secmem)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_digest_sign(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).EVP_DigestSign)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_digest_verify(a0: *mut APTR, a1: *const u8, a2: u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).EVP_DigestVerify)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ui_method_get_data_duplicator(a0: *const APTR) -> *mut () { ((*IAmiSSL).UI_method_get_data_duplicator)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ui_method_set_data_duplicator(a0: *mut APTR, a1: APTR, a2: APTR) -> i32 { ((*IAmiSSL).UI_method_set_data_duplicator)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ui_dup_user_data(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).UI_dup_user_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_method_get_data_destructor(a0: *const APTR) { ((*IAmiSSL).UI_method_get_data_destructor)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_strings_const(a0: *const APTR) -> i32 { ((*IAmiSSL).ERR_load_strings_const)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_time_to_tm(a0: *const APTR, a1: *mut tm) -> i32 { ((*IAmiSSL).ASN1_TIME_to_tm)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_time_set_string_x509(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_TIME_set_string_X509)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get1_id(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).OCSP_resp_get1_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_register_loader(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_register_loader)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_error(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_error)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get0_pkey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get0_PKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get_type(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_INFO_get_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_load_ossl_store_strings() -> i32 { ((*IAmiSSL).ERR_load_OSSL_STORE_strings)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_STORE_LOADER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get1_pkey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get1_PKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_STORE_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_eof(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_eof)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_new(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_LOADER_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get0_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get0_CERT)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_close(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_close)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_info_new_params(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_new_PARAMS)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_new_pkey(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_new_PKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get1_params(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get1_PARAMS)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get1_crl(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get1_CRL)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_error(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_error)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get1_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get1_CERT)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get0_params(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get0_PARAMS)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_type_string(a0: i32) -> *const APTR { ((*IAmiSSL).OSSL_STORE_INFO_type_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get1_name(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get1_NAME)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_load(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_load)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_get0_scheme(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_LOADER_get0_scheme)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_open(a0: *const APTR, a1: *const APTR, a2: *mut (), a3: APTR, a4: *mut ()) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_open)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_store_close(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_close)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_new_cert(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_new_CERT)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get0_crl(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get0_CRL)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_load(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_load)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_INFO_get0_NAME)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_unregister_loader(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_unregister_loader)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_new_crl(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_new_CRL)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_new_name(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_new_NAME)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_eof(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_eof)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_open(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_open)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_ctrl(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_ctrl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_vctrl(a0: *mut APTR, a1: i32, a2: *mut i32) -> i32 { ((*IAmiSSL).OSSL_STORE_vctrl)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get0_name_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_INFO_get0_NAME_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_set0_name_description(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_INFO_set0_NAME_description)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get1_name_description(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get1_NAME_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_do_all_loaders(a0: APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).OSSL_STORE_do_all_loaders)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_get0_engine(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_LOADER_get0_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_sha3_224() -> *const APTR { ((*IAmiSSL).EVP_sha3_224)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha3_256() -> *const APTR { ((*IAmiSSL).EVP_sha3_256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha3_384() -> *const APTR { ((*IAmiSSL).EVP_sha3_384)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha3_512() -> *const APTR { ((*IAmiSSL).EVP_sha3_512)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_shake128() -> *const APTR { ((*IAmiSSL).EVP_shake128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_shake256() -> *const APTR { ((*IAmiSSL).EVP_shake256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_scrypt_params_new() -> *mut APTR { ((*IAmiSSL).SCRYPT_PARAMS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_scrypt_params_free(a0: *mut APTR) { ((*IAmiSSL).SCRYPT_PARAMS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_scrypt_params(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_SCRYPT_PARAMS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_scrypt_params(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_SCRYPT_PARAMS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_scrypt_params_it() -> *const APTR { ((*IAmiSSL).SCRYPT_PARAMS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get0(a0: u32) -> *const APTR { ((*IAmiSSL).EVP_PKEY_meth_get0)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_count() -> u32 { ((*IAmiSSL).EVP_PKEY_meth_get_count)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rand_priv_bytes(a0: *mut u8, a1: i32) -> i32 { ((*IAmiSSL).RAND_priv_bytes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_priv_rand(a0: *mut APTR, a1: i32, a2: i32, a3: i32) -> i32 { ((*IAmiSSL).BN_priv_rand)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_priv_rand_range(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BN_priv_rand_range)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_time_normalize(a0: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_TIME_normalize)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_time_cmp_time_t(a0: *const APTR, a1: APTR) -> i32 { ((*IAmiSSL).ASN1_TIME_cmp_time_t)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_time_compare(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).ASN1_TIME_compare)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_ctrl_uint64(a0: *mut APTR, a1: i32, a2: i32, a3: i32, a4: APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_ctrl_uint64)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_digest_final_xof(a0: *mut APTR, a1: *mut u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_DigestFinalXOF)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_err_clear_last_mark() -> i32 { ((*IAmiSSL).ERR_clear_last_mark)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_ccm() -> *const APTR { ((*IAmiSSL).EVP_aria_192_ccm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_256_gcm() -> *const APTR { ((*IAmiSSL).EVP_aria_256_gcm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_256_ccm() -> *const APTR { ((*IAmiSSL).EVP_aria_256_ccm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_gcm() -> *const APTR { ((*IAmiSSL).EVP_aria_128_gcm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_128_ccm() -> *const APTR { ((*IAmiSSL).EVP_aria_128_ccm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_aria_192_gcm() -> *const APTR { ((*IAmiSSL).EVP_aria_192_gcm)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ui_get_result_length(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).UI_get_result_length)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ui_set_result_ex(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).UI_set_result_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ui_get_result_string_length(a0: *mut APTR) -> i32 { ((*IAmiSSL).UI_get_result_string_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_check(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_check)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_check(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_check(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_remove(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_meth_remove)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_reserve(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OPENSSL_sk_reserve)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_new_by_nid(a0: i32) -> *mut APTR { ((*IAmiSSL).DH_new_by_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_get_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_get_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_new_reserve(a0: APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OPENSSL_sk_new_reserve)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_check(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_siginf(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_siginf)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_sm4_ctr() -> *const APTR { ((*IAmiSSL).EVP_sm4_ctr)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sm4_cbc() -> *const APTR { ((*IAmiSSL).EVP_sm4_cbc)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sm4_ofb() -> *const APTR { ((*IAmiSSL).EVP_sm4_ofb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sm4_ecb() -> *const APTR { ((*IAmiSSL).EVP_sm4_ecb)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sm4_cfb128() -> *const APTR { ((*IAmiSSL).EVP_sm4_cfb128)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sm3() -> *const APTR { ((*IAmiSSL).EVP_sm3)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_rsa_get0_multi_prime_factors(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).RSA_get0_multi_prime_factors)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_public_check(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_public_check)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_param_check(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_param_check)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_public_check(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_public_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_param_check(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_param_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_public_check(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_public_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_param_check(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_param_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_public_check(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_public_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_param_check(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_param_check)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_check_ex(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_check_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_check_pub_key_ex(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).DH_check_pub_key_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_dh_check_params_ex(a0: *const APTR) -> i32 { ((*IAmiSSL).DH_check_params_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_generate_multi_prime_key(a0: *mut APTR, a1: i32, a2: i32, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).RSA_generate_multi_prime_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_get_multi_prime_extra_count(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_get_multi_prime_extra_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get0_signer(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).OCSP_resp_get0_signer)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rsa_get0_multi_prime_crt_params(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).RSA_get0_multi_prime_crt_params)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_rsa_set0_multi_prime_params(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: i32) -> i32 { ((*IAmiSSL).RSA_set0_multi_prime_params)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rsa_get_version(a0: *mut APTR) -> i32 { ((*IAmiSSL).RSA_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_get_multi_prime_keygen(a0: *const APTR) -> i32 { ((*IAmiSSL).RSA_meth_get_multi_prime_keygen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_meth_set_multi_prime_keygen(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).RSA_meth_set_multi_prime_keygen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_profession_info_get0_add_profession_info(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PROFESSION_INFO_get0_addProfessionInfo)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_admission_syntax_free(a0: *mut APTR) { ((*IAmiSSL).ADMISSION_SYNTAX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_admission_syntax(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ADMISSION_SYNTAX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_naming_authority_set0_authority_id(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).NAMING_AUTHORITY_set0_authorityId)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_naming_authority_set0_authority_url(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).NAMING_AUTHORITY_set0_authorityURL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_profession_info(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PROFESSION_INFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_naming_authority_it() -> *const APTR { ((*IAmiSSL).NAMING_AUTHORITY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_admission_syntax_get0_contents_of_admissions(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ADMISSION_SYNTAX_get0_contentsOfAdmissions)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_profession_info_set0_profession_items(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).PROFESSION_INFO_set0_professionItems)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_naming_authority_new() -> *mut APTR { ((*IAmiSSL).NAMING_AUTHORITY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_naming_authority_get0_authority_url(a0: *const APTR) -> *const APTR { ((*IAmiSSL).NAMING_AUTHORITY_get0_authorityURL)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_admission_syntax_get0_admission_authority(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ADMISSION_SYNTAX_get0_admissionAuthority)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_profession_info_new() -> *mut APTR { ((*IAmiSSL).PROFESSION_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_admissions_new() -> *mut APTR { ((*IAmiSSL).ADMISSIONS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_admission_syntax_set0_admission_authority(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).ADMISSION_SYNTAX_set0_admissionAuthority)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_profession_info_get0_profession_oids(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PROFESSION_INFO_get0_professionOIDs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_profession_info_it() -> *const APTR { ((*IAmiSSL).PROFESSION_INFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_i2d_profession_info(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PROFESSION_INFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_admissions_set0_profession_infos(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).ADMISSIONS_set0_professionInfos)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_profession_info_get0_naming_authority(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PROFESSION_INFO_get0_namingAuthority)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_profession_info_free(a0: *mut APTR) { ((*IAmiSSL).PROFESSION_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_profession_info_set0_add_profession_info(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).PROFESSION_INFO_set0_addProfessionInfo)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_profession_info_set0_registration_number(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).PROFESSION_INFO_set0_registrationNumber)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_admission_syntax_set0_contents_of_admissions(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).ADMISSION_SYNTAX_set0_contentsOfAdmissions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_naming_authority_get0_authority_id(a0: *const APTR) -> *const APTR { ((*IAmiSSL).NAMING_AUTHORITY_get0_authorityId)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_admission_syntax_it() -> *const APTR { ((*IAmiSSL).ADMISSION_SYNTAX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_i2d_admission_syntax(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ADMISSION_SYNTAX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_naming_authority_get0_authority_text(a0: *const APTR) -> *const APTR { ((*IAmiSSL).NAMING_AUTHORITY_get0_authorityText)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_profession_info_set0_naming_authority(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).PROFESSION_INFO_set0_namingAuthority)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_naming_authority(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_NAMING_AUTHORITY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_naming_authority_free(a0: *mut APTR) { ((*IAmiSSL).NAMING_AUTHORITY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_admissions_set0_admission_authority(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).ADMISSIONS_set0_admissionAuthority)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_admissions_free(a0: *mut APTR) { ((*IAmiSSL).ADMISSIONS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_profession_info_get0_registration_number(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PROFESSION_INFO_get0_registrationNumber)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_admissions(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ADMISSIONS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_admissions(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ADMISSIONS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_profession_info_get0_profession_items(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PROFESSION_INFO_get0_professionItems)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_admissions_get0_admission_authority(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ADMISSIONS_get0_admissionAuthority)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_profession_info_set0_profession_oids(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).PROFESSION_INFO_set0_professionOIDs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_naming_authority(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_NAMING_AUTHORITY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_admissions_it() -> *const APTR { ((*IAmiSSL).ADMISSIONS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_admissions_get0_naming_authority(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ADMISSIONS_get0_namingAuthority)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_naming_authority_set0_authority_text(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).NAMING_AUTHORITY_set0_authorityText)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_admissions_set0_naming_authority(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).ADMISSIONS_set0_namingAuthority)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_admissions_get0_profession_infos(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ADMISSIONS_get0_professionInfos)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_admission_syntax_new() -> *mut APTR { ((*IAmiSSL).ADMISSION_SYNTAX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha512_256() -> *const APTR { ((*IAmiSSL).EVP_sha512_256)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_sha512_224() -> *const APTR { ((*IAmiSSL).EVP_sha512_224)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ocsp_basic_sign_ctx(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: u32) -> i32 { ((*IAmiSSL).OCSP_basic_sign_ctx)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_store_search_by_alias(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_SEARCH_by_alias)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_bind(a0: i32, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).BIO_bind)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_expect(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_expect)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_expect(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_STORE_expect)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_search_by_key_fingerprint(a0: *const APTR, a1: *const u8, a2: u32) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_SEARCH_by_key_fingerprint)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_search_get0_serial(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_SEARCH_get0_serial)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_search_by_name(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_SEARCH_by_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_supports_search(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_STORE_supports_search)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_find(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_find)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_search_get_type(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_SEARCH_get_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_search_get0_bytes(a0: *const APTR, a1: *mut u32) -> *const u8 { ((*IAmiSSL).OSSL_STORE_SEARCH_get0_bytes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_search_get0_string(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_SEARCH_get0_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_search_by_issuer_serial(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_SEARCH_by_issuer_serial)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_search_get0_name(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_SEARCH_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_authority_key_id(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).X509_get0_authority_key_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_find(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_find)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_search_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_STORE_SEARCH_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_search_get0_digest(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_SEARCH_get0_digest)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_new_raw_private_key(a0: i32, a1: *mut APTR, a2: *const u8, a3: u32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_new_raw_private_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_new_raw_public_key(a0: i32, a1: *mut APTR, a2: *const u8, a3: u32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_new_raw_public_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_new_cmac_key(a0: *mut APTR, a1: *const u8, a2: u32, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_new_CMAC_key)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_set_priv_key(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_set_priv_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_set_pub_key(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_set_pub_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get_hostflags(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get_hostflags)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_get0_p(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DH_get0_p)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_get0_q(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DH_get0_q)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_get0_g(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DH_get0_g)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_get0_priv_key(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DH_get0_priv_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dh_get0_pub_key(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DH_get0_pub_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_get0_priv_key(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DSA_get0_priv_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_get0_pub_key(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DSA_get0_pub_key)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_get0_q(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DSA_get0_q)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_get0_p(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DSA_get0_p)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dsa_get0_g(a0: *const APTR) -> *const APTR { ((*IAmiSSL).DSA_get0_g)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_dmp1(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_dmp1)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_d(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_d)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_n(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_n)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_dmq1(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_dmq1)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_e(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_e)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_q(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_q)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_p(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_p)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_get0_iqmp(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_iqmp)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdsa_sig_get0_r(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ECDSA_SIG_get0_r)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ecdsa_sig_get0_s(a0: *const APTR) -> *const APTR { ((*IAmiSSL).ECDSA_SIG_get0_s)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_get_by_fingerprint(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_LOOKUP_meth_get_get_by_fingerprint)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_new(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_LOOKUP_meth_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_init(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_get_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_get_by_alias(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_LOOKUP_meth_get_get_by_alias)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_new_item(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_new_item)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_shutdown(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_shutdown)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_new_item(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_get_new_item)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_ctrl(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_ctrl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_get_by_issuer_serial(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_get_by_issuer_serial)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_get_store(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_LOOKUP_get_store)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_ctrl(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_LOOKUP_meth_get_ctrl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_get_by_alias(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_get_by_alias)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_get_by_subject(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_LOOKUP_meth_get_get_by_subject)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_free(a0: *const APTR) { ((*IAmiSSL).X509_LOOKUP_meth_get_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_get_by_subject(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_get_by_subject)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_free(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_free)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_shutdown(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_get_shutdown)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_set_method_data(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).X509_LOOKUP_set_method_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_get_method_data(a0: *const APTR) -> *mut () { ((*IAmiSSL).X509_LOOKUP_get_method_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_get_by_fingerprint(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_get_by_fingerprint)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_free(a0: *mut APTR) { ((*IAmiSSL).X509_LOOKUP_meth_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_object_set1_x509(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_OBJECT_set1_X509)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_get_get_by_issuer_serial(a0: *const APTR) -> APTR { ((*IAmiSSL).X509_LOOKUP_meth_get_get_by_issuer_serial)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_meth_set_init(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_meth_set_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_object_set1_x509_crl(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_OBJECT_set1_X509_CRL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_raw_public_key(a0: *const APTR, a1: *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_raw_public_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_raw_private_key(a0: *const APTR, a1: *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_raw_private_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_get_priv_key(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_get_priv_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_asn1_set_get_pub_key(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_asn1_set_get_pub_key)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rand_keep_random_devices_open(a0: i32) { ((*IAmiSSL).RAND_keep_random_devices_open)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_point_set_compressed_coordinates(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: i32, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_set_compressed_coordinates)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_set_affine_coordinates(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_set_affine_coordinates)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_point_get_affine_coordinates(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_POINT_get_affine_coordinates)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_group_set_curve(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_set_curve)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ec_group_get_curve(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_curve)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get0_tbs_sigalg(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OCSP_resp_get0_tbs_sigalg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ocsp_resp_get0_respdata(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OCSP_resp_get0_respdata)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_set_pkey_ctx(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).EVP_MD_CTX_set_pkey_ctx)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_digest_custom(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_digest_custom)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_digest_custom(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_digest_custom)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_dir_read(a0: *mut *mut APTR, a1: *const APTR) -> *const APTR { ((*IAmiSSL).OPENSSL_DIR_read)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_dir_end(a0: *mut *mut APTR) -> i32 { ((*IAmiSSL).OPENSSL_DIR_end)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_engine(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_get0_engine)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_authority_serial(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).X509_get0_authority_serial)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_get0_authority_issuer(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).X509_get0_authority_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_digestsign(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_digestsign)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_set_digestverify(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_set_digestverify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_digestverify(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_digestverify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_meth_get_digestsign(a0: *const APTR, a1: APTR) { ((*IAmiSSL).EVP_PKEY_meth_get_digestsign)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rsa_get0_pss_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).RSA_get0_pss_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_algor_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_ALGOR_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_set0_signature(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_REQ_set0_signature)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_set1_signature_algo(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_REQ_set1_signature_algo)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_decoded_from_explicit_params(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_KEY_decoded_from_explicit_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_x931_derive_ex_amiga_1(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *const APTR, a6: *mut ()) -> i32 { ((*IAmiSSL).RSA_X931_derive_ex_amiga_1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_rsa_x931_derive_ex_amiga_2(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: *const APTR, a6: *mut APTR) -> *mut () { ((*IAmiSSL).RSA_X931_derive_ex_amiga_2)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_async_callback(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_async_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_async_callback_arg(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).SSL_CTX_set_async_callback_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_async_callback(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_set_async_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_async_callback_arg(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).SSL_set_async_callback_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_async_status(a0: *mut APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).SSL_get_async_status)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_sendfile(a0: *mut APTR, a1: i32, a2: APTR, a3: u32, a4: i32) -> i32 { ((*IAmiSSL).SSL_sendfile)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_default_cipher_list() -> *const APTR { ((*IAmiSSL).OSSL_default_cipher_list)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_default_ciphersuites() -> *const APTR { ((*IAmiSSL).OSSL_default_ciphersuites)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_add_store_cert_subjects_to_stack(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_add_store_cert_subjects_to_stack)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_default_verify_store(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_default_verify_store)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_ctx_load_verify_file(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_load_verify_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_load_verify_dir(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_load_verify_dir)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_load_verify_store(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_CTX_load_verify_store)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_tlsext_ticket_key_evp_cb(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_tlsext_ticket_key_evp_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_new_ex(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_CTX_new_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_new_session_ticket(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_new_session_ticket)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_peer_certificate(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get0_peer_certificate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_load_client_ca_file_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_load_client_CA_file_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set0_tmp_dh_pkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_set0_tmp_dh_pkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set0_tmp_dh_pkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set0_tmp_dh_pkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_group_to_name(a0: *mut APTR, a1: i32) -> *const APTR { ((*IAmiSSL).SSL_group_to_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_crl_load_http(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).X509_CRL_load_http)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_http_parse_url(a0: *const APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR, a5: *mut i32, a6: *mut *mut APTR, a7: *mut *mut APTR, a8: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_HTTP_parse_url)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_exchange(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_exchange)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_new(a0: *mut APTR, a1: *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_new)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_load_http(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).X509_load_http)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_name_hash_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *mut i32) -> u32 { ((*IAmiSSL).X509_NAME_hash_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_set_request_line(a0: *mut APTR, a1: i32, a2: *const APTR, a3: *const APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_set_request_line)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_set1_req(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_set1_req)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_new(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_MAC_CTX_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_free(a0: *mut APTR) { ((*IAmiSSL).EVP_MAC_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_MAC_CTX_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_get0_mac(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_MAC_CTX_get0_mac)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_get_mac_size(a0: *mut APTR) -> u32 { ((*IAmiSSL).EVP_MAC_CTX_get_mac_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_q_mac(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: *const (), a6: u32, a7: *const u8, a8: u32, a9: *mut u8, a10: u32, a11: *mut u32) -> *mut u8 { ((*IAmiSSL).EVP_Q_mac)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11) }
#[inline]
pub unsafe fn amissl_evp_mac_init(a0: *mut APTR, a1: *const u8, a2: u32, a3: *const APTR) -> i32 { ((*IAmiSSL).EVP_MAC_init)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_mac_update(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_MAC_update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_mac_final(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: u32) -> i32 { ((*IAmiSSL).EVP_MAC_final)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_mac_final_xof(a0: *mut APTR, a1: *mut u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_MAC_finalXOF)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_ec_curve_nid2name(a0: i32) -> *const APTR { ((*IAmiSSL).OSSL_EC_curve_nid2name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_digestsign_supports_digest(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_digestsign_supports_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_srp_vbase_add0_user(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SRP_VBASE_add0_user)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_srp_user_pwd_new() -> *mut APTR { ((*IAmiSSL).SRP_user_pwd_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_srp_user_pwd_set_g_n(a0: *mut APTR, a1: *const APTR, a2: *const APTR) { ((*IAmiSSL).SRP_user_pwd_set_gN)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_srp_user_pwd_set1_ids(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).SRP_user_pwd_set1_ids)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_srp_user_pwd_set0_sv(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).SRP_user_pwd_set0_sv)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_openssl_version_major() -> APTR { ((*IAmiSSL).OPENSSL_version_major)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_version_minor() -> APTR { ((*IAmiSSL).OPENSSL_version_minor)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_version_patch() -> APTR { ((*IAmiSSL).OPENSSL_version_patch)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_version_pre_release() -> *const APTR { ((*IAmiSSL).OPENSSL_version_pre_release)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_openssl_version_build_metadata() -> *const APTR { ((*IAmiSSL).OPENSSL_version_build_metadata)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_get_callback(a0: *mut APTR, a1: *mut APTR, a2: *mut *mut ()) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_get_callback)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_set_callback(a0: *mut APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_set_callback)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_set_status(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_set_status)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_async_wait_ctx_get_status(a0: *mut APTR) -> i32 { ((*IAmiSSL).ASYNC_WAIT_CTX_get_status)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_free(a0: *mut APTR) { ((*IAmiSSL).EVP_KDF_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_reset(a0: *mut APTR) { ((*IAmiSSL).EVP_KDF_CTX_reset)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_get_kdf_size(a0: *mut APTR) -> u32 { ((*IAmiSSL).EVP_KDF_CTX_get_kdf_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_derive(a0: *mut APTR, a1: *mut u8, a2: u32, a3: *const APTR) -> i32 { ((*IAmiSSL).EVP_KDF_derive)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_kdf_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_get0_field(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EC_GROUP_get0_field)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_alloc_ex_data(a0: i32, a1: *mut (), a2: *mut APTR, a3: i32) -> i32 { ((*IAmiSSL).CRYPTO_alloc_ex_data)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_new() -> *mut APTR { ((*IAmiSSL).OSSL_LIB_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_LIB_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_lh_flush(a0: *mut APTR) { ((*IAmiSSL).OPENSSL_LH_flush)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_native2bn(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_native2bn)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_bn2nativepad(a0: *const APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).BN_bn2nativepad)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_trace_get_category_num(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_trace_get_category_num)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_trace_get_category_name(a0: i32) -> *const APTR { ((*IAmiSSL).OSSL_trace_get_category_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_trace_set_channel(a0: i32, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_trace_set_channel)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_trace_set_prefix(a0: i32, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_trace_set_prefix)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_trace_set_suffix(a0: i32, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_trace_set_suffix)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_trace_set_callback(a0: i32, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_trace_set_callback)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_trace_enabled(a0: i32) -> i32 { ((*IAmiSSL).OSSL_trace_enabled)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_trace_begin(a0: i32) -> *mut APTR { ((*IAmiSSL).OSSL_trace_begin)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_trace_end(a0: i32, a1: *mut APTR) { ((*IAmiSSL).OSSL_trace_end)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_provider_load(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_PROVIDER_load)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_provider_try_load(a0: *mut APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).OSSL_PROVIDER_try_load)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_provider_unload(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_unload)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_provider_add_builtin(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_add_builtin)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_provider_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_PROVIDER_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_provider_get_params(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_encryptedvalue(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_ENCRYPTEDVALUE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_encryptedvalue(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_ENCRYPTEDVALUE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedvalue_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDVALUE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedvalue_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDVALUE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedvalue_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDVALUE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_msg(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_MSG)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_msg(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_MSG)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_MSG_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_MSG_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_pbmparameter(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_PBMPARAMETER)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_pbmparameter(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_PBMPARAMETER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_pbmparameter_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_PBMPARAMETER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_pbmparameter_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_PBMPARAMETER_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_pbmparameter_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_PBMPARAMETER_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_certid(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_CERTID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_certid(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_CERTID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certid_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_CERTID_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certid_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_CERTID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certid_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_CERTID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certid_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_CERTID_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_pkipublicationinfo(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_PKIPUBLICATIONINFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_pkipublicationinfo(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_PKIPUBLICATIONINFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_pkipublicationinfo_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_PKIPUBLICATIONINFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_pkipublicationinfo_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_PKIPUBLICATIONINFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_pkipublicationinfo_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_PKIPUBLICATIONINFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_singlepubinfo(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_SINGLEPUBINFO)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_singlepubinfo(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_SINGLEPUBINFO)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_singlepubinfo_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_SINGLEPUBINFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_singlepubinfo_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_SINGLEPUBINFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_singlepubinfo_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_SINGLEPUBINFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_certtemplate(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_CERTTEMPLATE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_certtemplate(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_CERTTEMPLATE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_msgs(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_MSGS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_msgs(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_MSGS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msgs_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_MSGS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msgs_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSGS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msgs_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_MSGS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_pbmp_new(a0: *mut APTR, a1: u32, a2: i32, a3: u32, a4: i32) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_pbmp_new)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_crmf_pbm_new(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: u32, a5: *const u8, a6: u32, a7: *mut *mut u8, a8: *mut u32) -> i32 { ((*IAmiSSL).OSSL_CRMF_pbm_new)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get0_reg_ctrl_reg_token(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_get0_regCtrl_regToken)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set1_reg_ctrl_reg_token(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set1_regCtrl_regToken)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get0_reg_ctrl_authenticator(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_get0_regCtrl_authenticator)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set1_reg_ctrl_authenticator(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set1_regCtrl_authenticator)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_pkipublication_info_push0_single_pub_info(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_PKIPublicationInfo_push0_SinglePubInfo)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set0_single_pub_info(a0: *mut APTR, a1: i32, a2: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set0_SinglePubInfo)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get0_reg_ctrl_pki_publication_info(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_get0_regCtrl_pkiPublicationInfo)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set_pkipublication_info_action(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set_PKIPublicationInfo_action)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set1_reg_ctrl_pki_publication_info(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set1_regCtrl_pkiPublicationInfo)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get0_reg_ctrl_protocol_encr_key(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_get0_regCtrl_protocolEncrKey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set1_reg_ctrl_protocol_encr_key(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set1_regCtrl_protocolEncrKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get0_reg_ctrl_old_cert_id(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_get0_regCtrl_oldCertID)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set1_reg_ctrl_old_cert_id(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set1_regCtrl_oldCertID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certid_gen(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_CERTID_gen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get0_reg_info_utf8_pairs(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_get0_regInfo_utf8Pairs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set1_reg_info_utf8_pairs(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set1_regInfo_utf8Pairs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get0_reg_info_cert_req(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_get0_regInfo_certReq)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set1_reg_info_cert_req(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set1_regInfo_certReq)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set0_validity(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set0_validity)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set_cert_req_id(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set_certReqId)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get_cert_req_id(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_get_certReqId)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_set0_extensions(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_set0_extensions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_push0_extension(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_push0_extension)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_create_popo(a0: i32, a1: *mut APTR, a2: *mut APTR, a3: *const APTR, a4: *mut APTR, a5: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_create_popo)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msgs_verify_popo(a0: *const APTR, a1: i32, a2: i32, a3: *mut APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSGS_verify_popo)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_get0_tmpl(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_MSG_get0_tmpl)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_get0_serial_number(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_get0_serialNumber)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_get0_subject(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_get0_subject)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_get0_issuer(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_get0_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_get0_extensions(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_get0_extensions)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_fill(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_fill)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedvalue_get1_enc_cert(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDVALUE_get1_encCert)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_locate(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_PARAM_locate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_int_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut i32) { ((*IAmiSSL).OSSL_PARAM_construct_int_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_uint_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_uint_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_long_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_long_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_ulong_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_ulong_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_int32_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_int32_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_uint32_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_uint32_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_int64_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_int64_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_uint64_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_uint64_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_size_t_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut u32) { ((*IAmiSSL).OSSL_PARAM_construct_size_t_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_bn_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut u8, a3: u32) { ((*IAmiSSL).OSSL_PARAM_construct_BN_amiga)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_double_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_double_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_utf8_string_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: u32) { ((*IAmiSSL).OSSL_PARAM_construct_utf8_string_amiga)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_utf8_ptr_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut *mut APTR, a3: u32) { ((*IAmiSSL).OSSL_PARAM_construct_utf8_ptr_amiga)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_octet_string_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut (), a3: u32) { ((*IAmiSSL).OSSL_PARAM_construct_octet_string_amiga)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_octet_ptr_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut *mut (), a3: u32) { ((*IAmiSSL).OSSL_PARAM_construct_octet_ptr_amiga)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_get_int(a0: *const APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_int)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_uint(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_uint)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_long(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_long)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_ulong(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_ulong)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_int32(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_int32)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_uint32(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_uint32)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_int64(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_int64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_uint64(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_uint64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_size_t(a0: *const APTR, a1: *mut u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_size_t)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_int(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_int)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_uint(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_uint)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_long(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_long)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_ulong(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_ulong)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_int32(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_int32)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_uint32(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_uint32)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_int64(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_int64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_uint64(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_uint64)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_size_t(a0: *mut APTR, a1: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_size_t)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_double(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_double)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_double(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_double)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_bn(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_BN)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_bn(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_BN)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_utf8_string(a0: *const APTR, a1: *mut *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_utf8_string)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_set_utf8_string(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_utf8_string)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_octet_string(a0: *const APTR, a1: *mut *mut (), a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_octet_string)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_set_octet_string(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_octet_string)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_get_utf8_ptr(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_utf8_ptr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_utf8_ptr(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_utf8_ptr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_octet_ptr(a0: *const APTR, a1: *mut *mut APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_octet_ptr)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_set_octet_ptr(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_octet_ptr)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_set0_distinguishing_id(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_set0_distinguishing_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_get0_distinguishing_id(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_get0_distinguishing_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_MD_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_MD_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_set_default_properties(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_set_default_properties)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_end_amiga(a0: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_end_amiga)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ec_group_check_named_curve(a0: *const APTR, a1: i32, a2: *mut APTR) -> i32 { ((*IAmiSSL).EC_GROUP_check_named_curve)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_cipher_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_CIPHER_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_mode(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_get_mode)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_info(a0: i32) -> *const APTR { ((*IAmiSSL).OPENSSL_info)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_new(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_KDF_CTX_new)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_kdf(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_CTX_kdf)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_i2d_key_params(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_KeyParams)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_key_params(a0: i32, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).d2i_KeyParams)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_i2d_key_params_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_KeyParams_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_key_params_bio(a0: i32, a1: *mut *mut APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_KeyParams_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_pkistatus_it() -> *const APTR { ((*IAmiSSL).OSSL_CMP_PKISTATUS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_cmp_pkiheader(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CMP_PKIHEADER)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_cmp_pkiheader(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CMP_PKIHEADER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_pkiheader_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CMP_PKIHEADER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_pkiheader_new() -> *mut APTR { ((*IAmiSSL).OSSL_CMP_PKIHEADER_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_cmp_pkiheader_it() -> *const APTR { ((*IAmiSSL).OSSL_CMP_PKIHEADER_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_cmp_msg(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CMP_MSG)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_cmp_msg(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CMP_MSG)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_it() -> *const APTR { ((*IAmiSSL).OSSL_CMP_MSG_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_create(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_create)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_set0(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_CMP_ITAV_set0)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get0_type(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_get0_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get0_value(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_get0_value)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_push0_stack_item(a0: *mut *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ITAV_push0_stack_item)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CMP_ITAV_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CMP_MSG_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_set_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_CTX_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_MD_CTX_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bn_ctx_new_ex(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_CTX_new_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_ctx_secure_new_ex(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_CTX_secure_new_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_thread_stop_ex(a0: *mut APTR) { ((*IAmiSSL).OPENSSL_thread_stop_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_param_locate_const(a0: *const APTR, a1: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_PARAM_locate_const)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_set0_distinguishing_id(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_REQ_set0_distinguishing_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_get0_distinguishing_id(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_get0_distinguishing_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_rand_ex(a0: *mut APTR, a1: i32, a2: i32, a3: i32, a4: APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).BN_rand_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_priv_rand_ex(a0: *mut APTR, a1: i32, a2: i32, a3: i32, a4: APTR, a5: *mut APTR) -> i32 { ((*IAmiSSL).BN_priv_rand_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bn_rand_range_ex(a0: *mut APTR, a1: *const APTR, a2: APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_rand_range_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_priv_rand_range_ex(a0: *mut APTR, a1: *const APTR, a2: APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).BN_priv_rand_range_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_bn_generate_prime_ex2(a0: *mut APTR, a1: i32, a2: i32, a3: *const APTR, a4: *const APTR, a5: *mut APTR, a6: *mut APTR) -> i32 { ((*IAmiSSL).BN_generate_prime_ex2)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_keyexch_free(a0: *mut APTR) { ((*IAmiSSL).EVP_KEYEXCH_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keyexch_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_KEYEXCH_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keyexch_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_KEYEXCH_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_pad(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_pad)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_KEYMGMT_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_KEYMGMT_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_free(a0: *mut APTR) { ((*IAmiSSL).EVP_KEYMGMT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYMGMT_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_pubkey_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_PUBKEY_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_provider_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_PROVIDER_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_CIPHER_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_md_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_MD_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_keyexch_get0_provider(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_KEYEXCH_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_provider_available(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_available)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_new() { ((*IAmiSSL).ERR_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_err_set_debug(a0: *const APTR, a1: i32, a2: *const APTR) { ((*IAmiSSL).ERR_set_debug)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_err_vset_error(a0: i32, a1: i32, a2: *const APTR, a3: *mut i32) { ((*IAmiSSL).ERR_vset_error)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_self_signed(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_self_signed)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_hexstr2buf_ex(a0: *mut u8, a1: u32, a2: *mut u32, a3: *const APTR, a4: APTR) -> i32 { ((*IAmiSSL).OPENSSL_hexstr2buf_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_openssl_buf2hexstr_ex(a0: *mut APTR, a1: u32, a2: *mut u32, a3: *const u8, a4: u32, a5: APTR) -> i32 { ((*IAmiSSL).OPENSSL_buf2hexstr_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ossl_param_allocate_from_text(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: u32, a5: *mut i32) -> i32 { ((*IAmiSSL).OSSL_PARAM_allocate_from_text)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_md_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_settable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_CTX_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_gettable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_CTX_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_set_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_get_params(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_MD_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_mac_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_MAC_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_mac_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MAC_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_set_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_MAC_CTX_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_MAC_CTX_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_mac_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MAC_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_free(a0: *mut APTR) { ((*IAmiSSL).EVP_MAC_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_MAC_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_MAC_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_mac_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MAC_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MAC_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_MAC_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_mac_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MAC_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_free(a0: *mut APTR) { ((*IAmiSSL).EVP_MD_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_free(a0: *mut APTR) { ((*IAmiSSL).EVP_CIPHER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_KDF_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_free(a0: *mut APTR) { ((*IAmiSSL).EVP_KDF_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_KDF_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_KDF_CTX_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_KDF_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_KDF_CTX_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_set_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_KDF_CTX_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_kdf_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_KDF_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_signature_free(a0: *mut APTR) { ((*IAmiSSL).EVP_SIGNATURE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_signature_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_SIGNATURE_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_signature_get0_provider(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_SIGNATURE_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_signature_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_SIGNATURE_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_signature_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_signature_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_signature_md(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_signature_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_CTX_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_settable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_CTX_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_tag_length(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_tag_length)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_get_error_all(a0: *mut *mut APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut i32) -> u32 { ((*IAmiSSL).ERR_get_error_all)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_err_peek_error_func(a0: *mut *mut APTR) -> u32 { ((*IAmiSSL).ERR_peek_error_func)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_peek_error_data(a0: *mut *mut APTR, a1: *mut i32) -> u32 { ((*IAmiSSL).ERR_peek_error_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_peek_error_all(a0: *mut *mut APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut i32) -> u32 { ((*IAmiSSL).ERR_peek_error_all)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_err_peek_last_error_func(a0: *mut *mut APTR) -> u32 { ((*IAmiSSL).ERR_peek_last_error_func)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_peek_last_error_data(a0: *mut *mut APTR, a1: *mut i32) -> u32 { ((*IAmiSSL).ERR_peek_last_error_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_peek_last_error_all(a0: *mut *mut APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut i32) -> u32 { ((*IAmiSSL).ERR_peek_last_error_all)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_cipher_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_mac_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_MAC_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_new(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CMP_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_reinit(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_reinit)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_option(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_option)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get_option(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_get_option)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_log_cb(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_log_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_print_errors(a0: *const APTR) { ((*IAmiSSL).OSSL_CMP_CTX_print_errors)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_server_path(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_serverPath)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_server(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_server)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_server_port(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_serverPort)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_proxy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_proxy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_no_proxy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_no_proxy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_http_cb(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_http_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_http_cb_arg(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_http_cb_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get_http_cb_arg(a0: *const APTR) -> *mut () { ((*IAmiSSL).OSSL_CMP_CTX_get_http_cb_arg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_transfer_cb(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_transfer_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_transfer_cb_arg(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_transfer_cb_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get_transfer_cb_arg(a0: *const APTR) -> *mut () { ((*IAmiSSL).OSSL_CMP_CTX_get_transfer_cb_arg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_srv_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_srvCert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_expected_sender(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_expected_sender)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set0_trusted_store(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set0_trustedStore)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_trusted_store(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_trustedStore)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_untrusted(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_untrusted)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_untrusted(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_untrusted)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_cert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_pkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_pkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_build_cert_chain(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_build_cert_chain)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_reference_value(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_referenceValue)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_secret_value(a0: *mut APTR, a1: *const u8, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_secretValue)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_recipient(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_recipient)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_push0_geninfo_itav(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_push0_geninfo_ITAV)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_extra_certs_out(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_extraCertsOut)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set0_new_pkey(a0: *mut APTR, a1: i32, a2: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set0_newPkey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_new_pkey(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_newPkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_issuer(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_issuer)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_subject_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_subjectName)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_push1_subject_alt_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_push1_subjectAltName)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set0_req_extensions(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set0_reqExtensions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_req_extensions_have_san(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_reqExtensions_have_SAN)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_push0_policy(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_push0_policy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_old_cert(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_oldCert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_p10_csr(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_p10CSR)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_push0_genm_itav(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_push0_genm_ITAV)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_cert_conf_cb(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_certConf_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set_cert_conf_cb_arg(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set_certConf_cb_arg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get_cert_conf_cb_arg(a0: *const APTR) -> *mut () { ((*IAmiSSL).OSSL_CMP_CTX_get_certConf_cb_arg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get_status(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_get_status)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_status_string(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_statusString)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get_fail_info_code(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_get_failInfoCode)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_new_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_newCert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get1_new_chain(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get1_newChain)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get1_ca_pubs(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get1_caPubs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get1_extra_certs_in(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get1_extraCertsIn)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_transaction_id(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_transactionID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_sender_nonce(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_senderNonce)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_log_open() -> i32 { ((*IAmiSSL).OSSL_CMP_log_open)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_cmp_log_close() { ((*IAmiSSL).OSSL_CMP_log_close)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_cmp_print_to_bio(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: i32, a4: APTR, a5: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_print_to_bio)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ossl_cmp_print_errors_cb(a0: APTR) { ((*IAmiSSL).OSSL_CMP_print_errors_cb)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certid_get0_issuer(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_CRMF_CERTID_get0_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certid_get0_serial_number(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_CRMF_CERTID_get0_serialNumber)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_digest_sign_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).EVP_DigestSignUpdate)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_digest_verify_update(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).EVP_DigestVerifyUpdate)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_check_prime(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BN_check_prime)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_KEYMGMT_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_KEYMGMT_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_keyexch_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_KEYEXCH_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_keyexch_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_KEYEXCH_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_kdf_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_KDF_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_signature_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_SIGNATURE_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_signature_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_SIGNATURE_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_md_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_MD_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_cipher_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_CIPHER_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_mac_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_MAC_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_KEYMGMT_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_keyexch_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_KEYEXCH_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_kdf_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_KDF_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_signature_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_SIGNATURE_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_snprint_pkistatus(a0: *const APTR, a1: *mut APTR, a2: u32) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_snprint_PKIStatus)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_hdr_get0_transaction_id(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_HDR_get0_transactionID)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_hdr_get0_recip_nonce(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_HDR_get0_recipNonce)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_lookup_store() -> *mut APTR { ((*IAmiSSL).X509_LOOKUP_store)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_add_cert(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_add_cert)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_add_certs(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_add_certs)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_load_file(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_load_file)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_load_path(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_load_path)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_load_store(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_load_store)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_fromdata(a0: *mut APTR, a1: *mut *mut APTR, a2: i32, a3: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_fromdata)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_free(a0: *mut APTR) { ((*IAmiSSL).EVP_ASYM_CIPHER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_ASYM_CIPHER_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_get0_provider(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_ASYM_CIPHER_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_ASYM_CIPHER_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_ASYM_CIPHER_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_ASYM_CIPHER_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_ASYM_CIPHER_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_padding(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_padding)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_rsa_padding(a0: *mut APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_rsa_padding)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_mgf1_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_mgf1_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_mgf1_md_name(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_mgf1_md_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_rsa_mgf1_md(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_rsa_mgf1_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_oaep_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_oaep_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_oaep_md_name(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_oaep_md_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_rsa_oaep_md(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_rsa_oaep_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set0_rsa_oaep_label(a0: *mut APTR, a1: *mut (), a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set0_rsa_oaep_label)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_rsa_oaep_label(a0: *mut APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get0_rsa_oaep_label)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_rsa_mgf1_md_name(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_rsa_mgf1_md_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_rsa_oaep_md_name(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_rsa_oaep_md_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ENCODER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_ENCODER_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_ENCODER_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).OSSL_ENCODER_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_ENCODER_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_settable_ctx_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).OSSL_ENCODER_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_new() -> *mut APTR { ((*IAmiSSL).OSSL_ENCODER_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ENCODER_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_get0_properties(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_ENCODER_get0_properties)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_to_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_to_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_new_for_pkey(a0: *const APTR, a1: i32, a2: *const APTR, a3: *const APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_ENCODER_CTX_new_for_pkey)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_cipher(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_cipher)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_passphrase(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_passphrase)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_pem_password_cb(a0: *mut APTR, a1: *mut APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_pem_password_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_passphrase_ui(a0: *mut APTR, a1: *const APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_passphrase_ui)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pem_read_bio_x509_pubkey(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_X509_PUBKEY)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_x509_pubkey(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_X509_PUBKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_x509_pubkey_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_X509_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_x509_pubkey_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_X509_PUBKEY_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_cmp_timeframe(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_cmp_timeframe)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_get0_header(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_MSG_get0_header)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_get_bodytype(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_MSG_get_bodytype)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_update_transaction_id(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_MSG_update_transactionID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_setup_crm(a0: *mut APTR, a1: i32, a2: i32) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_setup_CRM)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_f_prefix() -> *const APTR { ((*IAmiSSL).BIO_f_prefix)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_new_from_name(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_CTX_new_from_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_new_from_pkey(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_CTX_new_from_pkey)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_self_test_set_callback(a0: *mut APTR, a1: *mut APTR, a2: *mut ()) { ((*IAmiSSL).OSSL_SELF_TEST_set_callback)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_self_test_get_callback(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut ()) { ((*IAmiSSL).OSSL_SELF_TEST_get_callback)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_time_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_TIME_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_utctime_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_UTCTIME_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_generalizedtime_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_GENERALIZEDTIME_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_priv_bytes_ex(a0: *mut APTR, a1: *mut u8, a2: u32, a3: APTR) -> i32 { ((*IAmiSSL).RAND_priv_bytes_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_rand_bytes_ex(a0: *mut APTR, a1: *mut u8, a2: u32, a3: APTR) -> i32 { ((*IAmiSSL).RAND_bytes_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_default_digest_name(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_default_digest_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_decrypt_set1_pkey_and_peer(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).CMS_decrypt_set1_pkey_and_peer)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_add1_recipient(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: APTR) -> *mut APTR { ((*IAmiSSL).CMS_add1_recipient)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kari_set0_pkey_and_peer(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kari_set0_pkey_and_peer)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs8_pkey_add1_attr(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).PKCS8_pkey_add1_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs8_pkey_add1_attr_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).PKCS8_pkey_add1_attr_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_private_check(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_private_check)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_pairwise_check(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_pairwise_check)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_asn1_item_verify_ctx(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const (), a4: *mut APTR) -> i32 { ((*IAmiSSL).ASN1_item_verify_ctx)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_item_sign_ex(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *const (), a5: *const APTR, a6: *mut APTR, a7: *const APTR, a8: *mut APTR, a9: *const APTR) -> i32 { ((*IAmiSSL).ASN1_item_sign_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_asn1_item_verify_ex(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const (), a4: *const APTR, a5: *mut APTR, a6: *mut APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).ASN1_item_verify_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_bio_socket_wait(a0: i32, a1: i32, a2: APTR) -> i32 { ((*IAmiSSL).BIO_socket_wait)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_wait(a0: *mut APTR, a1: APTR, a2: APTR) -> i32 { ((*IAmiSSL).BIO_wait)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bio_do_connect_retry(a0: *mut APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).BIO_do_connect_retry)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_parse_url(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR, a5: *mut i32, a6: *mut *mut APTR, a7: *mut *mut APTR, a8: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_parse_url)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_ossl_http_adapt_proxy(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: i32) -> *const APTR { ((*IAmiSSL).OSSL_HTTP_adapt_proxy)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_get_resp_len(a0: *const APTR) -> u32 { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_get_resp_len)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_set_expected(a0: *mut APTR, a1: *const APTR, a2: i32, a3: i32, a4: i32) -> i32 { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_set_expected)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_http_is_alive(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_HTTP_is_alive)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_http_open(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: i32, a5: *mut APTR, a6: *mut APTR, a7: APTR, a8: *mut (), a9: i32, a10: i32) -> *mut APTR { ((*IAmiSSL).OSSL_HTTP_open)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) }
#[inline]
pub unsafe fn amissl_ossl_http_proxy_connect(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: i32, a6: *mut APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).OSSL_HTTP_proxy_connect)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ossl_http_set1_request(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR, a5: *const APTR, a6: i32, a7: u32, a8: i32, a9: i32) -> i32 { ((*IAmiSSL).OSSL_HTTP_set1_request)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_ossl_http_exchange(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_HTTP_exchange)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_http_get_amiga_1(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR, a4: *mut APTR, a5: APTR, a6: *mut ()) -> *mut APTR { ((*IAmiSSL).OSSL_HTTP_get_amiga_1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ossl_http_get_amiga_2(a0: *mut (), a1: i32, a2: *const APTR, a3: *const APTR, a4: i32, a5: u32, a6: i32) -> *mut () { ((*IAmiSSL).OSSL_HTTP_get_amiga_2)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ossl_http_transfer_amiga_1(a0: *mut *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: i32, a5: *const APTR, a6: *const APTR, a7: *mut APTR, a8: *mut APTR, a9: APTR, a10: *mut ()) -> *mut APTR { ((*IAmiSSL).OSSL_HTTP_transfer_amiga_1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) }
#[inline]
pub unsafe fn amissl_ossl_http_transfer_amiga_2(a0: *mut (), a1: i32, a2: *const APTR, a3: *const APTR, a4: *mut APTR, a5: *const APTR, a6: i32, a7: u32, a8: i32, a9: i32) -> *mut () { ((*IAmiSSL).OSSL_HTTP_transfer_amiga_2)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_ossl_http_close(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_HTTP_close)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_item_i2d_mem_bio(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_item_i2d_mem_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_add_error_txt(a0: *const APTR, a1: *const APTR) { ((*IAmiSSL).ERR_add_error_txt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_err_add_error_mem_bio(a0: *const APTR, a1: *mut APTR) { ((*IAmiSSL).ERR_add_error_mem_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_print_verify_cb(a0: i32, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_print_verify_cb)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_get1_all_certs(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_get1_all_certs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_validate_msg(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_validate_msg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_validate_cert_path(a0: *const APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_validate_cert_path)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_ecdh_cofactor_mode(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_ecdh_cofactor_mode)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_ecdh_cofactor_mode(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_ecdh_cofactor_mode)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_ecdh_kdf_type(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_ecdh_kdf_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_ecdh_kdf_type(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_ecdh_kdf_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_ecdh_kdf_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_ecdh_kdf_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_ecdh_kdf_md(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_ecdh_kdf_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_ecdh_kdf_outlen(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_ecdh_kdf_outlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_ecdh_kdf_outlen(a0: *mut APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_ecdh_kdf_outlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set0_ecdh_kdf_ukm(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set0_ecdh_kdf_ukm)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_ecdh_kdf_ukm(a0: *mut APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get0_ecdh_kdf_ukm)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_pss_saltlen(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_pss_saltlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_rsa_pss_saltlen(a0: *mut APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_rsa_pss_saltlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_issuer_sign_tool(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_ISSUER_SIGN_TOOL)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_issuer_sign_tool(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_ISSUER_SIGN_TOOL)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_issuer_sign_tool_free(a0: *mut APTR) { ((*IAmiSSL).ISSUER_SIGN_TOOL_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_issuer_sign_tool_new() -> *mut APTR { ((*IAmiSSL).ISSUER_SIGN_TOOL_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_issuer_sign_tool_it() -> *const APTR { ((*IAmiSSL).ISSUER_SIGN_TOOL_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_self_test_new(a0: *mut APTR, a1: *mut ()) -> *mut APTR { ((*IAmiSSL).OSSL_SELF_TEST_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_self_test_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_SELF_TEST_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_self_test_onbegin(a0: *mut APTR, a1: *const APTR, a2: *const APTR) { ((*IAmiSSL).OSSL_SELF_TEST_onbegin)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_self_test_oncorrupt_byte(a0: *mut APTR, a1: *mut u8) -> i32 { ((*IAmiSSL).OSSL_SELF_TEST_oncorrupt_byte)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_self_test_onend(a0: *mut APTR, a1: i32) { ((*IAmiSSL).OSSL_SELF_TEST_onend)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_provider_set_default_search_path(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_set_default_search_path)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_digest_sig(a0: *const APTR, a1: *mut *mut APTR, a2: *mut i32) -> *mut APTR { ((*IAmiSSL).X509_digest_sig)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_MSG_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ossl_cmp_pkisi(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CMP_PKISI)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_cmp_pkisi(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CMP_PKISI)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_pkisi_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CMP_PKISI_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_pkisi_new() -> *mut APTR { ((*IAmiSSL).OSSL_CMP_PKISI_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_cmp_pkisi_it() -> *const APTR { ((*IAmiSSL).OSSL_CMP_PKISI_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_cmp_pkisi_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_PKISI_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_snprint_pkistatus_info(a0: *const APTR, a1: *mut APTR, a2: u32) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_snprint_PKIStatusInfo)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_statusinfo_new(a0: i32, a1: i32, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_STATUSINFO_new)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ossl_cmp_msg_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CMP_MSG_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_ossl_cmp_msg_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_OSSL_CMP_MSG_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_process_request(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_SRV_process_request)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_server_perform(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_server_perform)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_new(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_SRV_CTX_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CMP_SRV_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_init(a0: *mut APTR, a1: *mut (), a2: APTR, a3: APTR, a4: APTR, a5: APTR, a6: APTR, a7: APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_SRV_CTX_init)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_get0_cmp_ctx(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_SRV_CTX_get0_cmp_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_get0_custom_ctx(a0: *const APTR) -> *mut () { ((*IAmiSSL).OSSL_CMP_SRV_CTX_get0_custom_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_set_send_unprotected_errors(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_CMP_SRV_CTX_set_send_unprotected_errors)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_set_accept_unprotected(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_CMP_SRV_CTX_set_accept_unprotected)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_set_accept_raverified(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_CMP_SRV_CTX_set_accept_raverified)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_set_grant_implicit_confirm(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_CMP_SRV_CTX_set_grant_implicit_confirm)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_exec_certreq(a0: *mut APTR, a1: i32, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_exec_certreq)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_try_certreq(a0: *mut APTR, a1: i32, a2: *const APTR, a3: *mut i32) -> i32 { ((*IAmiSSL).OSSL_CMP_try_certreq)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_cmp_cert_conf_cb(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_certConf_cb)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_cmp_exec_rr_ses(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_exec_RR_ses)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_exec_genm_ses(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_exec_GENM_ses)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_http_perform(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_MSG_http_perform)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_read(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_MSG_read)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_write(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_MSG_write)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_q_vkeygen(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut i32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_Q_vkeygen)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_generate(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_generate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_keygen_bits(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_keygen_bits)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_keygen_pubexp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_keygen_pubexp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set1_rsa_keygen_pubexp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set1_rsa_keygen_pubexp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_keygen_primes(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_keygen_primes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_nconf_new_ex(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).NCONF_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_conf_modules_load_file_ex(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: u32) -> i32 { ((*IAmiSSL).CONF_modules_load_file_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_load_config(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_LIB_CTX_load_config)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_to_param(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_PARAM_BLD_to_param)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_int(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_int)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_uint(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_uint)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_long(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_long)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_ulong(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_ulong)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_int32(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_int32)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_uint32(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_uint32)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_int64(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_int64)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_uint64(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_uint64)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_size_t(a0: *mut APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_size_t)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_double(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_double)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_bn(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_BN)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_bn_pad(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_BN_pad)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_utf8_string(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_utf8_string)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_utf8_ptr(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_utf8_ptr)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_octet_string(a0: *mut APTR, a1: *const APTR, a2: *const (), a3: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_octet_string)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_octet_ptr(a0: *mut APTR, a1: *const APTR, a2: *mut (), a3: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_octet_ptr)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_new() -> *mut APTR { ((*IAmiSSL).OSSL_PARAM_BLD_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_BLD_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_type_by_keymgmt(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_set_type_by_keymgmt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ocsp_respid_set_by_key_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).OCSP_RESPID_set_by_key_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ocsp_respid_match_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).OCSP_RESPID_match_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_srp_create_verifier_ex(a0: *const APTR, a1: *const APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *const APTR, a5: *const APTR, a6: *mut APTR, a7: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_create_verifier_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_srp_create_verifier_bn_ex(a0: *const APTR, a1: *const APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *const APTR, a5: *const APTR, a6: *mut APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).SRP_create_verifier_BN_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_srp_calc_b_ex(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_B_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_srp_calc_u_ex(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_u_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_srp_calc_x_ex(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_x_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_srp_calc_client_key_ex(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR, a5: *const APTR, a6: *mut APTR, a7: *const APTR) -> *mut APTR { ((*IAmiSSL).SRP_Calc_client_key_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_evp_pkey_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_int_param(a0: *const APTR, a1: *const APTR, a2: *mut i32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_int_param)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_size_t_param(a0: *const APTR, a1: *const APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_size_t_param)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_bn_param(a0: *const APTR, a1: *const APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_bn_param)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_utf8_string_param(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: u32, a4: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_utf8_string_param)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_octet_string_param(a0: *const APTR, a1: *const APTR, a2: *mut u8, a3: u32, a4: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_octet_string_param)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_can_sign(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_can_sign)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_verify(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_verify)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ct_policy_eval_ctx_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).CT_POLICY_EVAL_CTX_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ctlog_new_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).CTLOG_new_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ctlog_new_from_base64_ex(a0: *mut *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).CTLOG_new_from_base64_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ctlog_store_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).CTLOG_STORE_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_ex_data(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_PKEY_set_ex_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_ex_data(a0: *const APTR, a1: i32) -> *mut () { ((*IAmiSSL).EVP_PKEY_get_ex_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_group_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_group_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_group_name(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_group_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_ec_paramgen_curve_nid(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_ec_paramgen_curve_nid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_private_key_ex(a0: i32, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: i32, a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).d2i_PrivateKey_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_d2i_auto_private_key_ex(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).d2i_AutoPrivateKey_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_d2i_private_key_ex_bio(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).d2i_PrivateKey_ex_bio)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_read_bio_private_key_ex(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut (), a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_PrivateKey_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dsa_paramgen_bits(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dsa_paramgen_bits)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dsa_paramgen_q_bits(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dsa_paramgen_q_bits)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dsa_paramgen_md_props(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dsa_paramgen_md_props)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dsa_paramgen_gindex(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dsa_paramgen_gindex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dsa_paramgen_type(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dsa_paramgen_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dsa_paramgen_seed(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dsa_paramgen_seed)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dsa_paramgen_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dsa_paramgen_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_paramgen_type(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_paramgen_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_paramgen_gindex(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_paramgen_gindex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_paramgen_seed(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_paramgen_seed)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_paramgen_prime_len(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_paramgen_prime_len)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_paramgen_subprime_len(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_paramgen_subprime_len)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_paramgen_generator(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_paramgen_generator)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_nid(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_nid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_rfc5114(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_rfc5114)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dhx_rfc5114(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dhx_rfc5114)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get0_host(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get0_host)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get0_email(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get0_email)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get1_ip_asc(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_VERIFY_PARAM_get1_ip_asc)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_param_modified(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_modified)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_param_set_all_unmodified(a0: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_set_all_unmodified)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_RAND_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_rand_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_RAND_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_free(a0: *mut APTR) { ((*IAmiSSL).EVP_RAND_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_RAND_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_RAND_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_rand_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_RAND_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_RAND_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_rand_ctx_new(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_RAND_CTX_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_rand_ctx_free(a0: *mut APTR) { ((*IAmiSSL).EVP_RAND_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_ctx_get0_rand(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_RAND_CTX_get0_rand)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_ctx_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_RAND_CTX_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_rand_ctx_set_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_RAND_CTX_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_rand_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_RAND_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_RAND_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_RAND_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_RAND_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_rand_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_RAND_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_rand_instantiate(a0: *mut APTR, a1: APTR, a2: i32, a3: *const u8, a4: u32, a5: *const APTR) -> i32 { ((*IAmiSSL).EVP_RAND_instantiate)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_rand_uninstantiate(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_RAND_uninstantiate)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_generate(a0: *mut APTR, a1: *mut u8, a2: u32, a3: APTR, a4: i32, a5: *const u8, a6: u32) -> i32 { ((*IAmiSSL).EVP_RAND_generate)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_rand_reseed(a0: *mut APTR, a1: i32, a2: *const u8, a3: u32, a4: *const u8, a5: u32) -> i32 { ((*IAmiSSL).EVP_RAND_reseed)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_rand_nonce(a0: *mut APTR, a1: *mut u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_RAND_nonce)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_rand_enable_locking(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_RAND_enable_locking)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_verify_zeroization(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_RAND_verify_zeroization)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_get_strength(a0: *mut APTR) -> APTR { ((*IAmiSSL).EVP_RAND_get_strength)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_get_state(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_RAND_get_state)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_default_properties_is_fips_enabled(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_default_properties_is_fips_enabled)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_default_properties_enable_fips(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_default_properties_enable_fips)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_new_raw_private_key_ex(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: u32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_new_raw_private_key_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_new_raw_public_key_ex(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: u32) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_new_raw_public_key_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_param_bld_push_time_t(a0: *mut APTR, a1: *const APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_BLD_push_time_t)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_construct_time_t_amiga(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_construct_time_t_amiga)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_get_time_t(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_time_t)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_time_t(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_time_t)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_attach(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const APTR, a4: *const APTR, a5: *mut (), a6: *const APTR, a7: APTR, a8: *mut ()) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_attach)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_attach(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_attach)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_pss_keygen_saltlen(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_pss_keygen_saltlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_pss_keygen_mgf1_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_pss_keygen_mgf1_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_pss_keygen_mgf1_md_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_pss_keygen_mgf1_md_name)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_provider_do_all(a0: *mut APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_group_get_field_type(a0: *const APTR) -> i32 { ((*IAmiSSL).EC_GROUP_get_field_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_pubkey_eq(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_PUBKEY_eq)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_eq(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_eq)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_parameters_eq(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_parameters_eq)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_provider_query_operation(a0: *const APTR, a1: i32, a2: *mut i32) -> *const APTR { ((*IAmiSSL).OSSL_PROVIDER_query_operation)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_provider_unquery_operation(a0: *const APTR, a1: i32, a2: *const APTR) { ((*IAmiSSL).OSSL_PROVIDER_unquery_operation)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_provider_get0_provider_ctx(a0: *const APTR) -> *mut () { ((*IAmiSSL).OSSL_PROVIDER_get0_provider_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_provider_get_capabilities(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut ()) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_get_capabilities)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_group_new_by_curve_name_ex(a0: *mut APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).EC_GROUP_new_by_curve_name_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_key_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_KEY_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ec_key_new_by_curve_name_ex(a0: *mut APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).EC_KEY_new_by_curve_name_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_set0_default(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_LIB_CTX_set0_default)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pem_x509_info_read_bio_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut (), a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).PEM_X509_INFO_read_bio_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_x509_req_verify_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).X509_REQ_verify_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_lookup_ctrl_ex(a0: *mut APTR, a1: i32, a2: *const APTR, a3: i32, a4: *mut *mut APTR, a5: *mut APTR, a6: *const APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_ctrl_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_x509_load_cert_file_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).X509_load_cert_file_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_load_cert_crl_file_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).X509_load_cert_crl_file_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_lookup_by_subject_ex(a0: *mut APTR, a1: APTR, a2: *const APTR, a3: *mut APTR, a4: *mut APTR, a5: *const APTR) -> i32 { ((*IAmiSSL).X509_LOOKUP_by_subject_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_x509_store_load_file_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_load_file_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_store_load_store_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_load_store_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_store_load_locations_ex(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_load_locations_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_store_set_default_paths_ex(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_STORE_set_default_paths_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_build_chain(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32, a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_build_chain)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_x509_v3_set_issuer_pkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509V3_set_issuer_pkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2s_asn1_utf8_string(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).i2s_ASN1_UTF8STRING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_s2i_asn1_utf8_string(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).s2i_ASN1_UTF8STRING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_open_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *mut (), a5: *const APTR, a6: APTR, a7: *mut ()) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_open_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ossl_decoder_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_DECODER_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_DECODER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_DECODER_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_get0_properties(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_DECODER_get0_properties)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).OSSL_DECODER_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_DECODER_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_settable_ctx_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).OSSL_DECODER_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_new() -> *mut APTR { ((*IAmiSSL).OSSL_DECODER_CTX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_DECODER_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_passphrase(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_passphrase)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_pem_password_cb(a0: *mut APTR, a1: *mut APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_pem_password_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_passphrase_ui(a0: *mut APTR, a1: *const APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_passphrase_ui)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_from_bio(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_from_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_add_decoder(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_add_decoder)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_add_extra(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_add_extra)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_get_num_decoders(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_get_num_decoders)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_input_type(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_input_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_export(a0: *mut APTR, a1: *mut (), a2: u32, a3: *mut APTR, a4: *mut ()) -> i32 { ((*IAmiSSL).OSSL_DECODER_export)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_decoder_instance_get_decoder(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_DECODER_INSTANCE_get_decoder)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_instance_get_decoder_ctx(a0: *mut APTR) -> *mut () { ((*IAmiSSL).OSSL_DECODER_INSTANCE_get_decoder_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_gettable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).OSSL_DECODER_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_new_for_pkey(a0: *mut *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: i32, a5: *mut APTR, a6: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_DECODER_CTX_new_for_pkey)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_construct(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_construct)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_construct_data(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_construct_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_cleanup(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_cleanup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_get_construct(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_DECODER_CTX_get_construct)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_get_construct_data(a0: *mut APTR) -> *mut () { ((*IAmiSSL).OSSL_DECODER_CTX_get_construct_data)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_get_cleanup(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_DECODER_CTX_get_cleanup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_get0_primary(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).RAND_get0_primary)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_get0_public(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).RAND_get0_public)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_get0_private(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).RAND_get0_private)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get0_bag_obj(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get0_bag_obj)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get0_bag_type(a0: *const APTR) -> *const APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get0_bag_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_create_secret(a0: i32, a1: i32, a2: *const u8, a3: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_create_secret)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs12_add1_attr_by_nid(a0: *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).PKCS12_add1_attr_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs12_add1_attr_by_txt(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).PKCS12_add1_attr_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs12_add_secret(a0: *mut *mut APTR, a1: i32, a2: *const u8, a3: i32) -> *mut APTR { ((*IAmiSSL).PKCS12_add_secret)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_smime_write_asn1_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: i32, a4: i32, a5: i32, a6: *mut APTR, a7: *const APTR, a8: *mut APTR, a9: *const APTR) -> i32 { ((*IAmiSSL).SMIME_write_ASN1_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_smime_read_asn1_ex(a0: *mut APTR, a1: i32, a2: *mut *mut APTR, a3: *const APTR, a4: *mut *mut APTR, a5: *mut APTR, a6: *const APTR) -> *mut APTR { ((*IAmiSSL).SMIME_read_ASN1_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_cms_content_info_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_ContentInfo_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_smime_read_cms_ex(a0: *mut APTR, a1: i32, a2: *mut *mut APTR, a3: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).SMIME_read_CMS_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_sign_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: APTR, a5: *mut APTR, a6: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_sign_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_cms_data_create_ex(a0: *mut APTR, a1: APTR, a2: *mut APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_data_create_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_cms_digest_create_ex(a0: *mut APTR, a1: *const APTR, a2: APTR, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_digest_create_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_encrypted_data_encrypt_ex(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: u32, a4: APTR, a5: *mut APTR, a6: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_EncryptedData_encrypt_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_cms_encrypt_ex(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: APTR, a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_encrypt_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_cms_enveloped_data_create_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_EnvelopedData_create_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_cms_receipt_request_create0_ex(a0: *mut u8, a1: i32, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_ReceiptRequest_create0_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_sign_final_ex(a0: *mut APTR, a1: *mut u8, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *const APTR) -> i32 { ((*IAmiSSL).EVP_SignFinal_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_verify_final_ex(a0: *mut APTR, a1: *const u8, a2: APTR, a3: *mut APTR, a4: *mut APTR, a5: *const APTR) -> i32 { ((*IAmiSSL).EVP_VerifyFinal_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_digest_sign_init_ex(a0: *mut APTR, a1: *mut *mut APTR, a2: *const APTR, a3: *mut APTR, a4: *const APTR, a5: *mut APTR, a6: *const APTR) -> i32 { ((*IAmiSSL).EVP_DigestSignInit_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_digest_verify_init_ex(a0: *mut APTR, a1: *mut *mut APTR, a2: *const APTR, a3: *mut APTR, a4: *const APTR, a5: *mut APTR, a6: *const APTR) -> i32 { ((*IAmiSSL).EVP_DigestVerifyInit_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs7_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs7_sign_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: i32, a5: *mut APTR, a6: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_sign_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs7_encrypt_ex(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: i32, a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_encrypt_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_smime_read_pkcs7_ex(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).SMIME_read_PKCS7_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_provider_self_test(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_self_test)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_tls1_prf_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_tls1_prf_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set1_tls1_prf_secret(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set1_tls1_prf_secret)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_add1_tls1_prf_seed(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_add1_tls1_prf_seed)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_hkdf_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_hkdf_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set1_hkdf_salt(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set1_hkdf_salt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set1_hkdf_key(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set1_hkdf_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_add1_hkdf_info(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_add1_hkdf_info)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_hkdf_mode(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_hkdf_mode)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set1_pbe_pass(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set1_pbe_pass)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set1_scrypt_salt(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set1_scrypt_salt)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_scrypt_n(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_scrypt_N)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_scrypt_r(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_scrypt_r)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_scrypt_p(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_scrypt_p)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_scrypt_maxmem_bytes(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_scrypt_maxmem_bytes)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_kdf_type(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_kdf_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_dh_kdf_type(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_dh_kdf_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set0_dh_kdf_oid(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set0_dh_kdf_oid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_dh_kdf_oid(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get0_dh_kdf_oid)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_kdf_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_kdf_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_dh_kdf_md(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_dh_kdf_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_dh_kdf_outlen(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_dh_kdf_outlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_dh_kdf_outlen(a0: *mut APTR, a1: *mut i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_dh_kdf_outlen)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set0_dh_kdf_ukm(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set0_dh_kdf_ukm)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_dh_kdf_ukm(a0: *mut APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get0_dh_kdf_ukm)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_updated_iv(a0: *mut APTR, a1: *mut (), a2: u32) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_updated_iv)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_original_iv(a0: *mut APTR, a1: *mut (), a2: u32) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_original_iv)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYMGMT_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_settable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYMGMT_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_gen_settable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYMGMT_gen_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_signature_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SIGNATURE_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_signature_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SIGNATURE_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keyexch_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYEXCH_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keyexch_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYEXCH_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pubkey_ex(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).d2i_PUBKEY_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_store_info_new_pubkey(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_new_PUBKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get0_pubkey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get0_PUBKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get1_pubkey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_get1_PUBKEY)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pem_read_bio_pubkey_ex(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut (), a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_PUBKEY_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pem_read_bio_parameters_ex(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_Parameters_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ec_group_new_from_params(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_new_from_params)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_set_open_ex(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_set_open_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_LOADER_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_LOADER_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_get0_properties(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_LOADER_get0_properties)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).OSSL_STORE_LOADER_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_STORE_LOADER_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_param_get_utf8_string_ptr(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_utf8_string_ptr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_get_octet_string_ptr(a0: *const APTR, a1: *mut *mut APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_get_octet_string_ptr)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_passphrase_cb(a0: *mut APTR, a1: *mut APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_passphrase_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_mac_key(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_mac_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_info_new(a0: i32, a1: *mut ()) -> *mut APTR { ((*IAmiSSL).OSSL_STORE_INFO_new)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_info_get0_data(a0: i32, a1: *const APTR) -> *mut () { ((*IAmiSSL).OSSL_STORE_INFO_get0_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkcs82_pkey_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKCS82PKEY_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set1_id(a0: *mut APTR, a1: *const (), a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set1_id)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get1_id(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get1_id)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get1_id_len(a0: *mut APTR, a1: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get1_id_len)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_auth_enveloped_data_create(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_AuthEnvelopedData_create)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_auth_enveloped_data_create_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_AuthEnvelopedData_create_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_ec_param_enc(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_ec_param_enc)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_type_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_get0_type_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYMGMT_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kem_free(a0: *mut APTR) { ((*IAmiSSL).EVP_KEM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kem_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_KEM_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kem_get0_provider(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_KEM_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kem_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_KEM_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_kem_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_KEM_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_kem_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_KEM_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_kem_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_KEM_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_encapsulate_init(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_encapsulate_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_encapsulate(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *mut u8, a4: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_encapsulate)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_decapsulate_init(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_decapsulate_init)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_decapsulate(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_decapsulate)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_kem_op(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_kem_op)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_gettable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).OSSL_ENCODER_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_get_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_output_type(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_output_type)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_add_encoder(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_add_encoder)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_add_extra(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_add_extra)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_get_num_encoders(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_get_num_encoders)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_selection(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_selection)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_instance_get_encoder(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_ENCODER_INSTANCE_get_encoder)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_instance_get_encoder_ctx(a0: *mut APTR) -> *mut () { ((*IAmiSSL).OSSL_ENCODER_INSTANCE_get_encoder_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_instance_get_output_type(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).OSSL_ENCODER_INSTANCE_get_output_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_construct(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_construct)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_construct_data(a0: *mut APTR, a1: *mut ()) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_construct_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_cleanup(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_cleanup)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_passphrase_cb(a0: *mut APTR, a1: *mut APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_passphrase_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_type_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_PKEY_type_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_instance_get_input_type(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).OSSL_DECODER_INSTANCE_get_input_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_ASYM_CIPHER_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_ASYM_CIPHER_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kem_gettable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEM_gettable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kem_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEM_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_type_is_other(a0: *mut APTR) -> i32 { ((*IAmiSSL).PKCS7_type_is_other)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs7_get_octet_string(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS7_get_octet_string)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_from_data(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).OSSL_DECODER_from_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_encoder_to_data(a0: *mut APTR, a1: *mut *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).OSSL_ENCODER_to_data)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_libctx(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_CTX_get0_libctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_propq(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_CTX_get0_propq)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_selection(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_selection)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_ctx_set_input_structure(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_DECODER_CTX_set_input_structure)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_decoder_instance_get_input_structure(a0: *mut APTR, a1: *mut i32) -> *const APTR { ((*IAmiSSL).OSSL_DECODER_INSTANCE_get_input_structure)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_ctx_set_output_structure(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ENCODER_CTX_set_output_structure)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_encoder_instance_get_output_structure(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).OSSL_ENCODER_INSTANCE_get_output_structure)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pem_write_bio_private_key_ex(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const u8, a4: i32, a5: *mut APTR, a6: *mut (), a7: *mut APTR, a8: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_PrivateKey_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_pem_write_bio_pubkey_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_PUBKEY_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_group_name(a0: *const APTR, a1: *mut APTR, a2: u32, a3: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_get_group_name)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_atomic_or(a0: *mut APTR, a1: APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_atomic_or)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_atomic_load(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_atomic_load)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_pss_keygen_md(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_pss_keygen_md)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_rsa_pss_keygen_md_name(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_rsa_pss_keygen_md_name)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_settable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_set_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_int_param(a0: *mut APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_set_int_param)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_size_t_param(a0: *mut APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_set_size_t_param)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_bn_param(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_set_bn_param)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_utf8_string_param(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_set_utf8_string_param)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_set_octet_string_param(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_set_octet_string_param)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_ec_point_conv_form(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_ec_point_conv_form)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_field_type(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_field_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_params(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_fromdata_init(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_fromdata_init)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_fromdata_settable(a0: *mut APTR, a1: i32) -> *const APTR { ((*IAmiSSL).EVP_PKEY_fromdata_settable)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_param_check_quick(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_param_check_quick)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_public_check_quick(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_public_check_quick)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_is_a(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_settable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_CTX_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_gettable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_CTX_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_gettable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_CTX_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_settable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_CTX_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_gettable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_MAC_CTX_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_settable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_MAC_CTX_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_ctx_gettable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_RAND_CTX_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_ctx_settable_params(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).EVP_RAND_CTX_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_set_drbg_type(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).RAND_set_DRBG_type)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_rand_set_seed_source_type(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).RAND_set_seed_source_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_mod_exp_mont_consttime_x2(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR, a4: *mut APTR, a5: *mut APTR, a6: *const APTR, a7: *const APTR, a8: *const APTR, a9: *mut APTR, a10: *mut APTR) -> i32 { ((*IAmiSSL).BN_mod_exp_mont_consttime_x2)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) }
#[inline]
pub unsafe fn amissl_bio_f_readbuffer() -> *const APTR { ((*IAmiSSL).BIO_f_readbuffer)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_ess_check_signing_certs(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: i32) -> i32 { ((*IAmiSSL).OSSL_ESS_check_signing_certs)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_ess_signing_cert_new_init(a0: *const APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).OSSL_ESS_signing_cert_new_init)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_ess_signing_cert_v2_new_init(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).OSSL_ESS_signing_cert_v2_new_init)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ess_signing_cert_it() -> *const APTR { ((*IAmiSSL).ESS_SIGNING_CERT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ess_signing_cert_v2_it() -> *const APTR { ((*IAmiSSL).ESS_SIGNING_CERT_V2_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_q_digest(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const (), a4: u32, a5: *mut u8, a6: *mut u32) -> i32 { ((*IAmiSSL).EVP_Q_digest)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_digest_init_ex2(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_DigestInit_ex2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_encrypt_init_ex2(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: *const u8, a4: *const APTR) -> i32 { ((*IAmiSSL).EVP_EncryptInit_ex2)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_decrypt_init_ex2(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: *const u8, a4: *const APTR) -> i32 { ((*IAmiSSL).EVP_DecryptInit_ex2)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_cipher_init_ex2(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: *const u8, a4: i32, a5: *const APTR) -> i32 { ((*IAmiSSL).EVP_CipherInit_ex2)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_pkey_sign_init_ex(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_sign_init_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_init_ex(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_init_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_recover_init_ex(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_recover_init_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_encrypt_init_ex(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_encrypt_init_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_decrypt_init_ex(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_decrypt_init_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_derive_init_ex(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_derive_init_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_resp_ctx_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).TS_RESP_CTX_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_req_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_REQ_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_dup(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rsa_pss_params_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).RSA_PSS_PARAMS_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_derive_set_peer_ex(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).EVP_PKEY_derive_set_peer_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_decoder_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_DECODER_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_ENCODER_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_decoder_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_DECODER_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_encoder_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_ENCODER_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_LOADER_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_mac_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MAC_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_RAND_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYMGMT_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_signature_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SIGNATURE_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_ASYM_CIPHER_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kem_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEM_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keyexch_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYEXCH_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kdf_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KDF_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_sk_find_all(a0: *mut APTR, a1: *const (), a2: *mut i32) -> i32 { ((*IAmiSSL).OPENSSL_sk_find_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_crl_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_CRL_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_PARAM_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_param_merge(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_PARAM_merge)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_PARAM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_todata(a0: *const APTR, a1: i32, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_todata)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_export(a0: *const APTR, a1: i32, a2: *mut APTR, a3: *mut ()) -> i32 { ((*IAmiSSL).EVP_PKEY_export)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_get0_md(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_MD_CTX_get0_md)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_get1_md(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_MD_CTX_get1_md)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get0_cipher(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_CIPHER_CTX_get0_cipher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get1_cipher(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_CIPHER_CTX_get1_cipher)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_get0_global_default() -> *mut APTR { ((*IAmiSSL).OSSL_LIB_CTX_get0_global_default)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_signature_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SIGNATURE_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_asym_cipher_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_ASYM_CIPHER_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_kem_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEM_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_keyexch_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYEXCH_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs5_v2_pbe_keyivgen_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *const APTR, a6: i32, a7: *mut APTR, a8: *const APTR) -> i32 { ((*IAmiSSL).PKCS5_v2_PBE_keyivgen_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_evp_pbe_scrypt_amiga_1(a0: *const APTR, a1: u32, a2: *const u8, a3: APTR, a4: APTR, a5: APTR, a6: APTR, a7: *mut ()) -> i32 { ((*IAmiSSL).EVP_PBE_scrypt_amiga_1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_evp_pbe_scrypt_amiga_2(a0: u32, a1: *mut u8, a2: u32) -> *mut () { ((*IAmiSSL).EVP_PBE_scrypt_amiga_2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pbe_scrypt_ex_amiga_2(a0: u32, a1: *mut u8, a2: u32, a3: *mut APTR, a4: *const APTR) -> *mut () { ((*IAmiSSL).EVP_PBE_scrypt_ex_amiga_2)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs5_v2_scrypt_keyivgen_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *const APTR, a6: i32, a7: *mut APTR, a8: *const APTR) -> i32 { ((*IAmiSSL).PKCS5_v2_scrypt_keyivgen_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_evp_pbe_cipher_init_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *mut APTR, a5: i32, a6: *mut APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).EVP_PBE_CipherInit_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_evp_pbe_find_ex(a0: i32, a1: i32, a2: *mut i32, a3: *mut i32, a4: *mut *mut APTR, a5: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PBE_find_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_create_pkcs8_encrypt_ex(a0: i32, a1: *const APTR, a2: i32, a3: *mut u8, a4: i32, a5: i32, a6: *mut APTR, a7: *mut APTR, a8: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_create_pkcs8_encrypt_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_pkcs8_decrypt_ex(a0: *const APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS8_decrypt_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs12_decrypt_skey_ex(a0: *const APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_decrypt_skey_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs8_encrypt_ex(a0: i32, a1: *const APTR, a2: *const APTR, a3: i32, a4: *mut u8, a5: i32, a6: i32, a7: *mut APTR, a8: *mut APTR, a9: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS8_encrypt_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_pkcs8_set0_pbe_ex(a0: *const APTR, a1: i32, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS8_set0_pbe_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pkcs12_pack_p7encdata_ex(a0: i32, a1: *const APTR, a2: i32, a3: *mut u8, a4: i32, a5: i32, a6: *mut APTR, a7: *mut APTR, a8: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_pack_p7encdata_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_pkcs12_pbe_crypt_ex(a0: *const APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32, a5: *mut *mut u8, a6: *mut i32, a7: i32, a8: *mut APTR, a9: *const APTR) -> *mut u8 { ((*IAmiSSL).PKCS12_pbe_crypt_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_pkcs12_item_decrypt_d2i_ex(a0: *const APTR, a1: *const APTR, a2: *const APTR, a3: i32, a4: *const APTR, a5: i32, a6: *mut APTR, a7: *const APTR) -> *mut () { ((*IAmiSSL).PKCS12_item_decrypt_d2i_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_pkcs12_item_i2d_encrypt_ex(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: i32, a4: *mut (), a5: i32, a6: *mut APTR, a7: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_item_i2d_encrypt_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_pkcs12_init_ex(a0: i32, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_init_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_key_gen_asc_ex(a0: *const APTR, a1: i32, a2: *mut u8, a3: i32, a4: i32, a5: i32, a6: i32, a7: *mut u8, a8: *const APTR, a9: *mut APTR, a10: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_key_gen_asc_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) }
#[inline]
pub unsafe fn amissl_pkcs12_key_gen_uni_ex(a0: *mut u8, a1: i32, a2: *mut u8, a3: i32, a4: i32, a5: i32, a6: i32, a7: *mut u8, a8: *const APTR, a9: *mut APTR, a10: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_key_gen_uni_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) }
#[inline]
pub unsafe fn amissl_pkcs12_key_gen_utf8_ex(a0: *const APTR, a1: i32, a2: *mut u8, a3: i32, a4: i32, a5: i32, a6: i32, a7: *mut u8, a8: *const APTR, a9: *mut APTR, a10: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_key_gen_utf8_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10) }
#[inline]
pub unsafe fn amissl_pkcs12_pbe_keyivgen_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *const APTR, a6: i32, a7: *mut APTR, a8: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_PBE_keyivgen_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_pkcs12_create_ex(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: i32, a6: i32, a7: i32, a8: i32, a9: i32, a10: *mut APTR, a11: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_create_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9, a10, a11) }
#[inline]
pub unsafe fn amissl_pkcs12_add_key_ex(a0: *mut *mut APTR, a1: *mut APTR, a2: i32, a3: i32, a4: i32, a5: *const APTR, a6: *mut APTR, a7: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_add_key_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_pkcs12_add_safe_ex(a0: *mut *mut APTR, a1: *mut APTR, a2: i32, a3: i32, a4: *const APTR, a5: *mut APTR, a6: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_add_safe_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs12_add_safes_ex(a0: *mut APTR, a1: i32, a2: *mut APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_add_safes_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe_set0_algor_ex(a0: *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: i32, a5: *mut APTR) -> i32 { ((*IAmiSSL).PKCS5_pbe_set0_algor_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe_set_ex(a0: i32, a1: i32, a2: *const u8, a3: i32, a4: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS5_pbe_set_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe2_set_iv_ex(a0: *const APTR, a1: i32, a2: *mut u8, a3: i32, a4: *mut u8, a5: i32, a6: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS5_pbe2_set_iv_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_pkcs5_pbkdf2_set_ex(a0: i32, a1: *mut u8, a2: i32, a3: i32, a4: i32, a5: *mut APTR) -> *mut APTR { ((*IAmiSSL).PKCS5_pbkdf2_set_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bio_new_from_core_bio(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_from_core_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_s_core() -> *const APTR { ((*IAmiSSL).BIO_s_core)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_get_line(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).BIO_get_line)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_new_from_dispatch(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_LIB_CTX_new_from_dispatch)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_new_child(a0: *const APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_LIB_CTX_new_child)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_provider_get0_dispatch(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_PROVIDER_get0_dispatch)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pkcs5_pbe_keyivgen_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *const APTR, a6: i32, a7: *mut APTR, a8: *const APTR) -> i32 { ((*IAmiSSL).PKCS5_PBE_keyivgen_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_evp_mac_ctx_get_block_size(a0: *mut APTR) -> u32 { ((*IAmiSSL).EVP_MAC_CTX_get_block_size)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_debug_callback_ex(a0: *mut APTR, a1: i32, a2: *const APTR, a3: u32, a4: i32, a5: i32, a6: i32, a7: *mut u32) -> i32 { ((*IAmiSSL).BIO_debug_callback_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_b2i_pvk_bio_ex(a0: *mut APTR, a1: *mut APTR, a2: *mut (), a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).b2i_PVK_bio_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_i2b_pvk_bio_ex(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut APTR, a4: *mut (), a5: *mut APTR, a6: *const APTR) -> i32 { ((*IAmiSSL).i2b_PVK_bio_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_nconf_get0_libctx(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).NCONF_get0_libctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_nconf_get_section_names(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).NCONF_get_section_names)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_pubkey_new_ex(a0: *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_PUBKEY_new_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_item_new_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_item_new_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_asn1_item_d2i_bio_ex(a0: *const APTR, a1: *mut APTR, a2: *mut (), a3: *mut APTR, a4: *const APTR) -> *mut () { ((*IAmiSSL).ASN1_item_d2i_bio_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_asn1_item_d2i_ex(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32, a3: *const APTR, a4: *mut APTR, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).ASN1_item_d2i_ex)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_asn1_time_print_ex(a0: *mut APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).ASN1_TIME_print_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_PKEY_CTX_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_strcasecmp(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OPENSSL_strcasecmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_strncasecmp(a0: *const APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).OPENSSL_strncasecmp)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_reset_geninfo_itavs(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_reset_geninfo_ITAVs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_rand_ctx_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_RAND_CTX_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_set0_public(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).RAND_set0_public)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_rand_set0_private(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).RAND_set0_private)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_MD_CTX_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_CIPHER_CTX_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bn_are_coprime(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).BN_are_coprime)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_update_recip_nonce(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_MSG_update_recipNonce)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_client_hello_get_extension_order(a0: *mut APTR, a1: *mut APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).SSL_client_hello_get_extension_order)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_quic_client_method() -> *const APTR { ((*IAmiSSL).OSSL_QUIC_client_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_quic_client_thread_method() -> *const APTR { ((*IAmiSSL).OSSL_QUIC_client_thread_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set1_cert_comp_preference(a0: *mut APTR, a1: *mut i32, a2: u32) -> i32 { ((*IAmiSSL).SSL_CTX_set1_cert_comp_preference)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set1_cert_comp_preference(a0: *mut APTR, a1: *mut i32, a2: u32) -> i32 { ((*IAmiSSL).SSL_set1_cert_comp_preference)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_compress_certs(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_CTX_compress_certs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_compress_certs(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_compress_certs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set1_compressed_cert(a0: *mut APTR, a1: i32, a2: *mut u8, a3: u32, a4: u32) -> i32 { ((*IAmiSSL).SSL_CTX_set1_compressed_cert)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_set1_compressed_cert(a0: *mut APTR, a1: i32, a2: *mut u8, a3: u32, a4: u32) -> i32 { ((*IAmiSSL).SSL_set1_compressed_cert)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get1_compressed_cert(a0: *mut APTR, a1: i32, a2: *mut *mut u8, a3: *mut u32) -> u32 { ((*IAmiSSL).SSL_CTX_get1_compressed_cert)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_get1_compressed_cert(a0: *mut APTR, a1: i32, a2: *mut *mut u8, a3: *mut u32) -> u32 { ((*IAmiSSL).SSL_get1_compressed_cert)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_get_rpoll_descriptor(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_rpoll_descriptor)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_wpoll_descriptor(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_wpoll_descriptor)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_set_blocking_mode(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_set_blocking_mode)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_blocking_mode(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_blocking_mode)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set1_initial_peer_addr(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).SSL_set1_initial_peer_addr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_net_read_desired(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_net_read_desired)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_net_write_desired(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_net_write_desired)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_shutdown_ex(a0: *mut APTR, a1: APTR, a2: *const APTR, a3: u32) -> i32 { ((*IAmiSSL).SSL_shutdown_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_stream_conclude(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_stream_conclude)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_inject_net_dgram(a0: *mut APTR, a1: *const u8, a2: u32, a3: *const APTR, a4: *const APTR) -> i32 { ((*IAmiSSL).SSL_inject_net_dgram)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_get0_peer_rpk(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).SSL_get0_peer_rpk)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_get0_peer_rpk(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_SESSION_get0_peer_rpk)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set1_client_cert_type(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SSL_set1_client_cert_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get0_client_cert_type(a0: *const APTR, a1: *mut *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).SSL_get0_client_cert_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set1_server_cert_type(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SSL_set1_server_cert_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get0_server_cert_type(a0: *const APTR, a1: *mut *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).SSL_get0_server_cert_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set1_client_cert_type(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SSL_CTX_set1_client_cert_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get0_client_cert_type(a0: *const APTR, a1: *mut *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).SSL_CTX_get0_client_cert_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set1_server_cert_type(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SSL_CTX_set1_server_cert_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get0_server_cert_type(a0: *const APTR, a1: *mut *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).SSL_CTX_get0_server_cert_type)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_negotiated_client_cert_type(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_negotiated_client_cert_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_negotiated_server_cert_type(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_get_negotiated_server_cert_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_add_expected_rpk(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_add_expected_rpk)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_ssl_session_ex(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).d2i_SSL_SESSION_ex)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_is_tls(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_is_tls)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_is_quic(a0: *const APTR) -> i32 { ((*IAmiSSL).SSL_is_quic)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_handshake_rtt(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_handshake_rtt)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_new_stream(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).SSL_new_stream)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get0_connection(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get0_connection)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_is_connection(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_is_connection)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_stream_type(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_stream_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_stream_id(a0: *mut APTR) -> APTR { ((*IAmiSSL).SSL_get_stream_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_set_default_stream_mode(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_set_default_stream_mode)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_accept_stream(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).SSL_accept_stream)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_accept_stream_queue_len(a0: *mut APTR) -> u32 { ((*IAmiSSL).SSL_get_accept_stream_queue_len)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_stream_reset(a0: *mut APTR, a1: *const APTR, a2: u32) -> i32 { ((*IAmiSSL).SSL_stream_reset)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get_stream_read_state(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_stream_read_state)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_stream_write_state(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_stream_write_state)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_stream_read_error_code(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_stream_read_error_code)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_stream_write_error_code(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_stream_write_error_code)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_conn_close_info(a0: *mut APTR, a1: *mut APTR, a2: u32) -> i32 { ((*IAmiSSL).SSL_get_conn_close_info)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_incoming_stream_policy(a0: *mut APTR, a1: i32, a2: APTR) -> i32 { ((*IAmiSSL).SSL_set_incoming_stream_policy)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_handle_events(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_handle_events)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get_event_timeout(a0: *mut APTR, a1: *mut timeval, a2: *mut i32) -> i32 { ((*IAmiSSL).SSL_get_event_timeout)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get0_group_name(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).SSL_get0_group_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_is_stream_local(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_is_stream_local)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_f_zlib() -> *const APTR { ((*IAmiSSL).BIO_f_zlib)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_pubkey_set0_public_key(a0: *mut APTR, a1: *mut u8, a2: i32) { ((*IAmiSSL).X509_PUBKEY_set0_public_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_stack_of_x509_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_STACK_OF_X509_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_trace_string(a0: *mut APTR, a1: i32, a2: i32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).OSSL_trace_string)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_bn_signed_bin2bn(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_signed_bin2bn)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_signed_bn2bin(a0: *const APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).BN_signed_bn2bin)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_signed_lebin2bn(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_signed_lebin2bn)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_signed_bn2lebin(a0: *const APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).BN_signed_bn2lebin)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_signed_native2bn(a0: *const u8, a1: i32, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).BN_signed_native2bn)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_bn_signed_bn2native(a0: *const APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).BN_signed_bn2native)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_async_set_mem_functions(a0: APTR, a1: APTR) -> i32 { ((*IAmiSSL).ASYNC_set_mem_functions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_async_get_mem_functions(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).ASYNC_get_mem_functions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_addr_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).BIO_ADDR_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_new_ca_certs(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_new_caCerts)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get0_ca_certs(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ITAV_get0_caCerts)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_get1_ca_certs(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_get1_caCerts)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_new_root_ca_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_new_rootCaCert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get0_root_ca_cert(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ITAV_get0_rootCaCert)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_new_root_ca_key_update(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_new_rootCaKeyUpdate)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get0_root_ca_key_update(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ITAV_get0_rootCaKeyUpdate)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_cmp_get1_root_ca_key_update(a0: *mut APTR, a1: *const APTR, a2: *mut *mut APTR, a3: *mut *mut APTR, a4: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_get1_rootCaKeyUpdate)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_libctx(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_libctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_propq(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_propq)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_validated_srv_cert(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_validatedSrvCert)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_set1_serial_number(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CTX_set1_serialNumber)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_get0_public_key(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_get0_publicKey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_final_digest(a0: *mut APTR, a1: *const u8, a2: APTR, a3: *mut APTR, a4: APTR) -> i32 { ((*IAmiSSL).CMS_final_digest)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_cms_enveloped_data_it() -> *const APTR { ((*IAmiSSL).CMS_EnvelopedData_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_enveloped_data_decrypt(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: APTR, a6: *mut APTR, a7: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_EnvelopedData_decrypt)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_cms_signed_data_free(a0: *mut APTR) { ((*IAmiSSL).CMS_SignedData_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_signed_data_new() -> *mut APTR { ((*IAmiSSL).CMS_SignedData_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_cms_signed_data_verify(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR, a6: APTR, a7: *mut APTR, a8: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_SignedData_verify)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8) }
#[inline]
pub unsafe fn amissl_bio_s_dgram_mem() -> *const APTR { ((*IAmiSSL).BIO_s_dgram_mem)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_recvmmsg(a0: *mut APTR, a1: *mut APTR, a2: u32, a3: u32, a4: APTR, a5: *mut u32) -> i32 { ((*IAmiSSL).BIO_recvmmsg)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bio_sendmmsg(a0: *mut APTR, a1: *mut APTR, a2: u32, a3: u32, a4: APTR, a5: *mut u32) -> i32 { ((*IAmiSSL).BIO_sendmmsg)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bio_meth_set_sendmmsg(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_sendmmsg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_get_sendmmsg(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_sendmmsg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_meth_set_recvmmsg(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).BIO_meth_set_recvmmsg)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_meth_get_recvmmsg(a0: *const APTR) -> i32 { ((*IAmiSSL).BIO_meth_get_recvmmsg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_err_is_non_fatal(a0: APTR) -> i32 { ((*IAmiSSL).BIO_err_is_non_fatal)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_s_dgram_pair() -> *const APTR { ((*IAmiSSL).BIO_s_dgram_pair)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_new_bio_dgram_pair(a0: *mut *mut APTR, a1: u32, a2: *mut *mut APTR, a3: u32) -> i32 { ((*IAmiSSL).BIO_new_bio_dgram_pair)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_pkey_auth_encapsulate_init(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_auth_encapsulate_init)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_auth_decapsulate_init(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_auth_decapsulate_init)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_set0_attrs(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).PKCS12_SAFEBAG_set0_attrs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_create_ex2_amiga_1(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: i32, a6: i32, a7: *mut ()) -> *mut APTR { ((*IAmiSSL).PKCS12_create_ex2_amiga_1)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_pkcs12_create_ex2_amiga_2(a0: i32, a1: i32, a2: i32, a3: *mut APTR, a4: *const APTR, a5: *mut APTR, a6: *mut ()) -> *mut () { ((*IAmiSSL).PKCS12_create_ex2_amiga_2)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ossl_sleep(a0: APTR) { ((*IAmiSSL).OSSL_sleep)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_get_thread_support_flags() -> APTR { ((*IAmiSSL).OSSL_get_thread_support_flags)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_set_max_threads(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_set_max_threads)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_get_max_threads(a0: *mut APTR) -> APTR { ((*IAmiSSL).OSSL_get_max_threads)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_comp_brotli() -> *mut APTR { ((*IAmiSSL).COMP_brotli)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_comp_brotli_oneshot() -> *mut APTR { ((*IAmiSSL).COMP_brotli_oneshot)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_brotli() -> *const APTR { ((*IAmiSSL).BIO_f_brotli)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_comp_zstd() -> *mut APTR { ((*IAmiSSL).COMP_zstd)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_comp_zstd_oneshot() -> *mut APTR { ((*IAmiSSL).COMP_zstd_oneshot)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_bio_f_zstd() -> *const APTR { ((*IAmiSSL).BIO_f_zstd)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_pubkey_ex_bio(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).d2i_PUBKEY_ex_bio)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_comp_zlib_oneshot() -> *mut APTR { ((*IAmiSSL).COMP_zlib_oneshot)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_hpke_keygen_amiga(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *mut *mut APTR, a4: *const u8, a5: u32, a6: *mut APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).OSSL_HPKE_keygen_amiga)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ossl_hpke_suite_check_amiga(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_HPKE_suite_check_amiga)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_hpke_get_grease_value(a0: *const APTR, a1: *mut APTR, a2: *mut u8, a3: *mut u32, a4: *mut u8, a5: u32, a6: *mut APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).OSSL_HPKE_get_grease_value)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ossl_hpke_str2suite(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_HPKE_str2suite)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_hpke_ctx_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_HPKE_CTX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_hpke_ctx_new_amiga(a0: i32, a1: *mut APTR, a2: i32, a3: *mut APTR, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_HPKE_CTX_new_amiga)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_hpke_ctx_set1_authpriv(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_HPKE_CTX_set1_authpriv)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_hpke_ctx_set1_authpub(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).OSSL_HPKE_CTX_set1_authpub)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_hpke_ctx_set1_psk(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: u32) -> i32 { ((*IAmiSSL).OSSL_HPKE_CTX_set1_psk)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_hpke_ctx_set1_ikme(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).OSSL_HPKE_CTX_set1_ikme)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_hpke_get_ciphertext_size_amiga(a0: *mut APTR, a1: u32) -> u32 { ((*IAmiSSL).OSSL_HPKE_get_ciphertext_size_amiga)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_hpke_get_public_encap_size_amiga(a0: *mut APTR) -> u32 { ((*IAmiSSL).OSSL_HPKE_get_public_encap_size_amiga)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_hpke_export(a0: *mut APTR, a1: *mut u8, a2: u32, a3: *const u8, a4: u32) -> i32 { ((*IAmiSSL).OSSL_HPKE_export)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_hpke_encap(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32, a5: *const u8, a6: u32) -> i32 { ((*IAmiSSL).OSSL_HPKE_encap)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ossl_hpke_decap(a0: *mut APTR, a1: *const u8, a2: u32, a3: *mut APTR, a4: *const u8, a5: u32) -> i32 { ((*IAmiSSL).OSSL_HPKE_decap)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ossl_hpke_seal(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32, a5: *const u8, a6: u32) -> i32 { ((*IAmiSSL).OSSL_HPKE_seal)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ossl_hpke_open(a0: *mut APTR, a1: *mut u8, a2: *mut u32, a3: *const u8, a4: u32, a5: *const u8, a6: u32) -> i32 { ((*IAmiSSL).OSSL_HPKE_open)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_ossl_hpke_ctx_get_seq(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_HPKE_CTX_get_seq)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_hpke_ctx_set_seq(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).OSSL_HPKE_CTX_set_seq)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_hpke_get_recommended_ikmelen_amiga(a0: *mut APTR) -> u32 { ((*IAmiSSL).OSSL_HPKE_get_recommended_ikmelen_amiga)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_provider_get0_default_search_path(a0: *mut APTR) -> *const APTR { ((*IAmiSSL).OSSL_PROVIDER_get0_default_search_path)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_bio_get_rpoll_descriptor(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).BIO_get_rpoll_descriptor)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_bio_get_wpoll_descriptor(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).BIO_get_wpoll_descriptor)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_asn1_item_unpack_ex(a0: *const APTR, a1: *const APTR, a2: *mut APTR, a3: *const APTR) -> *mut () { ((*IAmiSSL).ASN1_item_unpack_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get1_cert_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get1_cert_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_pkcs12_safebag_get1_crl_ex(a0: *const APTR, a1: *mut APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).PKCS12_SAFEBAG_get1_crl_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ec_group_to_params(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR) -> *mut APTR { ((*IAmiSSL).EC_GROUP_to_params)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_init_rpk(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).X509_STORE_CTX_init_rpk)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_get0_rpk(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_CTX_get0_rpk)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set0_rpk(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set0_rpk)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_atomic_load_int(a0: *mut i32, a1: *mut i32, a2: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_atomic_load_int)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_err_state_new() -> *mut APTR { ((*IAmiSSL).OSSL_ERR_STATE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_err_state_save(a0: *mut APTR) { ((*IAmiSSL).OSSL_ERR_STATE_save)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_err_state_restore(a0: *const APTR) { ((*IAmiSSL).OSSL_ERR_STATE_restore)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_err_state_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ERR_STATE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_err_count_to_mark() -> i32 { ((*IAmiSSL).ERR_count_to_mark)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_provider_load_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_PROVIDER_load_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_provider_try_load_ex(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: i32) -> *mut APTR { ((*IAmiSSL).OSSL_PROVIDER_try_load_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_err_state_save_to_mark(a0: *mut APTR) { ((*IAmiSSL).OSSL_ERR_STATE_save_to_mark)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_get_crl(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_CTX_set_get_crl)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_current_reasons(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).X509_STORE_CTX_set_current_reasons)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_store_delete(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: *mut (), a5: *const APTR) -> i32 { ((*IAmiSSL).OSSL_STORE_delete)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_bio_addr_copy(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).BIO_ADDR_copy)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_write_ex2(a0: *mut APTR, a1: *const (), a2: u32, a3: APTR, a4: *mut u32) -> i32 { ((*IAmiSSL).SSL_write_ex2)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ssl_get_value_uint(a0: *mut APTR, a1: APTR, a2: APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_value_uint)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_set_value_uint(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR) -> i32 { ((*IAmiSSL).SSL_set_value_uint)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ssl_poll(a0: *mut APTR, a1: u32, a2: u32, a3: *const timeval, a4: APTR, a5: *mut u32) -> i32 { ((*IAmiSSL).SSL_poll)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_ssl_session_get_time_ex(a0: *const APTR) -> APTR { ((*IAmiSSL).SSL_SESSION_get_time_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_session_set_time_ex(a0: *mut APTR, a1: APTR) -> APTR { ((*IAmiSSL).SSL_SESSION_set_time_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_ctx_get0_geninfo_itavs(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CTX_get0_geninfo_ITAVs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_hdr_get0_geninfo_itavs(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_HDR_get0_geninfo_ITAVs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_new0_cert_profile(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_new0_certProfile)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get0_cert_profile(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ITAV_get0_certProfile)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_msg_get0_certreq_publickey(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_MSG_get0_certreq_publickey)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_srv_ctx_init_trans(a0: *mut APTR, a1: APTR, a2: APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_SRV_CTX_init_trans)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_digest_squeeze(a0: *mut APTR, a1: *mut u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_DigestSqueeze)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_err_pop() -> i32 { ((*IAmiSSL).ERR_pop)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_store_get1_objects(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).X509_STORE_get1_objects)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_openssl_lh_set_thunks(a0: *mut APTR, a1: APTR, a2: APTR, a3: APTR, a4: APTR) -> *mut APTR { ((*IAmiSSL).OPENSSL_LH_set_thunks)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_openssl_lh_doall_arg_thunk(a0: *mut APTR, a1: APTR, a2: APTR, a3: *mut ()) { ((*IAmiSSL).OPENSSL_LH_doall_arg_thunk)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_http_req_ctx_set_max_response_hdr_lines(a0: *mut APTR, a1: u32) { ((*IAmiSSL).OSSL_HTTP_REQ_CTX_set_max_response_hdr_lines)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_flush_sessions_ex(a0: *mut APTR, a1: APTR) { ((*IAmiSSL).SSL_CTX_flush_sessions_ex)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_block_padding_ex(a0: *mut APTR, a1: u32, a2: u32) -> i32 { ((*IAmiSSL).SSL_CTX_set_block_padding_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_block_padding_ex(a0: *mut APTR, a1: u32, a2: u32) -> i32 { ((*IAmiSSL).SSL_set_block_padding_ex)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_get1_builtin_sigalgs(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get1_builtin_sigalgs)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_dist_point_name_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).DIST_POINT_NAME_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_general_name_set1_x509_name(a0: *mut *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).GENERAL_NAME_set1_X509_NAME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_crlstatus_create(a0: *const APTR, a1: *const APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CRLSTATUS_create)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_crlstatus_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CMP_CRLSTATUS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_crlstatus_get0(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_CRLSTATUS_get0)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_cmp_crlstatus_new1(a0: *const APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_CRLSTATUS_new1)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get0_crl_status_list(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ITAV_get0_crlStatusList)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get0_crls(a0: *const APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ITAV_get0_crls)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_new0_crl_status_list(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_new0_crlStatusList)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_new_crls(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_new_crls)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_get1_crl_update(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_get1_crlUpdate)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_new0_cert_req_template(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ITAV_new0_certReqTemplate)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_itav_get1_cert_req_template(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ITAV_get1_certReqTemplate)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_create(a0: *mut APTR, a1: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ATAV_create)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_set0(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR) { ((*IAmiSSL).OSSL_CMP_ATAV_set0)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_get0_type(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ATAV_get0_type)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_get0_value(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ATAV_get0_value)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_new_alg_id(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ATAV_new_algId)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_get0_alg_id(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ATAV_get0_algId)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_new_rsa_key_len(a0: i32) -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ATAV_new_rsaKeyLen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_get_rsa_key_len(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ATAV_get_rsaKeyLen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atav_push1(a0: *mut *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_ATAV_push1)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_get1_cert_req_template(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) -> i32 { ((*IAmiSSL).OSSL_CMP_get1_certReqTemplate)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ossl_cmp_atavs(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CMP_ATAVS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_cmp_atavs(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CMP_ATAVS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atavs_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CMP_ATAVS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atavs_new() -> *mut APTR { ((*IAmiSSL).OSSL_CMP_ATAVS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_cmp_atavs_it() -> *const APTR { ((*IAmiSSL).OSSL_CMP_ATAVS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_attributetypeandvalue_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_ATTRIBUTETYPEANDVALUE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_attributetypeandvalue_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_ATTRIBUTETYPEANDVALUE_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_certtemplate_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_CERTTEMPLATE_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_crypto_atomic_store(a0: *mut APTR, a1: APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_atomic_store)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_crypto_aligned_alloc(a0: u32, a1: u32, a2: *mut *mut (), a3: *const APTR, a4: i32) -> *mut () { ((*IAmiSSL).CRYPTO_aligned_alloc)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509v3_add_extensions(a0: *mut *mut APTR, a1: *const APTR) -> *mut APTR { ((*IAmiSSL).X509v3_add_extensions)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pkcs12_set_pbmac1_pbkdf2(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *mut u8, a4: i32, a5: i32, a6: *const APTR, a7: *const APTR) -> i32 { ((*IAmiSSL).PKCS12_set_pbmac1_pbkdf2)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_pbmac1_get1_pbkdf2_param(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).PBMAC1_get1_pbkdf2_param)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_pbmac1_param(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_PBMAC1PARAM)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_pbmac1_param(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PBMAC1PARAM)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_pbmac1_param_free(a0: *mut APTR) { ((*IAmiSSL).PBMAC1PARAM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_pbmac1_param_new() -> *mut APTR { ((*IAmiSSL).PBMAC1PARAM_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pbmac1_param_it() -> *const APTR { ((*IAmiSSL).PBMAC1PARAM_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_get_conf_diagnostics(a0: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_LIB_CTX_get_conf_diagnostics)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_set_conf_diagnostics(a0: *mut APTR, a1: i32) { ((*IAmiSSL).OSSL_LIB_CTX_set_conf_diagnostics)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_lib_ctx_get_data(a0: *mut APTR, a1: i32) -> *mut () { ((*IAmiSSL).OSSL_LIB_CTX_get_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_general_names_print(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).OSSL_GENERAL_NAMES_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set0_data(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_VERIFY_CTX_set0_data)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set0_imprint(a0: *mut APTR, a1: *mut u8, a2: i32) -> i32 { ((*IAmiSSL).TS_VERIFY_CTX_set0_imprint)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set0_store(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_VERIFY_CTX_set0_store)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ts_verify_ctx_set0_certs(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).TS_VERIFY_CTX_set0_certs)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_crypto_atomic_add64(a0: *mut APTR, a1: APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_atomic_add64)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_atomic_and(a0: *mut APTR, a1: APTR, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).CRYPTO_atomic_and)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_d2i_ossl_attributes_syntax(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ATTRIBUTES_SYNTAX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_attributes_syntax(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ATTRIBUTES_SYNTAX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_attributes_syntax_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ATTRIBUTES_SYNTAX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_attributes_syntax_new() -> *mut APTR { ((*IAmiSSL).OSSL_ATTRIBUTES_SYNTAX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_attributes_syntax_it() -> *const APTR { ((*IAmiSSL).OSSL_ATTRIBUTES_SYNTAX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_user_notice_syntax(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_USER_NOTICE_SYNTAX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_user_notice_syntax(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_USER_NOTICE_SYNTAX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_user_notice_syntax_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_USER_NOTICE_SYNTAX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_user_notice_syntax_new() -> *mut APTR { ((*IAmiSSL).OSSL_USER_NOTICE_SYNTAX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_user_notice_syntax_it() -> *const APTR { ((*IAmiSSL).OSSL_USER_NOTICE_SYNTAX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_indicator_set_callback(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).OSSL_INDICATOR_set_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_indicator_get_callback(a0: *mut APTR, a1: *mut *mut APTR) { ((*IAmiSSL).OSSL_INDICATOR_get_callback)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_strtoul(a0: *const APTR, a1: *mut *mut APTR, a2: i32, a3: *mut u32) -> i32 { ((*IAmiSSL).OPENSSL_strtoul)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_d2i_ossl_basic_attr_constraints(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_BASIC_ATTR_CONSTRAINTS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_basic_attr_constraints(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_BASIC_ATTR_CONSTRAINTS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_basic_attr_constraints_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_BASIC_ATTR_CONSTRAINTS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_basic_attr_constraints_new() -> *mut APTR { ((*IAmiSSL).OSSL_BASIC_ATTR_CONSTRAINTS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_basic_attr_constraints_it() -> *const APTR { ((*IAmiSSL).OSSL_BASIC_ATTR_CONSTRAINTS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_keymgmt_gen_gettable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_KEYMGMT_gen_gettable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_signature(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_signature)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_sign_init_ex2(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_sign_init_ex2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_sign_message_init(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_sign_message_init)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_sign_message_update(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_sign_message_update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_sign_message_final(a0: *mut APTR, a1: *mut u8, a2: *mut u32) -> i32 { ((*IAmiSSL).EVP_PKEY_sign_message_final)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_init_ex2(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_init_ex2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_message_init(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_message_init)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_message_update(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_message_update)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_message_final(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_message_final)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_pkey_verify_recover_init_ex2(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_verify_recover_init_ex2)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_md_xof(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_xof)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_md_ctx_get_size_ex(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_MD_CTX_get_size_ex)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_set_algor_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_set_algor_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_algor_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_algor_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_ctx_get_algor(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_CIPHER_CTX_get_algor)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_set_algor_params(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_set_algor_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_algor_params(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_algor_params)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_pkey_ctx_get_algor(a0: *mut APTR, a1: *mut *mut APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_CTX_get_algor)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_x509_acert(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_X509_ACERT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_x509_acert(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_X509_ACERT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_free(a0: *mut APTR) { ((*IAmiSSL).X509_ACERT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_new() -> *mut APTR { ((*IAmiSSL).X509_ACERT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_acert_it() -> *const APTR { ((*IAmiSSL).X509_ACERT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_acert_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).X509_ACERT_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_info_it() -> *const APTR { ((*IAmiSSL).X509_ACERT_INFO_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_acert_info_free(a0: *mut APTR) { ((*IAmiSSL).X509_ACERT_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_info_new() -> *mut APTR { ((*IAmiSSL).X509_ACERT_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_object_digest_info_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_OBJECT_DIGEST_INFO_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_object_digest_info_new() -> *mut APTR { ((*IAmiSSL).OSSL_OBJECT_DIGEST_INFO_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_issuer_serial_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ISSUER_SERIAL_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_issuer_serial_new() -> *mut APTR { ((*IAmiSSL).OSSL_ISSUER_SERIAL_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_x509_acert_issuer_v2_form_free(a0: *mut APTR) { ((*IAmiSSL).X509_ACERT_ISSUER_V2FORM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_issuer_v2_form_new() -> *mut APTR { ((*IAmiSSL).X509_ACERT_ISSUER_V2FORM_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pem_read_bio_x509_acert(a0: *mut APTR, a1: *mut *mut APTR, a2: *mut APTR, a3: *mut ()) -> *mut APTR { ((*IAmiSSL).PEM_read_bio_X509_ACERT)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_pem_write_bio_x509_acert(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).PEM_write_bio_X509_ACERT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_d2i_x509_acert_bio(a0: *mut APTR, a1: *mut *mut APTR) -> *mut APTR { ((*IAmiSSL).d2i_X509_ACERT_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_x509_acert_bio(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).i2d_X509_ACERT_bio)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_holder_entity_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_holder_entityName)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_holder_base_cert_id(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_holder_baseCertId)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_holder_digest(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_holder_digest)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_issuer_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_issuerName)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get_version(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_ACERT_get_version)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_signature(a0: *const APTR, a1: *mut *mut APTR, a2: *mut *mut APTR) { ((*IAmiSSL).X509_ACERT_get0_signature)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_acert_get_signature_nid(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_ACERT_get_signature_nid)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_info_sigalg(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_info_sigalg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_serial_number(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_serialNumber)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_not_before(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_notBefore)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_not_after(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_notAfter)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_issuer_uid(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_ACERT_get0_issuerUID)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_set_version(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).X509_ACERT_set_version)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_set0_holder_entity_name(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_ACERT_set0_holder_entityName)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_set0_holder_base_cert_id(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_ACERT_set0_holder_baseCertId)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_set0_holder_digest(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_ACERT_set0_holder_digest)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_set1_issuer_name(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_ACERT_set1_issuerName)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_set1_serial_number(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_ACERT_set1_serialNumber)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_set1_not_before(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_ACERT_set1_notBefore)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_set1_not_after(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).X509_ACERT_set1_notAfter)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_object_digest_info_get0_digest(a0: *const APTR, a1: *mut i32, a2: *mut *mut APTR, a3: *mut *mut APTR) { ((*IAmiSSL).OSSL_OBJECT_DIGEST_INFO_get0_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_object_digest_info_set1_digest(a0: *mut APTR, a1: i32, a2: *mut APTR, a3: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_OBJECT_DIGEST_INFO_set1_digest)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_issuer_serial_get0_issuer(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_ISSUER_SERIAL_get0_issuer)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_issuer_serial_get0_serial(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_ISSUER_SERIAL_get0_serial)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_issuer_serial_get0_issuer_uid(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_ISSUER_SERIAL_get0_issuerUID)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_issuer_serial_set1_issuer(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ISSUER_SERIAL_set1_issuer)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_issuer_serial_set1_serial(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ISSUER_SERIAL_set1_serial)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_issuer_serial_set1_issuer_uid(a0: *mut APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_ISSUER_SERIAL_set1_issuerUID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_print(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_ACERT_print)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_print_ex(a0: *mut APTR, a1: *mut APTR, a2: u32, a3: u32) -> i32 { ((*IAmiSSL).X509_ACERT_print_ex)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_acert_get_attr_count(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_ACERT_get_attr_count)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_acert_get_attr_by_nid(a0: *const APTR, a1: i32, a2: i32) -> i32 { ((*IAmiSSL).X509_ACERT_get_attr_by_NID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_acert_get_attr_by_obj(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).X509_ACERT_get_attr_by_OBJ)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_acert_get_attr(a0: *const APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_ACERT_get_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_delete_attr(a0: *mut APTR, a1: i32) -> *mut APTR { ((*IAmiSSL).X509_ACERT_delete_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_add1_attr(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_ACERT_add1_attr)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_add1_attr_by_obj(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).X509_ACERT_add1_attr_by_OBJ)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_acert_add1_attr_by_nid(a0: *mut APTR, a1: i32, a2: i32, a3: *const (), a4: i32) -> i32 { ((*IAmiSSL).X509_ACERT_add1_attr_by_NID)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_acert_add1_attr_by_txt(a0: *mut APTR, a1: *const APTR, a2: i32, a3: *const u8, a4: i32) -> i32 { ((*IAmiSSL).X509_ACERT_add1_attr_by_txt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_acert_sign(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).X509_ACERT_sign)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_acert_sign_ctx(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_ACERT_sign_ctx)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_verify(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).X509_ACERT_verify)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_acert_get_ext_d2i(a0: *const APTR, a1: i32, a2: *mut i32, a3: *mut i32) -> *mut () { ((*IAmiSSL).X509_ACERT_get_ext_d2i)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_x509_acert_add1_ext_i2d(a0: *mut APTR, a1: i32, a2: *mut (), a3: i32, a4: u32) -> i32 { ((*IAmiSSL).X509_ACERT_add1_ext_i2d)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_x509_acert_get0_extensions(a0: *const APTR) -> *const stack_st_X509_EXTENSION { ((*IAmiSSL).X509_ACERT_get0_extensions)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_value_it() -> *const APTR { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_VALUE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_value_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_VALUE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_value_new() -> *mut APTR { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_VALUE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_ietf_attr_syntax(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_IETF_ATTR_SYNTAX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_ietf_attr_syntax(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_IETF_ATTR_SYNTAX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_new() -> *mut APTR { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_it() -> *const APTR { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_get0_policy_authority(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_get0_policyAuthority)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_set0_policy_authority(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_set0_policyAuthority)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_get_value_num(a0: *const APTR) -> i32 { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_get_value_num)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_get0_value(a0: *const APTR, a1: i32, a2: *mut i32) -> *mut () { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_get0_value)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_add1_value(a0: *mut APTR, a1: i32, a2: *mut ()) -> i32 { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_add1_value)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_ietf_attr_syntax_print(a0: *mut APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).OSSL_IETF_ATTR_SYNTAX_print)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_acert_add_attr_nconf(a0: *mut APTR, a1: *const APTR, a2: *mut APTR) -> i32 { ((*IAmiSSL).X509_ACERT_add_attr_nconf)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ossl_target(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TARGET)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_target(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TARGET)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_target_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TARGET_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_target_new() -> *mut APTR { ((*IAmiSSL).OSSL_TARGET_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_target_it() -> *const APTR { ((*IAmiSSL).OSSL_TARGET_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_targets(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TARGETS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_targets(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TARGETS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_targets_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TARGETS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_targets_new() -> *mut APTR { ((*IAmiSSL).OSSL_TARGETS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_targets_it() -> *const APTR { ((*IAmiSSL).OSSL_TARGETS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_targeting_information(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TARGETING_INFORMATION)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_targeting_information(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TARGETING_INFORMATION)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_targeting_information_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TARGETING_INFORMATION_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_targeting_information_new() -> *mut APTR { ((*IAmiSSL).OSSL_TARGETING_INFORMATION_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_targeting_information_it() -> *const APTR { ((*IAmiSSL).OSSL_TARGETING_INFORMATION_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_set_quic_tls_cbs(a0: *mut APTR, a1: *const APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).SSL_set_quic_tls_cbs)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_quic_tls_transport_params(a0: *mut APTR, a1: *const u8, a2: u32) -> i32 { ((*IAmiSSL).SSL_set_quic_tls_transport_params)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ssl_set_quic_tls_early_data_enabled(a0: *mut APTR, a1: i32) -> i32 { ((*IAmiSSL).SSL_set_quic_tls_early_data_enabled)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_quic_server_method() -> *const APTR { ((*IAmiSSL).OSSL_QUIC_server_method)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ssl_is_listener(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_is_listener)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_listener(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get0_listener)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_new_listener(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).SSL_new_listener)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_accept_connection(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).SSL_accept_connection)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_accept_connection_queue_len(a0: *mut APTR) -> u32 { ((*IAmiSSL).SSL_get_accept_connection_queue_len)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_listen(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_listen)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_new_from_listener(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).SSL_new_from_listener)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_new_listener_from(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).SSL_new_listener_from)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_is_domain(a0: *mut APTR) -> i32 { ((*IAmiSSL).SSL_is_domain)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_get0_domain(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).SSL_get0_domain)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ssl_new_domain(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).SSL_new_domain)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_domain_flags(a0: *mut APTR, a1: APTR) -> i32 { ((*IAmiSSL).SSL_CTX_set_domain_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_get_domain_flags(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_CTX_get_domain_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_get_domain_flags(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).SSL_get_domain_flags)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ssl_ctx_set_new_pending_conn_cb(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).SSL_CTX_set_new_pending_conn_cb)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_x509_verify_param_get_purpose(a0: *const APTR) -> i32 { ((*IAmiSSL).X509_VERIFY_PARAM_get_purpose)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ossl_crmf_encryptedkey(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_CRMF_ENCRYPTEDKEY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_crmf_encryptedkey(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_CRMF_ENCRYPTEDKEY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedkey_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDKEY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedkey_new() -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDKEY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedkey_it() -> *const APTR { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDKEY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedkey_get1_enc_cert(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR, a4: APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDKEY_get1_encCert)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedvalue_decrypt(a0: *const APTR, a1: *mut APTR, a2: *const APTR, a3: *mut APTR, a4: *mut i32) -> *mut u8 { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDVALUE_decrypt)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedkey_get1_pkey(a0: *const APTR, a1: *mut APTR, a2: *mut APTR, a3: *mut APTR, a4: *mut APTR, a5: *mut APTR, a6: *mut APTR, a7: *const APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDKEY_get1_pkey)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7) }
#[inline]
pub unsafe fn amissl_ossl_crmf_msg_centralkeygen_requested(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).OSSL_CRMF_MSG_centralkeygen_requested)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_enveloped_data_dup(a0: *const APTR) -> *mut APTR { ((*IAmiSSL).CMS_EnvelopedData_dup)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_crmf_encryptedkey_init_envdata(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).OSSL_CRMF_ENCRYPTEDKEY_init_envdata)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_get1_default_properties(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).EVP_get1_default_properties)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_rand_set1_random_provider(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).RAND_set1_random_provider)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_x509_purpose_get_unused_id(a0: *mut APTR) -> i32 { ((*IAmiSSL).X509_PURPOSE_get_unused_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_d2i_ossl_authority_attribute_id_syntax(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_AUTHORITY_ATTRIBUTE_ID_SYNTAX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_authority_attribute_id_syntax(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_AUTHORITY_ATTRIBUTE_ID_SYNTAX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_authority_attribute_id_syntax_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_AUTHORITY_ATTRIBUTE_ID_SYNTAX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_authority_attribute_id_syntax_new() -> *mut APTR { ((*IAmiSSL).OSSL_AUTHORITY_ATTRIBUTE_ID_SYNTAX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_authority_attribute_id_syntax_it() -> *const APTR { ((*IAmiSSL).OSSL_AUTHORITY_ATTRIBUTE_ID_SYNTAX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_role_spec_cert_id(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ROLE_SPEC_CERT_ID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_role_spec_cert_id(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ROLE_SPEC_CERT_ID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_role_spec_cert_id_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ROLE_SPEC_CERT_ID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_role_spec_cert_id_new() -> *mut APTR { ((*IAmiSSL).OSSL_ROLE_SPEC_CERT_ID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_role_spec_cert_id_it() -> *const APTR { ((*IAmiSSL).OSSL_ROLE_SPEC_CERT_ID_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_role_spec_cert_id_syntax(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ROLE_SPEC_CERT_ID_SYNTAX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_role_spec_cert_id_syntax(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ROLE_SPEC_CERT_ID_SYNTAX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_role_spec_cert_id_syntax_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ROLE_SPEC_CERT_ID_SYNTAX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_role_spec_cert_id_syntax_new() -> *mut APTR { ((*IAmiSSL).OSSL_ROLE_SPEC_CERT_ID_SYNTAX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_role_spec_cert_id_syntax_it() -> *const APTR { ((*IAmiSSL).OSSL_ROLE_SPEC_CERT_ID_SYNTAX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_attribute_descriptor(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ATTRIBUTE_DESCRIPTOR)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_attribute_descriptor(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ATTRIBUTE_DESCRIPTOR)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_attribute_descriptor_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ATTRIBUTE_DESCRIPTOR_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_attribute_descriptor_new() -> *mut APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_DESCRIPTOR_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_attribute_descriptor_it() -> *const APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_DESCRIPTOR_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_hash_it() -> *const APTR { ((*IAmiSSL).OSSL_HASH_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_info_syntax_it() -> *const APTR { ((*IAmiSSL).OSSL_INFO_SYNTAX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_info_syntax_pointer_it() -> *const APTR { ((*IAmiSSL).OSSL_INFO_SYNTAX_POINTER_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_privilege_policy_id_it() -> *const APTR { ((*IAmiSSL).OSSL_PRIVILEGE_POLICY_ID_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_hash(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_HASH)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_hash(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_HASH)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_hash_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_HASH_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_hash_new() -> *mut APTR { ((*IAmiSSL).OSSL_HASH_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_info_syntax(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_INFO_SYNTAX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_info_syntax(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_INFO_SYNTAX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_info_syntax_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_INFO_SYNTAX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_info_syntax_new() -> *mut APTR { ((*IAmiSSL).OSSL_INFO_SYNTAX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_info_syntax_pointer(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_INFO_SYNTAX_POINTER)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_info_syntax_pointer(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_INFO_SYNTAX_POINTER)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_info_syntax_pointer_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_INFO_SYNTAX_POINTER_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_info_syntax_pointer_new() -> *mut APTR { ((*IAmiSSL).OSSL_INFO_SYNTAX_POINTER_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_privilege_policy_id(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_PRIVILEGE_POLICY_ID)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_privilege_policy_id(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_PRIVILEGE_POLICY_ID)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_privilege_policy_id_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_PRIVILEGE_POLICY_ID_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_privilege_policy_id_new() -> *mut APTR { ((*IAmiSSL).OSSL_PRIVILEGE_POLICY_ID_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_param_print_to_bio(a0: *const APTR, a1: *mut APTR, a2: i32) -> i32 { ((*IAmiSSL).OSSL_PARAM_print_to_bio)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ossl_day_time(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_DAY_TIME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_day_time(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_DAY_TIME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_day_time_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_DAY_TIME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_day_time_new() -> *mut APTR { ((*IAmiSSL).OSSL_DAY_TIME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_day_time_it() -> *const APTR { ((*IAmiSSL).OSSL_DAY_TIME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_day_time_band(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_DAY_TIME_BAND)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_day_time_band(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_DAY_TIME_BAND)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_day_time_band_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_DAY_TIME_BAND_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_day_time_band_new() -> *mut APTR { ((*IAmiSSL).OSSL_DAY_TIME_BAND_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_day_time_band_it() -> *const APTR { ((*IAmiSSL).OSSL_DAY_TIME_BAND_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_time_spec_day(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TIME_SPEC_DAY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_time_spec_day(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TIME_SPEC_DAY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_day_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TIME_SPEC_DAY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_day_new() -> *mut APTR { ((*IAmiSSL).OSSL_TIME_SPEC_DAY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_day_it() -> *const APTR { ((*IAmiSSL).OSSL_TIME_SPEC_DAY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_time_spec_weeks(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TIME_SPEC_WEEKS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_time_spec_weeks(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TIME_SPEC_WEEKS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_weeks_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TIME_SPEC_WEEKS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_weeks_new() -> *mut APTR { ((*IAmiSSL).OSSL_TIME_SPEC_WEEKS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_weeks_it() -> *const APTR { ((*IAmiSSL).OSSL_TIME_SPEC_WEEKS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_time_spec_month(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TIME_SPEC_MONTH)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_time_spec_month(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TIME_SPEC_MONTH)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_month_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TIME_SPEC_MONTH_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_month_new() -> *mut APTR { ((*IAmiSSL).OSSL_TIME_SPEC_MONTH_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_month_it() -> *const APTR { ((*IAmiSSL).OSSL_TIME_SPEC_MONTH_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_named_day(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_NAMED_DAY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_named_day(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_NAMED_DAY)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_named_day_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_NAMED_DAY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_named_day_new() -> *mut APTR { ((*IAmiSSL).OSSL_NAMED_DAY_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_named_day_it() -> *const APTR { ((*IAmiSSL).OSSL_NAMED_DAY_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_time_spec_x_day_of(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TIME_SPEC_X_DAY_OF)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_time_spec_x_day_of(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TIME_SPEC_X_DAY_OF)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_x_day_of_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TIME_SPEC_X_DAY_OF_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_x_day_of_new() -> *mut APTR { ((*IAmiSSL).OSSL_TIME_SPEC_X_DAY_OF_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_x_day_of_it() -> *const APTR { ((*IAmiSSL).OSSL_TIME_SPEC_X_DAY_OF_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_time_spec_absolute(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TIME_SPEC_ABSOLUTE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_time_spec_absolute(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TIME_SPEC_ABSOLUTE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_absolute_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TIME_SPEC_ABSOLUTE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_absolute_new() -> *mut APTR { ((*IAmiSSL).OSSL_TIME_SPEC_ABSOLUTE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_absolute_it() -> *const APTR { ((*IAmiSSL).OSSL_TIME_SPEC_ABSOLUTE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_time_spec_time(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TIME_SPEC_TIME)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_time_spec_time(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TIME_SPEC_TIME)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_time_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TIME_SPEC_TIME_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_time_new() -> *mut APTR { ((*IAmiSSL).OSSL_TIME_SPEC_TIME_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_time_it() -> *const APTR { ((*IAmiSSL).OSSL_TIME_SPEC_TIME_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_time_spec(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TIME_SPEC)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_time_spec(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TIME_SPEC)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TIME_SPEC_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_new() -> *mut APTR { ((*IAmiSSL).OSSL_TIME_SPEC_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_time_spec_it() -> *const APTR { ((*IAmiSSL).OSSL_TIME_SPEC_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_time_period(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_TIME_PERIOD)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_time_period(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_TIME_PERIOD)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_time_period_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_TIME_PERIOD_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_time_period_new() -> *mut APTR { ((*IAmiSSL).OSSL_TIME_PERIOD_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_time_period_it() -> *const APTR { ((*IAmiSSL).OSSL_TIME_PERIOD_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cipher_can_pipeline(a0: *const APTR, a1: i32) -> i32 { ((*IAmiSSL).EVP_CIPHER_can_pipeline)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_cipher_pipeline_encrypt_init(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: u32, a4: u32, a5: *mut *mut APTR, a6: u32) -> i32 { ((*IAmiSSL).EVP_CipherPipelineEncryptInit)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_cipher_pipeline_decrypt_init(a0: *mut APTR, a1: *const APTR, a2: *const u8, a3: u32, a4: u32, a5: *mut *mut APTR, a6: u32) -> i32 { ((*IAmiSSL).EVP_CipherPipelineDecryptInit)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_cipher_pipeline_update(a0: *mut APTR, a1: *mut *mut u8, a2: *mut u32, a3: *const u32, a4: *mut *mut APTR, a5: *const u32) -> i32 { ((*IAmiSSL).EVP_CipherPipelineUpdate)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_cipher_pipeline_final(a0: *mut APTR, a1: *mut *mut u8, a2: *mut u32, a3: *const u32) -> i32 { ((*IAmiSSL).EVP_CipherPipelineFinal)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_d2i_ossl_attribute_type_mapping(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ATTRIBUTE_TYPE_MAPPING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_attribute_type_mapping(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ATTRIBUTE_TYPE_MAPPING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_attribute_type_mapping_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ATTRIBUTE_TYPE_MAPPING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_attribute_type_mapping_new() -> *mut APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_TYPE_MAPPING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_attribute_type_mapping_it() -> *const APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_TYPE_MAPPING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_attribute_value_mapping(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ATTRIBUTE_VALUE_MAPPING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_attribute_value_mapping(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ATTRIBUTE_VALUE_MAPPING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_attribute_value_mapping_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ATTRIBUTE_VALUE_MAPPING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_attribute_value_mapping_new() -> *mut APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_VALUE_MAPPING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_attribute_value_mapping_it() -> *const APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_VALUE_MAPPING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_attribute_mapping(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ATTRIBUTE_MAPPING)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_attribute_mapping(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ATTRIBUTE_MAPPING)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_attribute_mapping_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ATTRIBUTE_MAPPING_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_attribute_mapping_new() -> *mut APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_MAPPING_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_attribute_mapping_it() -> *const APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_MAPPING_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_attribute_mappings(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ATTRIBUTE_MAPPINGS)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_attribute_mappings(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ATTRIBUTE_MAPPINGS)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_attribute_mappings_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ATTRIBUTE_MAPPINGS_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_attribute_mappings_new() -> *mut APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_MAPPINGS_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_attribute_mappings_it() -> *const APTR { ((*IAmiSSL).OSSL_ATTRIBUTE_MAPPINGS_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_atav(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ATAV)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_atav(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ATAV)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_atav_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ATAV_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_atav_new() -> *mut APTR { ((*IAmiSSL).OSSL_ATAV_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_atav_it() -> *const APTR { ((*IAmiSSL).OSSL_ATAV_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_allowed_attributes_choice(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ALLOWED_ATTRIBUTES_CHOICE)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_allowed_attributes_choice(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ALLOWED_ATTRIBUTES_CHOICE)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_choice_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_CHOICE_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_choice_new() -> *mut APTR { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_CHOICE_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_choice_it() -> *const APTR { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_CHOICE_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_allowed_attributes_item(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ALLOWED_ATTRIBUTES_ITEM)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_allowed_attributes_item(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ALLOWED_ATTRIBUTES_ITEM)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_item_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_ITEM_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_item_new() -> *mut APTR { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_ITEM_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_item_it() -> *const APTR { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_ITEM_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_d2i_ossl_allowed_attributes_syntax(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_ALLOWED_ATTRIBUTES_SYNTAX)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_allowed_attributes_syntax(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_ALLOWED_ATTRIBUTES_SYNTAX)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_syntax_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_SYNTAX_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_syntax_new() -> *mut APTR { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_SYNTAX_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_allowed_attributes_syntax_it() -> *const APTR { ((*IAmiSSL).OSSL_ALLOWED_ATTRIBUTES_SYNTAX_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_evp_cipher_init_skey(a0: *mut APTR, a1: *const APTR, a2: *mut APTR, a3: *const u8, a4: u32, a5: i32, a6: *const APTR) -> i32 { ((*IAmiSSL).EVP_CipherInit_SKEY)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6) }
#[inline]
pub unsafe fn amissl_evp_skey_import(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: i32, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_SKEY_import)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_skey_generate(a0: *mut APTR, a1: *const APTR, a2: *const APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_SKEY_generate)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_skey_import_raw_key(a0: *mut APTR, a1: *const APTR, a2: *mut u8, a3: u32, a4: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_SKEY_import_raw_key)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_evp_skey_get0_raw_key(a0: *const APTR, a1: *mut *mut APTR, a2: *mut u32) -> i32 { ((*IAmiSSL).EVP_SKEY_get0_raw_key)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_skey_export(a0: *const APTR, a1: i32, a2: *mut APTR, a3: *mut ()) -> i32 { ((*IAmiSSL).EVP_SKEY_export)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_evp_skey_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_SKEY_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skey_free(a0: *mut APTR) { ((*IAmiSSL).EVP_SKEY_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_fetch(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_SKEYMGMT_fetch)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_up_ref(a0: *mut APTR) -> i32 { ((*IAmiSSL).EVP_SKEYMGMT_up_ref)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_free(a0: *mut APTR) { ((*IAmiSSL).EVP_SKEYMGMT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_get0_provider(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SKEYMGMT_get0_provider)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_get0_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SKEYMGMT_get0_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_get0_description(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SKEYMGMT_get0_description)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_SKEYMGMT_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_do_all_provided(a0: *mut APTR, a1: APTR, a2: *mut ()) { ((*IAmiSSL).EVP_SKEYMGMT_do_all_provided)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_names_do_all(a0: *const APTR, a1: APTR, a2: *mut ()) -> i32 { ((*IAmiSSL).EVP_SKEYMGMT_names_do_all)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_mac_init_skey(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_MAC_init_SKEY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_skey_get0_key_id(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SKEY_get0_key_id)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skey_get0_skeymgmt_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SKEY_get0_skeymgmt_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skey_get0_provider_name(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SKEY_get0_provider_name)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_get0_gen_settable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SKEYMGMT_get0_gen_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skeymgmt_get0_imp_settable_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).EVP_SKEYMGMT_get0_imp_settable_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_evp_skey_is_a(a0: *const APTR, a1: *const APTR) -> i32 { ((*IAmiSSL).EVP_SKEY_is_a)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_evp_skey_to_provider(a0: *mut APTR, a1: *mut APTR, a2: *mut APTR, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_SKEY_to_provider)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_ossl_provider_add_conf_parameter(a0: *mut APTR, a1: *const APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_add_conf_parameter)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_provider_get_conf_parameters(a0: *const APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_get_conf_parameters)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_provider_conf_get_bool(a0: *const APTR, a1: *const APTR, a2: i32) -> i32 { ((*IAmiSSL).OSSL_PROVIDER_conf_get_bool)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_d2i_ossl_aa_dist_point(a0: *mut *mut APTR, a1: *mut *mut APTR, a2: i32) -> *mut APTR { ((*IAmiSSL).d2i_OSSL_AA_DIST_POINT)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_i2d_ossl_aa_dist_point(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_OSSL_AA_DIST_POINT)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_aa_dist_point_free(a0: *mut APTR) { ((*IAmiSSL).OSSL_AA_DIST_POINT_free)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_ossl_aa_dist_point_new() -> *mut APTR { ((*IAmiSSL).OSSL_AA_DIST_POINT_new)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_ossl_aa_dist_point_it() -> *const APTR { ((*IAmiSSL).OSSL_AA_DIST_POINT_it)(IAmiSSL) }
#[inline]
pub unsafe fn amissl_pem_asn1_write_bio_ctx(a0: *mut APTR, a1: *mut (), a2: *const APTR, a3: *mut APTR, a4: *const (), a5: *const APTR, a6: *const u8, a7: i32, a8: *mut APTR, a9: *mut ()) -> i32 { ((*IAmiSSL).PEM_ASN1_write_bio_ctx)(IAmiSSL, a0, a1, a2, a3, a4, a5, a6, a7, a8, a9) }
#[inline]
pub unsafe fn amissl_evp_pkey_get_security_category(a0: *const APTR) -> i32 { ((*IAmiSSL).EVP_PKEY_get_security_category)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_store_ctx_set_ocsp_resp(a0: *mut APTR, a1: *mut APTR) { ((*IAmiSSL).X509_STORE_CTX_set_ocsp_resp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_openssl_sk_set_thunks(a0: *mut APTR, a1: APTR) -> *mut APTR { ((*IAmiSSL).OPENSSL_sk_set_thunks)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_i2d_pkcs8_private_key(a0: *const APTR, a1: *mut *mut u8) -> i32 { ((*IAmiSSL).i2d_PKCS8PrivateKey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_ossl_param_set_octet_string_or_ptr(a0: *mut APTR, a1: *const (), a2: u32) -> i32 { ((*IAmiSSL).OSSL_PARAM_set_octet_string_or_ptr)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_ossl_store_loader_settable_ctx_params(a0: *const APTR) -> *const APTR { ((*IAmiSSL).OSSL_STORE_LOADER_settable_ctx_params)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_x509_crl_get0_tbs_sigalg(a0: *const APTR) -> *const APTR { ((*IAmiSSL).X509_CRL_get0_tbs_sigalg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kemri_cert_cmp(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kemri_cert_cmp)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kemri_set0_pkey(a0: *mut APTR, a1: *mut APTR) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kemri_set0_pkey)(IAmiSSL, a0, a1) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kemri_get0_ctx(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_RecipientInfo_kemri_get0_ctx)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kemri_get0_kdf_alg(a0: *mut APTR) -> *mut APTR { ((*IAmiSSL).CMS_RecipientInfo_kemri_get0_kdf_alg)(IAmiSSL, a0) }
#[inline]
pub unsafe fn amissl_cms_recipient_info_kemri_set_ukm(a0: *mut APTR, a1: *const u8, a2: i32) -> i32 { ((*IAmiSSL).CMS_RecipientInfo_kemri_set_ukm)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_pkey_derive_skey(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: u32, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_PKEY_derive_SKEY)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_kdf_ctx_set_skey(a0: *mut APTR, a1: *mut APTR, a2: *const APTR) -> i32 { ((*IAmiSSL).EVP_KDF_CTX_set_SKEY)(IAmiSSL, a0, a1, a2) }
#[inline]
pub unsafe fn amissl_evp_kdf_derive_skey(a0: *mut APTR, a1: *mut APTR, a2: *const APTR, a3: *const APTR, a4: u32, a5: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_KDF_derive_SKEY)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_evp_skey_import_skeymgmt(a0: *mut APTR, a1: *mut APTR, a2: i32, a3: *const APTR) -> *mut APTR { ((*IAmiSSL).EVP_SKEY_import_SKEYMGMT)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_malloc_array(a0: u32, a1: u32, a2: *const APTR, a3: i32) -> *mut () { ((*IAmiSSL).CRYPTO_malloc_array)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_calloc(a0: u32, a1: u32, a2: *const APTR, a3: i32) -> *mut () { ((*IAmiSSL).CRYPTO_calloc)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_aligned_alloc_array(a0: u32, a1: u32, a2: u32, a3: *mut *mut (), a4: *const APTR, a5: i32) -> *mut () { ((*IAmiSSL).CRYPTO_aligned_alloc_array)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_realloc_array(a0: *mut (), a1: u32, a2: u32, a3: *const APTR, a4: i32) -> *mut () { ((*IAmiSSL).CRYPTO_realloc_array)(IAmiSSL, a0, a1, a2, a3, a4) }
#[inline]
pub unsafe fn amissl_crypto_clear_realloc_array(a0: *mut (), a1: u32, a2: u32, a3: u32, a4: *const APTR, a5: i32) -> *mut () { ((*IAmiSSL).CRYPTO_clear_realloc_array)(IAmiSSL, a0, a1, a2, a3, a4, a5) }
#[inline]
pub unsafe fn amissl_crypto_secure_malloc_array(a0: u32, a1: u32, a2: *const APTR, a3: i32) -> *mut () { ((*IAmiSSL).CRYPTO_secure_malloc_array)(IAmiSSL, a0, a1, a2, a3) }
#[inline]
pub unsafe fn amissl_crypto_secure_calloc(a0: u32, a1: u32, a2: *const APTR, a3: i32) -> *mut () { ((*IAmiSSL).CRYPTO_secure_calloc)(IAmiSSL, a0, a1, a2, a3) }
