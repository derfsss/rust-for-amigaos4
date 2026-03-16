# Phase 9 — Timer & Clipboard Wrappers

Started: 2026-03-16
Completed: 2026-03-16

## Goal

Implement safe RAII wrappers for the timer device and clipboard (via IFFParse), unblocking the two deferred items from Phase 3.

## Lessons from Previous Phases

- Opaque types in amigaos4-sys require pointer-offset access (see window.rs pattern).
- All structs are `packed(2)` on PPC. Host tests must account for different sizes.
- RAII types must implement Drop and null-check before calling cleanup functions.
- Device I/O protocol: CreateMsgPort → CreateIORequest → OpenDevice → DoIO → CloseDevice → DeleteIORequest → DeleteMsgPort.
- IFFParse clipboard flow: OpenClipboard → AllocIFF → set iff_Stream → InitIFFasClip → OpenIFF → read/write chunks → CloseIFF → FreeIFF → CloseClipboard.

## Tasks

### 1. Timer Module (`amigaos4/src/timer.rs`)
- **Status:** Complete
- **Deliverable:** `amigaos4/src/timer.rs` (220 lines)
- **Contents:**
  - `TimerUnit` enum (MicroHz, VBlank, EClock, WaitUntil)
  - `AmigaTimer` RAII type with full Drop cleanup (CloseDevice → DeleteIORequest; port dropped automatically)
  - `AmigaTimer::open(unit)` — creates MsgPort, IORequest(40 bytes), opens timer.device
  - `AmigaTimer::delay(duration)` — synchronous delay via TR_ADDREQUEST + DoIO
  - `AmigaTimer::get_sys_time()` — system time via TR_GETSYSTIME + DoIO
  - `get_up_time()` free function — monotonic uptime via ITimer->GetUpTime
  - `micro_delay(us)` free function — busy-wait via ITimer->MicroDelay
  - 6 host-side unit tests (unit enum values, constants, offsets, name)
- **Notes:**
  - TimeRequest accessed via known PPC offsets: io_Command at 28, Time.Seconds at 32, Time.Microseconds at 36. Same pattern as window.rs.
  - `InitIFFasClip` is void (no return value) — confirmed from FFI bindings.

### 2. Clipboard Module (`amigaos4/src/clipboard.rs`)
- **Status:** Complete
- **Deliverable:** `amigaos4/src/clipboard.rs` (260 lines)
- **Contents:**
  - `read_text()` — opens clipboard, parses FTXT/CHRS, returns text as Vec<u8>
  - `write_text(data)` — opens clipboard, writes FTXT/CHRS, closes
  - Internal `cleanup()` helper with `iff_opened` flag to prevent calling CloseIFF on an unopened handle
  - MAX_CLIPBOARD_TEXT safety cap (1 MB)
  - 7 host-side unit tests (IFF ID values, constants, make_id roundtrip)
- **Notes:**
  - Uses free functions rather than a struct because each clipboard operation is self-contained (open → work → close).
  - Code review found: must track whether `open_iff` succeeded to avoid calling `close_iff` on an unopened handle. Fixed with `iff_opened` parameter in `cleanup()`.

### 3. Target-side Tests
- **Status:** Complete
- **Changes:** Added 8 new tests to `examples/test-harness/src/main.rs`:
  - Timer: open/close, short delay, get_sys_time, get_up_time, open VBlank unit
  - Clipboard: write+read roundtrip, write empty, read after write

### 4. Project Updates
- **Status:** Complete
- **Changes:**
  - Added `timer` and `iffparse` features to amigaos4 Cargo.toml dependency on amigaos4-sys
  - Registered `timer` and `clipboard` modules in lib.rs (core section, no clib4 needed)

---

## Issues Log

### ISSUE-010: CloseIFF on unopened handle (HIGH, fixed)
Initial clipboard implementation called `iffparse_close_iff()` in error paths even when `iffparse_open_iff()` had failed. Fixed by introducing a `cleanup()` helper with an `iff_opened` boolean parameter.

### ISSUE-011: InitIFFasClip return value (FALSE POSITIVE, dismissed)
Code review flagged `iffparse_init_iffas_clip()` as missing error handling. However, this is a void function — confirmed from the FFI binding signature. No return value to check.

---

## Changes from Original Plan

1. **Clipboard uses free functions instead of struct** — Plan called for `AmigaClipboard` RAII type. Implemented as `read_text()`/`write_text()` free functions instead because each clipboard operation is self-contained (open, work, close). A persistent `AmigaClipboard` struct would add complexity without benefit since the IFFHandle must be opened/closed per operation anyway.
2. **Cleanup helper function** — Not in original plan. Added `cleanup()` with `iff_opened` flag after code review found the close-on-unopened-handle bug.
3. **Timer offsets hardcoded for PPC** — Timer struct field access uses known PPC offsets (same pattern as window.rs) rather than defining repr(C) structs, since the types are opaque in amigaos4-sys.
