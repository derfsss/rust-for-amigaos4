//! Internal RAII helper for runtime-opened library interfaces.
//!
//! Most interfaces this crate uses (`IExec`, `IIntuition`, `IDOS`, ...)
//! are link-time globals provided by the C glue or `-lauto`. Libraries
//! outside that fixed set (asl.library, gadget class libraries, ...) are
//! opened here at runtime via `IExec->OpenLibrary` + `GetInterface`,
//! which works in every build mode and adds no link-time dependency.

use crate::cstr::require_nul;
use crate::error::{AmigaError, Result};
use amigaos4_sys::{CONST_STRPTR, Interface, Library};

/// An opened library + its `main` interface, released in reverse order
/// on drop (`DropInterface`, then `CloseLibrary`).
pub(crate) struct OpenedInterface {
    lib: *mut Library,
    iface: *mut Interface,
}

impl OpenedInterface {
    /// Open `lib_name` (null-terminated, e.g. `b"asl.library\0"`) at
    /// `version` and obtain its `main` interface.
    pub(crate) fn open(lib_name: &[u8], version: u32) -> Result<Self> {
        Self::open_named(lib_name, version, b"main\0", 1)
    }

    /// Like [`open`](Self::open), for libraries whose interface is not
    /// called `main` (e.g. application.library's `"application"`).
    pub(crate) fn open_named(
        lib_name: &[u8],
        version: u32,
        iface_name: &[u8],
        iface_version: u32,
    ) -> Result<Self> {
        let name_ptr = require_nul(lib_name)?;
        let iface_name_ptr = require_nul(iface_name)?;
        // SAFETY: name is null-terminated; OpenLibrary returns null on
        // failure, checked below.
        let lib = unsafe {
            amigaos4_sys::exec_open_library(name_ptr as CONST_STRPTR, version)
        };
        if lib.is_null() {
            return Err(AmigaError::NullPointer);
        }
        // SAFETY: lib is a valid opened library; the interface name is
        // null-terminated (validated above).
        let iface = unsafe {
            amigaos4_sys::exec_get_interface(
                lib,
                iface_name_ptr as CONST_STRPTR,
                iface_version,
                core::ptr::null(),
            )
        };
        if iface.is_null() {
            // SAFETY: lib was opened above and is not yet shared.
            unsafe { amigaos4_sys::exec_close_library(lib) };
            return Err(AmigaError::NullPointer);
        }
        Ok(Self { lib, iface })
    }

    /// The raw interface pointer. Cast to the concrete `*IFace` type at
    /// the call site. Valid for the lifetime of this value.
    #[inline]
    pub(crate) fn as_ptr(&self) -> *mut Interface {
        self.iface
    }
}

impl Drop for OpenedInterface {
    fn drop(&mut self) {
        // SAFETY: iface/lib were obtained in open() and are released
        // exactly once, interface before library.
        unsafe {
            amigaos4_sys::exec_drop_interface(self.iface);
            amigaos4_sys::exec_close_library(self.lib);
        }
    }
}
