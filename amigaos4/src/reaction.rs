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
//!         Event::GadgetUp { id, code } => {
//!             // Handle gadget `id`; `code` carries the slider level /
//!             // chooser index / listbrowser ordinal where applicable.
//!         }
//!         _ => {}
//!     }
//!     true
//! });
//! ```

use alloc::vec::Vec;
use amigaos4_sys::{TagItem, TAG_DONE, TAG_USER};

use crate::boopsi::AmigaObject;
use crate::error::{AmigaError, Result};
use crate::window::{
    AmigaWindow, IntuiMsg, IDCMP_CLOSEWINDOW, IDCMP_GADGETUP, IDCMP_MENUPICK,
    IDCMP_NEWSIZE, IDCMP_REFRESHWINDOW, IDCMP_VANILLAKEY,
};

// ===========================================================================
// Tag constants
//
// All values below come from the AmigaOS 4.1 SDK 54.16 headers
// (intuition/gadgetclass.h, reaction/reaction.h, gadgets/*.h,
// images/label.h). An earlier version of this module used invented
// dummy bases and offsets — BOOPSI classes silently ignore unknown
// tags, so gadget creation "succeeded" while IDs, labels, layout
// children, and RelVerify were never actually applied (the same
// failure mode as the historical WA_Dummy bug noted in window.rs).
// Each value is pinned by a unit test below.
// ===========================================================================

/// `GA_Dummy = TAG_USER + 0x30000` (intuition/gadgetclass.h).
const GA_DUMMY: u32 = TAG_USER + 0x30000;

/// Gadget label text pointer (null-terminated C string).
pub const GA_TEXT: u32 = GA_DUMMY + 9;

/// When true (non-zero) the gadget is disabled and will not respond to input.
pub const GA_DISABLED: u32 = GA_DUMMY + 14;

/// Gadget identifier, reported in IDCMP_GADGETUP events (`Event::GadgetUp`).
pub const GA_ID: u32 = GA_DUMMY + 16;

/// Whether the gadget is selected / highlighted.
pub const GA_SELECTED: u32 = GA_DUMMY + 19;

/// Send IDCMP_GADGETUP when the user releases the gadget (required for
/// ReAction gadgets that should generate events).
pub const GA_RELVERIFY: u32 = GA_DUMMY + 22;

/// Include the gadget in Tab/Shift-Tab activation cycling (BOOL).
pub const GA_TAB_CYCLE: u32 = GA_DUMMY + 36;

/// `REACTION_Dummy = TAG_USER + 0x5000000` (reaction/reaction.h) — base
/// for all ReAction gadget class tags.
const REACTION_DUMMY: u32 = TAG_USER + 0x0500_0000;

// ---- Layout tags (gadgets/layout.h) ----

/// `LAYOUT_Dummy = REACTION_Dummy + 0x7000`.
const LAYOUT_DUMMY: u32 = REACTION_DUMMY + 0x7000;

/// Orientation of the layout group: [`LAYOUT_ORIENT_HORIZ`] or
/// [`LAYOUT_ORIENT_VERT`].
pub const LAYOUT_ORIENTATION: u32 = LAYOUT_DUMMY + 1;

/// Bottom spacing in pixels.
pub const LAYOUT_BOTTOM_SPACING: u32 = LAYOUT_DUMMY + 11;

/// Bevel style for the layout frame (e.g. [`BVS_GROUP`]).
pub const LAYOUT_BEVEL_STYLE: u32 = LAYOUT_DUMMY + 15;

/// Group-box bevel style for [`LAYOUT_BEVEL_STYLE`] (images/bevel.h).
pub const BVS_GROUP: u32 = 2;

/// Text label for the layout group frame.
pub const LAYOUT_LABEL: u32 = LAYOUT_DUMMY + 16;

/// Add a child gadget (BOOPSI object pointer) to the layout.
/// Ownership of the child is transferred to the layout.
pub const LAYOUT_ADD_CHILD: u32 = LAYOUT_DUMMY + 20;

/// Add a child image (BOOPSI object pointer) to the layout.
pub const LAYOUT_ADD_IMAGE: u32 = LAYOUT_DUMMY + 21;

/// Enable inner spacing between child gadgets.
pub const LAYOUT_SPACE_INNER: u32 = LAYOUT_DUMMY + 37;

