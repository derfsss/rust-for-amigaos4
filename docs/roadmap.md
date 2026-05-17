# Rust for AmigaOS 4 — Roadmap

Last updated: 2026-04-14 (post-Phase-10 hardening)

## Current State

- 3 crates: `amigaos4-sys` (129 interface bindings), `amigaos4-alloc` (2 allocator backends), `amigaos4` (25 safe wrapper modules)
- Three build modes: application (clib4 + `-lauto`), driver (no CRT, ExecAllocator), shared library (Resident + interface vectors)
- Build infrastructure: `build.sh`/`build.bat`, Docker-based linking, fake linker trick
- CI pipeline: GitHub Actions cross-compiles all crates + 12 examples, runs host tests for all 3 crates
- **~255 total tests**: 193 host-side (173 unit + 13 integration + 5 sys + 2 alloc) + 2 doctests, 60 target-side (51 main + 5 GUI + 4 net)
- `serial_println!` macro for formatted debug output via `core::fmt::Write`
- Reusable panic handler with file/line/message output
- `cargo-amiga.sh`/`.bat` wrapper for project scaffolding and builds
- 3 templates: app, driver, library
- 12 examples: hello, hello-driver, hello-library, test-harness, test-harness-gui, test-harness-net, file-io-demo, timer-demo, thread-demo, gui-demo, net-demo, async-demo
- Tested on QEMU (`-M amigaone`) only
- All code is `no_std`; `Vec`, `String`, `format!`, `Box` work via global allocator
- C glue for 5 varargs-only SDK methods
- PPC inline asm for cache, MMIO, memory barriers
- Full public API documentation on all exported items

---

## Post-Phase-10 — Reproducibility & Real-Hardware Prep **[COMPLETE]**

- [x] **Rust toolchain pin** — `rust-toolchain.toml` fixes every cargo invocation to `nightly-2026-03-01`. `+nightly` overrides removed from `build.sh`, `build.bat`, `setup.sh`, `setup.bat`, CI workflow, and README examples.
- [x] **clib4 source submodule** — `clib4-src/` added as a submodule of `AmigaLabs/clib4`, tracking the upstream `development` branch (currently pinned at commit `778afb03`, tagged `nightly`). Reproduces `clib4-nightly/clib4.library` when rebuilt in Docker with `gmake -f GNUmakefile.os4`. Version string matches the shipped binary's `clib4.library 2.1`.
- [x] **Line-ending normalisation** — `.gitattributes` enforces LF for `*.sh`/`*.rs`/`*.toml`/`*.c`/`*.h` and CRLF for `*.bat`. All shell scripts converted to LF so `/bin/sh` can execute them under Linux.
- [x] **Upstream link correction** — README now points to `github.com/AmigaLabs/clib4` (the `afxgroup/clib4` URL 404s).
- [x] **Shared-library example fixes**:
  - Added missing `#include <dos/dos.h>` in `examples/hello-library/src/library_glue.c` (undefined `BPTR`).
  - Added `-Wl,--undefined=RomTag` to the Makefile so `--gc-sections` no longer drops the Resident struct. Verified the `0x4AFC` `RTC_MATCHWORD` is present in the linked binary's `.rodata`.
- [x] **Install docs** — README states plainly: `clib4.library` for Rust programs goes in `PROGDIR:` (same directory as the executable). Also documents how `build.sh` overlays `clib4-nightly/` into the Docker SDK at link time, and how to rebuild clib4 from the submodule and use that as the overlay.

Remaining deferred: real hardware testing (still requires physical Amiga hardware).

---

## Phase 1 — Stabilise and Harden (Foundation) **[COMPLETE]**

Goal: Make what exists reliable and easy to use before adding new features.
See `docs/phase1-progress.md` for full details and issues log.

- [x] **CI pipeline** — `.github/workflows/ci.yml`: cross-compiles all crates + examples, runs host-side tests
- [x] **Crate versioning and changelogs** — CHANGELOG.md per crate (Keep a Changelog format)
- [x] **Error handling audit** — Fixed 5 critical/important issues (allocator alignment, overflow, clock_gettime, env bounds, negative size)
- [x] **Alignment audit for ExecAllocator** — Both allocators now handle >16-byte alignment via over-allocation; overflow check added
- [x] **Test suite (host-side)** — 61 unit tests across error, time, tag, io modules; ppc_asm gated for host compilation
- [x] **Test harness (target-side)** — `examples/test-harness/` with 8 OS-call tests (Vec, String, TagList, AmigaVec, MsgPort, Duration, file I/O, env)
- [ ] **Real hardware testing** — Deferred (requires physical Amiga hardware; community ask)

