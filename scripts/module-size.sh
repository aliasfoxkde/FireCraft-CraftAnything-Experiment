#!/usr/bin/env bash
# Module size budget: warn above 1,200 lines, fail above 2,000 unless allowlisted.
#
# The allowlist records files that predate the budget and the seam that will split them.
# Removing an entry is progress; adding one needs a reason in docs/PROGRESS.md.
set -euo pipefail
root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
allow="$root/module-size-allow.txt"
warn=1200
fail=2000

is_allowed() {
    [ -f "$allow" ] && grep -qxF "$1" "$allow"
}

status=0
while IFS= read -r line; do
    lines=${line%% *}
    file=${line#* }
    rel=${file#"$root"/}
    if [ "$lines" -gt "$fail" ]; then
        if is_allowed "$rel"; then
            echo "allowlisted  $rel ($lines)"
        else
            echo "OVER BUDGET  $rel ($lines > $fail)"
            status=1
        fi
    elif [ "$lines" -gt "$warn" ]; then
        echo "warn         $rel ($lines)"
    fi
done < <(find "$root/app/src" -name '*.rs' -print0 | xargs -0 wc -l | grep -v ' total$' | sed 's/^ *//')

if [ "$status" -eq 0 ]; then
    echo "module-size: no file above the hard budget"
fi
exit "$status"
