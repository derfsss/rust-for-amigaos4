#!/bin/sh
# cargo-amiga.sh — Project management wrapper for Rust-for-AmigaOS4
#
# Usage:
#   cargo-amiga.sh new <name> [--mode app|driver]
#   cargo-amiga.sh build [<project-path>]
#   cargo-amiga.sh clean [<project-path>]
#   cargo-amiga.sh setup
#   cargo-amiga.sh help

set -e

REPO_ROOT="$(cd "$(dirname "$0")" && pwd)"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

usage() {
    cat <<'USAGE'
cargo-amiga.sh — Project management wrapper for Rust-for-AmigaOS4

Subcommands:

  new <name> [--mode app|driver]
      Create a new project from a template.
      --mode app      Application with clib4 and -lauto (default)
      --mode driver   Handler/driver — no CRT, no clib4

  build [<project-path>]
      Build a project (delegates to build.sh).
      If no path is given, uses the current directory.

  clean [<project-path>]
      Clean a project (delegates to build.sh … clean).
      If no path is given, uses the current directory.

  run [<project-path>] [--target NAME] [--args "..."]
      Build, then deploy + launch the exe on a fleet target (QEMU or hardware)
      via the amiga-fleet CLI. Starts + stops QEMU around the run.

  test [<project-path>] --wait REGEX [--target NAME] [--args "..."] [--timeout N]
      Like 'run', but wait for REGEX in the target's serial output and exit
      non-zero if it is not seen within --timeout seconds (a headless smoke).
      Needs the amiga-fleet CLI: a sibling MCP-AmigaOS4 checkout, or set
      AMIGA_FLEET_MCP_HOST=/path/to/MCP-AmigaOS4/host.

  setup
      Run first-time setup (delegates to setup.sh).

  help
      Show this message.

Examples:

  ./cargo-amiga.sh new my-cool-app
  ./cargo-amiga.sh new ram-handler --mode driver
  ./cargo-amiga.sh build my-cool-app
  ./cargo-amiga.sh run my-cool-app --target qemu
  ./cargo-amiga.sh test examples/test-harness --wait '51/51 tests passed'
  ./cargo-amiga.sh clean my-cool-app
  ./cargo-amiga.sh setup
USAGE
}

die() {
    echo "ERROR: $*" >&2
    exit 1
}

# Convert a name to the lib-safe variant (hyphens -> underscores)
to_lib_name() {
    echo "$1" | tr '-' '_'
}

# Convert a name to the binary-safe variant (underscores -> hyphens)
to_bin_name() {
    echo "$1" | tr '_' '-'
}

# Validate a project name: non-empty, only [a-z0-9_-], must start with a letter
validate_name() {
    case "$1" in
        "") die "Project name must not be empty." ;;
    esac
    # Check for invalid characters
    cleaned="$(echo "$1" | tr -d 'a-z0-9_-')"
    if [ -n "$cleaned" ]; then
        die "Invalid project name '$1'. Use only lowercase letters, digits, hyphens, and underscores."
    fi
    # Must start with a letter
    first="$(echo "$1" | cut -c1)"
    case "$first" in
        [a-z]) ;;
        *) die "Project name '$1' must start with a lowercase letter." ;;
    esac
}

# ---------------------------------------------------------------------------
# new — scaffold a project from a template
# ---------------------------------------------------------------------------

