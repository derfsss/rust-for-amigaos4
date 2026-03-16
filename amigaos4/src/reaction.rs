//! ReAction GUI helpers for AmigaOS 4.
//!
//! This module provides constants, gadget creation functions, a layout
//! builder, and an event loop abstraction for AmigaOS 4 ReAction (the
//! built-in widget toolkit).
//!
//! # Overview
//!
//! ReAction is AmigaOS 4's native GUI toolkit built on BOOPSI classes.
//! Each widget (button, string gadget, checkbox, etc.) is a BOOPSI object
//! created with class-specific tags. Layouts arrange children using
//! `layout.gadget`, which takes ownership of child objects via
//! `LAYOUT_ADD_CHILD`.
//!
//! This module layers on top of [`AmigaObject`](crate::boopsi::AmigaObject)
//! and [`AmigaWindow`](crate::window::AmigaWindow) to provide:
//!
//! - **Tag constants** for gadget attributes, layout, button, string,
//!   checkbox, integer, and label classes.
//! - **Gadget creation helpers** ([`button`], [`string_gadget`],
//!   [`checkbox`]) that wrap `AmigaObject::new` with common tag patterns.
//! - **[`LayoutBuilder`]** for composing nested vertical/horizontal layouts
//!   with a builder pattern.
//! - **[`Event`] enum and [`event_loop`]** for processing IDCMP messages
//!   at a higher level than raw `IntuiMsg` values.
//!
//! # Example
//!
//! ```ignore
//! use amigaos4::tag::TagListBuilder;
//! use amigaos4::window::*;
//! use amigaos4::reaction::*;
//!
//! // Create a layout with two buttons.
//! let layout = LayoutBuilder::vertical()
//!     .add(button(1, b"OK\0")?)
//!     .add(button(2, b"Cancel\0")?)
//!     .build()?;
//!
//! // Open a window with the layout as its root gadget.
//! let tags = TagListBuilder::new()
//!     .tag(WA_LEFT, 100)
//!     .tag(WA_TOP, 100)
//!     .tag(WA_WIDTH, 300)
//!     .tag(WA_HEIGHT, 150)
//!     .tag(WA_TITLE, b"ReAction Demo\0".as_ptr() as u32)
//!     .tag(WA_IDCMP, IDCMP_CLOSEWINDOW | IDCMP_GADGETUP)
//!     .tag(WA_CLOSE_GADGET, 1)
//!     .tag(WA_DRAG_BAR, 1)
//!     .tag(WA_DEPTH_GADGET, 1)
//!     .tag(WA_ACTIVATE, 1)
//!     .tag(WA_GADGETS, layout.as_ptr() as u32)
//!     .build();
//!
//! let win = AmigaWindow::open(&tags)?;
//!
//! // Run the event loop.
//! event_loop(&win, |event| {
//!     match event {
//!         Event::Close => return false,
//!         Event::GadgetUp(id) => {
//!             // Handle gadget with the given ID.
//!         }
//!         _ => {}
//!     }
//!     true
//! });
//! ```

use alloc::vec::Vec;
use amigaos4_sys::{TagItem, TAG_DONE, TAG_USER};

use crate::boopsi::AmigaObject;
use crate::error::Result;
use crate::window::{
    AmigaWindow, IntuiMsg, IDCMP_CLOSEWINDOW, IDCMP_GADGETUP, IDCMP_NEWSIZE,
    IDCMP_REFRESHWINDOW, IDCMP_VANILLAKEY,
};

// ===========================================================================
// Gadget attribute tags (reaction/reaction.h)
// ===========================================================================

const GA_DUMMY: u32 = TAG_USER + 0x30000;

/// Gadget identifier, used to distinguish gadgets in IDCMP_GADGETUP events.
pub const GA_ID: u32 = GA_DUMMY + 0x01;

/// When true (non-zero) the gadget is disabled and will not respond to input.
pub const GA_DISABLED: u32 = GA_DUMMY + 0x02;

/// Whether the gadget is selected / highlighted.
pub const GA_SELECTED: u32 = GA_DUMMY + 0x05;

/// Send IDCMP_GADGETUP when the user releases the gadget (required for
/// ReAction gadgets that should generate events).
pub const GA_RELVERIFY: u32 = GA_DUMMY + 0x1A;

