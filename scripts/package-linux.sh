#!/usr/bin/env bash
# Build the portable Linux tarball, Debian package, AppImage, and Flatpak bundle.
#
# Usage: scripts/package-linux.sh [path/to/firecraft] [output-directory]
# Defaults: app/target/release/firecraft and app/dist/linux
#
# Requires an x86_64 Linux host, dpkg-deb, Flatpak + flatpak-builder, curl, jq,
# file, ldd, tar, and a functional X11/OpenGL-compatible build environment.
# Flatpak builds download the Freedesktop 24.08 runtime/SDK from Flathub.
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
version="$("$root/scripts/version.sh")"
exe_arg="${1:-$root/app/target/release/firecraft}"
out_arg="${2:-$root/app/dist/linux}"

if [[ ! -f "$exe_arg" ]]; then
  echo "missing executable: $exe_arg" >&2
  exit 1
fi
exe="$(cd "$(dirname "$exe_arg")" && pwd)/$(basename "$exe_arg")"
mkdir -p "$out_arg"
out="$(cd "$out_arg" && pwd)"

for tool in dpkg-deb flatpak flatpak-builder curl jq file ldd tar gzip sha256sum install rg; do
  command -v "$tool" >/dev/null || {
    echo "missing required Linux packaging tool: $tool" >&2
    exit 1
  }
done

[[ "$(uname -m)" == "x86_64" ]] || {
  echo "Linux package script currently supports x86_64 only (host: $(uname -m))" >&2
  exit 1
}

file "$exe" | rg -q 'ELF 64-bit.*x86-64' || {
  echo "expected an x86_64 ELF executable: $exe" >&2
  exit 1
}
if ldd "$exe" | rg -q 'not found'; then
  echo "executable has unresolved dynamic libraries:" >&2
  ldd "$exe" >&2
  exit 1
fi
"$exe" --version | rg -q "^firecraft ${version} " || {
  echo "executable version does not match app/Cargo.toml (${version})" >&2
  exit 1
}
for required in LICENSE README.md CHANGELOG.md GLOSSARY.md THIRD-PARTY-NOTICES.md \
  app/MECHANICS.md app/assets/logo.png packaging/linux/io.github.aliasfoxkde.FireCraft.desktop \
  packaging/linux/io.github.aliasfoxkde.FireCraft.metainfo.xml \
  packaging/linux/io.github.aliasfoxkde.FireCraft.yml; do
  [[ -f "$root/$required" ]] || {
    echo "missing required package input: $required" >&2
    exit 1
  }
done

# These SHA-256 pins are fetched from the corresponding upstream release assets.
# AppImageKit's continuous release does not publish an asset digest, so its digest
# was independently calculated from the downloaded binary on 2026-11-26.
linuxdeploy_sha256="df908b19f1ed1cdafa79508d415aa12d6c56ce4d56d4149aa8c09286718706e5"
appimagetool_sha256="b90f4a8b18967545fda78a445b27680a1642f1ef9488ced28b65398f2be7add2"

tmp="$(mktemp -d "${TMPDIR:-/tmp}/firecraft-linux-package.XXXXXX")"
cleanup() {
  rm -rf "$tmp"
}
trap cleanup EXIT

name="firecraft-v${version}-linux-x86_64"
tar_name="${name}.tar.gz"
appimage_name="${name}.AppImage"
deb_name="firecraft_${version}_amd64.deb"
flatpak_name="${name}.flatpak"
app_id="io.github.aliasfoxkde.FireCraft"
date_utc="$(date -u +%Y-%m-%d)"

portable="$tmp/$name"
mkdir -p "$portable/share/applications" "$portable/share/icons/hicolor/512x512/apps" \
  "$portable/share/metainfo" "$portable/docs/tutorials" "$portable/docs/features"
install -Dm755 "$exe" "$portable/firecraft"
for file_name in LICENSE README.md CHANGELOG.md GLOSSARY.md THIRD-PARTY-NOTICES.md; do
  install -Dm644 "$root/$file_name" "$portable/$file_name"
done
install -Dm644 "$root/app/MECHANICS.md" "$portable/MECHANICS.md"
cp -R "$root/docs/tutorials/." "$portable/docs/tutorials/"
cp -R "$root/docs/features/." "$portable/docs/features/"
install -Dm644 "$root/packaging/linux/$app_id.desktop" \
  "$portable/share/applications/$app_id.desktop"
install -Dm644 "$root/app/assets/logo.png" \
  "$portable/share/icons/hicolor/512x512/apps/$app_id.png"
install -Dm644 "$root/packaging/linux/$app_id.metainfo.xml" \
  "$portable/share/metainfo/$app_id.metainfo.xml"
cat > "$portable/BUILD-INFO.txt" <<EOF
FireCraft $version
Target: x86_64-unknown-linux-gnu
Build date (UTC): $date_utc
Source revision: $(git -C "$root" rev-parse HEAD 2>/dev/null || echo unavailable)
EOF

