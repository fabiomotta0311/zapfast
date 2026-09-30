#!/usr/bin/env bash
set -euo pipefail

variant="${1:-custom}"
case "$variant" in
    test|custom) ;;
    *) printf 'Usage: %s {test|custom}\n' "$0" >&2; exit 2 ;;
esac

root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
branch="$(git branch --show-current)"
commit="$(git rev-parse --short=12 HEAD)"
target_dir="${CARGO_TARGET_DIR:-$root/target}"
cargo build --locked --release --bin zapfast

suffix=""
if [[ "${OS:-}" == "Windows_NT" || "${MSYSTEM:-}" != "" ]]; then
    suffix=".exe"
fi
name="zapfast_${variant}${suffix}"
mkdir -p build/local-variants
cp "$target_dir/release/zapfast${suffix}" "build/local-variants/$name"
printf 'Built %s from %s at %s\n' "$name" "$branch" "$commit"
printf 'Output: %s/build/local-variants/%s\n' "$root" "$name"
