#!/bin/sh
# setup.sh — One-time setup for Rust-for-AmigaOS4
#
# Installs Rust nightly, rust-src component, and pulls the Docker image.
# Run once before your first build.

set -e

echo "=== Rust for AmigaOS 4 — Setup ==="
echo

# 1. Check rustup
if ! command -v rustup >/dev/null 2>&1; then
    echo "ERROR: rustup not found. Install from https://rustup.rs/"
    exit 1
fi

# 2. Install nightly toolchain
echo "[1/3] Installing Rust nightly toolchain..."
rustup toolchain install nightly
echo "  Done."

# 3. Install rust-src (needed for build-std)
echo "[2/3] Installing rust-src component..."
rustup component add rust-src --toolchain nightly
echo "  Done."

# 4. Pull Docker image
if ! command -v docker >/dev/null 2>&1; then
    echo "WARNING: Docker not found. Install Docker to link AmigaOS binaries."
    echo "  https://docs.docker.com/get-docker/"
else
    echo "[3/3] Pulling AmigaOS cross-compiler Docker image..."
    docker pull walkero/amigagccondocker:os4-gcc11
    echo "  Done."
fi

echo
echo "=== Setup complete ==="
echo
echo "Next steps:"
echo "  cd examples/hello"
echo "  ./build.sh        # or build.bat on Windows"
