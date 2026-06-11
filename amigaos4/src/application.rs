//! application.library registration.
//!
//! Registering with application.library gives a program a system-wide
//! identity: single-instance enforcement, blanker/notification
//! integration, and prefs handling. [`AppRegistration`] is the RAII
//! wrapper — register at startup, drop (unregister) at exit.
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::application::AppRegistration;
//! use amigaos4::amstr;
//!
//! let _app = AppRegistration::new(amstr!("MyApp"))
//!     .unique()          // refuse a second instance
//!     .register()?;
//! // ... run ...
//! // dropped at scope end -> UnregisterApplication
//! ```

use alloc::vec::Vec;
use crate::error::{AmigaError, Result};
use crate::iface::OpenedInterface;
use crate::tag::TagListBuilder;
use amigaos4_sys::{ApplicationIFace, CONST_STRPTR, TagItem};

// libraries/application.h — REGAPP_* tags are plain TAG_USER offsets.
const TAG_USER: u32 = amigaos4_sys::TAG_USER;

/// (BOOL) Refuse to register a second instance of the same name.
const REGAPP_UNIQUE_APPLICATION: u32 = TAG_USER + 1;
/// (BOOL) Do not show an AppIcon entry.
const REGAPP_NO_ICON: u32 = TAG_USER + 30;
/// (BOOL) Register hidden (no UI presence).
const REGAPP_HIDDEN: u32 = TAG_USER + 31;

/// Builder for an application.library registration.
pub struct AppRegistration {
    name: Vec<u8>,
    unique: bool,
    hidden: bool,
    no_icon: bool,
    invalid: bool,
}

impl AppRegistration {
    /// Start a registration for `name` (null-terminated — use
    /// [`amstr!`](crate::amstr)).
    pub fn new(name: &[u8]) -> Self {
        let invalid = name.last() != Some(&0);
        Self {
            name: name.to_vec(),
            unique: false,
            hidden: false,
            no_icon: false,
            invalid,
        }
    }

    /// Enforce a single instance of this application name.
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    }

    /// Register without any UI presence.
    pub fn hidden(mut self) -> Self {
        self.hidden = true;
        self
    }

    /// Suppress the AppIcon.
    pub fn no_icon(mut self) -> Self {
        self.no_icon = true;
        self
    }

    /// Register with application.library.
    ///
    /// # Errors
    ///
    /// `NotNulTerminated` if `name` was missing its `\0`; `NullPointer`
    /// if application.library cannot be opened; `AllocationFailed` if
    /// registration is refused (e.g. unique-instance conflict).
    pub fn register(self) -> Result<RegisteredApp> {
        if self.invalid {
            return Err(AmigaError::NotNulTerminated);
        }

        // application.library exposes its API as the "application"
        // interface (version 2), not "main".
        let lib = OpenedInterface::open_named(
            b"application.library\0",
            53,
            b"application\0",
            2,
        )?;
        let iapp = lib.as_ptr() as *mut ApplicationIFace;

        let mut tags = TagListBuilder::new();
        if self.unique {
            tags = tags.tag(REGAPP_UNIQUE_APPLICATION, 1);
        }
        if self.hidden {
            tags = tags.tag(REGAPP_HIDDEN, 1);
        }
        if self.no_icon {
            tags = tags.tag(REGAPP_NO_ICON, 1);
        }
        let tags = tags.build();

        // SAFETY: iapp is the open application interface; name is
        // null-terminated (validated in new()); tags is
        // TAG_DONE-terminated.
        let app_id = unsafe {
            ((*iapp).RegisterApplicationA)(
                iapp,
                self.name.as_ptr() as CONST_STRPTR,
                tags.as_ptr() as *mut TagItem,
            )
        };
        if app_id == 0 {
            return Err(AmigaError::AllocationFailed);
        }

        Ok(RegisteredApp { lib, app_id })
    }
}

/// A live application.library registration; unregisters on drop.
pub struct RegisteredApp {
    lib: OpenedInterface,
    app_id: u32,
}

impl RegisteredApp {
    /// The application ID assigned by application.library.
    #[inline]
    pub fn app_id(&self) -> u32 {
        self.app_id
    }
}

impl Drop for RegisteredApp {
    fn drop(&mut self) {
        let iapp = self.lib.as_ptr() as *mut ApplicationIFace;
        let tags = [amigaos4_sys::TagItem {
            ti_Tag: amigaos4_sys::TAG_DONE,
            ti_Data: 0,
        }];
        // SAFETY: app_id came from RegisterApplicationA and is
        // unregistered exactly once; the library closes afterwards
        // when self.lib drops.
        unsafe {
            ((*iapp).UnregisterApplicationA)(iapp, self.app_id, tags.as_ptr() as *mut TagItem);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn regapp_tags_match_application_h() {
        assert_eq!(REGAPP_UNIQUE_APPLICATION, TAG_USER + 1);
        assert_eq!(REGAPP_NO_ICON, TAG_USER + 30);
        assert_eq!(REGAPP_HIDDEN, TAG_USER + 31);
    }

    #[test]
    fn builder_rejects_unterminated() {
        let b = AppRegistration::new(b"no nul");
        assert!(b.invalid);
    }
}
