#!/usr/bin/env bash
set -euo pipefail

revision="${1:-HEAD}"
variant="${2:-custom}"
case "$variant" in
    test|custom) ;;
    *) printf 'Usage: %s [commit-or-ref] {test|custom}\n' "$0" >&2; exit 2 ;;
esac

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"
if [[ -n "$(git status --porcelain --untracked-files=all)" ]]; then
    printf 'Refusing to build a dirty tree. Commit the source first; only that exact commit is transferred.\n' >&2
    exit 2
fi
sha="$(git rev-parse --verify "${revision}^{commit}")"
remote_root="/docker/zapfast-builder"
remote_source="$remote_root/workspaces/$sha"
name="zapfast_${variant}"
local_artifacts="$repo_root/target/vps-artifacts/$sha"
mkdir -p "$local_artifacts"

printf 'Sending ZapFast %s source at %s to vps-hermes\n' "$variant" "$sha"
git archive --format=tar "$sha" | ssh -o BatchMode=yes vps-hermes \
    "mkdir -p '$remote_source' && tar -xf - -C '$remote_source'"

ssh -o BatchMode=yes vps-hermes \
    "ROOT='$remote_root' SOURCE='$remote_source' SHA='$sha' NAME='$name' bash -s" <<'REMOTE'
set -euo pipefail
exec 9>"$ROOT/build.lock"
flock 9
export CARGO_HOME="$ROOT/cargo"
export RUSTUP_HOME="$ROOT/rustup"
export PATH="$ROOT/bin:$ROOT/cargo/bin:/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin"
export CARGO_TARGET_DIR="$ROOT/target"
export CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 CARGO_PROFILE_RELEASE_DEBUG=0
export CARGO_PROFILE_RELEASE_LTO=false CARGO_PROFILE_RELEASE_CODEGEN_UNITS=4
cd "$SOURCE"
artifact_dir="$ROOT/artifacts/$SHA"
test_artifact="$artifact_dir/zapfast_test.exe"
custom_artifact="$artifact_dir/zapfast_custom.exe"
mkdir -p "$artifact_dir"

# Flavor is selected at runtime from the executable filename, so one build
# per source commit can safely serve both isolated executable names.
if [[ ! -f "$test_artifact" && ! -f "$custom_artifact" ]]; then
    cross build --locked --release --target x86_64-pc-windows-gnu --bin zapfast --jobs 1
    cp "$CARGO_TARGET_DIR/x86_64-pc-windows-gnu/release/zapfast.exe" "$test_artifact"
fi
if [[ ! -f "$test_artifact" ]]; then cp "$custom_artifact" "$test_artifact"; fi
if [[ ! -f "$custom_artifact" ]]; then cp "$test_artifact" "$custom_artifact"; fi
if ! cmp -s "$test_artifact" "$custom_artifact"; then
    printf 'Variant artifacts differ for the same source commit.\n' >&2
    exit 1
fi
file "$artifact_dir/$NAME.exe"
sha256sum "$artifact_dir/$NAME.exe"
REMOTE

scp -q "vps-hermes:$remote_root/artifacts/$sha/$name.exe" "$local_artifacts/$name.exe"
printf 'Local artifact: %s\n' "$local_artifacts/$name.exe"
sha256sum "$local_artifacts/$name.exe"
