//
// GUI Demo — AmigaOS 4 ReAction + Rust
//
// Demonstrates creating a window with ReAction gadgets:
// buttons, string input, checkbox, and layout management.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::boopsi::AmigaObject;
use amigaos4::tag::TagListBuilder;
use amigaos4::window::*;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// ---------------------------------------------------------------------------
// ReAction class names (null-terminated, matching SDK headers)
// ---------------------------------------------------------------------------

const LAYOUT_CLASS: &[u8] = b"layout.gadget\0";
const BUTTON_CLASS: &[u8] = b"button.gadget\0";
const CHECKBOX_CLASS: &[u8] = b"checkbox.gadget\0";
const STRING_CLASS: &[u8] = b"string.gadget\0";
const INTEGER_CLASS: &[u8] = b"integer.gadget\0";
const LABEL_CLASS: &[u8] = b"label.image\0";

// ---------------------------------------------------------------------------
// ReAction tag constants (from SDK gadgets/ headers)
// ---------------------------------------------------------------------------

// GA_ tags — gadgets/gadgetclass.h
// GA_Dummy = TAG_USER + 0x30000
const GA_DUMMY: u32 = amigaos4_sys::TAG_USER + 0x30000;
const GA_ID: u32 = GA_DUMMY + 0x0001;         // Gadget ID
const GA_TEXT: u32 = GA_DUMMY + 0x000F;        // Gadget label text
const GA_SELECTED: u32 = GA_DUMMY + 0x0005;    // Selected state (checkbox)
const GA_RELVERIFY: u32 = GA_DUMMY + 0x0006;   // Notify on release
const GA_TAB_CYCLE: u32 = GA_DUMMY + 0x000C;   // Tab-cycle participation

// LAYOUT_ tags — gadgets/layout.h
// LAYOUT_Dummy = TAG_USER + 0x40000
const LAYOUT_DUMMY: u32 = amigaos4_sys::TAG_USER + 0x40000;
const LAYOUT_ORIENTATION: u32 = LAYOUT_DUMMY + 0x0001;
const LAYOUT_ADD_CHILD: u32 = LAYOUT_DUMMY + 0x0006;
const LAYOUT_LABEL: u32 = LAYOUT_DUMMY + 0x0014;
const LAYOUT_MODIFY_CHILD: u32 = LAYOUT_DUMMY + 0x000D;
const LAYOUT_SPACE_INNER: u32 = LAYOUT_DUMMY + 0x0010;
const LAYOUT_SPACE_OUTER: u32 = LAYOUT_DUMMY + 0x0011;
const LAYOUT_BEVEL_STYLE: u32 = LAYOUT_DUMMY + 0x0012;

// LAYOUT_Orientation values
const LAYOUT_ORIENT_VERT: u32 = 1;
const LAYOUT_ORIENT_HORIZ: u32 = 0;

// CHILD_ tags — layout child weighting
const CHILD_DUMMY: u32 = amigaos4_sys::TAG_USER + 0x40100;
const CHILD_WEIGHT_EQUAL: u32 = CHILD_DUMMY + 0x000B;
const CHILD_MIN_WIDTH: u32 = CHILD_DUMMY + 0x0003;
const CHILD_MIN_HEIGHT: u32 = CHILD_DUMMY + 0x0004;

// STRINGA_ tags — gadgets/string.h
const STRINGA_DUMMY: u32 = amigaos4_sys::TAG_USER + 0x32000;
const STRINGA_MAX_CHARS: u32 = STRINGA_DUMMY + 0x0005;
const STRINGA_TEXT_VAL: u32 = STRINGA_DUMMY + 0x0002;

// INTEGER_ tags — gadgets/integer.h
const INTEGER_DUMMY: u32 = amigaos4_sys::TAG_USER + 0x43000;
const INTEGER_NUMBER: u32 = INTEGER_DUMMY + 0x0001;
const INTEGER_MINIMUM: u32 = INTEGER_DUMMY + 0x0003;
const INTEGER_MAXIMUM: u32 = INTEGER_DUMMY + 0x0004;
const INTEGER_ARROWS: u32 = INTEGER_DUMMY + 0x0005;

// BevelStyle — images/bevel.h
const BVS_GROUP: u32 = 0x0002;

