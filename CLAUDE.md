# CLAUDE.md — Project Context for Claude Code

## Project

Rust for AmigaOS 4 — Write native AmigaOS 4.1 applications, device drivers, and shared libraries in Rust (`no_std`).

**All 10 roadmap phases complete. ~255 tests, 25 modules, 3 templates, shared library output.**

## Repository Layout

```
rust-for-amigaos4/
  amigaos4-sys/       Raw FFI bindings (129 feature-gated interfaces, C glue, PPC asm)
  amigaos4-alloc/     Global allocator backends (Clib4Allocator, ExecAllocator)
  amigaos4/           Safe wrappers: 25 modules (GUI, networking, async, DOS, timer, clipboard, POSIX)
  clib4-nightly/      Pre-built clib4 C library overlay (binaries only)
  clib4-src/          clib4 source pinned via submodule at commit 778afb03 (development tip)
  rust-toolchain.toml Pins the exact Rust nightly (2026-03-01)
  target-spec/        Custom Rust target JSON + fake linker scripts
  templates/          app/, driver/, and library/ starter templates
  examples/           12 examples (hello, hello-driver, hello-library, test-harness,
                      test-harness-gui, test-harness-net, file-io-demo, timer-demo,
                      thread-demo, gui-demo, net-demo, async-demo)
  docs/               Roadmap, 10 phase progress logs, nostd-ecosystem guide
  .github/workflows/  CI pipeline (builds all crates + 12 examples, runs host tests)
  cargo-amiga.sh/.bat Project scaffolding and build wrapper
```

## amigaos4 Crate Modules (25 total)

**Core (always available):**
error, tag, mem, port, screen, boopsi, window, gfx, requester, reaction, dos, locale, io, fmt, panic, async_rt, timer, clipboard

**Application-only (clib4, feature-gated):**
fs, time, env, thread, net, dns, http

## Conventions

- Debug: `amigaos4::serial_println!("x = {}", x)`
- Panic: `amigaos4::panic::default_panic_handler(info)`
- RAII: `Amiga*` prefix with Drop impls
- GUI: `LayoutBuilder::vertical().add(button(1, b"OK\0")?).build()?`
- Events: `event_loop(&win, |event| match event { ... })`
- Async: `Executor::new()?.spawn(async { ... }); exec.run();`
- Networking: `TcpStream::connect(&addr)?`, `http::get(host, port, path)?`
- OS strings: `b"text\0"` (null-terminated)
- FFI pointer validation: check returned pointers are within expected buffer ranges
- C string reads: always capped with MAX_*_LEN constants
