//! IExpat global(s) and convenience wrappers for expat.library.

use crate::types::*;
use crate::interfaces::expat::*;

#[cfg(target_arch = "powerpc")]
extern "C" { pub static IExpat: *mut ExpatIFace; }
#[cfg(not(target_arch = "powerpc"))]
#[allow(non_upper_case_globals)]
pub static mut IExpat: *mut ExpatIFace = core::ptr::null_mut();

#[inline]
pub unsafe fn expat_xml_parser_create(enc: *const APTR) -> APTR { ((*IExpat).XML_ParserCreate)(IExpat, enc) }
#[inline]
pub unsafe fn expat_xml_parser_create_ns(enc: *const APTR, sep: APTR) -> APTR { ((*IExpat).XML_ParserCreateNS)(IExpat, enc, sep) }
#[inline]
pub unsafe fn expat_xml_parser_create_mm(enc: *const APTR, ms: *const APTR, sep: *const APTR) -> APTR {
    ((*IExpat).XML_ParserCreate_MM)(IExpat, enc, ms, sep)
}
#[inline]
pub unsafe fn expat_xml_external_entity_parser_create(p: APTR, ctx: *const APTR, enc: *const APTR) -> APTR {
    ((*IExpat).XML_ExternalEntityParserCreate)(IExpat, p, ctx, enc)
}
#[inline]
pub unsafe fn expat_xml_parser_free(p: APTR) { ((*IExpat).XML_ParserFree)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_parse(p: APTR, buf: *const APTR, len: i32, fin: i32) -> APTR { ((*IExpat).XML_Parse)(IExpat, p, buf, len, fin) }
#[inline]
pub unsafe fn expat_xml_parse_buffer(p: APTR, len: i32, fin: i32) -> APTR { ((*IExpat).XML_ParseBuffer)(IExpat, p, len, fin) }
#[inline]
pub unsafe fn expat_xml_get_buffer(p: APTR, len: i32) -> *mut () { ((*IExpat).XML_GetBuffer)(IExpat, p, len) }
#[inline]
pub unsafe fn expat_xml_set_start_element_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetStartElementHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_end_element_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetEndElementHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_element_handler(p: APTR, s: APTR, e: APTR) { ((*IExpat).XML_SetElementHandler)(IExpat, p, s, e) }
#[inline]
pub unsafe fn expat_xml_set_character_data_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetCharacterDataHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_processing_instruction_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetProcessingInstructionHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_comment_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetCommentHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_start_cdata_section_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetStartCdataSectionHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_end_cdata_section_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetEndCdataSectionHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_cdata_section_handler(p: APTR, s: APTR, e: APTR) { ((*IExpat).XML_SetCdataSectionHandler)(IExpat, p, s, e) }
#[inline]
pub unsafe fn expat_xml_set_default_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetDefaultHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_default_handler_expand(p: APTR, h: APTR) { ((*IExpat).XML_SetDefaultHandlerExpand)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_external_entity_ref_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetExternalEntityRefHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_external_entity_ref_handler_arg(p: APTR, arg: *mut ()) { ((*IExpat).XML_SetExternalEntityRefHandlerArg)(IExpat, p, arg) }
#[inline]
pub unsafe fn expat_xml_set_unknown_encoding_handler(p: APTR, h: APTR, data: *mut ()) { ((*IExpat).XML_SetUnknownEncodingHandler)(IExpat, p, h, data) }
#[inline]
pub unsafe fn expat_xml_set_start_namespace_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetStartNamespaceDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_end_namespace_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetEndNamespaceDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_namespace_decl_handler(p: APTR, s: APTR, e: APTR) { ((*IExpat).XML_SetNamespaceDeclHandler)(IExpat, p, s, e) }
#[inline]
pub unsafe fn expat_xml_set_xml_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetXmlDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_start_doctype_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetStartDoctypeDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_end_doctype_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetEndDoctypeDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_doctype_decl_handler(p: APTR, s: APTR, e: APTR) { ((*IExpat).XML_SetDoctypeDeclHandler)(IExpat, p, s, e) }
#[inline]
pub unsafe fn expat_xml_set_element_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetElementDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_attlist_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetAttlistDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_entity_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetEntityDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_unparsed_entity_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetUnparsedEntityDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_notation_decl_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetNotationDeclHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_set_not_standalone_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetNotStandaloneHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_get_error_code(p: APTR) -> APTR { ((*IExpat).XML_GetErrorCode)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_error_string(code: APTR) -> *const APTR { ((*IExpat).XML_ErrorString)(IExpat, code) }
#[inline]
pub unsafe fn expat_xml_get_current_byte_index(p: APTR) -> i32 { ((*IExpat).XML_GetCurrentByteIndex)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_get_current_line_number(p: APTR) -> i32 { ((*IExpat).XML_GetCurrentLineNumber)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_get_current_column_number(p: APTR) -> i32 { ((*IExpat).XML_GetCurrentColumnNumber)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_get_current_byte_count(p: APTR) -> i32 { ((*IExpat).XML_GetCurrentByteCount)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_get_input_context(p: APTR, off: *mut i32, size: *mut i32) -> *const APTR { ((*IExpat).XML_GetInputContext)(IExpat, p, off, size) }
#[inline]
pub unsafe fn expat_xml_set_user_data(p: APTR, data: *mut ()) { ((*IExpat).XML_SetUserData)(IExpat, p, data) }
#[inline]
pub unsafe fn expat_xml_default_current(p: APTR) { ((*IExpat).XML_DefaultCurrent)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_use_parser_as_handler_arg(p: APTR) { ((*IExpat).XML_UseParserAsHandlerArg)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_set_base(p: APTR, base: *const APTR) -> APTR { ((*IExpat).XML_SetBase)(IExpat, p, base) }
#[inline]
pub unsafe fn expat_xml_get_base(p: APTR) -> *const APTR { ((*IExpat).XML_GetBase)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_get_specified_attribute_count(p: APTR) -> i32 { ((*IExpat).XML_GetSpecifiedAttributeCount)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_get_id_attribute_index(p: APTR) -> i32 { ((*IExpat).XML_GetIdAttributeIndex)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_set_encoding(p: APTR, enc: *const APTR) -> APTR { ((*IExpat).XML_SetEncoding)(IExpat, p, enc) }
#[inline]
pub unsafe fn expat_xml_set_param_entity_parsing(p: APTR, mode: APTR) -> i32 { ((*IExpat).XML_SetParamEntityParsing)(IExpat, p, mode) }
#[inline]
pub unsafe fn expat_xml_set_return_nstriplet(p: APTR, do_nst: i32) { ((*IExpat).XML_SetReturnNSTriplet)(IExpat, p, do_nst) }
#[inline]
pub unsafe fn expat_xml_expat_version() -> *const APTR { ((*IExpat).XML_ExpatVersion)(IExpat) }
#[inline]
pub unsafe fn expat_xml_expat_version_info() -> APTR { ((*IExpat).XML_ExpatVersionInfo)(IExpat) }
#[inline]
pub unsafe fn expat_xml_parser_reset(p: APTR, enc: *const APTR) -> APTR { ((*IExpat).XML_ParserReset)(IExpat, p, enc) }
#[inline]
pub unsafe fn expat_xml_set_skipped_entity_handler(p: APTR, h: APTR) { ((*IExpat).XML_SetSkippedEntityHandler)(IExpat, p, h) }
#[inline]
pub unsafe fn expat_xml_use_foreign_dtd(p: APTR, use_dtd: APTR) -> APTR { ((*IExpat).XML_UseForeignDTD)(IExpat, p, use_dtd) }
#[inline]
pub unsafe fn expat_xml_get_feature_list() -> *const APTR { ((*IExpat).XML_GetFeatureList)(IExpat) }
#[inline]
pub unsafe fn expat_xml_stop_parser(p: APTR, resumable: APTR) -> APTR { ((*IExpat).XML_StopParser)(IExpat, p, resumable) }
#[inline]
pub unsafe fn expat_xml_resume_parser(p: APTR) -> APTR { ((*IExpat).XML_ResumeParser)(IExpat, p) }
#[inline]
pub unsafe fn expat_xml_get_parsing_status(p: APTR, status: *mut APTR) { ((*IExpat).XML_GetParsingStatus)(IExpat, p, status) }
#[inline]
pub unsafe fn expat_xml_free_content_model(p: APTR, model: *mut APTR) { ((*IExpat).XML_FreeContentModel)(IExpat, p, model) }
#[inline]
pub unsafe fn expat_xml_mem_malloc(p: APTR, size: u32) -> *mut () { ((*IExpat).XML_MemMalloc)(IExpat, p, size) }
#[inline]
pub unsafe fn expat_xml_mem_realloc(p: APTR, ptr: *mut (), size: u32) -> *mut () { ((*IExpat).XML_MemRealloc)(IExpat, p, ptr, size) }
#[inline]
pub unsafe fn expat_xml_mem_free(p: APTR, ptr: *mut ()) { ((*IExpat).XML_MemFree)(IExpat, p, ptr) }
#[inline]
pub unsafe fn expat_xml_set_hash_salt(p: APTR, salt: u32) -> i32 { ((*IExpat).XML_SetHashSalt)(IExpat, p, salt) }
