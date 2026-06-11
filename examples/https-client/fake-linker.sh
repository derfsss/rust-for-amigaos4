#!/bin/sh
# Fake linker — stub that creates an empty output file.
# Used during `cargo build` on the host so rustc thinks linking succeeded.
# The real linking is done by ppc-amigaos-gcc inside Docker.
#
# Works on Linux, macOS, and WSL (Windows uses fake-linker.bat instead).
while [ "$#" -gt 0 ]; do
    if [ "$1" = "-o" ]; then touch "$2"; break; fi
    shift
done
exit 0
