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

  setup
      Run first-time setup (delegates to setup.sh).

  help
      Show this message.

Examples:

  ./cargo-amiga.sh new my-cool-app
  ./cargo-amiga.sh new ram-handler --mode driver
  ./cargo-amiga.sh build my-cool-app
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
        # App template placeholders: name = "myapp", lib name = "myapp", libmyapp.a
        sed -i "s/name = \"myapp\"/name = \"$BIN_NAME\"/" "$TARGET_DIR/Cargo.toml"
        # lib name uses underscores (second occurrence)
        # The template has: name = "myapp" twice — package name and lib name
        # After the first sed, the package name is replaced. The lib name is still "myapp".
        # Actually both were "myapp" and now both are replaced. We need lib name with underscores.
        # Let's do a targeted replacement:
        #   [lib]\nname = "<bin_name>" -> [lib]\nname = "<lib_name>"
        # Use a simpler approach: replace in the [lib] section specifically
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
    setup)  cmd_setup ;;
    help|-h|--help)  usage ;;
    *)      die "Unknown subcommand: $SUBCOMMAND. Run 'cargo-amiga.sh help' for usage." ;;
esac
