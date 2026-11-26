#!/usr/bin/env bash
# Capture the standard FireCraft state set in one invocation (task 16.8).
#
# Usage: scripts/gui-suite.sh OUTDIR [SIZE]
#   scripts/gui-suite.sh checks/gui-suite-2026-11-26 1360x860
#
# Each state is captured through the app's own --screenshot path under Xvfb
# (deterministic, exact size, no window manager). `checks/` keeps the output
# as evidence; this script is the only thing that needs to be re-run.
set -euo pipefail

out_dir=${1:?usage: gui-suite.sh OUTDIR [SIZE]}
size=${2:-1360x860}
script_dir=$(cd "$(dirname "$0")" && pwd)
mkdir -p "$out_dir"

shot() {
    local name=$1; shift
    "$script_dir/gui-shot.sh" "$out_dir/$name.png" "$size" -- "$@"
}

shot home --demo
shot splash --demo --splash
shot state-menu --demo --state menu
shot state-flyout --demo --state flyout
shot state-find-replace --demo --state find-replace
shot about --demo --about

echo "suite complete: $out_dir"
