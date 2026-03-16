# Phase 1 — Stabilise and Harden (Foundation)

Started: 2026-03-15
Completed: 2026-03-15

## Goal

Make what exists reliable and easy to use before adding new features.

## Tasks

### 1. Real hardware testing
- **Status:** Deferred (community ask)
- **Notes:** Requires physical Amiga hardware (AmigaOne, Sam460, X5000, X1000). Cannot be done in current environment. Documented as a community ask in the roadmap.

### 2. CI pipeline
- **Status:** Complete
- **Deliverable:** `.github/workflows/ci.yml`
- **Notes:** GitHub Actions workflow on ubuntu-latest. Installs Rust nightly + rust-src, patches fake-linker for Linux, cross-compiles all 3 library crates + 3 examples to PPC, runs host-side unit tests. Docker link stage is excluded (too heavy for CI).

### 3. Crate versioning and changelogs
- **Status:** Complete
- **Deliverables:** `amigaos4-sys/CHANGELOG.md`, `amigaos4-alloc/CHANGELOG.md`, `amigaos4/CHANGELOG.md`
- **Notes:** Keep a Changelog format. Initial releases documented. Unreleased section ready for future changes.

### 4. Error handling audit
- **Status:** Complete
- **Fixes applied:**
  - `clib4_alloc.rs`: Added alignment handling for >16-byte alignment requests (was ignoring alignment entirely)
  - `exec_alloc.rs`: Added overflow check for `size + overhead` via `checked_add`
  - `fs.rs`: Added negative file size check in `read_to_vec` before casting `i64` to `usize`
  - `time.rs`: Added `clock_gettime` return value check in `Instant::now()` (was silently ignoring errors)
  - `env.rs`: Added `MAX_ENV_LEN` (8192) safety limit on `getenv` C string reads to prevent runaway pointer dereference
- **Remaining known items (acceptable risk):**
  - Null-termination of `&[u8]` paths passed to POSIX functions is caller's responsibility (documented in API)
  - Same for `class_id` in `AmigaObject::new` and screen names in `PubScreen::lock`
  - `io.rs` uses `AmigaError::IoError(0)` for unexpected EOF (errno 0 = ESUCCESS) — acceptable sentinel

### 5. Alignment audit for ExecAllocator
- **Status:** Complete
- **Fixes applied:**
  - `exec_alloc.rs`: Added `checked_add` to prevent overflow in over-allocation path
  - `clib4_alloc.rs`: Added matching alignment handling (was previously missing — only `malloc(size)` with no alignment awareness)
- **Notes:** Both allocators now handle alignment > 16 bytes by over-allocating and storing the original pointer before the aligned region. The `Layout` type in Rust guarantees alignment is a power of 2, so no additional validation needed.

### 6. Test suite (host-side)
- **Status:** Complete
- **Deliverables:** 61 unit tests across 4 modules
  - `error.rs`: 12 tests (Display formatting, equality, copy/clone, Result usage)
  - `time.rs`: 26 tests (Duration constructors, conversions, ZERO constant, arithmetic, ordering, roundtrips, large values)
  - `tag.rs`: 9 tests (empty build, single/multiple tags, tag_if true/false, mixed, edge cases)
  - `io.rs`: 14 tests (MockReader/MockWriter, chunked reads/writes, read_exact, write_all, EOF handling, failure paths)
- **Infrastructure:** `ppc_asm` module gated behind `#[cfg(target_arch = "powerpc")]` so host-side tests compile on x86_64

### 7. Test harness (target-side)
- **Status:** Complete
- **Deliverable:** `examples/test-harness/` (full project with Cargo.toml, Makefile, fake-linker scripts, main.rs)
- **Notes:** 8 tests exercising Vec, String/format!, TagListBuilder, AmigaVec, AmigaMsgPort, Duration arithmetic, file I/O (write/read/delete), and environment (current_dir). Reports PASS/FAIL per test via DebugPrintF, returns exit code 0 on all-pass.

---

## Issues Log

### ISSUE-001: Clib4Allocator ignored alignment (CRITICAL, fixed)
`Clib4Allocator::alloc()` called `malloc(layout.size())` without regard for `layout.align()`. While clib4's malloc returns 16-byte aligned memory, Rust types can require higher alignment. Fixed by adding over-allocation path matching ExecAllocator's approach.

### ISSUE-002: ExecAllocator size+overhead overflow (CRITICAL, fixed)
`layout.size() + overhead` could theoretically overflow on pathological inputs. Fixed with `checked_add`, returning null on overflow.

### ISSUE-003: clock_gettime error ignored (CRITICAL, fixed)
`Instant::now()` called `clock_gettime()` and used the zeroed Timespec regardless of the return value. Fixed to check return code.

### ISSUE-004: Unbounded C string read in env.rs (CRITICAL, fixed)
`var()` iterated `*ptr.add(len)` with no upper bound when reading getenv results. Fixed with MAX_ENV_LEN (8192) limit.

### ISSUE-005: Negative file size in read_to_vec (IMPORTANT, fixed)
`metadata.size()` returns `i64` but was cast to `usize` without checking for negative values. Fixed with explicit check.

### ISSUE-006: MMIO volatile semantics (reviewed, no change needed)
Code review flagged MMIO reads as needing volatile semantics. In Rust's `asm!` macro, blocks without `options(pure)` are never optimized away — the current code is correct.

---

## Changes from Original Plan

1. **Added alignment handling to Clib4Allocator** — Not in original plan but discovered during audit. The allocator was silently ignoring alignment requirements > 16 bytes.
2. **Gated ppc_asm behind cfg** — Required to enable host-side testing. Not a behavioral change for PPC targets.
3. **Added test-harness to CI** — Extended CI scope beyond the original plan to include the new test harness example.
