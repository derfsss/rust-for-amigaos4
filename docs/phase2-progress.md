# Phase 2 — Developer Experience

Started: 2026-03-15
Completed: 2026-03-15

## Goal

Make it easy for new users to start building AmigaOS apps in Rust.

## Lessons from Phase 1

- All `&[u8]` paths passed to POSIX FFI must be null-terminated — callers must remember this
- `ppc_asm` is gated behind `#[cfg(target_arch = "powerpc")]` — any new PPC-only code must follow this pattern
- Host-side tests are in `#[cfg(test)]` modules within source files + `tests/host_tests.rs`
- CI runs both cross-compilation and host-side tests
- `read_to_vec` needed bounds checks for negative and oversized file sizes (carried from Phase 1)

## Tasks

### 1. Better panic handler
- **Status:** Complete
- **Deliverable:** `amigaos4/src/panic.rs`
- **Notes:** `default_panic_handler(info)` prints `PANIC at file:line: message\n` via the Serial sink. Handles all combinations of present/absent location and message. Uses `PanicInfo::message()` with `as_str()` fast path for literal strings. All templates and examples updated to use it.

### 2. Debug formatting (Serial sink)
- **Status:** Complete
- **Deliverable:** `amigaos4/src/fmt.rs` (rewritten)
- **Notes:** `Serial` struct implements `core::fmt::Write`. Batches bytes into 127-byte stack buffer, null-terminates each chunk before calling `amiga_debug_str`. New macros `serial_print!` and `serial_println!` accept format arguments (`serial_println!("x = {}", x)`). Legacy `debug_print!`/`debug_println!` macros kept for backward compatibility.

### 3. `cargo-amiga` wrapper script
- **Status:** Complete
- **Deliverables:** `cargo-amiga.sh`, `cargo-amiga.bat`
- **Notes:** Subcommands: `new` (scaffold from template), `build`, `clean`, `setup`, `help`. The `new` command copies template, renames placeholders in Cargo.toml and Makefile, validates project names (`[a-z0-9_-]`). Windows version uses PowerShell for text replacement.

### 4. Project scaffolding
- **Status:** Complete (integrated into cargo-amiga `new` subcommand)
- **Notes:** `cargo-amiga.sh new myproject --mode app|driver` copies templates/app or templates/driver, renames `myapp`/`my-handler` to the given name. Handles hyphen/underscore conversion (binary name uses hyphens, Rust lib name uses underscores).

### 5. More examples
- **Status:** Complete
- **Deliverables:**
  - `examples/file-io-demo/` — Demonstrates fs module: write, read, metadata, delete, mkdir
  - `examples/timer-demo/` — Demonstrates time module: Instant, elapsed, Duration arithmetic
  - `examples/thread-demo/` — Demonstrates thread module: spawn 3 threads, join, sum results
- **Notes:** All examples use `serial_println!` for output, `default_panic_handler` for panics, and proper error handling (no unwrap on fallible operations).

### 6. Documentation
- **Status:** Complete
- **Notes:** Added module-level rustdoc comments (`//!`) to all modules in the `amigaos4` crate: tag, mem, port, boopsi, screen, io, fs, time, thread, env, fmt, panic. Describes purpose, key types, and feature requirements.

---

## Issues Log

### ISSUE-001: fs::read_to_vec large file bounds check (MINOR, fixed)
Code review found that `metadata.size()` (i64) could exceed `usize::MAX` on a theoretical 64-bit port. Added explicit bounds check: `raw_size as u64 > usize::MAX as u64`. Practical impact on 32-bit PPC is nil, but prevents silent truncation for correctness.

---

## Changes from Original Plan

1. **Merged tasks 3 and 4** — `cargo-amiga` wrapper script and project scaffolding combined into one tool since scaffolding is a subcommand of the wrapper.
2. **Kept legacy macros** — `debug_print!`/`debug_println!` retained for backward compatibility alongside the new `serial_print!`/`serial_println!`.
3. **Added 3 new examples to CI** — Extended the build matrix to include file-io-demo, timer-demo, and thread-demo.
