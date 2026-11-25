#!/usr/bin/env bash
# Capture the FireCraft window under Xvfb for visual review.
#
# Usage: scripts/gui-shot.sh OUT.png [WIDTHxHEIGHT] [-- extra firecraft args...]
#   scripts/gui-shot.sh app/shots/home.png 1360x860 -- --demo
#   scripts/gui-shot.sh app/shots/small.png 960x640 -- --demo
#
# Uses the app's own `--screenshot` path, which renders N frames and exits, so the
# capture is deterministic and exactly the requested size (no window manager or
# window decorations). A throwaway XDG home stops eframe from restoring a persisted
# window size over the requested one.
#
# Requires: Xvfb. Software GL is forced for hosts without a GPU.
set -euo pipefail

out=${1:?usage: gui-shot.sh OUT.png [WIDTHxHEIGHT] [-- args...]}
size=${2:-1360x860}
shift 2 || true
if [[ ${1:-} == "--" ]]; then shift; fi

app_dir=$(cd "$(dirname "$0")/../app" && pwd)
mkdir -p "$(dirname "$out")"
out=$(cd "$(dirname "$out")" && pwd)/$(basename "$out")

display=:$((120 + RANDOM % 40))
screen_w=$(( ${size%x*} + 120 ))
screen_h=$(( ${size#*x} + 160 ))
Xvfb "$display" -screen 0 ${screen_w}x${screen_h}x24 -nolisten tcp >/tmp/firecraft-xvfb.log 2>&1 &
xvfb_pid=$!
home=$(mktemp -d)
cleanup() {
    kill "$xvfb_pid" 2>/dev/null || true
    rm -rf "$home"
}
trap cleanup EXIT

sleep 2
DISPLAY=$display \
    XDG_DATA_HOME="$home/data" \
    XDG_CONFIG_HOME="$home/config" \
    LIBGL_ALWAYS_SOFTWARE=1 \
    "$app_dir/target/debug/firecraft" --size "$size" --screenshot "$out" "$@" \
    >/tmp/firecraft-app.log 2>&1 || {
        echo "capture failed; see /tmp/firecraft-app.log" >&2
        tail -20 /tmp/firecraft-app.log >&2 || true
        exit 1
    }

if [[ ! -s $out ]]; then
    echo "no screenshot written; see /tmp/firecraft-app.log" >&2
    tail -20 /tmp/firecraft-app.log >&2
    exit 1
fi
echo "captured $out ($(identify -format '%wx%h' "$out"))"