// Gadget IDs
const GID_NAME: u32 = 1;
const GID_AGE: u32 = 2;
const GID_ENABLED: u32 = 3;
const GID_OK: u32 = 4;
const GID_CANCEL: u32 = 5;

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("=== GUI Demo ===");

    // Build the layout
    let layout = match build_ui() {
        Ok(l) => l,
        Err(e) => {
            amigaos4::serial_println!("Failed to create layout: {}", e);
            return 1;
        }
    };

    // Open a window with the layout as its root gadget
    let win_tags = TagListBuilder::new()
        .tag(WA_LEFT, 100)
        .tag(WA_TOP, 100)
        .tag(WA_WIDTH, 350)
        .tag(WA_HEIGHT, 200)
        .tag(WA_TITLE, b"Rust GUI Demo\0".as_ptr() as u32)
        .tag(WA_IDCMP, IDCMP_CLOSEWINDOW | IDCMP_GADGETUP | IDCMP_VANILLAKEY)
        .tag(WA_CLOSE_GADGET, 1)
        .tag(WA_DRAG_BAR, 1)
        .tag(WA_DEPTH_GADGET, 1)
        .tag(WA_SIZE_GADGET, 1)
        .tag(WA_ACTIVATE, 1)
        .tag(WA_GADGETS, layout.into_raw() as u32) // Transfer ownership to window
        .build();

    let win = match AmigaWindow::open(&win_tags) {
        Ok(w) => w,
        Err(e) => {
            amigaos4::serial_println!("Failed to open window: {}", e);
            return 1;
        }
    };

    amigaos4::serial_println!("Window opened. Entering event loop...");

    // Event loop
    loop {
        let msg = win.wait_msg();

        match msg.class {
            IDCMP_CLOSEWINDOW => {
                amigaos4::serial_println!("Close gadget clicked. Exiting.");
                break;
            }

            IDCMP_GADGETUP => {
                let gadget_id = msg.code as u32;
                match gadget_id {
                    GID_OK => {
                        amigaos4::serial_println!("OK button clicked!");
                        break;
                    }
                    GID_CANCEL => {
                        amigaos4::serial_println!("Cancel button clicked!");
                        break;
                    }
                    GID_NAME => {
                        amigaos4::serial_println!("Name field changed (gadget {})", gadget_id);
                    }
                    GID_AGE => {
                        amigaos4::serial_println!("Age field changed (gadget {})", gadget_id);
                    }
                    GID_ENABLED => {
                        amigaos4::serial_println!("Checkbox toggled (gadget {})", gadget_id);
                    }
                    _ => {
                        amigaos4::serial_println!("Unknown gadget: {}", gadget_id);
                    }
                }
            }

            IDCMP_VANILLAKEY => {
                let ch = msg.code as u8;
                amigaos4::serial_println!("Key pressed: '{}' ({})", ch as char, ch);
                if ch == 27 {
                    // ESC
                    amigaos4::serial_println!("ESC pressed. Exiting.");
                    break;
                }
            }

            _ => {}
        }
    }

    amigaos4::serial_println!("=== GUI Demo complete ===");
    0
}

// ---------------------------------------------------------------------------
// UI construction
// ---------------------------------------------------------------------------