---

## Phase 2 — Developer Experience **[COMPLETE]**

Goal: Make it easy for new users to start building AmigaOS apps in Rust.
See `docs/phase2-progress.md` for full details.

- [x] **Better panic handler** — `amigaos4::panic::default_panic_handler()` prints file/line/message via Serial sink
- [x] **Debug formatting** — `Serial` struct with `core::fmt::Write`, `serial_print!`/`serial_println!` macros with format args
- [x] **`cargo-amiga` wrapper** — `cargo-amiga.sh`/`.bat` with `new`, `build`, `clean`, `setup` subcommands
- [x] **Project scaffolding** — `cargo-amiga.sh new <name> --mode app|driver` copies and renames templates
- [x] **Documentation** — Module-level rustdoc for all `amigaos4` crate modules
- [x] **More examples** — file-io-demo, timer-demo, thread-demo (3 new examples)

---

## Phase 3 — Expand Safe Wrappers (`amigaos4` crate) **[COMPLETE]**

Goal: Cover the most-used AmigaOS APIs with safe, idiomatic Rust wrappers.
See `docs/phase3-progress.md` for full details.

- [x] **DOS wrappers** — `AmigaLock` (RAII Lock/UnLock), `DirScanner`, `file_part`/`path_part` utilities
- [x] **Window wrapper** — `AmigaWindow` (RAII) with `wait_msg`/`get_msg` IDCMP event handling, WA_*/IDCMP_* constants
- [x] **Requester wrappers** — `easy_request`/`easy_request_on` for modal dialogs via EasyRequestArgs
- [x] **Graphics wrappers** — `DrawContext<'a>` with pen, line, rect, text drawing on borrowed RastPort
- [x] **Locale wrapper** — `AmigaCatalog` (RAII OpenCatalog/CloseCatalog) with `get()` string lookup
- [x] **Timer wrapper** — Implemented in Phase 9 (`amigaos4/src/timer.rs`)
- [x] **Clipboard wrapper** — Implemented in Phase 9 (`amigaos4/src/clipboard.rs`)
- [ ] **Notification/AppMessage** — Deferred (niche use case)

---

## Phase 4 — ReAction / GUI **[COMPLETE]**

Goal: Build GUI applications entirely in Rust.
See `docs/phase4-progress.md` for full details.

- [x] **ReAction widget bindings** — `reaction` module with GA_*, LAYOUT_*, BUTTON_*, STRING_*, CHECKBOX_*, INTEGER_* constants + `button()`, `string_gadget()`, `checkbox()` helpers
- [x] **Event loop abstraction** — `Event` enum (Close, GadgetUp, Key, Resize, Refresh, Other) + `event_loop()` with FnMut handler
- [x] **Layout builder DSL** — `LayoutBuilder` with `vertical()`/`horizontal()`, `.add()`/`.add_labeled()`, `.build()`
- [x] **Working GUI example** — `examples/gui-demo/` preferences-editor with string, integer, checkbox, buttons, event loop

---

## Phase 5 — Networking **[COMPLETE]**

Goal: Enable network-capable applications.
See `docs/phase5-progress.md` for full details.

- [x] **Socket wrappers** — `TcpStream` (RAII connect/read/write), `TcpListener` (RAII bind/listen/accept), `SocketAddr` with parser
- [x] **DNS resolution** — `resolve()` via gethostbyname with safety cap
- [x] **HTTP client** — `http::get()` for HTTP/1.1 GET requests, `HttpResponse` with status code + body
- [x] **Network example** — `examples/net-demo/` with DNS, TCP, and HTTP demos
- [ ] **TLS** — Deferred (AmiSSL integration too complex for this phase)

---

## Phase 6 — Advanced / Long-Term **[COMPLETE]**

Goal: Push the boundaries of what Rust on AmigaOS can do.
See `docs/phase6-progress.md` for full details.

