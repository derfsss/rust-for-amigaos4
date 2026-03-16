# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
