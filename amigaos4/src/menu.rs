//! Window menus via the OS4 Intuition menuclass.
//!
//! [`MenuBuilder`] declares a menu tree; [`MenuStrip`] owns the built
//! menuclass object and attaches it to a window. Menu picks arrive as
//! `Event::MenuPick` in the event loop (the window must include
//! `IDCMP_MENUPICK` in its `WA_IDCMP` tags); the selected item IDs are
//! then read with [`MenuStrip::each_select`].
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::menu::MenuBuilder;
//! use amigaos4::amstr;
//!
//! const MI_OPEN: u32 = 1;
//! const MI_QUIT: u32 = 2;
//!
//! let strip = MenuBuilder::new()
//!     .menu(amstr!("Project"))
//!     .item(MI_OPEN, amstr!("Open..."))
//!     .item_key(MI_QUIT, amstr!("Quit"), amstr!("Q"))
//!     .build()?;
//! strip.attach(&win)?;
//!
//! event_loop(&win, |event| match event {
//!     Event::MenuPick => {
//!         strip.each_select(|id| match id {
//!             MI_OPEN => { /* ... */ }
//!             MI_QUIT => { /* ... */ }
//!             _ => {}
//!         });
//!         true
//!     }
//!     Event::Close => false,
//!     _ => true,
//! });
//! // strip.detach(&win) before the window closes, or close the window
//! // first and let MenuStrip's Drop dispose the object tree.
//! ```

use alloc::vec::Vec;
use crate::boopsi::AmigaObject;
use crate::error::{AmigaError, Result};
use crate::tag::TagListBuilder;
use crate::window::AmigaWindow;
use amigaos4_sys::{Menu, TAG_USER};

// ---------------------------------------------------------------------------
// intuition/menuclass.h constants (AmigaOS 4.1 SDK 54.16)
// ---------------------------------------------------------------------------

/// `MA_Dummy = TAG_USER + 0x440000`.
const MA_DUMMY: u32 = TAG_USER + 0x44_0000;

/// (int32) Object type: [`T_ROOT`], [`T_MENU`], or [`T_ITEM`].
const MA_TYPE: u32 = MA_DUMMY + 1;
/// (STRPTR) Label text of a menu or item.
const MA_LABEL: u32 = MA_DUMMY + 2;
/// (BOOL) Render this item as a separator bar.
const MA_SEPARATOR: u32 = MA_DUMMY + 4;
/// (uint32) Application-assigned item ID (must be non-zero).
const MA_ID: u32 = MA_DUMMY + 5;
/// (STRPTR) Keyboard shortcut for the item.
const MA_KEY: u32 = MA_DUMMY + 6;
/// (BOOL) Item is disabled.
const MA_DISABLED: u32 = MA_DUMMY + 8;
/// (Object *) Add a child object during NewObject/SetAttrs.
const MA_ADD_CHILD: u32 = MA_DUMMY + 50;

/// Menu tree root type for [`MA_TYPE`].
const T_ROOT: u32 = -1i32 as u32;
/// Menu (title) type for [`MA_TYPE`].
const T_MENU: u32 = 0;
/// Menu item type for [`MA_TYPE`].
const T_ITEM: u32 = 1;

/// `MM_NEXTSELECT` method ID: get next ID in the selection chain.
const MM_NEXTSELECT: u32 = 4009;

/// Invalid menu ID — terminates the MM_NEXTSELECT chain. Item IDs
/// passed to [`MenuBuilder::item`] must not be this value (zero).
pub const NO_MENU_ID: u32 = 0;

/// BOOPSI class name for the Intuition menu class.
const MENU_CLASS: &[u8] = b"menuclass\0";

// ---------------------------------------------------------------------------
// Builder
// ---------------------------------------------------------------------------

enum Entry {
    Menu { label: Vec<u8> },
    Item { id: u32, label: Vec<u8>, key: Option<Vec<u8>>, disabled: bool },
    Separator,
}

/// Declarative builder for a menuclass menu tree.
///
/// Call [`menu`](Self::menu) to start a menu (title), then
/// [`item`](Self::item) / [`separator`](Self::separator) for its
/// entries; repeat for further menus, and finish with
/// [`build`](Self::build).
pub struct MenuBuilder {
    entries: Vec<Entry>,
    /// First string that was missing its `\0`, surfaced in build().
    invalid: bool,
}

