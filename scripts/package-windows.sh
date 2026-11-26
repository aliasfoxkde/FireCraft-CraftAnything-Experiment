#!/usr/bin/env bash
# Assemble the portable Windows bundle from a built executable.
#
# Usage: scripts/package-windows.sh [path/to/firecraft.exe]
# Default exe: app/target/x86_64-pc-windows-gnu/release/firecraft.exe
#
# Output: app/dist/windows/ (bundle) and app/dist/firecraft-v<version>-windows-x86_64.zip
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$("$root/scripts/version.sh")"
exe="${1:-$root/app/target/x86_64-pc-windows-gnu/release/firecraft.exe}"
out="$root/app/dist/windows"
zip="$root/app/dist/firecraft-v${version}-windows-x86_64.zip"

[ -f "$exe" ] || { echo "missing executable: $exe" >&2; exit 1; }

rm -rf "$out"
mkdir -p "$out/docs"
cp "$exe" "$out/firecraft.exe"
cp "$root/README.md" "$out/README.md"
cp "$root/LICENSE" "$out/LICENSE"
cp "$root/CHANGELOG.md" "$out/CHANGELOG.md"
cp "$root/GLOSSARY.md" "$out/GLOSSARY.md"
cp "$root/THIRD-PARTY-NOTICES.md" "$out/THIRD-PARTY-NOTICES.md"
cp "$root/app/MECHANICS.md" "$out/MECHANICS.md"
cp -R "$root/docs/." "$out/docs/"
# Internal planning documents are not part of the user-facing bundle.
rm -f "$out/docs/AUDIT.md" "$out/docs/PLAN.md" "$out/docs/TASKS.md" "$out/docs/PROGRESS.md"

# Stamp the bundle with the version it was built from.
printf '%s\n' "$version" > "$out/VERSION"
{
  echo "FireCraft $version"
  echo "built: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "source: $(git -C "$root" rev-parse HEAD 2>/dev/null || echo unknown)"
} > "$out/BUILD-INFO.txt"

( cd "$out" && sha256sum firecraft.exe > firecraft.exe.sha256 )
rm -f "$zip"
if command -v zip >/dev/null 2>&1; then
  ( cd "$out" && zip -qr "$zip" . )
elif command -v powershell.exe >/dev/null 2>&1 && command -v cygpath >/dev/null 2>&1; then
  # GitHub's Windows runner includes PowerShell but not the Unix `zip` utility.
  # shellcheck disable=SC2016
  FC_ZIP_SOURCE="$(cygpath -w "$out")" FC_ZIP_DESTINATION="$(cygpath -w "$zip")" powershell.exe \
    -NoProfile -NonInteractive -Command \
    '$ErrorActionPreference = "Stop"; Compress-Archive -Path (Join-Path -Path $env:FC_ZIP_SOURCE -ChildPath "*") -DestinationPath $env:FC_ZIP_DESTINATION -Force'
else
  echo "archive creation requires zip, or PowerShell with cygpath on Windows" >&2
  exit 1
fi
( cd "$root/app/dist" && sha256sum "$(basename "$zip")" > "$(basename "$zip").sha256" )
echo "packaged $zip"
cat "$root/app/dist/$(basename "$zip").sha256"
