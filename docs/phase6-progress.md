# Phase 6 — Advanced / Long-Term

Started: 2026-03-15
Completed: 2026-03-15

## Goal

Push the boundaries of what Rust on AmigaOS can do.

## Tasks

### 1. Async runtime
- **Status:** Complete
- **Deliverable:** `amigaos4/src/async_rt.rs` (285 lines)
- **Contents:**
  - `Executor` — single-threaded cooperative executor backed by Exec signals
  - `spawn()` — enqueue async tasks
  - `run()` — drive all tasks to completion; sleeps via `exec_wait` when idle
  - `block_on()` — run a single future and return its result
  - Custom `RawWaker`/`RawWakerVTable` that signals the executor's task
  - `WakerData` heap-allocated per waker, proper clone/wake/drop lifecycle
  - RAII signal bit cleanup in `Executor::Drop`

### 2. no_std ecosystem documentation
- **Status:** Complete
- **Deliverable:** `docs/nostd-ecosystem.md`
- **Contents:**
  - 10 categories of compatible crates (data structures, serialization, parsing, bit manipulation, formatting, hashing, random, error handling, math, encoding)
  - Cargo.toml snippets with correct `default-features = false` and `features = ["alloc"]` where needed
  - Proc macro workaround section (build.rs code generation, include! pattern)
  - Integration pattern with bitflags example
  - Notes on PPC cross-compilation constraints

### 3. Async example
- **Status:** Complete
- **Deliverable:** `examples/async-demo/` (7 files)
- **Contents:** Spawns multiple async tasks, demonstrates chain_ops, block_on with return value

### 4-6. Deferred items
- **Shared library output** — Deferred (needs custom CRT glue + interface table)
- **USB device drivers** — Deferred (needs real hardware)
- **Upstream Rust target** — Deferred (needs community proposal)

---

## Issues Log

No issues found during implementation or code review.

---

## Changes from Original Plan

1. **Scoped to achievable items** — Focused on async runtime, ecosystem docs, and proc macro workarounds. Deferred shared lib, USB, and upstream target as community efforts.
2. **Async runtime uses conservative polling** — All tasks re-polled on any wake signal, rather than per-task tracking. Simple and correct for single-threaded use.
3. **block_on uses Rc<RefCell<Option<T>>>** — Stores the future's result in a shared cell, extracted after run() completes.