cmd_new() {
    NAME=""
    MODE="app"

    # Parse arguments
    while [ $# -gt 0 ]; do
        case "$1" in
            --mode)
                shift
                MODE="$1"
                ;;
            -*)
                die "Unknown option: $1"
                ;;
            *)
                if [ -z "$NAME" ]; then
                    NAME="$1"
                else
                    die "Unexpected argument: $1"
                fi
                ;;
        esac
        shift
    done

    if [ -z "$NAME" ]; then
        die "Usage: cargo-amiga.sh new <name> [--mode app|driver]"
    fi

    validate_name "$NAME"

    case "$MODE" in
        app|driver) ;;
        *) die "Unknown mode '$MODE'. Use 'app' or 'driver'." ;;
    esac

    TEMPLATE_DIR="$REPO_ROOT/templates/$MODE"
    TARGET_DIR="$REPO_ROOT/$NAME"

    if [ ! -d "$TEMPLATE_DIR" ]; then
        die "Template directory not found: templates/$MODE"
    fi

    if [ -e "$TARGET_DIR" ]; then
        die "Directory already exists: $NAME"
    fi

    BIN_NAME="$(to_bin_name "$NAME")"
    LIB_NAME="$(to_lib_name "$NAME")"

    echo "Creating $MODE project '$NAME'..."

    # Copy template tree
    cp -R "$TEMPLATE_DIR" "$TARGET_DIR"

    # --- Cargo.toml replacements ---
    if [ "$MODE" = "app" ]; then
        # App template has name = "myapp" twice (package + lib). Replace both
        # with the hyphenated name, then fix the [lib] one to use underscores.
        sed -i "s/name = \"myapp\"/name = \"$BIN_NAME\"/" "$TARGET_DIR/Cargo.toml"
        sed -i "/^\[lib\]$/,/^\[/ s/name = \"$BIN_NAME\"/name = \"$LIB_NAME\"/" "$TARGET_DIR/Cargo.toml"
    else
        # Driver template placeholders: name = "my-handler" (package), name = "my_handler" (lib)
        sed -i "s/name = \"my-handler\"/name = \"$BIN_NAME\"/" "$TARGET_DIR/Cargo.toml"
        sed -i "s/name = \"my_handler\"/name = \"$LIB_NAME\"/" "$TARGET_DIR/Cargo.toml"
    fi

    # --- Makefile replacements ---
    if [ "$MODE" = "app" ]; then
        sed -i "s/TARGET    = myapp/TARGET    = $BIN_NAME/" "$TARGET_DIR/Makefile"
        sed -i "s|target/powerpc-amigaos/release/libmyapp\.a|target/powerpc-amigaos/release/lib${LIB_NAME}.a|" "$TARGET_DIR/Makefile"
    else
        sed -i "s/TARGET    = my-handler/TARGET    = $BIN_NAME/" "$TARGET_DIR/Makefile"
        sed -i "s|target/powerpc-amigaos/release/libmy_handler\.a|target/powerpc-amigaos/release/lib${LIB_NAME}.a|" "$TARGET_DIR/Makefile"
    fi

    echo
    echo "Project '$NAME' created ($MODE mode)."
    echo
    echo "Next steps:"
    echo "  ./cargo-amiga.sh build $NAME"
    echo "  # or:"
    echo "  ./build.sh $NAME"
}

# ---------------------------------------------------------------------------
# build — compile a project
# ---------------------------------------------------------------------------

cmd_build() {
    if [ -n "$1" ]; then
        PROJECT="$1"
    else
        # Use current directory, expressed relative to repo root
        CWD="$(pwd)"
        PREFIX="$REPO_ROOT/"
        case "$CWD" in
            "$REPO_ROOT"/*)
                PROJECT="${CWD#$PREFIX}"
                ;;
            *)
                die "Current directory is not inside the repository."
                ;;
        esac
    fi

    exec "$REPO_ROOT/build.sh" "$PROJECT"
}

# ---------------------------------------------------------------------------
# clean — clean a project
# ---------------------------------------------------------------------------

cmd_clean() {
    if [ -n "$1" ]; then
        PROJECT="$1"
    else
        CWD="$(pwd)"
        PREFIX="$REPO_ROOT/"
        case "$CWD" in
            "$REPO_ROOT"/*)
                PROJECT="${CWD#$PREFIX}"
                ;;
            *)
                die "Current directory is not inside the repository."
                ;;
        esac
    fi

    exec "$REPO_ROOT/build.sh" "$PROJECT" clean
}

# ---------------------------------------------------------------------------
# run / test — build, then deploy + launch on a fleet target (QEMU or hardware)
# via the amiga-fleet CLI (MCP-AmigaOS4). `run` just launches; `test` also waits
# for a serial marker and exits non-zero if it is not seen (a headless smoke).
# ---------------------------------------------------------------------------

# Locate the MCP-AmigaOS4 host dir (which has the fleet CLI). Override with
# AMIGA_FLEET_MCP_HOST; else look for a sibling checkout.
fleet_host() {
    if [ -n "${AMIGA_FLEET_MCP_HOST:-}" ]; then
        echo "$AMIGA_FLEET_MCP_HOST"; return 0
    fi
    for h in "$REPO_ROOT/../../MCP-AmigaOS4/host" "$REPO_ROOT/../MCP-AmigaOS4/host"; do
        if [ -d "$h/src/amiga_fleet_mcp" ]; then echo "$h"; return 0; fi
    done
    return 1
}

# Run the fleet CLI (python -m amiga_fleet_mcp.cli) with the given args.
fleet_cli() {
    host="$(fleet_host)" || die "amiga-fleet CLI not found. Set AMIGA_FLEET_MCP_HOST=/path/to/MCP-AmigaOS4/host or place MCP-AmigaOS4 as a sibling of this repo."
    py="$host/.venv/Scripts/python.exe"
    [ -f "$py" ] || py="$host/.venv/bin/python"
    [ -f "$py" ] || py="python3"
    ( cd "$host" && PYTHONPATH=src "$py" -m amiga_fleet_mcp.cli "$@" )
}

# Resolve a project path (arg or cwd-relative-to-repo, like cmd_build) and the
# executable it produces (the Makefile's TARGET).
_resolve_project() {
    if [ -n "$1" ]; then
        PROJECT="$1"
    else
        CWD="$(pwd)"; PREFIX="$REPO_ROOT/"
        case "$CWD" in
            "$REPO_ROOT"/*) PROJECT="${CWD#$PREFIX}" ;;
            *) die "Current directory is not inside the repository (pass a project path)." ;;
        esac
    fi
    case "$PROJECT" in /*) PROJ_DIR="$PROJECT" ;; *) PROJ_DIR="$REPO_ROOT/$PROJECT" ;; esac
    [ -f "$PROJ_DIR/Makefile" ] || die "no Makefile in $PROJ_DIR"
    TARGET_NAME="$(grep -E '^TARGET[[:space:]]*=' "$PROJ_DIR/Makefile" | head -1 | sed -E 's/^TARGET[[:space:]]*=[[:space:]]*//' | tr -d ' \r')"
    [ -n "$TARGET_NAME" ] || die "could not read TARGET from $PROJ_DIR/Makefile"
    EXE="$PROJ_DIR/$TARGET_NAME"
    # The fleet CLI is a native Windows Python under MSYS/Git Bash —
    # hand it a Windows-style path (bash's automatic conversion is
    # disabled when MSYS_NO_PATHCONV=1 is set for build.sh's docker run).
    if command -v cygpath >/dev/null 2>&1; then
        EXE="$(cygpath -m "$EXE")"
    fi
}

