#!/usr/bin/env bash
# Coverage gate with a ratchet floor. The floor only moves up.
#
# Usage: scripts/coverage.sh [--update]
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$root/app"
floor_file="$root/coverage-floor.txt"
floor="$(cat "$floor_file" 2>/dev/null || echo 18.70)"

out="$(cargo llvm-cov --summary-only -- --test-threads=1 2>&1)"
echo "$out" | tail -20

lines="$(echo "$out" | awk '/^ *TOTAL / { gsub("%", "", $10); print $10; exit }')"
if [ -z "$lines" ]; then
  echo "coverage: could not parse the line total" >&2
  exit 1
fi

echo "coverage: $lines% lines (floor $floor%)"
if [ "${1:-}" = "--update" ]; then
  printf '%s\n' "$lines" > "$floor_file"
  echo "coverage: floor updated to $lines%"
  exit 0
fi

awk -v have="$lines" -v need="$floor" 'BEGIN { exit !(have + 0 >= need + 0) }' || {
  echo "coverage: $lines% is below the $floor% floor" >&2
  exit 1
}