/// Gadget label text pointer (null-terminated C string).
pub const GA_TEXT: u32 = GA_DUMMY + 0x28;

// ===========================================================================
// Layout tags (gadgets/layout.h)
// ===========================================================================

const LAYOUT_DUMMY: u32 = TAG_USER + 0x100000;

/// Orientation of the layout group: [`LAYOUT_ORIENT_HORIZ`] or
/// [`LAYOUT_ORIENT_VERT`].
pub const LAYOUT_ORIENTATION: u32 = LAYOUT_DUMMY + 0x01;

/// Enable inner spacing between child gadgets.
pub const LAYOUT_SPACE_INNER: u32 = LAYOUT_DUMMY + 0x05;

/// Enable outer spacing around the layout border.
pub const LAYOUT_SPACE_OUTER: u32 = LAYOUT_DUMMY + 0x06;

/// Add a child gadget (BOOPSI object pointer) to the layout.
/// Ownership of the child is transferred to the layout.
pub const LAYOUT_ADD_CHILD: u32 = LAYOUT_DUMMY + 0x0C;

/// Add a child image (BOOPSI object pointer) to the layout.
pub const LAYOUT_ADD_IMAGE: u32 = LAYOUT_DUMMY + 0x0D;

/// Text label for the layout group frame.
pub const LAYOUT_LABEL: u32 = LAYOUT_DUMMY + 0x12;

/// Bottom spacing in pixels.
pub const LAYOUT_BOTTOM_SPACING: u32 = LAYOUT_DUMMY + 0x19;

/// Horizontal orientation value for [`LAYOUT_ORIENTATION`].
pub const LAYOUT_ORIENT_HORIZ: u32 = 0;

/// Vertical orientation value for [`LAYOUT_ORIENTATION`].
pub const LAYOUT_ORIENT_VERT: u32 = 1;

// ===========================================================================
// Button tags (gadgets/button.h)
// ===========================================================================

const BUTTON_DUMMY: u32 = TAG_USER + 0x110000;

/// Button label text pointer (null-terminated C string).
pub const BUTTON_TEXT: u32 = BUTTON_DUMMY + 0x01;

/// When true, the button acts as a toggle (push) button rather than a
/// momentary click button.
pub const BUTTON_PUSH_BUTTON: u32 = BUTTON_DUMMY + 0x02;

// ===========================================================================
// String gadget tags (gadgets/string.h)
// ===========================================================================

const STRING_DUMMY: u32 = TAG_USER + 0x120000;

/// Pointer to the initial text value (null-terminated C string).
pub const STRINGA_TEXT_VAL: u32 = STRING_DUMMY + 0x01;

/// Maximum number of characters the user may enter.
pub const STRINGA_MAX_CHARS: u32 = STRING_DUMMY + 0x02;

// ===========================================================================
// Checkbox tags (gadgets/checkbox.h)
// ===========================================================================

const CHECKBOX_DUMMY: u32 = TAG_USER + 0x130000;

/// Initial checked state: 1 = checked, 0 = unchecked.
pub const CHECKBOX_CHECKED: u32 = CHECKBOX_DUMMY + 0x01;

// ===========================================================================
// Integer gadget tags (gadgets/integer.h)
// ===========================================================================

const INTEGER_DUMMY: u32 = TAG_USER + 0x140000;

/// Initial numeric value.
pub const INTEGER_NUMBER: u32 = INTEGER_DUMMY + 0x01;

/// Minimum allowed value.
pub const INTEGER_MINIMUM: u32 = INTEGER_DUMMY + 0x04;

/// Maximum allowed value.
pub const INTEGER_MAXIMUM: u32 = INTEGER_DUMMY + 0x05;

// ===========================================================================
// Label image tags (images/label.h)
// ===========================================================================

const LABEL_DUMMY: u32 = TAG_USER + 0x200000;

/// Label text pointer (null-terminated C string).
pub const LABEL_TEXT: u32 = LABEL_DUMMY + 0x01;

// ===========================================================================
// Child tags for layout (used alongside LAYOUT_ADD_CHILD)
// ===========================================================================

const CHILD_DUMMY: u32 = TAG_USER + 0x180000;

