#!/bin/sh
# build.sh — Build any project in this repository
#
# Usage:
#   ./build.sh <project-path>           Build a project
#   ./build.sh <project-path> clean     Clean a project
#
# Examples:
#   ./build.sh examples/hello           Build the hello example
#   ./build.sh examples/hello-driver    Build the hello-driver example
#   ./build.sh templates/app            Build the app template
#   ./build.sh examples/hello clean     Clean the hello example

set -e

if [ -z "$1" ]; then
    echo "Usage: ./build.sh <project-path> [clean]"
    echo
    echo "Examples:"
    echo "  ./build.sh examples/hello"
    echo "  ./build.sh examples/hello-driver"
    echo "  ./build.sh examples/hello clean"
    exit 1
fi

PROJECT="$1"
REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"

if [ ! -d "$REPO_ROOT/$PROJECT" ]; then
    echo "ERROR: Project directory not found: $PROJECT"
    exit 1
fi

if [ ! -f "$REPO_ROOT/$PROJECT/Cargo.toml" ]; then
    echo "ERROR: No Cargo.toml found in $PROJECT"
    exit 1
fi

if [ "$2" = "clean" ]; then
    echo "Cleaning $PROJECT..."
    rm -rf "$REPO_ROOT/$PROJECT/target" "$REPO_ROOT/$PROJECT/build"
    rm -f "$REPO_ROOT/$PROJECT/fake-linker.log"
    docker run --rm \
        -v "$REPO_ROOT":/repo \
        -w "/repo/$PROJECT" \
        walkero/amigagccondocker:os4-gcc11 \
        make clean 2>/dev/null || true
    echo "Done."
    exit 0
fi

echo "=== Building: $PROJECT ==="
echo

echo "[1/2] Compiling Rust (host)..."
cd "$REPO_ROOT/$PROJECT"
cargo +nightly build --release
echo "  Rust staticlib built."
echo

echo "[2/2] Linking (Docker)..."
docker run --rm \
    -v "$REPO_ROOT":/repo \
    -w "/repo/$PROJECT" \
    walkero/amigagccondocker:os4-gcc11 \
    make clean

docker run --rm \
    -v "$REPO_ROOT":/repo \
    -w "/repo/$PROJECT" \
    walkero/amigagccondocker:os4-gcc11 \
    make all

echo
echo "=== Build complete: $PROJECT ==="
