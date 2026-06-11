//! Safe wrapper for AmigaOS 4.1 locale catalog access.
//!
//! [`AmigaCatalog`] provides RAII management of locale catalogs via
//! `OpenCatalogA`/`CloseCatalog`, with safe string lookup via `GetCatalogStr`.
//!
//! All name arguments must be null-terminated byte slices (convention from Phase 1).

use crate::error::{AmigaError, Result};
use amigaos4_sys::{Catalog, CONST_STRPTR, TAG_DONE, TagItem};

/// RAII wrapper around `OpenCatalogA` / `CloseCatalog`.
///
/// Provides localized string lookup by numeric ID with a default fallback.
/// The catalog is automatically closed on drop.
pub struct AmigaCatalog {
    ptr: *mut Catalog,
}

impl AmigaCatalog {
    /// Open a locale catalog by name. `name` must be null-terminated.
    ///
    /// Passes `NULL` for the locale parameter (uses the system default locale)
    /// and an empty tag list.
    pub fn open(name: &[u8]) -> Result<Self> {
        let name_ptr = crate::cstr::require_nul(name)?;
        let tags = [TagItem { ti_Tag: TAG_DONE, ti_Data: 0 }];
        let ptr = unsafe {
            amigaos4_sys::locale_open_catalog_a(
                core::ptr::null_mut(),
                name_ptr as CONST_STRPTR,
                tags.as_ptr(),
            )
        };
        if ptr.is_null() {
            Err(AmigaError::NullPointer)
        } else {
            Ok(Self { ptr })
        }
    }

    /// Look up a localized string by ID, returning `default` if not found.
    ///
    /// `default` must be null-terminated. The returned slice is valid for the
    /// lifetime of this catalog (or is the `default` pointer if no translation
    /// exists). If `default` is missing its terminator, `NULL` is passed to
    /// the OS instead (the lookup still works; an untranslated ID falls back
    /// to returning `default` unchanged).
    ///
    /// The returned `&[u8]` does not include the trailing null.
    pub fn get<'a>(&'a self, id: i32, default: &'a [u8]) -> &'a [u8] {
        let default_ptr = if default.last() == Some(&0) {
            default.as_ptr() as CONST_STRPTR
        } else {
            core::ptr::null()
        };
        let result = unsafe {
            amigaos4_sys::locale_get_catalog_str(self.ptr, id, default_ptr)
        };
        if result.is_null() {
            // Strip trailing null from default if present.
            match default.iter().position(|&b| b == 0) {
                Some(end) => &default[..end],
                None => default,
            }
        } else {
            // Measure the C string length with a safety cap.
            const MAX_CATALOG_STR: usize = 16384;
            let mut len = 0usize;
            unsafe {
                while len < MAX_CATALOG_STR && *result.add(len) != 0 {
                    len += 1;
                }
                core::slice::from_raw_parts(result, len)
            }
        }
    }

    /// Return the raw catalog pointer.
    #[inline]
    pub fn as_ptr(&self) -> *mut Catalog {
        self.ptr
    }
}

impl Drop for AmigaCatalog {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { amigaos4_sys::locale_close_catalog(self.ptr) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_struct_size() {
        // AmigaCatalog is just a pointer-sized wrapper.
        assert_eq!(
            core::mem::size_of::<AmigaCatalog>(),
            core::mem::size_of::<*mut Catalog>()
        );
    }
}
