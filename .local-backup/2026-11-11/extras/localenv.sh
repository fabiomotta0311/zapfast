#!/usr/bin/env bash
# Local ZapFast compiler dependencies (CMake and Strawberry Perl) were removed.
# Build Windows variants with the persistent VPS builder instead:
#   cd C:/Users/fabio/dev/wt-zap-local-builds
#   bash scripts/build-vps-windows.sh <commit-or-ref> {test|custom}
# Windows Sandbox is used only for isolated UI smoke tests; it is disposable.
if [[ "${BASH_SOURCE[0]}" != "$0" ]]; then
  printf '%s\n' 'Local ZapFast build environment removed. Use scripts/build-vps-windows.sh from wt-zap-local-builds.' >&2
  return 2
fi
printf '%s\n' 'Local ZapFast build environment removed. Use scripts/build-vps-windows.sh from wt-zap-local-builds.' >&2
exit 2
