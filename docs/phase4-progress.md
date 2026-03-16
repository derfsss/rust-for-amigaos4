# Phase 4 — ReAction / GUI

Started: 2026-03-15
Completed: 2026-03-15

## Goal

Build GUI applications entirely in Rust using AmigaOS ReAction widgets.

## Lessons from Previous Phases

- Use `AmigaObject::into_raw()` when transferring ownership to parent layouts
- Tag constants follow `TAG_USER + base_offset + field_offset` pattern
- Null-terminated byte strings for all class names and labels
- `TagListBuilder` for constructing tag arrays; `AmigaObject::new` for BOOPSI creation

## Tasks

### 1. ReAction constants and gadget helpers
- **Status:** Complete
- **Deliverable:** `amigaos4/src/reaction.rs` (constants + helpers section)
- **Contents:**
  - GA_* constants (ID, Disabled, Selected, RelVerify, Text)
  - LAYOUT_* constants (Orientation, AddChild, SpaceInner, SpaceOuter, Label)
  - BUTTON_* constants (Text, PushButton)
  - STRINGA_* constants (TextVal, MaxChars)
  - CHECKBOX_*, INTEGER_* constants
  - LABEL_*, CHILD_* constants
  - Class name strings (LAYOUT_CLASS, BUTTON_CLASS, STRING_CLASS, etc.)
  - `button()`, `string_gadget()`, `checkbox()` helper functions

### 2. Layout builder
- **Status:** Complete
- **Deliverable:** `LayoutBuilder` in `reaction.rs`
- **Contents:**
  - `LayoutBuilder::vertical()` / `horizontal()` constructors
  - `.add(child)` — transfers ownership via into_raw
  - `.add_labeled(child, label)` — with CHILD_LABEL
  - `.build()` — produces AmigaObject with orientation + spacing tags

### 3. Event loop
- **Status:** Complete
- **Deliverable:** `Event` enum + `event_loop()` in `reaction.rs`
- **Contents:**
  - `Event::Close`, `GadgetUp(u32)`, `Key(u8)`, `Resize`, `Refresh`, `Other(IntuiMsg)`
  - `event_loop(window, handler)` — blocking loop with FnMut dispatch

### 4. GUI example
- **Status:** Complete
- **Deliverable:** `examples/gui-demo/` (7 files)
- **Contents:**
  - Preferences-editor-style app with:
    - String gadget (name field, 64 chars)
    - Integer gadget (age, 0-150, with arrows)
    - Checkbox (enabled toggle)
    - OK/Cancel buttons in horizontal row
    - Vertical main layout with bevel group frame
  - Full error handling on every OS call
  - IDCMP event loop with gadget ID dispatch
  - ESC key to exit
  - All output via serial_println!

---

## Issues Log

No issues found during implementation or code review.

---

## Changes from Original Plan

1. **GUI example uses raw tag construction** — The example defines its own local constants and constructs tags directly via `TagListBuilder` rather than exclusively using the `reaction` module helpers. This demonstrates both the high-level API (via `reaction.rs`) and the low-level approach (direct tags), serving as more comprehensive documentation.
2. **No menu support yet** — The roadmap mentioned menus, but menu bars require additional Intuition structures not yet wrapped. Deferred.
3. **ListBrowser, Chooser, ClickTab, Scroller, RadioButton** — These additional widget types are not included in the initial helpers but can be created using `AmigaObject::new()` with the appropriate class names and tags. The framework is extensible.
