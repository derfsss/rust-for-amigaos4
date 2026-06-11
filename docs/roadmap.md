# Rust for AmigaOS 4 — Roadmap

Last updated: 2026-06-11 (post-1.0 improvement plan, first wave landed)

## Current State

- 3 crates: `amigaos4-sys` (129 interface bindings), `amigaos4-alloc` (2 allocator backends), `amigaos4` (29 safe wrapper modules)
- Three build modes: application (clib4 + `-lauto`), driver (no CRT, ExecAllocator), shared library (Resident + interface vectors)
- Build infrastructure: `build.sh`/`build.bat`, Docker-based linking, fake linker trick
- CI pipeline: GitHub Actions cross-compiles all crates + 24 examples, runs host tests for all 3 crates; rustdoc published via Pages
- **~365 total tests**: 308 host-side (213 amigaos4 unit + 3 doctests + 5 sys + 2 alloc + 85 black-box in `Tests/`), 60 target-side (51 main + 5 GUI + 4 net). In-source tests of the clib4 POSIX modules compile only for the PPC target; their behaviour is exercised by the target-side harnesses.
- `serial_println!` macro for formatted debug output via `core::fmt::Write`
- Reusable panic handler with file/line/message output
- `cargo-amiga.sh`/`.bat` wrapper for project scaffolding, builds, and deploy/run/test on fleet targets
- 3 templates: app, driver, library
- 24 examples: hello, hello-driver, hello-library, test-harness, test-harness-gui, test-harness-net, file-io-demo, timer-demo, thread-demo, gui-demo, net-demo, async-demo, thread-amissl-probe, http-client, zlib-roundtrip, picture-viewer, wbstartup-hello, xadmaster-list, async-net-echo, iff-dump, locale-i18n-hello, audio-tone, ram-device, aminet-browser
- Tested on QEMU (`-M amigaone`) and real X5000 hardware (P5020, Kickstart 54.57)
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

Real hardware testing: completed 2026-06-11 on an X5000 (see the post-1.0
improvement plan, section B).

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

## Post-roadmap maintenance

Additions made after the 10 phases were complete:

- **`amigaos4::mem::AmigaPool`** — a safe RAII exec memory pool
  (`CreatePool`/`AllocPooled`/`DeletePool`), companion to `AmigaVec`.
  Sub-allocations are freed atomically when the pool is dropped — useful for a
  contained unit of work. The raw `exec_*_pool` FFI already existed in
  `amigaos4-sys`; this is the safe wrapper.
- **`amigaos4::window::WA_REPORT_MOUSE`** — window tag constant.
- **clib4 refreshed to upstream `778afb03`** — fixes a pthread
  per-thread-init crash (`clib4-nightly/` binaries + `clib4-src/` submodule pin).
- **CI scoped to `main`** — `.github/workflows/ci.yml` triggers on `main` only.

---

## Post-1.0 Improvement Plan (2026-06)

Outcome of the June 2026 review pass. Ordered by impact on the project's
purpose (writing real AmigaOS 4.1 apps, drivers, and libraries in Rust).

### A. Correctness & API soundness

- [x] **`amstr!` + nul-termination validation** — Every safe API that takes a
  "must be null-terminated" `&[u8]` used to pass `as_ptr()` straight to C;
  a missing `\0` was an out-of-bounds read from safe code. Now there is a
  compile-time `amstr!("text")` macro and runtime validation (error, not
  UB) in every such entry point.
- [x] **Richer `AmigaError`** — `NotNulTerminated` and `HostNotFound`
  variants added; dns failures no longer collapse into `NullPointer`.
- [x] **SDK-verified ReAction tags** *(found during implementation)* — every
  gadget tag constant was wrong (invented bases/offsets, silently ignored
  by BOOPSI); all values now match the SDK 54.16 headers, are pinned by
  unit tests, and `IDCMP_GADGETUP` reads the real `GadgetID` from
  `IAddress`. Verified on QEMU (GUI harness 5/5).

### B. Test reality

- [x] **Host-testable parsers** — pure logic lives in the always-compiled
  `parse` module; its tests run on every host `cargo test`.