impl MenuBuilder {
    /// Start an empty menu tree.
    pub fn new() -> Self {
        Self { entries: Vec::new(), invalid: false }
    }

    /// Start a new menu (title). `label` must be null-terminated.
    pub fn menu(mut self, label: &[u8]) -> Self {
        match copy_nul(label) {
            Some(label) => self.entries.push(Entry::Menu { label }),
            None => self.invalid = true,
        }
        self
    }

    /// Add an item to the current menu. `id` must be non-zero and
    /// unique; `label` must be null-terminated.
    pub fn item(self, id: u32, label: &[u8]) -> Self {
        self.push_item(id, label, None, false)
    }

    /// Like [`item`](Self::item), with a keyboard shortcut (e.g.
    /// `amstr!("Q")` for RAmiga-Q). `key` must be null-terminated.
    pub fn item_key(self, id: u32, label: &[u8], key: &[u8]) -> Self {
        let key = copy_nul(key);
        let invalid = key.is_none();
        let mut s = self.push_item_inner(id, label, key, false);
        s.invalid |= invalid;
        s
    }

    /// Add a disabled item to the current menu.
    pub fn item_disabled(self, id: u32, label: &[u8]) -> Self {
        self.push_item(id, label, None, true)
    }

    /// Add a separator bar to the current menu.
    pub fn separator(mut self) -> Self {
        self.entries.push(Entry::Separator);
        self
    }

    fn push_item(self, id: u32, label: &[u8], key: Option<Vec<u8>>, disabled: bool) -> Self {
        self.push_item_inner(id, label, key, disabled)
    }

    fn push_item_inner(
        mut self,
        id: u32,
        label: &[u8],
        key: Option<Vec<u8>>,
        disabled: bool,
    ) -> Self {
        match copy_nul(label) {
            Some(label) if id != NO_MENU_ID => {
                self.entries.push(Entry::Item { id, label, key, disabled });
            }
            _ => self.invalid = true,
        }
        self
    }

    /// Build the menuclass object tree.
    ///
    /// # Errors
    ///
    /// `NotNulTerminated` if any label/key was missing its `\0` (or an
    /// item used the reserved ID 0); `AllocationFailed` if an object
    /// could not be created; `NullPointer` if an item appears before
    /// the first [`menu`](Self::menu).
    pub fn build(self) -> Result<MenuStrip> {
        if self.invalid {
            return Err(AmigaError::NotNulTerminated);
        }

        // The strip owns every label/key buffer: menuclass keeps the
        // MA_Label/MA_Key pointers rather than copying the text.
        let mut strings: Vec<Vec<u8>> = Vec::new();

        let root = AmigaObject::new(
            MENU_CLASS,
            &TagListBuilder::new().tag(MA_TYPE, T_ROOT).build(),
        )?;

        let mut current_menu: Option<AmigaObject> = None;

        for entry in self.entries {
            match entry {
                Entry::Menu { label } => {
                    // Attach the previous menu before starting the next.
                    if let Some(menu) = current_menu.take() {
                        add_child(&root, menu);
                    }
                    let tags = TagListBuilder::new()
                        .tag(MA_TYPE, T_MENU)
                        .tag(MA_LABEL, label.as_ptr() as u32)
                        .build();
                    strings.push(label);
                    current_menu = Some(AmigaObject::new(MENU_CLASS, &tags)?);
                }
                Entry::Item { id, label, key, disabled } => {
                    let menu = current_menu.as_ref().ok_or(AmigaError::NullPointer)?;
                    let mut tags = TagListBuilder::new()
                        .tag(MA_TYPE, T_ITEM)
                        .tag(MA_LABEL, label.as_ptr() as u32)
                        .tag(MA_ID, id);
                    if let Some(ref k) = key {
                        tags = tags.tag(MA_KEY, k.as_ptr() as u32);
                    }
                    if disabled {
                        tags = tags.tag(MA_DISABLED, 1);
                    }
                    let item = AmigaObject::new(MENU_CLASS, &tags.build())?;
                    strings.push(label);
                    if let Some(k) = key {
                        strings.push(k);
                    }
                    add_child(menu, item);
                }
                Entry::Separator => {
                    let menu = current_menu.as_ref().ok_or(AmigaError::NullPointer)?;
                    let tags = TagListBuilder::new()
                        .tag(MA_TYPE, T_ITEM)
                        .tag(MA_SEPARATOR, 1)
                        .build();
                    let item = AmigaObject::new(MENU_CLASS, &tags)?;
                    add_child(menu, item);
                }
            }
        }
        if let Some(menu) = current_menu.take() {
            add_child(&root, menu);
        }

        Ok(MenuStrip { root, _strings: strings })
    }
}

