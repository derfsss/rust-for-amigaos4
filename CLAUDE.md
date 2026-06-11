# CLAUDE.md — Project Context for Claude Code

## Project

Rust for AmigaOS 4 — Write native AmigaOS 4.1 applications, device drivers, and shared libraries in Rust (`no_std`).

**All 10 roadmap phases complete. ~365 tests, 31 modules, 3 templates, shared library output. Validated on QEMU and real X5000 hardware.**

## Repository Layout

```
rust-for-amigaos4/
  amigaos4-sys/       Raw FFI bindings (129 feature-gated interfaces, C glue, PPC asm)
  amigaos4-alloc/     Global allocator backends (Clib4Allocator, ExecAllocator)
  amigaos4/           Safe wrappers: 31 modules (GUI, menus, ASL, networking, async, DOS, timer, clipboard, POSIX)
  clib4-nightly/      Pre-built clib4 C library overlay (binaries only)
  clib4-src/          clib4 source pinned via submodule at commit 778afb03 (development tip)
  rust-toolchain.toml Pins the exact Rust nightly (2026-03-01)
  target-spec/        Custom Rust target JSON + fake linker scripts
  templates/          app/, driver/, and library/ starter templates
  examples/           28 examples (hello, hello-driver, hello-library, test-harness,
                      test-harness-gui, test-harness-net, file-io-demo, timer-demo,
                      thread-demo, gui-demo, net-demo, async-demo,
                      thread-amissl-probe, http-client, zlib-roundtrip,
                      picture-viewer, wbstartup-hello, xadmaster-list,
                      async-net-echo, iff-dump, locale-i18n-hello, audio-tone, ram-device, aminet-browser, https-client, sqlite3-demo, json-config, gameboy-test)
  docs/               Roadmap, 10 phase progress logs, nostd-ecosystem guide
  .github/workflows/  CI pipeline (builds all crates + 28 examples, runs host tests)
  cargo-amiga.sh/.bat Project scaffolding, build, and run/test wrapper
```

## amigaos4 Crate Modules (31 total)

**Core (always available):**
application, asl, cstr, datatypes, error, tag, mem, port, screen, boopsi, window, gfx, requester, reaction, dos, locale, io, fmt, menu, panic, async_rt, timer, clipboard

**Application-only (clib4, feature-gated):**
fs, time, env, thread, net, dns, http, https (tls feature)

## Post-1.0 maintenance additions

- `amigaos4::mem::AmigaPool` — safe RAII wrapper over exec `CreatePool`/
  `AllocPooled`/`FreePooled`/`DeletePool` (alongside `AmigaVec`). Sub-allocations
  are freed atomically when the pool drops (`DeletePool`), so it suits a unit of
  work whose memory should all go away together. The raw FFI already existed in
  `amigaos4-sys/src/wrappers/exec.rs`; this adds the safe layer.
- `amigaos4::window` — `WA_REPORT_MOUSE` window tag constant.
- `clib4-nightly` / `clib4-src` refreshed to upstream clib4 `778afb03` (pthread
  per-thread-init crash fix).
- CI (`.github/workflows/ci.yml`) runs on `main` only.
- `cargo-amiga.sh` / `cargo-amiga.bat` — `run` and `test` subcommands: build,
  then deploy + launch on a fleet target (QEMU or hardware) via the
  amiga-fleet CLI (sibling MCP-AmigaOS4 checkout, or `AMIGA_FLEET_MCP_HOST`).
  `test` waits for a serial-output regex (`--wait`) as a headless smoke.
- v0.6.0 (2026-06-11): `amstr!` + nul-termination validation in every
  OS-string entry point; ReAction tag values corrected against the SDK
  54.16 headers (the old ones were invented and silently ignored — see
  amigaos4/CHANGELOG.md); menus (menuclass), ASL file requester,
  listbrowser/chooser/slider, application.library registration; async
  timer + IDCMP futures; `DmaBuffer`; HTTP redirects + chunked;
  ram-device (a complete exec .device) and aminet-browser examples;
  rustdoc publishing workflow.
- v0.7.0 (2026-06-11, GitHub release with prebuilt-examples zip):
  `https`/`TlsStream` over the clib4-built OpenSSL (`tls` feature);
  `datatypes::DtPicture`; clib4-userland examples (sqlite3-demo,
  json-config); gameboy-test proving unmodified crates.io code
  (padme-core passes Blargg cpu_instrs 11/11 on QEMU). Real-hardware
  validation on the X5000 (51/51 + 5/5 + driver mode). The clib4 apt
  repo (clib4pkg.amigasoft.net) fixes the SDK's broken libcurl
  (curl7-clib4) — see roadmap for the /usr-vs-/opt SDK path gotcha.

## Conventions

- Debug: `amigaos4::serial_println!("x = {}", x)`
- Panic: `amigaos4::panic::default_panic_handler(info)`
- RAII: `Amiga*` prefix with Drop impls
- GUI: `LayoutBuilder::vertical().add(button(1, amstr!("OK"))?).build()?`
- Events: `event_loop(&win, |event| match event { Event::GadgetUp { id, code } => ..., ... })`
- Menus: `MenuBuilder::new().menu(amstr!("Project")).item(ID, amstr!("Open...")).build()?`
- Async: `Executor::new()?.spawn(async { ... }); exec.run();`
- Networking: `TcpStream::connect(&addr)?`, `http::get(host, port, path)?`
- OS strings: build with `amstr!("text")` (compile-time `\0`); every wrapper
  validates the terminator at the FFI boundary (`cstr::require_nul`)
- FFI pointer validation: check returned pointers are within expected buffer ranges
- C string reads: always capped with MAX_*_LEN constants
- New Amiga tag constants: take values from the SDK headers
  (`C:\msys64\home\rich_\sdk\include\include_h`) and pin them with unit tests
- Big stack frames (large by-value structs, e.g. third-party emulator
  cores): the default shell stack is 64 KB — embed a `$STACK:` cookie
  (`#[used] #[no_mangle] static STACK_COOKIE`) and pin it with
  `-Wl,--undefined=STACK_COOKIE`, like examples/gameboy-test

## Target testing

- QEMU: `MSYS_NO_PATHCONV=1 ./cargo-amiga.sh test examples/test-harness
  --target qemu-amigaone --wait '51/51 tests passed'` (GUI harness marker:
  `5/5 tests passed`). `MSYS_NO_PATHCONV=1` is required under Git Bash.
- Real X5000: deploy test files to `Files:Temp/Rust/` (persists across
  reboots — do not use RAM:). clib4 2.1 is installed in `LIBS:` (2.0
  backup: `LIBS:clib4.library_2.0.bak`). Do not use ColdReboot on the
  X5000 (it wedges) — power-cycle via the fleet power tools instead.
  ahi.device unit 0 is unavailable there (no AHI-supported card), so
  audio playback is QEMU-only.
