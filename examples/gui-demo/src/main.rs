//
// GUI Demo — AmigaOS 4 ReAction + Rust
//
// Demonstrates creating a window with ReAction gadgets — buttons,
// string input, integer input, checkbox, slider, chooser — plus a
// menuclass menu bar, all via the amigaos4 crate's safe wrappers.
//

#![no_std]
#![no_main]

extern crate alloc;

use core::panic::PanicInfo;
use amigaos4_alloc::Clib4Allocator;
use amigaos4::amstr;
use amigaos4::boopsi::AmigaObject;
use amigaos4::menu::MenuBuilder;
use amigaos4::reaction::*;
use amigaos4::tag::TagListBuilder;
use amigaos4::window::*;

#[global_allocator]
static ALLOCATOR: Clib4Allocator = Clib4Allocator;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    amigaos4::panic::default_panic_handler(info)
}

// Gadget IDs
const GID_NAME: u32 = 1;
const GID_AGE: u32 = 2;
const GID_ENABLED: u32 = 3;
const GID_OK: u32 = 4;
const GID_CANCEL: u32 = 5;
const GID_VOLUME: u32 = 6;
const GID_MODE: u32 = 7;

// Menu item IDs (must be non-zero)
const MID_ABOUT: u32 = 100;
const MID_QUIT: u32 = 101;

// ---------------------------------------------------------------------------
// Entry point
// ---------------------------------------------------------------------------

#[no_mangle]
pub extern "C" fn main(_argc: i32, _argv: *const *const u8) -> i32 {
    amigaos4::serial_println!("=== GUI Demo ===");

    // Chooser labels must outlive the gadget (and so the window).
    let mode_labels = match ChooserLabels::new(&[
        amstr!("Beginner"),
        amstr!("Advanced"),
        amstr!("Expert"),
    ]) {
        Ok(l) => l,
        Err(e) => {
            amigaos4::serial_println!("Failed to build chooser labels: {}", e);
            return 1;
        }
    };

    // Build the layout
    let layout = match build_ui(&mode_labels) {
        Ok(l) => l,
        Err(e) => {
            amigaos4::serial_println!("Failed to create layout: {}", e);
            return 1;
        }
    };

    // Menu bar
    let strip = match MenuBuilder::new()
        .menu(amstr!("Project"))
        .item(MID_ABOUT, amstr!("About..."))
        .separator()
        .item_key(MID_QUIT, amstr!("Quit"), amstr!("Q"))
        .build()
    {
        Ok(s) => s,
        Err(e) => {
            amigaos4::serial_println!("Failed to build menu: {}", e);
            return 1;
        }
    };

    // Open a window with the layout as its root gadget
    let win_tags = TagListBuilder::new()
        .tag(WA_LEFT, 100)
        .tag(WA_TOP, 100)
        .tag(WA_WIDTH, 350)
        .tag(WA_HEIGHT, 240)
        .tag(WA_TITLE, amstr!("Rust GUI Demo").as_ptr() as u32)
        .tag(
            WA_IDCMP,
            IDCMP_CLOSEWINDOW | IDCMP_GADGETUP | IDCMP_VANILLAKEY | IDCMP_MENUPICK,
        )
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

    if let Err(e) = strip.attach(&win) {
        amigaos4::serial_println!("Failed to attach menu strip: {}", e);
    }

    amigaos4::serial_println!("Window opened. Entering event loop...");

    event_loop(&win, |event| match event {
        Event::Close => {
            amigaos4::serial_println!("Close gadget clicked. Exiting.");
            false
        }
        Event::GadgetUp { id, code } => match id {
            GID_OK => {
                amigaos4::serial_println!("OK button clicked!");
                false
            }
            GID_CANCEL => {
                amigaos4::serial_println!("Cancel button clicked!");
                false
            }
            GID_NAME => {
                amigaos4::serial_println!("Name field changed");
                true
            }
            GID_AGE => {
                amigaos4::serial_println!("Age field changed");
                true
            }
            GID_ENABLED => {
                amigaos4::serial_println!("Checkbox toggled");
                true
            }
            GID_VOLUME => {
                amigaos4::serial_println!("Volume slider: {}", code);
                true
            }
            GID_MODE => {
                amigaos4::serial_println!("Mode chooser: entry {}", code);
                true
            }
            _ => {
                amigaos4::serial_println!("Unknown gadget: {}", id);
                true
            }
        },
        Event::MenuPick => {
            let mut quit = false;
            strip.each_select(|mid| match mid {
                MID_ABOUT => amigaos4::serial_println!("About selected"),
                MID_QUIT => {
                    amigaos4::serial_println!("Quit selected");
                    quit = true;
                }
                _ => {}
            });
            !quit
        }
        Event::Key(27) => {
            amigaos4::serial_println!("ESC pressed. Exiting.");
            false
        }
        _ => true,
    });

    strip.detach(&win);
    drop(win);

    amigaos4::serial_println!("=== GUI Demo complete ===");
    0
}

