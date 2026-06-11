# Rust for AmigaOS 4

Write native AmigaOS 4.1 applications, device drivers, and shared libraries in Rust.

**Status: Beta** — 29 safe wrapper modules, 129 SDK interface bindings, ~365 tests, 3 build modes. Tested on QEMU (`-M amigaone`) **and real X5000 hardware**.

> **Made with AI** — This project (code, bindings, build infrastructure, and documentation) was built using [Claude Code](https://claude.ai/claude-code) by Anthropic.

---

## Features

- **Full `no_std` Rust** — `core` and `alloc` crates compile to PowerPC
- **`Vec`, `String`, `format!`, `Box`** — heap allocation via global allocator
- **Three build modes** — application (clib4), driver (ExecAllocator), shared library (Resident + interface vectors)
- **29 safe wrapper modules** — GUI (ReAction), menus, ASL file requesters, networking (TCP/DNS/HTTP), async runtime, timer device, clipboard (IFFParse), DOS, file I/O, threads, and more
- **129 AmigaOS SDK interface bindings** — Exec, DOS, Intuition, Graphics, Timer, IFFParse, and 123 more, all feature-gated
- **Direct vtable dispatch** — call any interface method from Rust via `#[repr(C)]` structs (no overhead)
- **RAII everywhere** — `AmigaWindow`, `AmigaTimer`, `AmigaLock`, `AmigaVec`, `TcpStream`, `PubScreen`, and 14 more auto-cleanup on drop
- **Checked OS strings** — `amstr!("text")` builds null-terminated strings at compile time; every wrapper validates at the FFI boundary
- **ReAction GUI** — `LayoutBuilder` DSL, `event_loop`, button/string/checkbox/integer/slider/chooser/listbrowser gadgets
- **Menus** — `MenuBuilder` DSL over the OS4 menuclass, with `MM_NEXTSELECT` pick decoding
- **ASL file requester** — Open/Save/Drawer dialogs with pattern filters
- **application.library** — RAII registration with single-instance enforcement
- **Networking** — `TcpStream`, `TcpListener`, `SocketAddr` parser, DNS resolution, HTTP/1.1 GET client with redirects + chunked decoding
- **Async runtime** — cooperative executor with Exec signal-based waking; awaitable timer delays and IDCMP window events
- **Timer device** — `AmigaTimer` RAII with `delay()`, `get_sys_time()`, `get_up_time()`, `micro_delay()`, async `delay_async()`
- **Clipboard** — `read_text()` / `write_text()` via IFFParse FTXT/CHRS format
- **Shared library output** — template with Resident struct, RTF_AUTOINIT, interface vector tables
- **Exec device support** — `examples/ram-device` is a complete `.device` (BeginIO/AbortIO, quick I/O) in Rust; `DmaBuffer` pairs MEMF_SHARED with cache maintenance
- **PPC inline assembly** — cache flush/invalidate, MMIO read/write (8/16/32-bit), memory barriers
- **~365 tests** — host-side per-crate unit tests, doctests, and the black-box suite in `Tests/`, plus 60 target-side integration tests run on QEMU
- **CI pipeline** — GitHub Actions cross-compiles all 3 crates + 24 examples, runs host tests, publishes rustdoc

---

## Tested Versions

This project was developed and tested with the following exact versions. **Other versions are not tested and are not supported.**

| Component | Version | Notes |
|-----------|---------|-------|
| Rust | `nightly-2026-03-01` | Pinned in `rust-toolchain.toml` (nightly required for `-Zbuild-std`) |
| AmigaOS SDK | `54.16 (22.08.2022)` | AmigaOS 4.1 Final Edition SDK |
| Cross-compiler | `ppc-amigaos-gcc 11.5.0` (adtools build) | Inside Docker image |
| C library | `clib4` commit `778afb03` (2026-05-14, upstream `development` / `nightly`) | Source pinned as submodule in `clib4-src/`; pre-built binaries in `clib4-nightly/` overlay at link time |
| Docker image | `walkero/amigagccondocker:os4-gcc11` (built 2025-08-18) | Contains GCC + SDK + clib4 |
| QEMU | `qemu-system-ppc -M amigaone` | Test target |

---

## Software & Links

| Software | Purpose | Link |
|----------|---------|------|
| Rust (rustup) | Host compiler | https://rustup.rs/ |
| adtools (ppc-amigaos-gcc) | AmigaOS cross-compiler | https://github.com/sba1/adtools |
| clib4 | POSIX-compatible C library for AmigaOS 4 | https://github.com/AmigaLabs/clib4 |
| Docker | Container runtime for cross-compiler | https://www.docker.com/ |
| amigagccondocker | Pre-built Docker image with GCC + SDK | https://hub.docker.com/r/walkero/amigagccondocker |
| AmigaOS 4.1 SDK | Headers, libraries, autodocs | https://www.hyperion-entertainment.com/ |
| QEMU | PPC emulator for testing | https://www.qemu.org/ |

---

## Quick Start

### 1. Clone with submodules

```bash
git clone --recurse-submodules https://github.com/<user>/rust-for-amigaos4.git
# or, if you already cloned without --recurse-submodules:
git submodule update --init --recursive
```

The `clib4-src/` submodule pins the exact clib4 source (commit `778afb03`, tip of the upstream `development` branch) the pre-built `clib4-nightly/` binaries were produced from. Checkout is required for reproducible local rebuilds; it is not needed to run `build.sh`, which uses the pre-built binaries directly.

### 2. Run Setup

```bash
# Linux / macOS
chmod +x setup.sh && ./setup.sh

# Windows
setup.bat
```

This installs the pinned Rust toolchain (read from `rust-toolchain.toml`) with `rust-src`, initialises submodules, and pulls the Docker cross-compiler image.

### 3. Build an Example

```bash
# Application
./build.sh examples/hello

# Driver
./build.sh examples/hello-driver

# Shared library
./build.sh examples/hello-library
```

### 4. Create Your Own Project

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

## clib4: Build-time Overlay and Runtime Install

Application-mode binaries (built with `-mcrt=clib4 -lauto`, the default in `templates/app/`) are linked against a specific `clib4` build and **open `clib4.library` at runtime**. Driver and shared-library modes do not need `clib4` at all. Two concerns to keep straight:

### Build time (Docker / cross-compiler)

`build.sh` overlays the contents of `clib4-nightly/` onto the cross-compiler's SDK inside the Docker container on every link:

```bash
cp -r /repo/clib4-nightly/lib/*     /opt/ppc-amigaos/ppc-amigaos/SDK/clib4/lib/
cp -r /repo/clib4-nightly/include/* /opt/ppc-amigaos/ppc-amigaos/SDK/clib4/include/
```

You do not need to install anything into the Docker image yourself — the overlay is applied by the build script. The clib4 binaries that ship in `clib4-nightly/` are reproducible from the `clib4-src/` submodule at commit `778afb03` (see "Rebuilding clib4" below).

If you want to **replace** the overlaid clib4 with your own build, drop new `lib/`, `include/`, and `clib4.library` / `clib4.library.debug` files into `clib4-nightly/` and re-run `./build.sh <project>`. Nothing else needs to change.

### Rebuilding clib4 from the submodule

The submodule is pinned to the exact source the pre-built `clib4-nightly/` binaries came from:

```bash
# Build clib4.library + clib4.library.debug + lib*.a + CRT objects inside Docker
docker run --rm \
    -v "$(pwd)":/repo \
    -w /repo/clib4-src \
    walkero/amigagccondocker:os4-gcc11 \
    gmake -f GNUmakefile.os4 -j"$(nproc)"

# Outputs land in clib4-src/build/
ls clib4-src/build/clib4.library clib4-src/build/clib4.library.debug
ls clib4-src/build/lib/
```

To use that build as the overlay, copy `clib4-src/build/lib/*` and `clib4-src/build/clib4.library*` into `clib4-nightly/` (replacing the shipped binaries). Versions reported by the freshly-built library will match `clib4.library 2.1 (<build-date>)`.

### Runtime (AmigaOS 4.1FE target)

`clib4.library` for Rust programs must be placed in `PROGDIR:` — the same directory as the executable.

**Checklist for application mode:**

1. Build your project: `./build.sh myproject`
2. Copy into the target directory on the Amiga:
   - The executable (`myproject/myproject`)
   - `clib4-nightly/clib4.library` (or your own build of it)
3. Run from a Shell and observe with `dumpdebugbuffer` to see `serial_println!` output.

Driver-mode (`hello-driver`) and shared-library-mode binaries do not need `clib4.library` at runtime — they use `ExecAllocator` and talk to `IExec` directly.

---

## What Does Not Work

- **No `std` crate** — this is `no_std` only; the Rust standard library does not support AmigaOS
- **No stack unwinding** — panic strategy is `abort`
- **No 64-bit atomics** — PPC G3/G4 supports only 32-bit atomic operations
- **No TLS/SSL yet** — the AmiSSL init chain is validated (see `thread-amissl-probe`); the socket plumbing plan is in the roadmap
- **No Rust-native varargs** — 5 varargs methods require C glue wrappers (provided)
- **Audio needs an AHI-supported card** — `audio-tone` plays on QEMU; on machines where AHI finds no sound card, `OpenDevice("ahi.device")` fails cleanly

---

## Repository Layout

```
rust-for-amigaos4/
  amigaos4-sys/       Raw FFI bindings (129 feature-gated interfaces, C glue, PPC asm)
  amigaos4-alloc/     Global allocator backends (Clib4Allocator, ExecAllocator)
  amigaos4/           Safe wrappers: 29 modules (GUI, menus, ASL, networking, async, DOS, timer, clipboard, POSIX)
  clib4-nightly/      Pre-built clib4 C library overlay (binaries only)
  clib4-src/          clib4 source pinned via submodule at commit 778afb03 (development tip)
  rust-toolchain.toml Pins the exact Rust nightly (2026-03-01) used for all builds
  target-spec/        Custom Rust target JSON + fake linker scripts
  templates/          app/, driver/, and library/ starter templates
  examples/           24 examples (hello, hello-driver, hello-library, test-harness,
                      test-harness-gui, test-harness-net, file-io-demo, timer-demo,
                      thread-demo, gui-demo, net-demo, async-demo,
                      thread-amissl-probe, http-client, zlib-roundtrip,
                      picture-viewer, wbstartup-hello, xadmaster-list,
                      async-net-echo, iff-dump, locale-i18n-hello, audio-tone, ram-device, aminet-browser)
  docs/               Roadmap, 10 phase progress logs, nostd-ecosystem guide
  .github/workflows/  CI pipeline (builds all crates + 24 examples, runs host tests)
  cargo-amiga.sh/.bat Project scaffolding, build, and run/test wrapper
```

## amigaos4 Crate — 29 Modules

**Core (always available, no clib4 needed):**
application, asl, cstr, error, tag, mem, port, screen, boopsi, window, gfx, requester, reaction, dos, locale, io, fmt, menu, panic, async_rt, timer, clipboard

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
| **Link flags** | `-mcrt=clib4 -lauto` | `-nostartfiles -nodefaultlibs -lgcc` | `-nostartfiles -nodefaultlibs -Wl,--undefined=RomTag -lgcc` |
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
| cargo build        |    | + Rust staticlib (.a)        |
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
cd amigaos4 && cargo test
cd amigaos4-sys && cargo test
cd amigaos4-alloc && cargo test --features exec

# Cross-compile target-side test harness (run on QEMU/real hardware)
./build.sh examples/test-harness
```

---

## Tested Platforms

- QEMU (`qemu-system-ppc -M amigaone`) — application, driver, and library modes; full target harnesses (51/51 main, 5/5 GUI) + AHI playback
- **AmigaOne X5000 (P5020, Kickstart 54.57) — real hardware**: 51/51 main harness, 5/5 GUI harness, driver-mode (`hello-driver`) verified on 2026-06-11. Requires clib4 2.1 in `LIBS:` (or `PROGDIR:` when no older clib4 shadows it)

## License

MIT
