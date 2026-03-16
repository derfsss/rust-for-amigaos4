# Rust for AmigaOS 4

Write native AmigaOS 4.1 applications, device drivers, and shared libraries in Rust.

**Status: Beta** — 25 safe wrapper modules, 129 SDK interface bindings, ~255 tests, 3 build modes. Tested on QEMU (`-M amigaone`).

> **Made with AI** — This project (code, bindings, build infrastructure, and documentation) was built using [Claude Code](https://claude.ai/claude-code) by Anthropic.

---

## Features

- **Full `no_std` Rust** — `core` and `alloc` crates compile to PowerPC
- **`Vec`, `String`, `format!`, `Box`** — heap allocation via global allocator
- **Three build modes** — application (clib4), driver (ExecAllocator), shared library (Resident + interface vectors)
- **25 safe wrapper modules** — GUI (ReAction), networking (TCP/DNS/HTTP), async runtime, timer device, clipboard (IFFParse), DOS, file I/O, threads, and more
- **129 AmigaOS SDK interface bindings** — Exec, DOS, Intuition, Graphics, Timer, IFFParse, and 123 more, all feature-gated
- **Direct vtable dispatch** — call any interface method from Rust via `#[repr(C)]` structs (no overhead)
- **RAII everywhere** — `AmigaWindow`, `AmigaTimer`, `AmigaLock`, `AmigaVec`, `TcpStream`, `PubScreen`, and 7 more auto-cleanup on drop
- **ReAction GUI** — `LayoutBuilder` DSL, `event_loop`, button/string/checkbox/integer gadgets
- **Networking** — `TcpStream`, `TcpListener`, `SocketAddr` parser, DNS resolution, HTTP/1.1 GET client
- **Async runtime** — cooperative executor with Exec signal-based waking, `spawn`/`run`/`block_on`
- **Timer device** — `AmigaTimer` RAII with `delay()`, `get_sys_time()`, `get_up_time()`, `micro_delay()`
- **Clipboard** — `read_text()` / `write_text()` via IFFParse FTXT/CHRS format
- **Shared library output** — template with Resident struct, RTF_AUTOINIT, interface vector tables
- **PPC inline assembly** — cache flush/invalidate, MMIO read/write (8/16/32-bit), memory barriers
- **~255 tests** — 193 host-side unit/integration tests + 60 target-side integration tests
- **CI pipeline** — GitHub Actions cross-compiles all 3 crates + 12 examples, runs host tests

---

## Tested Versions

This project was developed and tested with the following exact versions. **Other versions are not tested and are not supported.**

| Component | Version | Notes |
|-----------|---------|-------|
| Rust | `rustc 1.96.0-nightly (80381278a 2026-03-01)` | Nightly required for `-Zbuild-std` |
| AmigaOS SDK | `54.16 (22.08.2022)` | AmigaOS 4.1 Final Edition SDK |
| Cross-compiler | `ppc-amigaos-gcc 11.5.0` (adtools build) | Inside Docker image |
| C library | `clib4 nightly (0d5fe579, 2026-01-28)` | Pre-built in `clib4-nightly/`, overlaid at build time |
| Docker image | `walkero/amigagccondocker:os4-gcc11` (built 2025-08-18) | Contains GCC + SDK + clib4 |
| QEMU | `qemu-system-ppc -M amigaone` | Test target |

---

## Software & Links

| Software | Purpose | Link |
|----------|---------|------|
| Rust (rustup) | Host compiler | https://rustup.rs/ |
| adtools (ppc-amigaos-gcc) | AmigaOS cross-compiler | https://github.com/sba1/adtools |
| clib4 | POSIX-compatible C library for AmigaOS 4 | https://github.com/afxgroup/clib4 |
| Docker | Container runtime for cross-compiler | https://www.docker.com/ |
| amigagccondocker | Pre-built Docker image with GCC + SDK | https://hub.docker.com/r/walkero/amigagccondocker |
| AmigaOS 4.1 SDK | Headers, libraries, autodocs | https://www.hyperion-entertainment.com/ |
| QEMU | PPC emulator for testing | https://www.qemu.org/ |

---

## Quick Start

### 1. Run Setup

```bash
# Linux / macOS
chmod +x setup.sh && ./setup.sh

# Windows
setup.bat
```

This installs Rust nightly + rust-src and pulls the Docker cross-compiler image.

### 2. Build an Example

```bash
# Application
./build.sh examples/hello

# Driver
./build.sh examples/hello-driver

# Shared library
./build.sh examples/hello-library
```

### 3. Create Your Own Project

```bash
# Application (clib4, -lauto)
cp -r templates/app myproject

# Driver / Handler (no CRT)
cp -r templates/driver mydriver

# Shared Library (.library)
cp -r templates/library mylib
```

Edit `Cargo.toml` (name), `Makefile` (TARGET, RUST_LIB), and `src/main.rs` (your code), then:

```bash
./build.sh myproject
```

---

## What Does Not Work

- **No `std` crate** — this is `no_std` only; the Rust standard library does not support AmigaOS
- **No stack unwinding** — panic strategy is `abort`
- **No 64-bit atomics** — PPC G3/G4 supports only 32-bit atomic operations
- **No TLS/SSL** — AmiSSL integration is too complex for safe wrappers
- **No Rust-native varargs** — 5 varargs methods require C glue wrappers (provided)
- **Not tested on real hardware** — only verified on QEMU; real AmigaOne/Sam/X5000 may differ

---

## Repository Layout

```
rust-for-amigaos4/
  amigaos4-sys/       Raw FFI bindings (129 feature-gated interfaces, C glue, PPC asm)
  amigaos4-alloc/     Global allocator backends (Clib4Allocator, ExecAllocator)
  amigaos4/           Safe wrappers: 25 modules (GUI, networking, async, DOS, timer, clipboard, POSIX)
  clib4-nightly/      Pre-built clib4 C library overlay
  target-spec/        Custom Rust target JSON + fake linker scripts
  templates/          app/, driver/, and library/ starter templates
  examples/           12 examples (hello, hello-driver, hello-library, test-harness,
                      test-harness-gui, test-harness-net, file-io-demo, timer-demo,
                      thread-demo, gui-demo, net-demo, async-demo)
  docs/               Roadmap, 10 phase progress logs, nostd-ecosystem guide
  .github/workflows/  CI pipeline (builds all crates + 12 examples, runs host tests)
  cargo-amiga.sh/.bat Project scaffolding and build wrapper
```

## amigaos4 Crate — 25 Modules

**Core (always available, no clib4 needed):**
error, tag, mem, port, screen, boopsi, window, gfx, requester, reaction, dos, locale, io, fmt, panic, async_rt, timer, clipboard

**Application-only (clib4, feature-gated):**
fs, time, env, thread, net, dns, http

---

## Three Build Modes

| | Application | Driver | Shared Library |
|-|-------------|--------|----------------|
| **CRT** | clib4 | None | None |
| **Entry point** | `main()` | `_start()` → `rust_handler_main()` | Resident + `libInit()` |
| **Allocator** | `Clib4Allocator` | `ExecAllocator` | `ExecAllocator` |
| **POSIX modules** | fs, time, env, thread, net, dns, http | Not available | Not available |
| **Link flags** | `-mcrt=clib4 -lauto` | `-nostartfiles -nodefaultlibs -lgcc` | `-nostartfiles -nodefaultlibs -lgcc` |
| **Output** | Executable | Handler/device | `.library` file |

All three modes support `Vec`, `String`, `format!`, `Box` via the global allocator.

---

## Architecture

### Two-Stage Cross-Compilation

```
 Stage 1 (Your Machine)              Stage 2 (Docker)
+---------------------+    +------------------------------+
| Rust source code    |    | ppc-amigaos-gcc links:       |
|         v           |    |   C glue (.o)                |
| cargo +nightly      |    | + Rust staticlib (.a)        |
|         v           |    | + clib4 / SDK libs           |
| libmyapp.a (PPC)   |--->|         v                    |
+---------------------+    | Native AmigaOS binary        |
                            +------------------------------+
```

### Interface Vtable Dispatch

```rust
// Direct vtable call (no wrapper needed)
unsafe { ((*IExec).FindTask)(IExec, core::ptr::null()) }

// Or using convenience wrapper
unsafe { exec_find_task(core::ptr::null()) }
```

### Safe Wrappers

```rust
// RAII — auto-cleanup on drop
let win = AmigaWindow::open(&tags)?;
let msg = win.wait_msg(); // copies and replies IDCMP message

// GUI builder DSL
let layout = LayoutBuilder::vertical()
    .add(button(1, b"OK\0")?)
    .add(button(2, b"Cancel\0")?)
    .build()?;

// Networking
let mut stream = TcpStream::connect(&addr)?;
stream.write_all(b"Hello")?;

// Async
let mut exec = Executor::new()?;
let answer: u32 = exec.block_on(async { 42 });

// Timer
let mut timer = AmigaTimer::open(TimerUnit::MicroHz)?;
timer.delay(TimerVal::from_millis(500))?;

// Clipboard
clipboard::write_text(b"Copied from Rust!")?;
let text = clipboard::read_text()?;
```

---

## Troubleshooting

### "`.json` target specs require -Zjson-target-spec"

Already included in all templates. Add to `.cargo/config.toml`:
```toml
[unstable]
json-target-spec = true
```

### "undefined reference to `memset`" / `memcpy`

Already included in all templates. Add to `.cargo/config.toml`:
```toml
[unstable]
build-std-features = ["compiler-builtins-mem"]
```

### AllocVecTagList returns NULL

Check MEMF constants: `MEMF_PRIVATE = 1 << 11` (0x800), `MEMF_SHARED = 1 << 12` (0x1000).

### "fake-linker" not found

- **Linux/macOS**: `chmod +x fake-linker.sh`
- **Windows**: Ensure `fake-linker.bat` is next to `powerpc-amigaos.json`

### Link errors about missing IExec / IIntuition

- **Driver/library mode**: `driver_glue.c` or `library_glue.c` must define `struct ExecIFace *IExec = NULL;`
- **Application mode**: Link with `-lauto`

---

## Testing

```bash
# Host-side tests (runs on your machine, no QEMU needed)
cd amigaos4 && cargo +nightly test
cd amigaos4-sys && cargo +nightly test
cd amigaos4-alloc && cargo +nightly test --features exec

# Cross-compile target-side test harness (run on QEMU/real hardware)
./build.sh examples/test-harness
```

---

## Tested Platform

- QEMU (`qemu-system-ppc -M amigaone`) — application, driver, and library modes

## License

MIT