/// Build the complete ReAction gadget layout.
///
/// The layout tree looks like:
///
/// ```text
/// Vertical layout (main)
///   +-- "Name:" label + String gadget
///   +-- "Age:" label + Integer gadget (with arrows, 0..150)
///   +-- "Enabled:" label + Checkbox gadget
///   +-- Horizontal layout (button row)
///         +-- [OK] button
///         +-- [Cancel] button
/// ```
fn build_ui() -> amigaos4::Result<AmigaObject> {
    // ---- Individual gadgets ----

    // String gadget for name entry
    let name_gad = create_string_gadget(GID_NAME, b"\0", 64)?;

    // Integer gadget for age
    let age_gad = create_integer_gadget(GID_AGE, 25, 0, 150)?;

    // Checkbox for enabled/disabled toggle
    let enabled_gad = create_checkbox(GID_ENABLED, true)?;

    // OK and Cancel buttons
    let ok_btn = create_button(GID_OK, b"_OK\0")?;
    let cancel_btn = create_button(GID_CANCEL, b"_Cancel\0")?;

    // ---- Button row (horizontal layout) ----
    let button_row_tags = TagListBuilder::new()
        .tag(LAYOUT_ORIENTATION, LAYOUT_ORIENT_HORIZ)
        .tag(LAYOUT_SPACE_INNER, 1)
        .tag(LAYOUT_ADD_CHILD, ok_btn.into_raw() as u32)
        .tag(CHILD_WEIGHT_EQUAL, 1)
        .tag(LAYOUT_ADD_CHILD, cancel_btn.into_raw() as u32)
        .tag(CHILD_WEIGHT_EQUAL, 1)
        .build();

    let button_row = AmigaObject::new(LAYOUT_CLASS, &button_row_tags)?;

    // ---- Main layout (vertical) ----
    let main_tags = TagListBuilder::new()
        .tag(LAYOUT_ORIENTATION, LAYOUT_ORIENT_VERT)
        .tag(LAYOUT_SPACE_OUTER, 1)
        .tag(LAYOUT_SPACE_INNER, 1)
        .tag(LAYOUT_BEVEL_STYLE, BVS_GROUP)
        .tag(LAYOUT_LABEL, b"Preferences\0".as_ptr() as u32)
        // Name field with label
        .tag(LAYOUT_ADD_CHILD, name_gad.into_raw() as u32)
        .tag(CHILD_MIN_WIDTH, 200)
        .tag(LAYOUT_LABEL, b"Name:\0".as_ptr() as u32)
        // Age field with label
        .tag(LAYOUT_ADD_CHILD, age_gad.into_raw() as u32)
        .tag(CHILD_MIN_WIDTH, 80)
        .tag(LAYOUT_LABEL, b"Age:\0".as_ptr() as u32)
        // Checkbox with label
        .tag(LAYOUT_ADD_CHILD, enabled_gad.into_raw() as u32)
        .tag(LAYOUT_LABEL, b"Enabled:\0".as_ptr() as u32)
        // Button row
        .tag(LAYOUT_ADD_CHILD, button_row.into_raw() as u32)
        .build();

    let main_layout = AmigaObject::new(LAYOUT_CLASS, &main_tags)?;

    amigaos4::serial_println!("Layout built successfully.");
    Ok(main_layout)
}

// ---------------------------------------------------------------------------
// Gadget creation helpers
// ---------------------------------------------------------------------------

/// Create a ReAction button gadget.
fn create_button(id: u32, label: &[u8]) -> amigaos4::Result<AmigaObject> {
    let tags = TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_TEXT, label.as_ptr() as u32)
        .tag(GA_RELVERIFY, 1)
        .tag(GA_TAB_CYCLE, 1)
        .build();

    AmigaObject::new(BUTTON_CLASS, &tags)
}

/// Create a ReAction string gadget.
fn create_string_gadget(id: u32, initial: &[u8], max_chars: u32) -> amigaos4::Result<AmigaObject> {
    let tags = TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(GA_TAB_CYCLE, 1)
        .tag(STRINGA_TEXT_VAL, initial.as_ptr() as u32)
        .tag(STRINGA_MAX_CHARS, max_chars)
        .build();

    AmigaObject::new(STRING_CLASS, &tags)
}

/// Create a ReAction integer gadget with arrows.
fn create_integer_gadget(
    id: u32,
    value: u32,
    min: u32,
    max: u32,
) -> amigaos4::Result<AmigaObject> {
    let tags = TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(GA_TAB_CYCLE, 1)
        .tag(INTEGER_NUMBER, value)
        .tag(INTEGER_MINIMUM, min)
        .tag(INTEGER_MAXIMUM, max)
        .tag(INTEGER_ARROWS, 1)
        .build();

    AmigaObject::new(INTEGER_CLASS, &tags)
}

/// Create a ReAction checkbox gadget.
fn create_checkbox(id: u32, checked: bool) -> amigaos4::Result<AmigaObject> {
    let tags = TagListBuilder::new()
        .tag(GA_ID, id)
        .tag(GA_RELVERIFY, 1)
        .tag(GA_TAB_CYCLE, 1)
        .tag(GA_SELECTED, if checked { 1 } else { 0 })
        .build();

    AmigaObject::new(CHECKBOX_CLASS, &tags)
}