/// Give the child equal weight in the layout (boolean).
pub const CHILD_WEIGHT_EQUAL: u32 = CHILD_DUMMY + 0x02;

/// Minimum width of the child in pixels.
pub const CHILD_MIN_WIDTH: u32 = CHILD_DUMMY + 0x03;

/// Minimum height of the child in pixels.
pub const CHILD_MIN_HEIGHT: u32 = CHILD_DUMMY + 0x04;

/// Text label displayed next to the child gadget (pointer to
/// null-terminated C string).
pub const CHILD_LABEL: u32 = CHILD_DUMMY + 0x08;

// ===========================================================================
// Class name strings (null-terminated)
// ===========================================================================

/// BOOPSI class name for `layout.gadget`.
pub const LAYOUT_CLASS: &[u8] = b"layout.gadget\0";

/// BOOPSI class name for `button.gadget`.
pub const BUTTON_CLASS: &[u8] = b"button.gadget\0";

/// BOOPSI class name for `string.gadget`.
pub const STRING_CLASS: &[u8] = b"string.gadget\0";

/// BOOPSI class name for `checkbox.gadget`.
pub const CHECKBOX_CLASS: &[u8] = b"checkbox.gadget\0";

/// BOOPSI class name for `integer.gadget`.
pub const INTEGER_CLASS: &[u8] = b"integer.gadget\0";

/// BOOPSI class name for `label.image`.
pub const LABEL_CLASS: &[u8] = b"label.image\0";

// ===========================================================================
// Gadget creation helpers
// ===========================================================================

/// Create a button gadget.
///
/// `id` is the gadget identifier returned in [`Event::GadgetUp`] when the
/// button is clicked. `label` must be a null-terminated byte string
/// (e.g. `b"OK\0"`).
///
/// The button is created with `GA_RELVERIFY` so it generates
/// `IDCMP_GADGETUP` events.
///
/// # Errors
///
/// Returns `Err(AllocationFailed)` if the BOOPSI object could not be
/// created.
pub fn button(id: u32, label: &[u8]) -> Result<AmigaObject> {
    let tags = crate::tag::TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(GA_TEXT, label.as_ptr() as u32)
        .build();
    AmigaObject::new(BUTTON_CLASS, &tags)
}

/// Create a string (text input) gadget.
///
/// `id` is the gadget identifier. `max_chars` is the maximum number of
/// characters the user may type.
///
/// # Errors
///
/// Returns `Err(AllocationFailed)` if the BOOPSI object could not be
/// created.
pub fn string_gadget(id: u32, max_chars: u32) -> Result<AmigaObject> {
    let tags = crate::tag::TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(STRINGA_MAX_CHARS, max_chars)
        .build();
    AmigaObject::new(STRING_CLASS, &tags)
}

/// Create a checkbox gadget.
///
/// `id` is the gadget identifier. `checked` sets the initial state.
///
/// # Errors
///
/// Returns `Err(AllocationFailed)` if the BOOPSI object could not be
/// created.
pub fn checkbox(id: u32, checked: bool) -> Result<AmigaObject> {
    let tags = crate::tag::TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(CHECKBOX_CHECKED, if checked { 1 } else { 0 })
        .build();
    AmigaObject::new(CHECKBOX_CLASS, &tags)
}

// ===========================================================================
// LayoutBuilder
// ===========================================================================

/// Builder for constructing nested ReAction layouts.
///
/// A layout is a BOOPSI object of class `layout.gadget` that arranges its
/// children either horizontally or vertically. Children are added via
/// [`add`](Self::add) or [`add_labeled`](Self::add_labeled), which
/// transfer ownership of the child object to the layout (via
/// [`AmigaObject::into_raw`]).
///
/// # Example
///
/// ```ignore
/// use amigaos4::reaction::*;
///
/// let layout = LayoutBuilder::vertical()
///     .add(button(1, b"OK\0")?)
///     .add(button(2, b"Cancel\0")?)
///     .build()?;
/// ```
///
/// Layouts can be nested by adding one layout as a child of another:
///
/// ```ignore
/// let buttons = LayoutBuilder::horizontal()
///     .add(button(1, b"OK\0")?)
///     .add(button(2, b"Cancel\0")?)
///     .build()?;
///
/// let root = LayoutBuilder::vertical()
///     .add(string_gadget(3, 128)?)
///     .add(buttons)
///     .build()?;
/// ```
pub struct LayoutBuilder {
    tags: Vec<TagItem>,
    orientation: u32,
}

