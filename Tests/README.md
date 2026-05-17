# Tests — black-box test suite for rust-for-amigaos4

A standalone Rust crate that verifies project-level invariants from outside
the three production crates. Sits alongside the in-crate unit tests and
doctests, and orchestrates them as a single combined run.

## What it covers

| Category | File | What it checks |
|---|---|---|
| Build & reproducibility | `tests/repro.rs` | Cargo targets resolve; example/template Makefiles parse; clib4-nightly binaries match submodule HEAD |
| Public API smoke | `tests/host_amigaos4.rs` | Pure-Rust integration tests of the amigaos4 crate (migrated from `amigaos4/tests/host_tests.rs`). Gated behind `--features link-amigaos4` because the link is Linux-only today (see "Known limitations") |
| Documentation claims | `tests/doc_claims.rs` | Counts and assertions in README/CLAUDE.md/roadmap match reality (interface count, module count, example count, etc.) |
| Regression coverage | `tests/regression.rs` | Specific past bugs cannot return silently: `-s` flag in clib4 LDFLAGS, CRLF on LF-declared files, submodule/binary drift, `_start` / `__amigaos4__` symbols present |

In-source unit tests (`#[cfg(test)] mod tests {}` inside each library's source files) and doctests stay where they are — they exercise private internals and are inherently tied to their source. The runner picks them up by spawning `cargo test` per-crate.

## Running

```bash
# Everything that runs on this host
cargo run --bin amiga-test-runner

# Just the Tests/ own integration tests
cargo test

# Including the migrated amigaos4 host tests (Linux/CI only — see below)
cargo test --features link-amigaos4
```

## Known limitations

**No QEMU layer**: Target-side test harnesses (`examples/test-harness*`) are still cross-compiled to PowerPC and intended to be run on QEMU or real hardware. The Tests/ orchestrator does not drive QEMU today.
