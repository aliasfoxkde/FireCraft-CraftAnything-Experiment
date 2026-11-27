#!/usr/bin/env bash
# Package an already-built universal macOS binary as an app bundle, ZIP and DMG.
#
# Usage: scripts/package-macos.sh <universal-firecraft-binary> <output-directory>
# Requires macOS tooling: lipo, Swift, iconutil, ditto, hdiutil, shasum, plutil.
# The output is unsigned and unnotarized. Gatekeeper-ready distribution requires the
# maintainer's Developer ID signing certificate and Apple notarization credentials.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$("$root/scripts/version.sh")"
binary_arg=${1:?usage: package-macos.sh <universal-firecraft-binary> <output-directory>}
output_arg=${2:?usage: package-macos.sh <universal-firecraft-binary> <output-directory>}

[[ "$(uname -s)" == Darwin ]] || {
  echo "package-macos.sh must run on macOS" >&2
  exit 1
}

for tool in lipo swift iconutil ditto hdiutil shasum plutil; do
  command -v "$tool" >/dev/null || { echo "required macOS tool not found: $tool" >&2; exit 1; }
done

[[ -f "$binary_arg" ]] || { echo "missing executable: $binary_arg" >&2; exit 1; }
[[ -x "$binary_arg" ]] || { echo "executable bit is not set: $binary_arg" >&2; exit 1; }
binary="$(cd "$(dirname "$binary_arg")" && pwd)/$(basename "$binary_arg")"
mkdir -p "$output_arg"
output="$(cd "$output_arg" && pwd)"

app="$output/FireCraft.app"
zip_path="$output/firecraft-v${version}-macos-universal.zip"
dmg_path="$output/firecraft-v${version}-macos-universal.dmg"
iconset="$output/.FireCraft.iconset"
stage="$(mktemp -d "${TMPDIR:-/tmp}/firecraft-macos-package.XXXXXX")"
cleanup() {
  hdiutil detach -quiet "$stage/mount" >/dev/null 2>&1 || true
  rm -rf "$iconset" "$stage"
}
trap cleanup EXIT

case "$version" in
  *[!A-Za-z0-9.+_-]*) echo "unsafe version string: $version" >&2; exit 1 ;;
esac
bundle_version="${version%%-*}"

# Require both architectures so the artifact name and support claim are accurate.
lipo "$binary" -verify_arch arm64 x86_64

# The caller owns output_arg; remove only this script's named products.
rm -rf "$app"
rm -f "$zip_path" "$dmg_path" "$output/SHA256SUMS.txt"
mkdir -p "$app/Contents/MacOS" "$app/Contents/Resources/docs/tutorials" \
  "$app/Contents/Resources/docs/features" "$iconset"
install -m 0755 "$binary" "$app/Contents/MacOS/FireCraft"

# Generate 8-bit RGBA PNG members for a standard .icns asset from the approved mark.
logo="$root/app/assets/logo.png"
[[ -f "$logo" ]] || { echo "missing application logo: $logo" >&2; exit 1; }
swift "$root/packaging/macos/make-iconset.swift" "$logo" "$iconset"
iconutil --convert icns --output "$app/Contents/Resources/FireCraft.icns" "$iconset"

cat > "$app/Contents/Info.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<plist version="1.0">
<dict>
  <key>CFBundleDevelopmentRegion</key><string>en</string>
  <key>CFBundleExecutable</key><string>FireCraft</string>
  <key>CFBundleIconFile</key><string>FireCraft</string>
  <key>CFBundleIdentifier</key><string>io.github.aliasfoxkde.FireCraft</string>
  <key>CFBundleInfoDictionaryVersion</key><string>6.0</string>
  <key>CFBundleName</key><string>FireCraft</string>
  <key>CFBundleDisplayName</key><string>FireCraft</string>
  <key>CFBundlePackageType</key><string>APPL</string>
  <key>CFBundleShortVersionString</key><string>${version}</string>
  <key>CFBundleVersion</key><string>${bundle_version}</string>
  <key>CFBundleSignature</key><string>????</string>
  <key>LSMinimumSystemVersion</key><string>11.0</string>
  <key>LSApplicationCategoryType</key><string>public.app-category.graphics-design</string>
  <key>NSHighResolutionCapable</key><true/>
  <key>NSPrincipalClass</key><string>NSApplication</string>