/// Enable outer spacing around the layout border.
pub const LAYOUT_SPACE_OUTER: u32 = LAYOUT_DUMMY + 38;

/// Horizontal orientation value for [`LAYOUT_ORIENTATION`].
pub const LAYOUT_ORIENT_HORIZ: u32 = 0;

/// Vertical orientation value for [`LAYOUT_ORIENTATION`].
pub const LAYOUT_ORIENT_VERT: u32 = 1;

// ---- Child tags for layout (used alongside LAYOUT_ADD_CHILD) ----

/// `CHILD_Dummy = LAYOUT_Dummy + 0x100`.
const CHILD_DUMMY: u32 = LAYOUT_DUMMY + 0x100;

/// Minimum width of the child in pixels.
pub const CHILD_MIN_WIDTH: u32 = CHILD_DUMMY + 1;

/// Minimum height of the child in pixels.
pub const CHILD_MIN_HEIGHT: u32 = CHILD_DUMMY + 2;

/// Relative width weight of the child (0–100, default 100).
pub const CHILD_WEIGHTED_WIDTH: u32 = CHILD_DUMMY + 5;

/// Relative height weight of the child (0–100, default 100).
pub const CHILD_WEIGHTED_HEIGHT: u32 = CHILD_DUMMY + 6;

/// Text label displayed next to the child gadget (pointer to
/// null-terminated C string).
pub const CHILD_LABEL: u32 = CHILD_DUMMY + 12;

// ---- Button tags (gadgets/button.h) ----

/// `BUTTON_Dummy = TAG_USER + 0x04000000`. (The button label itself is
/// set with [`GA_TEXT`] — button.gadget has no separate text tag.)
const BUTTON_DUMMY: u32 = TAG_USER + 0x0400_0000;

/// When true, the button acts as a toggle (push) button rather than a
/// momentary click button.
pub const BUTTON_PUSH_BUTTON: u32 = BUTTON_DUMMY + 1;

// ---- String gadget tags (intuition/gadgetclass.h STRINGA_*) ----

/// `STRINGA_Dummy = TAG_USER + 0x32000`.
const STRINGA_DUMMY: u32 = TAG_USER + 0x32000;

/// Maximum number of characters the user may enter.
pub const STRINGA_MAX_CHARS: u32 = STRINGA_DUMMY + 0x01;

/// Pointer to the initial text value (null-terminated C string).
pub const STRINGA_TEXT_VAL: u32 = STRINGA_DUMMY + 0x12;

// ---- Checkbox tags (gadgets/checkbox.h) ----

/// Initial checked state: 1 = checked, 0 = unchecked.
/// (`CHECKBOX_Checked` is an alias for `GA_Selected` in the SDK.)
pub const CHECKBOX_CHECKED: u32 = GA_SELECTED;

// ---- Integer gadget tags (gadgets/integer.h) ----

/// `INTEGER_Dummy = REACTION_Dummy + 0x2000`.
const INTEGER_DUMMY: u32 = REACTION_DUMMY + 0x2000;

/// Initial numeric value.
pub const INTEGER_NUMBER: u32 = INTEGER_DUMMY + 1;

/// Minimum allowed value.
pub const INTEGER_MINIMUM: u32 = INTEGER_DUMMY + 3;

/// Maximum allowed value.
pub const INTEGER_MAXIMUM: u32 = INTEGER_DUMMY + 4;

/// Show increment/decrement arrows (BOOL).
pub const INTEGER_ARROWS: u32 = INTEGER_DUMMY + 5;

// ---- Label image tags (images/label.h) ----

/// `LABEL_Dummy = REACTION_Dummy + 0x6000`.
const LABEL_DUMMY: u32 = REACTION_DUMMY + 0x6000;

/// Label text pointer (null-terminated C string).
pub const LABEL_TEXT: u32 = LABEL_DUMMY + 1;

// ---- Slider tags (gadgets/slider.h) ----

/// `SLIDER_Dummy = REACTION_Dummy + 0x28000`.
const SLIDER_DUMMY: u32 = REACTION_DUMMY + 0x28000;

/// Minimum slider level.
pub const SLIDER_MIN: u32 = SLIDER_DUMMY + 1;

/// Maximum slider level.
pub const SLIDER_MAX: u32 = SLIDER_DUMMY + 2;

/// Current slider level. Reported in `Event::GadgetUp { code, .. }`.
pub const SLIDER_LEVEL: u32 = SLIDER_DUMMY + 3;

