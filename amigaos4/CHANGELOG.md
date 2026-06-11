# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2026-06-11

### Added
- `https` module (feature `tls`, link `-lssl -lcrypto -lz`): `TlsStream`
  (Read/Write RAII over the clib4-built OpenSSL, SNI, opt-in CA-file
  verification via `TlsConfig`) and `https::get()` with https-redirect
  following and chunked decoding. Verified end-to-end on QEMU
  (`examples/https-client`: status 200 from https://example.com).
- `datatypes` module: `DtPicture` — RAII picture loading + attribute
  query (`dimensions()`, `get_attr()`) over a runtime-opened
  datatypes.library; `picture-viewer` now uses it.
- `parse::parse_url`: scheme-aware (`http`/`https`) URL parsing.
- Examples: `sqlite3-demo` and `json-config` — the clib4-userland FFI
  pattern (link the SDK's clib4-built `libsqlite3.a` / `libjansson.a`),
  both QEMU-verified.

## [0.6.0] - 2026-06-11

### Added
- `cstr` module: `amstr!("text")` compile-time null-terminated strings;
  `require_nul()` runtime validation used by every OS-string entry point
- `menu` module: `MenuBuilder`/`MenuStrip` over the OS4 menuclass
  (items, shortcuts, separators, `each_select()` pick decoding)
- `asl` module: `FileRequester` builder (Open/Save/Drawer, patterns)
- `application` module: `AppRegistration` -> `RegisteredApp` RAII
  application.library registration with single-instance support
- `reaction`: slider, chooser (+`ChooserLabels`), listbrowser
  (+`ListBrowserNodes`) gadgets; `GA_TAB_CYCLE`, `LAYOUT_BEVEL_STYLE`,
  `INTEGER_ARROWS`, `BVS_GROUP`
- Async: `timer::delay_async()` and `AmigaWindow::next_event()` futures;
  executor waits on registered external signals
- `mem::DmaBuffer`: MEMF_SHARED + dcbst/dcbf cache maintenance
- `fs::File::seek`/`seek_end`
- `http`: follows redirects (up to 5 hops), decodes chunked bodies
- `parse` module (internal, always compiled): pure network parsing,
  host-tested
- `AmigaError::NotNulTerminated`, `AmigaError::HostNotFound`
- `AmigaMsgPort::signal_mask()`

### Changed
- **Soundness**: every safe API taking a null-terminated `&[u8]` now
  validates the terminator instead of risking an out-of-bounds read
- **`reaction` tag values corrected against the SDK 54.16 headers** —
  the previous invented bases/offsets were silently ignored by BOOPSI
  (GA_ID, GA_TEXT, GA_RELVERIFY, all LAYOUT_/CHILD_/BUTTON_/STRINGA_/
  CHECKBOX_/INTEGER_/LABEL_ tags); values now pinned by unit tests
- `Event::GadgetUp` is now `{ id, code }`: `id` from the gadget's real
  `GadgetID` (via `IntuiMsg.gadget_id`), `code` carrying the
  slider/chooser/listbrowser value; `Event::MenuPick` added
- `dns::resolve` reports `HostNotFound` (was `NullPointer`) and
  validates `h_length`
- HTTP status parsing rejects partially-numeric codes

### Removed
- `reaction::BUTTON_TEXT` (never existed in the SDK — labels use
  `GA_TEXT`) and `CHILD_WEIGHT_EQUAL` (replaced by the real
  `CHILD_WEIGHTED_WIDTH`/`CHILD_WEIGHTED_HEIGHT`)

## [Unreleased]

### Added
- `timer` module: `AmigaTimer` RAII wrapper for timer.device with `delay()`, `get_sys_time()`; `TimerVal` type; free functions `get_up_time()`, `micro_delay()` (Phase 9-10)
- `clipboard` module: `read_text()` / `write_text()` via IFFParse FTXT/CHRS clipboard format (Phase 9)
- Comprehensive test suite: 170 unit tests across 17 modules (Phases 7-9)
- `reaction` module: 14 unit tests for constants, class names, Event enum
- `window` module: 11 unit tests for struct layouts, IDCMP constants, IntuiMsg
- `net` module: 16 unit tests for IPv4/u16 parsing edge cases, SockAddrIn layout
- `http` module: 8 unit tests for status code parsing, body detection, strip_nul
- `dos` module: 4 unit tests for AmigaLock struct size, lock constants
- `async_rt` module: 2 unit tests for WakerData size, Executor struct
- `fmt`, `mem`, `boopsi` modules: structural tests
- Rustdoc comments on all IDCMP_* and WA_* window constants
- Rustdoc comments on all Duration methods and AmigaVec accessors
- `dos` module: `AmigaLock` (RAII Lock/UnLock), `DirScanner` (directory iteration), `file_part`/`path_part` utilities
- `window` module: `AmigaWindow` (RAII OpenWindowTagList/CloseWindow), `IntuiMsg`, IDCMP event handling
- `gfx` module: `DrawContext` with pen, line, rect, text drawing on RastPort
- `requester` module: `easy_request`/`easy_request_on` for simple modal dialogs
- `locale` module: `AmigaCatalog` (RAII OpenCatalog/CloseCatalog) with `get()` string lookup
- `panic` module with `default_panic_handler()` — prints file/line/message to serial debug
- `Serial` struct implementing `core::fmt::Write` for formatted debug output
- `serial_print!` and `serial_println!` macros with format argument support
- `AmigaError::UnexpectedEof` variant for read/write EOF conditions
- Module-level rustdoc comments for all modules
- Crate-level documentation with usage examples
- WA_* and IDCMP_* constants for window creation

### Changed
- `io::Read::read_exact()` and `io::Write::write_all()` now return `UnexpectedEof` instead of `IoError(0)` on premature EOF
- `AmigaVec::alloc()` and `alloc_cleared()` now reject sizes > `u32::MAX`
- `AmigaObject::new()` now validates null-termination of `class_id`
- `Instant::now()` validates timespec values (rejects negative sec/nsec)

### Fixed
- `AmigaVec::alloc()` silently truncated sizes > 4GB on 64-bit (now returns error)
- `AmigaObject::new()` accepted non-null-terminated class IDs (now validates)
- `Instant::now()` accepted negative timespec values from clock_gettime (now validated)

## [0.1.0] - 2026-03-15

Initial release.

### Added

- `AmigaError` enum with `Display` impl covering `NullPointer`,
  `AllocationFailed`, `IoError`, and `DosError` variants.
- `TagListBuilder` for safe tag array construction with support for conditional
  tags.
- `AmigaVec` providing RAII memory allocation via `AllocVecTagList`.
- `AmigaMsgPort` providing RAII message port creation and cleanup.
- `AmigaObject` providing RAII BOOPSI object management with method dispatch.
- `PubScreen` and `AmigaDrawInfo` for RAII screen locking with
  lifetime-bounded `DrawInfo` access.
- `io::Read` and `io::Write` traits.
- `debug_print!` and `debug_println!` macros for serial/debug output.
- Feature-gated POSIX modules (enabled by default via the `app` feature):
  - `fs`: `File`, `read_to_vec`, `write_file`, `metadata`.
  - `time`: `Duration`, `Instant`.
  - `env`: `current_dir`, `var`.
  - `thread`: `spawn`/`join` via pthreads.
- `app` and `driver` feature flags for application vs. driver contexts.

[Unreleased]: https://github.com/anthropic/rust-for-amigaos4/compare/amigaos4-v0.1.0...HEAD
[0.1.0]: https://github.com/anthropic/rust-for-amigaos4/releases/tag/amigaos4-v0.1.0
