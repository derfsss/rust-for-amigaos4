# Phase 10 — Shared Library Output

Started: 2026-03-16
Completed: 2026-03-16

## Goal

Create a reusable template for building AmigaOS 4 shared libraries (`.library` files) in Rust, using the clib4 source code as a reference implementation.

## Lessons from Previous Phases

- Driver template pattern: C glue defines entry + IExec, links with Rust staticlib via -nostartfiles -nodefaultlibs -lgcc.
- clib4.c provides the exact shared library pattern: Resident → RTF_AUTOINIT → CLT tags → manager vectors → main vectors.
- Timer module initially depended on feature-gated `time::Duration` — broke in driver mode. Must use self-contained types for core modules.

## Tasks

### 1. Template: `templates/library/`
- **Status:** Complete
- **Deliverables:**
  - `src/library_glue.c` — Full Resident struct, RTF_AUTOINIT tag list, manager interface (Obtain/Release/Open/Close/Expunge), main interface vector table with placeholder Rust functions
  - `src/main.rs` — Rust library logic with lifecycle callbacks (init/open/close) and example public functions
  - `Cargo.toml`, `.cargo/config.toml`, `powerpc-amigaos.json`, `Makefile`, `fake-linker.sh/.bat`
- **Pattern:** Identical to driver template but with Resident + interface vectors instead of _start() entry point

### 2. Example: `examples/hello-library/`
- **Status:** Complete
- **Deliverable:** Working shared library example with 3 exported functions (add, multiply, version)
- **Cross-compilation verified:** `cargo +nightly build --release` succeeds

### 3. Timer module fix
- **Status:** Complete
- **Issue:** `timer.rs` imported `crate::time::Duration` which is gated behind the `time` feature (app-only). This broke compilation in driver/library mode.
- **Fix:** Replaced `Duration` dependency with self-contained `TimerVal` struct (secs + micros) that works in both app and driver modes.

---

## Issues Log

### ISSUE-012: Timer module depended on feature-gated time::Duration (HIGH, fixed)
`timer.rs` used `use crate::time::Duration` which is only available with the `time` feature (enabled by `app` but not `driver`). Since the timer module is in the "core" section of lib.rs, it must work without any features. Fixed by introducing `TimerVal` — a lightweight secs+micros struct with no feature dependencies.

---

## Changes from Original Plan

1. **Introduced `TimerVal` type** — Not in original plan. Required because `time::Duration` is feature-gated and the timer module must work in driver/library mode (no app features).
2. **Library template uses driver features** — Shared libraries use the same `-nostartfiles -nodefaultlibs` mode as drivers (ExecAllocator, no clib4). This means they use `features = ["driver"]` in Cargo.toml.
3. **No per-process state** — Unlike clib4's libOpen which allocates per-process context, the template's libOpen is minimal (just calls rust_lib_open). Per-process state can be added by the library author if needed.