/// Copy a byte string if (and only if) it is null-terminated.
fn copy_nul(s: &[u8]) -> Option<Vec<u8>> {
    if s.last() == Some(&0) {
        Some(s.to_vec())
    } else {
        None
    }
}

/// Transfer ownership of `child` into `parent` via `MA_AddChild`.
fn add_child(parent: &AmigaObject, child: AmigaObject) {
    let tags = TagListBuilder::new()
        .tag(MA_ADD_CHILD, child.into_raw() as u32)
        .build();
    parent.set_attrs(&tags);
}

// ---------------------------------------------------------------------------
// MenuStrip
// ---------------------------------------------------------------------------

/// An owned menuclass menu strip.
///
/// Detach (or close the window) before dropping; dropping disposes the
/// whole object tree.
pub struct MenuStrip {
    root: AmigaObject,
    /// Label/key buffers; menuclass keeps the pointers.
    _strings: Vec<Vec<u8>>,
}

impl MenuStrip {
    /// Attach this strip to a window (`SetMenuStrip`).
    pub fn attach(&self, window: &AmigaWindow) -> Result<()> {
        // SAFETY: window and root are valid; menuclass objects are
        // accepted by SetMenuStrip on OS4.
        let ok = unsafe {
            amigaos4_sys::intuition_set_menu_strip(
                window.as_ptr(),
                self.root.as_ptr() as *mut Menu,
            )
        };
        if ok != 0 {
            Ok(())
        } else {
            Err(AmigaError::AllocationFailed)
        }
    }

    /// Detach this strip from a window (`ClearMenuStrip`).
    pub fn detach(&self, window: &AmigaWindow) {
        // SAFETY: window is valid; clearing an unset strip is harmless.
        unsafe { amigaos4_sys::intuition_clear_menu_strip(window.as_ptr()) };
    }

    /// After an `Event::MenuPick`, call `f` with each selected item ID
    /// (multi-select yields several). Uses `MM_NEXTSELECT`.
    pub fn each_select<F: FnMut(u32)>(&self, mut f: F) {
        let mut current = NO_MENU_ID;
        loop {
            current = self.root.do_method_1(MM_NEXTSELECT, current);
            if current == NO_MENU_ID {
                break;
            }
            f(current);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ma_tags_match_menuclass_h() {
        assert_eq!(MA_DUMMY, TAG_USER + 0x440000);
        assert_eq!(MA_TYPE, MA_DUMMY + 1);
        assert_eq!(MA_LABEL, MA_DUMMY + 2);
        assert_eq!(MA_SEPARATOR, MA_DUMMY + 4);
        assert_eq!(MA_ID, MA_DUMMY + 5);
        assert_eq!(MA_KEY, MA_DUMMY + 6);
        assert_eq!(MA_DISABLED, MA_DUMMY + 8);
        assert_eq!(MA_ADD_CHILD, MA_DUMMY + 50);
        assert_eq!(MM_NEXTSELECT, 4009);
        assert_eq!(NO_MENU_ID, 0);
    }

    #[test]
    fn t_root_is_minus_one() {
        assert_eq!(T_ROOT as i32, -1);
        assert_eq!(T_MENU, 0);
        assert_eq!(T_ITEM, 1);
    }

    #[test]
    fn builder_rejects_unterminated_label() {
        let b = MenuBuilder::new().menu(b"no nul");
        assert!(b.invalid);
    }

    #[test]
    fn builder_rejects_zero_id() {
        let b = MenuBuilder::new().menu(b"M\0").item(NO_MENU_ID, b"x\0");
        assert!(b.invalid);
    }
}
