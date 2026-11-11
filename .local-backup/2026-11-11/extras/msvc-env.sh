#!/usr/bin/env bash
# Puts the MSVC x64 tools ahead of the Git-for-Windows coreutils, whose
# link.exe otherwise shadows the real linker and breaks every build script.
# Local helper for this machine only; nothing here is installed system-wide.
export VCTOOLS="C:/Program Files/Microsoft Visual Studio/2022/Preview/VC/Tools/MSVC/14.39.33321"
export PATH="$VCTOOLS/bin/Hostx64/x64:$PATH"
export INCLUDE="$VCTOOLS/include;${INCLUDE:-}"
export LIB="$VCTOOLS/lib/x64;${LIB:-}"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-C:/Users/fabio/zapfast/target}"
# Git for Windows injects its own usr/bin ahead of PATH, where coreutils
# ships a `link` that is not a linker. Rustc finds it first, so pin the real
# one by absolute path for every cargo invocation.
export CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER="$VCTOOLS/bin/Hostx64/x64/link.exe"