// ---------------------------------------------------------------------------
// UI construction
// ---------------------------------------------------------------------------

/// Build the complete ReAction gadget layout.
///
/// ```text
/// Vertical layout ("Preferences" group)
///   +-- "Name:"    String gadget
///   +-- "Age:"     Integer gadget (with arrows, 0..150)
///   +-- "Enabled:" Checkbox gadget
///   +-- "Volume:"  Slider gadget (0..64)
///   +-- "Mode:"    Chooser gadget
///   +-- Horizontal layout: [OK] [Cancel]
/// ```
fn build_ui(mode_labels: &ChooserLabels) -> amigaos4::Result<AmigaObject> {
    // ---- Individual gadgets ----
    let name_gad = create_string_gadget(GID_NAME, amstr!(""), 64)?;
    let age_gad = create_integer_gadget(GID_AGE, 25, 0, 150)?;
    let enabled_gad = checkbox(GID_ENABLED, true)?;
    let volume_gad = slider(GID_VOLUME, 0, 64, 32)?;
    let mode_gad = chooser(GID_MODE, mode_labels, 0)?;
    let ok_btn = button(GID_OK, amstr!("_OK"))?;
    let cancel_btn = button(GID_CANCEL, amstr!("_Cancel"))?;

    // ---- Button row (horizontal layout) ----
    let button_row = LayoutBuilder::horizontal()
        .add(ok_btn)
        .add(cancel_btn)
        .build()?;

    // ---- Main layout (vertical, labelled group frame) ----
    let main_tags = TagListBuilder::new()
        .tag(LAYOUT_ORIENTATION, LAYOUT_ORIENT_VERT)
        .tag(LAYOUT_SPACE_OUTER, 1)
        .tag(LAYOUT_SPACE_INNER, 1)
        .tag(LAYOUT_BEVEL_STYLE, BVS_GROUP)
        .tag(LAYOUT_LABEL, amstr!("Preferences").as_ptr() as u32)
        // Each child, with a per-child CHILD_LABEL beside it
        .tag(LAYOUT_ADD_CHILD, name_gad.into_raw() as u32)
        .tag(CHILD_LABEL, amstr!("Name:").as_ptr() as u32)
        .tag(LAYOUT_ADD_CHILD, age_gad.into_raw() as u32)
        .tag(CHILD_LABEL, amstr!("Age:").as_ptr() as u32)
        .tag(LAYOUT_ADD_CHILD, enabled_gad.into_raw() as u32)
        .tag(CHILD_LABEL, amstr!("Enabled:").as_ptr() as u32)
        .tag(LAYOUT_ADD_CHILD, volume_gad.into_raw() as u32)
        .tag(CHILD_LABEL, amstr!("Volume:").as_ptr() as u32)
        .tag(LAYOUT_ADD_CHILD, mode_gad.into_raw() as u32)
        .tag(CHILD_LABEL, amstr!("Mode:").as_ptr() as u32)
        // Button row
        .tag(LAYOUT_ADD_CHILD, button_row.into_raw() as u32)
        .build();

    let main_layout = AmigaObject::new(LAYOUT_CLASS, &main_tags)?;

    amigaos4::serial_println!("Layout built successfully.");
    Ok(main_layout)
}

// ---------------------------------------------------------------------------
// Gadget creation helpers (variants with extra tags beyond the crate's
// stock builders)
// ---------------------------------------------------------------------------

/// Create a ReAction string gadget with an initial value and Tab cycling.
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