tar --sort=name --mtime='UTC 1970-01-01' --owner=0 --group=0 --numeric-owner \
  -cf "$tmp/$name.tar" -C "$tmp" "$name"
gzip --no-name --best --stdout "$tmp/$name.tar" > "$tmp/$tar_name"
mkdir -p "$tmp/tar-smoke"
tar -xzf "$tmp/$tar_name" -C "$tmp/tar-smoke"
"$tmp/tar-smoke/$name/firecraft" --version >/dev/null

# Build a small Debian package. This executable currently links only libc and
# libgcc_s dynamically (checked with ldd); the graphics stack is loaded from the
# host at runtime so users keep their system OpenGL drivers.
deb_root="$tmp/deb-root"
mkdir -p "$deb_root/DEBIAN" "$deb_root/usr/bin" "$deb_root/usr/share/applications" \
  "$deb_root/usr/share/icons/hicolor/512x512/apps" "$deb_root/usr/share/metainfo" \
  "$deb_root/usr/share/doc/firecraft"
install -Dm755 "$exe" "$deb_root/usr/bin/firecraft"
install -Dm644 "$root/packaging/linux/$app_id.desktop" \
  "$deb_root/usr/share/applications/$app_id.desktop"
install -Dm644 "$root/app/assets/logo.png" \
  "$deb_root/usr/share/icons/hicolor/512x512/apps/$app_id.png"
install -Dm644 "$root/packaging/linux/$app_id.metainfo.xml" \
  "$deb_root/usr/share/metainfo/$app_id.metainfo.xml"
for file_name in LICENSE README.md CHANGELOG.md THIRD-PARTY-NOTICES.md; do
  install -Dm644 "$root/$file_name" "$deb_root/usr/share/doc/firecraft/$file_name"
done
cat > "$deb_root/DEBIAN/control" <<EOF
Package: firecraft
Version: $version
Section: graphics
Priority: optional
Architecture: amd64
Depends: libc6 (>= 2.35), libgcc-s1
Maintainer: FireCraft contributors
Homepage: https://github.com/aliasfoxkde/FireCraft-CraftAnything-Experiment
Description: Open Source Raster and Vector Graphics Editor
 A clean-room, local-first editor for raster and vector graphics, web design,
 pages, states, layers, effects, and export workflows.
EOF
dpkg-deb --build --root-owner-group "$deb_root" "$tmp/$deb_name" >/dev/null
dpkg-deb --info "$tmp/$deb_name" >/dev/null
mkdir -p "$tmp/deb-smoke"
dpkg-deb --extract "$tmp/$deb_name" "$tmp/deb-smoke"
"$tmp/deb-smoke/usr/bin/firecraft" --version >/dev/null

# Package an AppImage. Both downloaded tools are checksum-verified; no mutable
# `latest` URL is trusted without matching the pinned digest.
tools_dir="$tmp/tools"
mkdir -p "$tools_dir"
linuxdeploy="$tools_dir/linuxdeploy-x86_64.AppImage"
appimagetool="$tools_dir/appimagetool-x86_64.AppImage"
curl --fail --location --retry 3 \
  https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage \
  --output "$linuxdeploy"
curl --fail --location --retry 3 \
  https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage \
  --output "$appimagetool"
printf '%s  %s\n' "$linuxdeploy_sha256" "$linuxdeploy" | sha256sum --check --status || {
  echo "linuxdeploy SHA-256 mismatch; update the pinned packaging tool only after review" >&2
  exit 1
}
printf '%s  %s\n' "$appimagetool_sha256" "$appimagetool" | sha256sum --check --status || {
  echo "appimagetool SHA-256 mismatch; update the pinned packaging tool only after review" >&2
  exit 1
}
chmod +x "$linuxdeploy" "$appimagetool"
appdir="$tmp/AppDir"
mkdir -p "$appdir/usr/share/applications" "$appdir/usr/share/icons/hicolor/512x512/apps" \
  "$appdir/usr/share/metainfo" "$appdir/usr/share/licenses/firecraft" \
  "$appdir/usr/share/doc/firecraft"
install -Dm644 "$root/packaging/linux/$app_id.desktop" \
  "$appdir/usr/share/applications/$app_id.desktop"
install -Dm644 "$root/packaging/linux/$app_id.metainfo.xml" \
  "$appdir/usr/share/metainfo/$app_id.metainfo.xml"
install -Dm644 "$root/app/assets/logo.png" \
  "$appdir/usr/share/icons/hicolor/512x512/apps/$app_id.png"
install -Dm644 "$root/LICENSE" "$appdir/usr/share/licenses/firecraft/LICENSE"
install -Dm644 "$root/THIRD-PARTY-NOTICES.md" \
  "$appdir/usr/share/doc/firecraft/THIRD-PARTY-NOTICES.md"
