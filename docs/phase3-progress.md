# Phase 3 — Expand Safe Wrappers

Started: 2026-03-15
Completed: 2026-03-15

## Goal

Cover the most-used AmigaOS APIs with safe, idiomatic Rust wrappers.

## Lessons from Phase 1 & 2

- Validate null-termination of `&[u8]` before passing to FFI
- Validate size casts (usize -> u32) before calling 32-bit OS APIs
- Use `AmigaError::UnexpectedEof` for EOF, not `IoError(0)`
- Add safety caps on C string reads (MAX_ENV_LEN pattern)
- Use `buf.resize(n, 0)` instead of `buf.set_len(n)` for zero-initialized buffers

## Tasks

### 1. DOS wrappers
- **Status:** Complete
- **Deliverable:** `amigaos4/src/dos.rs` (289 lines)
- **Contents:**
  - `AmigaLock` — RAII Lock/UnLock with `shared()`, `exclusive()`, `name()`, `parent()`, `same_as()`
  - `DirScanner` — RAII ObtainDirContext with `next()` returning raw ExamineData pointers
  - `file_part()` / `path_part()` — Path utility functions
  - Constants: `SHARED_LOCK`, `EXCLUSIVE_LOCK`, `LOCK_SAME`, `LOCK_SAME_VOLUME`, `LOCK_DIFFERENT`
  - 3 unit tests for constants

### 2. Window wrapper
- **Status:** Complete
- **Deliverable:** `amigaos4/src/window.rs` (309 lines)
- **Contents:**
  - `AmigaWindow` — RAII OpenWindowTagList/CloseWindow with `wait_msg()`, `get_msg()`, `rast_port()`, `into_raw()`
  - `IntuiMsg` — Owned copy of IDCMP message fields (class, code, qualifier, mouse_x, mouse_y)
  - `RawIntuiMessage` — Partial repr(C) struct for reading IDCMP fields from raw pointer
  - Window struct field offsets for UserPort (86) and RPort (50)
  - 25+ WA_* constants and 11 IDCMP_* constants

### 3. Requester wrappers
- **Status:** Complete
- **Deliverable:** `amigaos4/src/requester.rs` (81 lines)
- **Contents:**
  - `easy_request()` — Show modal dialog on default screen
  - `easy_request_on()` — Show modal dialog relative to a specific window
  - `EasyStructData` — repr(C) struct matching AmigaOS EasyStruct

### 4. Graphics wrappers
- **Status:** Complete
- **Deliverable:** `amigaos4/src/gfx.rs` (103 lines)
- **Contents:**
  - `DrawContext<'a>` — Borrowed RastPort with lifetime-bounded drawing ops
  - Methods: `set_pen`, `set_bg_pen`, `move_to`, `draw_to`, `fill_rect`, `draw_text`, `text_width`
  - Text length bounds-checked to u16::MAX

### 5. Locale wrapper
- **Status:** Complete
- **Deliverable:** `amigaos4/src/locale.rs` (101 lines)
- **Contents:**
  - `AmigaCatalog` — RAII OpenCatalog/CloseCatalog with `get()` for string lookup
  - C string read safety cap (MAX_CATALOG_STR = 16384)
  - 1 unit test

---

## Issues Log

### ISSUE-001: Unbounded C string read in locale get() (found during review, fixed)
Same pattern as Phase 1 ISSUE-004 (env.rs). Added MAX_CATALOG_STR safety cap.

### ISSUE-002: Uninitialized buffer in dos.rs name() (found during review, fixed)
Used `buf.set_len()` on uninitialized memory. Changed to `buf.resize(n, 0)` for zero-init.

---

## Changes from Original Plan

1. **Deferred Timer wrapper** — Needs I/O device protocol (SendIO/WaitIO/AbortIO), too complex for this phase
2. **Deferred Clipboard wrapper** — Needs IFFParse protocol, deferred
3. **Deferred Notification/AppMessage** — Niche use case, deferred
4. **DirScanner returns raw pointers** — ExamineData is opaque in our bindings, so DirScanner returns `*mut ExamineData` rather than a safe struct. Users need unsafe to read fields.
5. **Window struct offsets** — UserPort (86) and RPort (50) are hard-coded from the SDK. This is standard practice for AmigaOS FFI but fragile if the struct changes (which it hasn't since OS4 launch).
