#!/usr/bin/env bash
# Documentation gate: every relative Markdown link must resolve, and version
# literals must not be hardcoded outside the single source (app/Cargo.toml).
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root"

status=0
# Documentation set: tracked Markdown, excluding generated bundles and audit evidence.
if ! tracked_markdown="$(git -c safe.directory="$root" ls-files '*.md')"; then
  echo "docs: unable to list tracked Markdown; run from a valid Git checkout" >&2
  exit 2
fi
if [ -z "$tracked_markdown" ]; then
  echo "docs: no tracked Markdown files found; refusing to pass an empty check" >&2
  exit 2
fi
mapfile -t docs < <(printf '%s\n' "$tracked_markdown" | grep -v -E '^(app/dist/|checks/|investigation\.cany/)' || true)
if [ "${#docs[@]}" -eq 0 ]; then
  echo "docs: no eligible Markdown files found; refusing to pass an empty check" >&2
  exit 2
fi

for f in "${docs[@]}"; do
  dir="$(dirname "$f")"
  # Print "line link" pairs for inline Markdown links, skipping http(s), mailto and anchors.
  while IFS=$'\t' read -r line target; do
    [ -z "${target:-}" ] && continue
    case "$target" in
      http://*|https://*|mailto:*|\#*) continue ;;
    esac
    target="${target%%#*}"
    [ -z "$target" ] && continue
    if [ ! -e "$dir/$target" ]; then
      echo "BROKEN LINK $f:$line -> $target"
      status=1
    fi
  done < <(grep -noE '\]\([^)]+\)' "$f" 2>/dev/null | sed -E 's/^([0-9]+):\]\((.*)\)$/\1\t\2/' || true)
done

# Trailing whitespace and duplicated headings inside a file make diffs and anchors unreliable.
for f in "${docs[@]}"; do
  # docs/INSTRUCTIONS.md is an external brief; its trailing spaces are preserved on purpose.
  if [ "$f" != "docs/INSTRUCTIONS.md" ] && grep -nE ' +$' "$f" >/dev/null 2>&1; then
    echo "TRAILING WHITESPACE $f"
    grep -nE ' +$' "$f" | head -3
    status=1
  fi
  # A changelog and a dated progress log repeat section names per entry, by design.
  if [ "$f" != ".github/CHANGELOG.md" ] && [ "$f" != "docs/PROGRESS.md" ]; then
    dup="$(grep -E '^#{1,6} ' "$f" | sort | uniq -d || true)"
    if [ -n "$dup" ]; then
      echo "DUPLICATE HEADING $f: $dup"
      status=1
    fi
  fi
done

# Version literals: releases and workflows must derive the version, never embed one.
if grep -rnE 'firecraft-v[0-9]+\.[0-9]+\.[0-9]+' .github .gitforge.yml scripts 2>/dev/null; then
  echo "HARDCODED VERSION: derive it from app/Cargo.toml via scripts/version.sh"
  status=1
fi

# MECHANICS.md claims must match mechanics.json.
for pair in "default canvas width 1920" "default canvas height 1080" "default export resolution 144"; do
  set -- $pair
  name="$1 $2 $3"; value="$4"
  if ! grep -q "\"value\": $value" app/mechanics.json; then
    echo "MECHANICS: mechanics.json no longer has '$name = $value'"
    status=1
  fi
done

if [ "$status" -eq 0 ]; then
  echo "docs: ${#docs[@]} files checked, links and claims OK"
fi
exit "$status"