impl LayoutBuilder {
    /// Create a layout builder with vertical orientation.
    ///
    /// Children will be stacked top-to-bottom.
    pub fn vertical() -> Self {
        Self {
            tags: Vec::new(),
            orientation: LAYOUT_ORIENT_VERT,
        }
    }

    /// Create a layout builder with horizontal orientation.
    ///
    /// Children will be arranged left-to-right.
    pub fn horizontal() -> Self {
        Self {
            tags: Vec::new(),
            orientation: LAYOUT_ORIENT_HORIZ,
        }
    }

    /// Add a child gadget to the layout.
    ///
    /// Ownership of `child` is transferred to the layout via
    /// [`AmigaObject::into_raw`] -- the child will be disposed when the
    /// layout is disposed.
    pub fn add(mut self, child: AmigaObject) -> Self {
        self.tags.push(TagItem {
            ti_Tag: LAYOUT_ADD_CHILD,
            ti_Data: child.into_raw() as u32,
        });
        self
    }

    /// Add a child gadget with a text label displayed beside it.
    ///
    /// `label` must be a null-terminated byte string (e.g. `b"Name:\0"`).
    /// Ownership of `child` is transferred to the layout.
    pub fn add_labeled(mut self, child: AmigaObject, label: &[u8]) -> Self {
        self.tags.push(TagItem {
            ti_Tag: LAYOUT_ADD_CHILD,
            ti_Data: child.into_raw() as u32,
        });
        self.tags.push(TagItem {
            ti_Tag: CHILD_LABEL,
            ti_Data: label.as_ptr() as u32,
        });
        self
    }

    /// Build the layout, returning an [`AmigaObject`].
    ///
    /// The orientation, inner spacing, and outer spacing tags are
    /// prepended automatically, followed by any children added via
    /// [`add`](Self::add) or [`add_labeled`](Self::add_labeled), and
    /// finally `TAG_DONE`.
    ///
    /// # Errors
    ///
    /// Returns `Err(AllocationFailed)` if the BOOPSI object could not be
    /// created.
    pub fn build(mut self) -> Result<AmigaObject> {
        let mut final_tags = Vec::with_capacity(4 + self.tags.len());
        final_tags.push(TagItem {
            ti_Tag: LAYOUT_ORIENTATION,
            ti_Data: self.orientation,
        });
        final_tags.push(TagItem {
            ti_Tag: LAYOUT_SPACE_INNER,
            ti_Data: 1,
        });
        final_tags.push(TagItem {
            ti_Tag: LAYOUT_SPACE_OUTER,
            ti_Data: 1,
        });
        final_tags.append(&mut self.tags);
        final_tags.push(TagItem {
            ti_Tag: TAG_DONE,
            ti_Data: 0,
        });

        AmigaObject::new(LAYOUT_CLASS, &final_tags)
    }
}

// ===========================================================================
// Event abstraction
// ===========================================================================

/// High-level event from the IDCMP message loop.
///
/// Wraps the most common IDCMP message classes as ergonomic enum variants.
/// Less common message classes are captured by [`Event::Other`] with the
/// raw [`IntuiMsg`] for manual inspection.
pub enum Event {
    /// The window close gadget was clicked (`IDCMP_CLOSEWINDOW`).
    Close,
    /// A gadget was activated and released (`IDCMP_GADGETUP`).
    /// Contains the gadget ID from `msg.code`.
    GadgetUp(u32),
    /// A key was pressed (`IDCMP_VANILLAKEY`).
    /// Contains the character as a `u8`.
    Key(u8),
    /// The window was resized (`IDCMP_NEWSIZE`).
    Resize,
    /// The window needs a refresh (`IDCMP_REFRESHWINDOW`).
    Refresh,
    /// An IDCMP message class not explicitly handled above.
    Other(IntuiMsg),
}