APPIMAGE_EXTRACT_AND_RUN=1 "$linuxdeploy" --appdir "$appdir" --executable "$exe" \
  --desktop-file "$root/packaging/linux/$app_id.desktop" \
  --icon-file "$root/app/assets/logo.png" --icon-filename "$app_id"
ARCH=x86_64 APPIMAGE_EXTRACT_AND_RUN=1 "$appimagetool" "$appdir" "$tmp/$appimage_name"
APPIMAGE_EXTRACT_AND_RUN=1 "$tmp/$appimage_name" --version >/dev/null

# Build and export the Flatpak from the same release executable. The sandbox
# grants home-directory access because FireCraft's current file browser accesses
# local paths directly; remove this permission when portal-backed file access lands.
flatpak_user="$tmp/flatpak-user"
flatpak_smoke_user="$tmp/flatpak-smoke-user"
flatpak_repo="$tmp/flatpak-repo"
flatpak_build="$tmp/flatpak-build"
flatpak_stage="$tmp/flatpak-source"
mkdir -p "$flatpak_stage"
install -Dm755 "$exe" "$flatpak_stage/firecraft"
install -Dm644 "$root/packaging/linux/$app_id.desktop" "$flatpak_stage/$app_id.desktop"
install -Dm644 "$root/packaging/linux/$app_id.metainfo.xml" "$flatpak_stage/$app_id.metainfo.xml"
install -Dm644 "$root/app/assets/logo.png" "$flatpak_stage/logo.png"
install -Dm644 "$root/LICENSE" "$flatpak_stage/LICENSE"
install -Dm644 "$root/THIRD-PARTY-NOTICES.md" "$flatpak_stage/THIRD-PARTY-NOTICES.md"
flatpak_manifest="$flatpak_stage/$app_id.yml"
cp "$root/packaging/linux/$app_id.yml" "$flatpak_manifest"
flatpak_remote_file="$tmp/flathub.flatpakrepo"
curl --fail --location --retry 3 https://dl.flathub.org/repo/flathub.flatpakrepo \
  --output "$flatpak_remote_file"
FLATPAK_USER_DIR="$flatpak_user" flatpak remote-add --user --if-not-exists flathub \
  "$flatpak_remote_file"
(
  cd "$flatpak_stage"
  FLATPAK_USER_DIR="$flatpak_user" flatpak-builder --user --force-clean \
    --install-deps-from=flathub --repo="$flatpak_repo" "$flatpak_build" "$flatpak_manifest"
)
FLATPAK_USER_DIR="$flatpak_user" flatpak build-bundle "$flatpak_repo" \
  "$tmp/$flatpak_name" "$app_id" stable \
  --runtime-repo=https://dl.flathub.org/repo/flathub.flatpakrepo
[[ -s "$tmp/$flatpak_name" ]] || {
  echo "Flatpak bundle is missing or empty" >&2
  exit 1
}
FLATPAK_USER_DIR="$flatpak_smoke_user" flatpak install --user --noninteractive \
  --bundle "$tmp/$flatpak_name"
FLATPAK_USER_DIR="$flatpak_smoke_user" flatpak run --user "$app_id" --version \
  | rg -q "^firecraft ${version} "
FLATPAK_USER_DIR="$flatpak_smoke_user" flatpak uninstall --user --noninteractive "$app_id"

# Create evidence sidecars from the final package bytes, then promote only after
# every package has passed its applicable smoke check.
(
  cd "$tmp"
  sha256sum "$tar_name" "$appimage_name" "$deb_name" "$flatpak_name" > SHA256SUMS.txt
)
source_revision="$(git -C "$root" rev-parse HEAD 2>/dev/null || echo unavailable)"
working_tree_clean=false
if [[ -n "$(git -C "$root" status --porcelain 2>/dev/null)" ]]; then
  working_tree_clean=false
else
  working_tree_clean=true
fi
jq -n \
  --arg name FireCraft \
  --arg version "$version" \
  --arg target x86_64-unknown-linux-gnu \
  --arg source_revision "$source_revision" \
  --arg build_date_utc "$(date -u +%Y-%m-%dT%H:%M:%SZ)" \
  --argjson working_tree_clean "$working_tree_clean" \
  --slurpfile sums <(awk '{print $1, $2}' "$tmp/SHA256SUMS.txt" | \
    jq -Rn '[inputs | split(" ") | {sha256:.[0], filename:.[1]}]') \
  '{name:$name, version:$version, target:$target, source_revision:$source_revision,
    working_tree_clean:$working_tree_clean, build_date_utc:$build_date_utc,
    artifacts:$sums[0]}' > "$tmp/PACKAGE-MANIFEST.json"

for filename in "$tar_name" "$appimage_name" "$deb_name" "$flatpak_name" SHA256SUMS.txt PACKAGE-MANIFEST.json; do
  mv -f "$tmp/$filename" "$out/$filename"
done
echo "Linux packages written to $out"
cat "$out/SHA256SUMS.txt"