- [x] **Target-harness smoke in Tests/** — `target_smoke.rs`, env-gated via
  `AMIGA_TARGET_SMOKE=1`, drives `cargo-amiga test` on a fleet target.
- [x] **Real-hardware run** — Done 2026-06-11 on an AmigaOne X5000 (P5020,
  Kickstart 54.57): main harness 51/51, GUI harness 5/5, driver-mode
  `hello-driver` all pass. Notes: an older `LIBS:clib4.library` (2.0)
  shadows `PROGDIR:`, so clib4 2.1 must be installed in `LIBS:` (the
  resident library does not expunge — a reboot is needed); ahi.device
  unit 0 is unavailable on this machine (AHI finds no supported card),
  so audio playback remains QEMU-verified only.

### C. Application-essential features

- [x] **Menus** — `MenuBuilder`/`MenuStrip` over OS4 menuclass;
  `Event::MenuPick` + `each_select()` decoding.
- [x] **ASL file requester** — `FileRequester` builder (open/save/drawer)
  over asl.library, opened at runtime.
- [x] **More ReAction gadgets** — listbrowser (+`ListBrowserNodes`),
  chooser (+`ChooserLabels`), slider.
- [x] **HTTP improvements** — follows redirects (5 hops), decodes chunked
  transfer encoding.
- [ ] **HTTPS via OpenSSL (clib4)** — the Docker SDK ships a full
  clib4-built userland under `SDK/local/clib4/` (134 pkg-config
  packages), including **OpenSSL 1.1.1k+quic (`-lssl -lcrypto`,
  default) and OpenSSL 3 (`lib/openssl3/`)**. A TLS client links
  cleanly in application mode (verified:
  `ppc-amigaos-gcc -mcrt=clib4 t.c -lssl -lcrypto -lz -lpthread
  -lauto` with `SSL_CTX_new(TLS_client_method())`). Because these
  OpenSSL builds sit on clib4's own socket fds, `SSL_set_fd` works
  directly with our `TcpStream` fd — no ISocket plumbing at all.
  Plan: `https` module (feature `tls`) over extern OpenSSL calls,
  Makefile gains `-lssl -lcrypto`; cert verification off by default
  (no system CA store on AmigaOS) with an opt-in CA-file API.
  The earlier AmiSSL route (probe-validated, see
  `thread-amissl-probe`) remains the runtime-library alternative for
  binaries that must not statically link OpenSSL.
  Note: the shipped `libcurl.a` does NOT currently link (built
  against OpenSSL 3 while the default `-L` resolves the 1.1 libs
  first, and full-path linking still misses `EVP_PKEY_get_bn_param`
  providers) — revisit if the Docker image updates.
- [x] **Async integration** — `timer::delay_async()` and
  `AmigaWindow::next_event()` futures; the executor waits on registered
  external signals, so one executor selects over sockets, timers, and
  GUI input.
- [x] **More wrappers (partial)** — application.library registration
  (`AppRegistration`, single-instance); AHI actual playback
  (`examples/audio-tone` plays a synthesised tone via CMD_WRITE,
  verified on QEMU). Remaining: datatypes beyond load/query.
- [ ] **clib4 userland integration** — the Docker SDK's
  `SDK/local/clib4/` carries clib4-built static libs ready to link
  into application-mode Rust via extern FFI + Makefile `-l` flags:
  sqlite3, jansson (JSON), libxml2/expat/yaml, libpng/jpeg/tiff/webp/
  gif, freetype/harfbuzz/cairo/pixman/fontconfig, SDL2 (+image/mixer/
  ttf/net, gl4es GL), ffmpeg (avcodec/avformat), FLAC/ogg/vorbis/opus/
  mp3lame/sndfile, ncursesw/readline, pcre2, lua 5.4, libffi, boost,
  icu, gmp/mpfr, zip/lz4/zstd-family, c-ares, nghttp2/3. Candidate
  next examples: sqlite3-demo, json-config (jansson), sdl2-demo.
  (`zlib-roundtrip` already demonstrates the pattern with `-lz`.)

### D. Driver story

- [x] **Real exec device example** — `examples/ram-device`: Resident,
  `DevInit`, `BeginIO`/`AbortIO` dispatch, CMD_READ/CMD_WRITE over a RAM
  buffer; plus `mem::DmaBuffer` pairing MEMF_SHARED with the cache-flush
  asm.

### E. Adoption

- [x] **Publish rustdoc** — `docs.yml` builds PPC-target rustdoc and
  deploys to GitHub Pages.
- [ ] **Tagged release with artifacts** — prebuilt examples + clib4 bundle.
- [x] **Dogfood app** — `examples/aminet-browser`: application.library +
  menus + listbrowser + ASL + HTTP (+ fs) in one real program.

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