- [x] **Async runtime** — `Executor` with `spawn()`, `run()`, `block_on()` using Exec signals for waking
- [x] **Proc macro workarounds** — Documented in `docs/nostd-ecosystem.md` (build.rs code generation patterns)
- [x] **no_std ecosystem** — `docs/nostd-ecosystem.md` with 10 categories of compatible crates + integration patterns
- [x] **Async example** — `examples/async-demo/` demonstrating cooperative tasks + block_on
- [x] **Shared library output** — Implemented in Phase 10 (`templates/library/`, `examples/hello-library/`)
- [ ] **USB device drivers** — Deferred (community effort: needs real hardware)
- [ ] **Upstream Rust target** — Deferred (community effort: sustained maintenance)

---

## Phase 7 — Comprehensive Testing **[COMPLETE]**

Goal: Achieve comprehensive test coverage across all modules.
See `docs/phase7-progress.md` for full details.

- [x] **Unsafe code verification** — Struct layout, offset, and constant tests for window, net, http, dos modules
- [x] **Zero-coverage modules** — Tests for reaction (14), fmt (3), mem (1), boopsi (1), async_rt (2)
- [x] **Extended coverage** — Additional tests for time (5), error (3), io (3), tag (3)
- [x] **Target-side integration** — Expanded test-harness from 8 to 43 tests; new test-harness-gui (5 tests) and test-harness-net (4 tests)
- [x] **FFI type tests** — amigaos4-sys TagItem layout/alignment verification (5 tests)
- [x] **Allocator tests** — ZST verification for both allocators (2 tests)
- [x] **CI expansion** — Host tests for all 3 crates; builds for 11 examples

---

## Phase 8 — Documentation & Polish **[COMPLETE]**

Goal: Full public API documentation, fix all doctest issues, update project docs.
See `docs/phase8-progress.md` for full details.

- [x] **Public API docs** — Rustdoc on all ~60 undocumented items (IDCMP/WA constants, Duration methods, accessor methods)
- [x] **Safety comments** — Added SAFETY comment for fs.rs `set_len()` on uninitialized memory
- [x] **Doctest fixes** — Fixed 3 pre-existing doctest failures (fmt.rs, panic.rs)
- [x] **Changelog updates** — Updated all 3 CHANGELOG.md files with Phase 7-8 changes
- [x] **Roadmap update** — Added Phase 7-8 entries, updated statistics
- [x] **Full code review** — Verified error handling, unsafe safety docs, RAII cleanup, consistency

---

## Phase 9 — Timer & Clipboard Wrappers **[COMPLETE]**

Goal: Implement the two deferred wrappers from Phase 3 using SDK documentation.
See `docs/phase9-progress.md` for full details.

- [x] **Timer wrapper** — `AmigaTimer` RAII type with `open()`, `delay()`, `get_sys_time()`; free functions `get_up_time()`, `micro_delay()`
- [x] **Clipboard wrapper** — `read_text()` / `write_text()` via IFFParse FTXT/CHRS format
- [x] **Target-side tests** — 8 new tests (5 timer, 3 clipboard) in test-harness

---

## Phase 10 — Shared Library Output **[COMPLETE]**

Goal: Create a template for building AmigaOS 4 shared libraries in Rust.
See `docs/phase10-progress.md` for full details.

- [x] **Library template** — `templates/library/` with Resident struct, RTF_AUTOINIT, manager interface, main interface vector table
- [x] **Working example** — `examples/hello-library/` (hello.library with add/multiply/version functions)
- [x] **Timer module fix** — Replaced feature-gated `time::Duration` dependency with self-contained `TimerVal` type

---

## Non-Goals

These are explicitly out of scope:

- **Full `std` library port** — Too large; `no_std` + targeted safe wrappers is the strategy
- **68k (AmigaOS 3.x) support** — This project targets AmigaOS 4.1 (PPC) only
- **MorphOS / AROS support** — Different ABIs and interfaces; would require separate bindings
- **Replacing C for all OS development** — Rust complements C; some OS-level code (interrupt handlers, ROM modules) will always be C/asm

---

## Contributing

If you want to help, the most impactful areas are:

1. **Real hardware testing** — Report results on actual Amiga hardware
2. **Safe wrappers** — Pick an unwrapped interface from Phase 3 and submit a PR
3. **Examples** — Working code teaches better than documentation
4. **Bug reports** — Especially link errors, alignment issues, or ABI mismatches