</dict>
</plist>
PLIST
plutil -lint "$app/Contents/Info.plist"
printf 'APPL????' > "$app/Contents/PkgInfo"

copy_required() {
  local source=$1 destination=$2
  [[ -f "$source" ]] || { echo "required bundle file missing: $source" >&2; exit 1; }
  cp "$source" "$destination"
}
resources="$app/Contents/Resources"
copy_required "$root/README.md" "$resources/README.md"
copy_required "$root/LICENSE" "$resources/LICENSE"
copy_required "$root/.github/CHANGELOG.md" "$resources/CHANGELOG.md"
copy_required "$root/docs/GLOSSARY.md" "$resources/GLOSSARY.md"
copy_required "$root/THIRD-PARTY-NOTICES.md" "$resources/THIRD-PARTY-NOTICES.md"
copy_required "$root/app/MECHANICS.md" "$resources/MECHANICS.md"
cat > "$resources/MACOS-DISTRIBUTION.txt" <<'NOTICE'
This macOS package is unsigned and has not been notarized by Apple.
Gatekeeper may prevent it from opening under the default security settings.
A signed and notarized package requires the maintainer's Developer ID credentials.

Before opening, verify the downloaded archive or disk image against SHA256SUMS.txt.
If you trust the verified download and macOS blocks the app, try opening it once,
then open System Settings > Privacy & Security and choose Open Anyway for FireCraft.
Review the warning and choose Open only if you still trust the app.

Apple's current instructions: https://support.apple.com/en-us/102445
NOTICE
copy_required "$root/docs/tutorials/README.md" "$resources/docs/tutorials/README.md"
cp -R "$root/docs/tutorials/." "$resources/docs/tutorials/"
cp -R "$root/docs/features/." "$resources/docs/features/"

source_sha="$(git -C "$root" rev-parse HEAD 2>/dev/null || echo unknown)"
if [[ -n "$(git -C "$root" status --porcelain 2>/dev/null || true)" ]]; then
  tree_state=dirty
else
  tree_state=clean
fi
{
  printf 'FireCraft %s\n' "$version"
  printf 'source: %s\n' "$source_sha"
  printf 'working-tree: %s\n' "$tree_state"
  printf 'architecture: arm64,x86_64 (universal)\n'
  printf 'minimum-macos: 11.0\n'
  printf 'signature: unsigned\n'
  printf 'notarization: none\n'
  printf 'built: %s\n' "$(date -u +%Y-%m-%dT%H:%M:%SZ)"
} > "$resources/BUILD-INFO.txt"

# Exercise the packaged executable, not only the pre-bundle input.
packaged="$app/Contents/MacOS/FireCraft"
"$packaged" --version | grep -F "firecraft $version " >/dev/null
command_count="$("$packaged" list-commands | wc -l | tr -d ' ')"
[[ "$command_count" -ge 60 ]] || {
  echo "packaged binary listed only $command_count commands" >&2
  exit 1
}
"$packaged" info --demo >/dev/null

ditto -c -k --sequesterRsrc --keepParent "$app" "$zip_path"

# Make a drag-install DMG with an Applications shortcut.
mkdir -p "$stage/FireCraft"
ditto "$app" "$stage/FireCraft/FireCraft.app"
ln -s /Applications "$stage/FireCraft/Applications"
hdiutil create -quiet -volname "FireCraft $version" -srcfolder "$stage/FireCraft" \
  -ov -format UDZO "$dmg_path"

# Verify both disk images contain the same runnable application bundle.
mountpoint="$stage/mount"
mkdir -p "$mountpoint"
hdiutil attach -quiet -readonly -nobrowse -mountpoint "$mountpoint" "$dmg_path"
mounted_app="$mountpoint/FireCraft.app/Contents/MacOS/FireCraft"
[[ -x "$mounted_app" ]] || { echo "DMG is missing the app executable" >&2; exit 1; }
"$mounted_app" --version | grep -F "firecraft $version " >/dev/null
hdiutil detach -quiet "$mountpoint"

(
  cd "$output"
  shasum -a 256 "FireCraft.app/Contents/MacOS/FireCraft" \
    "$(basename "$zip_path")" "$(basename "$dmg_path")" > SHA256SUMS.txt
)
echo "packaged macOS app: $app"
echo "packaged macOS ZIP: $zip_path"
echo "packaged macOS DMG: $dmg_path"
cat "$output/SHA256SUMS.txt"