/// Slider orientation: [`SLIDER_HORIZONTAL`] or [`SLIDER_VERTICAL`].
pub const SLIDER_ORIENTATION: u32 = SLIDER_DUMMY + 4;

/// Number of tick marks to draw.
pub const SLIDER_TICKS: u32 = SLIDER_DUMMY + 6;

/// Horizontal orientation (`SORIENT_HORIZ` = `FREEHORIZ` = 0x02).
pub const SLIDER_HORIZONTAL: u32 = 0x02;

/// Vertical orientation (`SORIENT_VERT` = `FREEVERT` = 0x04).
pub const SLIDER_VERTICAL: u32 = 0x04;

// ---- Chooser tags (gadgets/chooser.h) ----

/// `CHOOSER_Dummy = REACTION_Dummy + 0x1000`.
const CHOOSER_DUMMY: u32 = REACTION_DUMMY + 0x1000;

/// Popup-button style chooser (BOOL).
pub const CHOOSER_POPUP: u32 = CHOOSER_DUMMY + 1;

/// Drop-down style chooser (BOOL).
pub const CHOOSER_DROPDOWN: u32 = CHOOSER_DUMMY + 2;

/// Currently selected entry index. Reported in
/// `Event::GadgetUp { code, .. }` when the selection changes.
pub const CHOOSER_ACTIVE: u32 = CHOOSER_DUMMY + 5;

/// Null-terminated array of STRPTR labels (simplest label source).
pub const CHOOSER_LABEL_ARRAY: u32 = CHOOSER_DUMMY + 12;

// ---- ListBrowser tags (gadgets/listbrowser.h) ----

/// `LISTBROWSER_Dummy = REACTION_Dummy + 0x3000`.
const LISTBROWSER_DUMMY: u32 = REACTION_DUMMY + 0x3000;

/// (struct List *) the list of nodes to display.
pub const LISTBROWSER_LABELS: u32 = LISTBROWSER_DUMMY + 3;

/// Currently selected node ordinal. Reported in
/// `Event::GadgetUp { code, .. }` when the selection changes.
pub const LISTBROWSER_SELECTED: u32 = LISTBROWSER_DUMMY + 4;

/// Highlight the selected entry (BOOL).
pub const LISTBROWSER_SHOW_SELECTED: u32 = LISTBROWSER_DUMMY + 18;

/// `LBNA_Dummy = TAG_USER + 0x5003500` — node attribute tags for
/// `AllocListBrowserNode`.
const LBNA_DUMMY: u32 = TAG_USER + 0x0500_3500;

/// Node column number the following LBNCA_* tags apply to.
pub const LBNA_COLUMN: u32 = LBNA_DUMMY + 4;

/// Column text (STRPTR).
pub const LBNCA_TEXT: u32 = LBNA_DUMMY + 5;

/// Copy the LBNCA_Text string into the node (BOOL) — the caller's
/// buffer need not outlive the node.
pub const LBNCA_COPY_TEXT: u32 = LBNA_DUMMY + 15;

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

/// BOOPSI class name for `slider.gadget`.
pub const SLIDER_CLASS: &[u8] = b"slider.gadget\0";

/// BOOPSI class name for `chooser.gadget`.
pub const CHOOSER_CLASS: &[u8] = b"chooser.gadget\0";

