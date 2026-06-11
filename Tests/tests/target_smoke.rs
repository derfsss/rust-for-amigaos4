//! Target-side smoke test (opportunistic).
//!
//! Builds `examples/test-harness` through the Docker cross-toolchain
//! and runs it on a fleet target (QEMU or real hardware) via
//! `cargo-amiga.sh test`, asserting the harness prints its all-pass
//! marker on serial.
//!
//! This needs Docker, the amiga-fleet CLI (sibling MCP-AmigaOS4
//! checkout or `AMIGA_FLEET_MCP_HOST`), and a configured target — far
//! more than a host CI runner has — so it only executes when
//! `AMIGA_TARGET_SMOKE=1` is set; otherwise it passes with a notice,
//! the same opportunistic pattern as `rust_sdk_audit.rs`.

use amiga_tests::repo_root;
use std::process::Command;

/// The serial marker test-harness prints when every test passed.
const PASS_MARKER: &str = "51/51 tests passed";

#[test]
fn target_harness_smoke() {
    if std::env::var("AMIGA_TARGET_SMOKE").as_deref() != Ok("1") {
        eprintln!(
            "target_harness_smoke: skipped (set AMIGA_TARGET_SMOKE=1 to build \
             + run examples/test-harness on a fleet target)"
        );
        return;
    }

    // Run through bash so the same invocation works from Windows
    // (Git Bash / MSYS) and Linux. MSYS_NO_PATHCONV stops Git Bash
    // mangling the /repo paths inside build.sh's docker run.
    let output = Command::new("bash")
        .arg("cargo-amiga.sh")
        .args(["test", "examples/test-harness", "--wait", PASS_MARKER])
        .env("MSYS_NO_PATHCONV", "1")
        .current_dir(repo_root())
        .output()
        .expect("failed to spawn bash cargo-amiga.sh (is bash on PATH?)");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        output.status.success(),
        "cargo-amiga test failed (status {:?}).\n--- stdout ---\n{}\n--- stderr ---\n{}",
        output.status.code(),
        stdout,
        stderr
    );
}