cmd_run() {
    PROJECT=""; TGT=""; RUN_ARGS=""
    while [ $# -gt 0 ]; do
        case "$1" in
            --target) TGT="$2"; shift 2 ;;
            --args) RUN_ARGS="$2"; shift 2 ;;
            -*) die "run: unknown flag $1" ;;
            *) PROJECT="$1"; shift ;;
        esac
    done
    _resolve_project "$PROJECT"
    "$REPO_ROOT/build.sh" "$PROJECT"
    set -- qemu_run --exe "$EXE" --start --stop
    [ -n "$TGT" ] && set -- "$@" --target "$TGT"
    [ -n "$RUN_ARGS" ] && set -- "$@" "--args=$RUN_ARGS"
    fleet_cli "$@"
}

cmd_test() {
    PROJECT=""; TGT=""; RUN_ARGS=""; WAIT=""; TIMEOUT="240"
    while [ $# -gt 0 ]; do
        case "$1" in
            --target) TGT="$2"; shift 2 ;;
            --args) RUN_ARGS="$2"; shift 2 ;;
            --wait) WAIT="$2"; shift 2 ;;
            --timeout) TIMEOUT="$2"; shift 2 ;;
            -*) die "test: unknown flag $1" ;;
            *) PROJECT="$1"; shift ;;
        esac
    done
    [ -n "$WAIT" ] || die "test: --wait REGEX is required (the serial marker that means success)."
    _resolve_project "$PROJECT"
    "$REPO_ROOT/build.sh" "$PROJECT"
    set -- qemu_run --exe "$EXE" --wait "$WAIT" --timeout "$TIMEOUT" --start --stop
    [ -n "$TGT" ] && set -- "$@" --target "$TGT"
    [ -n "$RUN_ARGS" ] && set -- "$@" "--args=$RUN_ARGS"
    fleet_cli "$@"
}

# ---------------------------------------------------------------------------
# setup — one-time setup
# ---------------------------------------------------------------------------

cmd_setup() {
    exec "$REPO_ROOT/setup.sh"
}

# ---------------------------------------------------------------------------
# Main dispatch
# ---------------------------------------------------------------------------

SUBCOMMAND="${1:-help}"
shift 2>/dev/null || true

case "$SUBCOMMAND" in
    new)    cmd_new "$@" ;;
    build)  cmd_build "$@" ;;
    clean)  cmd_clean "$@" ;;
    run)    cmd_run "$@" ;;
    test)   cmd_test "$@" ;;
    setup)  cmd_setup ;;
    help|-h|--help)  usage ;;
    *)      die "Unknown subcommand: $SUBCOMMAND. Run 'cargo-amiga.sh help' for usage." ;;
esac