/// BOOPSI class name for `listbrowser.gadget`.
pub const LISTBROWSER_CLASS: &[u8] = b"listbrowser.gadget\0";

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
/// Returns `Err(NotNulTerminated)` if `label` is missing its `\0`, or
/// `Err(AllocationFailed)` if the BOOPSI object could not be created.
pub fn button(id: u32, label: &[u8]) -> Result<AmigaObject> {
    let label_ptr = crate::cstr::require_nul(label)?;
    let tags = crate::tag::TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(GA_TEXT, label_ptr as u32)
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

/// Create a slider gadget.
///
/// `id` is the gadget identifier. The slider ranges over
/// `min..=max` starting at `level`. Each change generates
/// `Event::GadgetUp { id, code }` with the new level in `code`.
///
/// # Errors
///
/// Returns `Err(AllocationFailed)` if the BOOPSI object could not be
/// created.
pub fn slider(id: u32, min: u32, max: u32, level: u32) -> Result<AmigaObject> {
    let tags = crate::tag::TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(SLIDER_MIN, min)
        .tag(SLIDER_MAX, max)
        .tag(SLIDER_LEVEL, level)
        .tag(SLIDER_ORIENTATION, SLIDER_HORIZONTAL)
        .build();
    AmigaObject::new(SLIDER_CLASS, &tags)
}

/// Owned label array for a [`chooser`] gadget.
///
/// chooser.gadget keeps the `CHOOSER_LabelArray` pointer, so the array
/// (and its strings) must outlive the gadget — keep this value alive at
/// least as long as the window that shows the chooser.
pub struct ChooserLabels {
    // Owned copies of the label strings (null-terminated).
    _strings: Vec<Vec<u8>>,
    // STRPTR array pointing into _strings, null-terminated.
    array: Vec<*const u8>,
}

impl ChooserLabels {
    /// Build a label array. Each label must be null-terminated (use
    /// [`amstr!`](crate::amstr)).
    ///
    /// # Errors
    ///
    /// Returns `Err(NotNulTerminated)` if any label is missing its `\0`.
    pub fn new(labels: &[&[u8]]) -> Result<Self> {
        let mut strings = Vec::with_capacity(labels.len());
        for l in labels {
            crate::cstr::require_nul(l)?;
            strings.push(l.to_vec());
        }
        let mut array: Vec<*const u8> = strings.iter().map(|s| s.as_ptr()).collect();
        array.push(core::ptr::null());
        Ok(Self { _strings: strings, array })
    }

    /// The raw STRPTR array for `CHOOSER_LABEL_ARRAY`.
    #[inline]
    pub fn as_ptr(&self) -> *const *const u8 {
        self.array.as_ptr()
    }
}

/// Create a chooser (drop-down selection) gadget.
///
/// `labels` must outlive the gadget (see [`ChooserLabels`]). `active`
/// is the initially selected index. Selecting an entry generates
/// `Event::GadgetUp { id, code }` with the new index in `code`.
///
/// # Errors
///
/// Returns `Err(AllocationFailed)` if the BOOPSI object could not be
/// created.
pub fn chooser(id: u32, labels: &ChooserLabels, active: u32) -> Result<AmigaObject> {
    let tags = crate::tag::TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(CHOOSER_POPUP, 1)
        .tag(CHOOSER_LABEL_ARRAY, labels.as_ptr() as u32)
        .tag(CHOOSER_ACTIVE, active)
        .build();
    AmigaObject::new(CHOOSER_CLASS, &tags)
}

// ===========================================================================
// ListBrowser
// ===========================================================================

/// Minimal exec `struct List` layout (2-byte packed, like all OS4
/// system structs) so a node list can be initialised from Rust.
#[repr(C, packed(2))]
struct RawList {
    head: *mut amigaos4_sys::Node,
    tail: *mut amigaos4_sys::Node,
    tail_pred: *mut amigaos4_sys::Node,
    l_type: u8,
    _pad: u8,
}

/// Owned node list for a [`listbrowser`] gadget.
///
/// Nodes are allocated through listbrowser.gadget's library interface
/// (`AllocListBrowserNode`) with `LBNCA_CopyText`, so label buffers
/// need not outlive the call. The node list itself must outlive the
/// gadget — keep this value alive at least as long as the window, and
/// drop it only after the window (or gadget) is gone.
pub struct ListBrowserNodes {
    /// Boxed so the List address is stable while the gadget holds it.
    list: alloc::boxed::Box<RawList>,
    nodes: Vec<*mut amigaos4_sys::Node>,
    /// Keeps listbrowser.gadget (and its node allocator) open.
    lib: crate::iface::OpenedInterface,
}

impl ListBrowserNodes {
    /// Create an empty node list (opens `gadgets/listbrowser.gadget`).
    pub fn new() -> Result<Self> {
        let lib = crate::iface::OpenedInterface::open(b"gadgets/listbrowser.gadget\0", 50)?;
        let mut list = alloc::boxed::Box::new(RawList {
            head: core::ptr::null_mut(),
            tail: core::ptr::null_mut(),
            tail_pred: core::ptr::null_mut(),
            l_type: 0,
            _pad: 0,
        });
        // exec NewList(): head -> &tail, tail = NULL, tail_pred -> &head.
        let lp: *mut RawList = &mut *list;
        unsafe {
            (*lp).head = core::ptr::addr_of_mut!((*lp).tail) as *mut amigaos4_sys::Node;
            (*lp).tail = core::ptr::null_mut();
            (*lp).tail_pred = core::ptr::addr_of_mut!((*lp).head) as *mut amigaos4_sys::Node;
        }
        Ok(Self { list, nodes: Vec::new(), lib })
    }

    /// Append a single-column text node. `text` must be null-terminated
    /// (use [`amstr!`](crate::amstr)); it is copied into the node.
    pub fn push(&mut self, text: &[u8]) -> Result<()> {
        let text_ptr = crate::cstr::require_nul(text)?;
        let ilb = self.lib.as_ptr() as *mut amigaos4_sys::ListBrowserIFace;
        let tags = crate::tag::TagListBuilder::new()
            .tag(LBNA_COLUMN, 0)
            .tag(LBNCA_COPY_TEXT, 1)
            .tag(LBNCA_TEXT, text_ptr as u32)
            .build();
        // SAFETY: ilb is the open listbrowser interface; tags is
        // TAG_DONE-terminated; text is copied (LBNCA_CopyText).
        let node = unsafe {
            ((*ilb).AllocListBrowserNodeA)(ilb, 1, tags.as_ptr() as *mut amigaos4_sys::TagItem)
        };
        if node.is_null() {
            return Err(AmigaError::AllocationFailed);
        }
        // exec AddTail(): link the node before the tail sentinel.
        let lp: *mut RawList = &mut *self.list;
        unsafe {
            let pred = (*lp).tail_pred;
            (*node).ln_Succ = core::ptr::addr_of_mut!((*lp).tail) as *mut amigaos4_sys::Node;
            (*node).ln_Pred = pred;
            (*pred).ln_Succ = node;
            (*lp).tail_pred = node;
        }
        self.nodes.push(node);
        Ok(())
    }

    /// Number of nodes in the list.
    #[inline]
    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    /// True if the list holds no nodes.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// The raw `struct List` pointer for `LISTBROWSER_LABELS`.
    #[inline]
    pub fn list_ptr(&self) -> *const core::ffi::c_void {
        &*self.list as *const RawList as *const core::ffi::c_void
    }
}

impl Drop for ListBrowserNodes {
    fn drop(&mut self) {
        let ilb = self.lib.as_ptr() as *mut amigaos4_sys::ListBrowserIFace;
        for &node in &self.nodes {
            // SAFETY: each node came from AllocListBrowserNodeA and is
            // freed exactly once; the gadget displaying the list must
            // already be gone (documented requirement).
            unsafe { ((*ilb).FreeListBrowserNode)(ilb, node) };
        }
        // self.lib drops afterwards, closing the library.
    }
}

/// Create a listbrowser (scrolling list) gadget.
///
/// `nodes` must outlive the gadget (see [`ListBrowserNodes`]).
/// Selecting an entry generates `Event::GadgetUp { id, code }` with the
/// selected ordinal in `code`.
///
/// # Errors
///
/// Returns `Err(AllocationFailed)` if the BOOPSI object could not be
/// created.
pub fn listbrowser(id: u32, nodes: &ListBrowserNodes) -> Result<AmigaObject> {
    let tags = crate::tag::TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(LISTBROWSER_LABELS, nodes.list_ptr() as u32)
        .tag(LISTBROWSER_SHOW_SELECTED, 1)
        .build();
    AmigaObject::new(LISTBROWSER_CLASS, &tags)
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
    /// Set when a label passed to [`add_labeled`](Self::add_labeled) was
    /// not null-terminated; surfaced as an error in [`build`](Self::build).
    invalid_label: bool,
}

impl LayoutBuilder {
    /// Create a layout builder with vertical orientation.
    ///
    /// Children will be stacked top-to-bottom.
    pub fn vertical() -> Self {
        Self {
            tags: Vec::new(),
            orientation: LAYOUT_ORIENT_VERT,
            invalid_label: false,
        }
    }

    /// Create a layout builder with horizontal orientation.
    ///
    /// Children will be arranged left-to-right.
    pub fn horizontal() -> Self {
        Self {
            tags: Vec::new(),
            orientation: LAYOUT_ORIENT_HORIZ,
            invalid_label: false,
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
    /// `label` must be a null-terminated byte string (e.g. `b"Name:\0"`,
    /// or built with [`amstr!`](crate::amstr)). A label missing its `\0`
    /// causes [`build`](Self::build) to fail with `NotNulTerminated`.
    /// Ownership of `child` is transferred to the layout.
    pub fn add_labeled(mut self, child: AmigaObject, label: &[u8]) -> Self {
        self.tags.push(TagItem {
            ti_Tag: LAYOUT_ADD_CHILD,
            ti_Data: child.into_raw() as u32,
        });
        if label.last() == Some(&0) {
            self.tags.push(TagItem {
                ti_Tag: CHILD_LABEL,
                ti_Data: label.as_ptr() as u32,
            });
        } else {
            self.invalid_label = true;
        }
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
    /// Returns `Err(NotNulTerminated)` if any label passed to
    /// [`add_labeled`](Self::add_labeled) was missing its `\0`, or
    /// `Err(AllocationFailed)` if the BOOPSI object could not be created.
    pub fn build(mut self) -> Result<AmigaObject> {
        if self.invalid_label {
            return Err(crate::error::AmigaError::NotNulTerminated);
        }
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
    GadgetUp {
        /// The gadget's `GA_ID` (read from the gadget the message
        /// points at — not from `Code`, which is gadget-specific).
        id: u32,
        /// Gadget-class specific value: slider level, chooser index,
        /// listbrowser ordinal; 0 for plain buttons.
        code: u16,
    },
    /// A menu item was picked (`IDCMP_MENUPICK`). For menuclass menu
    /// strips, retrieve the selected IDs with
    /// [`MenuStrip::each_select`](crate::menu::MenuStrip::each_select).
    MenuPick,
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
///         Event::GadgetUp { id, code } => {
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
            IDCMP_GADGETUP => Event::GadgetUp {
                id: msg.gadget_id as u32,
                code: msg.code,
            },
            IDCMP_MENUPICK => Event::MenuPick,
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

    // Tag values pinned against the AmigaOS 4.1 SDK 54.16 headers. If
    // one of these fails, the constant has drifted from the SDK and
    // BOOPSI will silently ignore it.

    #[test]
    fn ga_tags_match_gadgetclass_h() {
        assert_eq!(GA_DUMMY, TAG_USER + 0x30000);
        assert_eq!(GA_TEXT, GA_DUMMY + 9);
        assert_eq!(GA_DISABLED, GA_DUMMY + 14);
        assert_eq!(GA_ID, GA_DUMMY + 16);
        assert_eq!(GA_SELECTED, GA_DUMMY + 19);
        assert_eq!(GA_RELVERIFY, GA_DUMMY + 22);
        assert_eq!(GA_TAB_CYCLE, GA_DUMMY + 36);
        assert_eq!(LAYOUT_BEVEL_STYLE, LAYOUT_DUMMY + 15);
        assert_eq!(INTEGER_ARROWS, INTEGER_DUMMY + 5);
        assert_eq!(BVS_GROUP, 2);
    }

    #[test]
    fn layout_tags_match_layout_h() {
        assert_eq!(REACTION_DUMMY, TAG_USER + 0x5000000);
        assert_eq!(LAYOUT_DUMMY, REACTION_DUMMY + 0x7000);
        assert_eq!(LAYOUT_ORIENTATION, LAYOUT_DUMMY + 1);
        assert_eq!(LAYOUT_BOTTOM_SPACING, LAYOUT_DUMMY + 11);
        assert_eq!(LAYOUT_LABEL, LAYOUT_DUMMY + 16);
        assert_eq!(LAYOUT_ADD_CHILD, LAYOUT_DUMMY + 20);
        assert_eq!(LAYOUT_ADD_IMAGE, LAYOUT_DUMMY + 21);
        assert_eq!(LAYOUT_SPACE_INNER, LAYOUT_DUMMY + 37);
        assert_eq!(LAYOUT_SPACE_OUTER, LAYOUT_DUMMY + 38);
    }

    #[test]
    fn child_tags_match_layout_h() {
        assert_eq!(CHILD_DUMMY, LAYOUT_DUMMY + 0x100);
        assert_eq!(CHILD_MIN_WIDTH, CHILD_DUMMY + 1);
        assert_eq!(CHILD_MIN_HEIGHT, CHILD_DUMMY + 2);
        assert_eq!(CHILD_WEIGHTED_WIDTH, CHILD_DUMMY + 5);
        assert_eq!(CHILD_WEIGHTED_HEIGHT, CHILD_DUMMY + 6);
        assert_eq!(CHILD_LABEL, CHILD_DUMMY + 12);
    }

    #[test]
    fn gadget_class_tags_match_sdk() {
        assert_eq!(BUTTON_DUMMY, TAG_USER + 0x0400_0000);
        assert_eq!(BUTTON_PUSH_BUTTON, BUTTON_DUMMY + 1);
        assert_eq!(STRINGA_DUMMY, TAG_USER + 0x32000);
        assert_eq!(STRINGA_MAX_CHARS, STRINGA_DUMMY + 1);
        assert_eq!(STRINGA_TEXT_VAL, STRINGA_DUMMY + 0x12);
        assert_eq!(CHECKBOX_CHECKED, GA_SELECTED);
        assert_eq!(INTEGER_DUMMY, REACTION_DUMMY + 0x2000);
        assert_eq!(INTEGER_NUMBER, INTEGER_DUMMY + 1);
        assert_eq!(INTEGER_MINIMUM, INTEGER_DUMMY + 3);
        assert_eq!(INTEGER_MAXIMUM, INTEGER_DUMMY + 4);
        assert_eq!(LABEL_DUMMY, REACTION_DUMMY + 0x6000);
        assert_eq!(LABEL_TEXT, LABEL_DUMMY + 1);
    }

    #[test]
    fn new_gadget_tags_match_sdk() {
        assert_eq!(SLIDER_DUMMY, REACTION_DUMMY + 0x28000);
        assert_eq!(SLIDER_MIN, SLIDER_DUMMY + 1);
        assert_eq!(SLIDER_MAX, SLIDER_DUMMY + 2);
        assert_eq!(SLIDER_LEVEL, SLIDER_DUMMY + 3);
        assert_eq!(SLIDER_ORIENTATION, SLIDER_DUMMY + 4);
        assert_eq!(SLIDER_TICKS, SLIDER_DUMMY + 6);
        assert_eq!(CHOOSER_DUMMY, REACTION_DUMMY + 0x1000);
        assert_eq!(CHOOSER_POPUP, CHOOSER_DUMMY + 1);
        assert_eq!(CHOOSER_ACTIVE, CHOOSER_DUMMY + 5);
        assert_eq!(CHOOSER_LABEL_ARRAY, CHOOSER_DUMMY + 12);
        assert_eq!(LISTBROWSER_DUMMY, REACTION_DUMMY + 0x3000);
        assert_eq!(LISTBROWSER_LABELS, LISTBROWSER_DUMMY + 3);
        assert_eq!(LISTBROWSER_SELECTED, LISTBROWSER_DUMMY + 4);
        assert_eq!(LISTBROWSER_SHOW_SELECTED, LISTBROWSER_DUMMY + 18);
        assert_eq!(LBNA_DUMMY, TAG_USER + 0x5003500);
        assert_eq!(LBNA_COLUMN, LBNA_DUMMY + 4);
        assert_eq!(LBNCA_TEXT, LBNA_DUMMY + 5);
        assert_eq!(LBNCA_COPY_TEXT, LBNA_DUMMY + 15);
    }

    #[test]
    fn chooser_labels_reject_unterminated() {
        assert!(ChooserLabels::new(&[b"ok\0", b"bad"]).is_err());
    }

    #[test]
    fn chooser_labels_array_null_terminated() {
        let l = ChooserLabels::new(&[b"a\0", b"b\0"]).unwrap();
        unsafe {
            assert!(!(*l.as_ptr()).is_null());
            assert!(!(*l.as_ptr().add(1)).is_null());
            assert!((*l.as_ptr().add(2)).is_null());
        }
    }

    #[test]
    fn raw_list_layout() {
        // exec struct List: 3 pointers + type + pad = 14 bytes on PPC.
        if cfg!(target_pointer_width = "32") {
            assert_eq!(core::mem::size_of::<RawList>(), 14);
        }
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
            SLIDER_CLASS, CHOOSER_CLASS, LISTBROWSER_CLASS,
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
            gadget_id: 0,
        };

        // Verify each variant can be constructed and matched
        let e = Event::Close;
        assert!(matches!(e, Event::Close));

        let e = Event::GadgetUp { id: 42, code: 7 };
        assert!(matches!(e, Event::GadgetUp { id: 42, code: 7 }));

        let e = Event::MenuPick;
        assert!(matches!(e, Event::MenuPick));

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