/// Run a blocking event loop on a window.
///
/// Waits for IDCMP messages on `window` and converts each one into an
/// [`Event`]. The `handler` closure is called for every event; return
/// `true` to continue the loop or `false` to exit.
///
/// The window must have been opened with the appropriate `WA_IDCMP` flags
/// for the events you want to receive (e.g. `IDCMP_CLOSEWINDOW |
/// IDCMP_GADGETUP`).
///
/// # Example
///
/// ```ignore
/// use amigaos4::reaction::{event_loop, Event};
///
/// event_loop(&win, |event| {
///     match event {
///         Event::Close => false,
///         Event::GadgetUp(id) => {
///             // handle gadget press
///             true
///         }
///         _ => true,
///     }
/// });
/// ```
pub fn event_loop<F>(window: &AmigaWindow, mut handler: F)
where
    F: FnMut(Event) -> bool,
{
    loop {
        let msg = window.wait_msg();
        let event = match msg.class {
            IDCMP_CLOSEWINDOW => Event::Close,
            IDCMP_GADGETUP => Event::GadgetUp(msg.code as u32),
            IDCMP_VANILLAKEY => Event::Key(msg.code as u8),
            IDCMP_NEWSIZE => Event::Resize,
            IDCMP_REFRESHWINDOW => Event::Refresh,
            _ => Event::Other(msg),
        };
        if !handler(event) {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_orient_horiz_is_0() {
        assert_eq!(LAYOUT_ORIENT_HORIZ, 0);
    }

    #[test]
    fn layout_orient_vert_is_1() {
        assert_eq!(LAYOUT_ORIENT_VERT, 1);
    }

    #[test]
    fn ga_dummy_base() {
        assert_eq!(GA_DUMMY, TAG_USER + 0x30000);
    }

    #[test]
    fn ga_id_offset() {
        assert_eq!(GA_ID, GA_DUMMY + 1);
    }

    #[test]
    fn layout_dummy_base() {
        assert_eq!(LAYOUT_DUMMY, TAG_USER + 0x100000);
    }

    #[test]
    fn button_dummy_base() {
        assert_eq!(BUTTON_DUMMY, TAG_USER + 0x110000);
    }

    #[test]
    fn string_dummy_base() {
        assert_eq!(STRING_DUMMY, TAG_USER + 0x120000);
    }

    #[test]
    fn checkbox_dummy_base() {
        assert_eq!(CHECKBOX_DUMMY, TAG_USER + 0x130000);
    }

    #[test]
    fn integer_dummy_base() {
        assert_eq!(INTEGER_DUMMY, TAG_USER + 0x140000);
    }

    #[test]
    fn label_dummy_base() {
        assert_eq!(LABEL_DUMMY, TAG_USER + 0x200000);
    }

    #[test]
    fn child_dummy_base() {
        assert_eq!(CHILD_DUMMY, TAG_USER + 0x180000);
    }

    #[test]
    fn class_names_null_terminated() {
        let classes: &[&[u8]] = &[
            LAYOUT_CLASS, BUTTON_CLASS, STRING_CLASS,
            CHECKBOX_CLASS, INTEGER_CLASS, LABEL_CLASS,
        ];
        for cls in classes {
            assert_eq!(cls.last(), Some(&0u8), "class name not null-terminated");
        }
    }

    #[test]
    fn class_names_non_empty() {
        let classes: &[&[u8]] = &[
            LAYOUT_CLASS, BUTTON_CLASS, STRING_CLASS,
            CHECKBOX_CLASS, INTEGER_CLASS, LABEL_CLASS,
        ];
        for cls in classes {
            assert!(cls.len() > 1, "class name is empty (only null)");
        }
    }

    #[test]
    fn event_enum_variants_match() {
        let msg = IntuiMsg {
            class: 0,
            code: 0,
            qualifier: 0,
            mouse_x: 0,
            mouse_y: 0,
        };

        // Verify each variant can be constructed and matched
        let e = Event::Close;
        assert!(matches!(e, Event::Close));

        let e = Event::GadgetUp(42);
        assert!(matches!(e, Event::GadgetUp(42)));

        let e = Event::Key(b'A');
        assert!(matches!(e, Event::Key(b'A')));

        let e = Event::Resize;
        assert!(matches!(e, Event::Resize));

        let e = Event::Refresh;
        assert!(matches!(e, Event::Refresh));

        let e = Event::Other(msg);
        assert!(matches!(e, Event::Other(_)));
    }
}
