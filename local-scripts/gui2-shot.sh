#!/usr/bin/env bash
# gui2-shot.sh — capture the GUI-2 selection highlight under Xvfb +
# lavapipe, following docs/gui-shots/2026-08-27/README.md.
#
# LANE-LOCAL and not a repo fixture: it exists so the capture is one
# command with its PIDs recorded, and it is deleted before the PR lands.
set -euo pipefail

WORK="${HOME}/.local/share/cad-work/gui2-lane"
OUT="${1:?usage: gui2-shot.sh <output-dir>}"
DISP=":99"
mkdir -p "$WORK" "$OUT"

cleanup() {
  for f in "$WORK/viewer.pid" "$WORK/xvfb.pid"; do
    [ -f "$f" ] || continue
    pid=$(cat "$f")
    kill "$pid" 2>/dev/null || true
    rm -f "$f"
  done
}
trap cleanup EXIT

setsid Xvfb "$DISP" -screen 0 1280x800x24 >"$WORK/xvfb.log" 2>&1 &
echo $! >"$WORK/xvfb.pid"
sleep 2

DISPLAY="$DISP" setsid ./target/debug/viewer >"$WORK/viewer.log" 2>&1 &
echo $! >"$WORK/viewer.pid"
sleep 12

export DISPLAY="$DISP"
xdotool search --name viewer | tail -1 >"$WORK/win.txt" || true
import -window root "$OUT/05-startup.png"

# The window is 800x600 inside the 1280x800 root and the viewport pane is
# its left two thirds, so every cursor below is x < 520.
#
# The plate's hole wall, which the pane centre looks straight through.
xdotool mousemove 330 300
sleep 3
import -window root "$OUT/06-hover-highlight.png"

xdotool click 1
sleep 3
import -window root "$OUT/07-face-selected.png"

# A DIFFERENT face — the plate's top cap. Single-select: this must
# replace the hole wall, not join it.
xdotool mousemove 150 260
sleep 2
xdotool click 1
sleep 3
import -window root "$OUT/08-second-face-selected.png"

# Empty space inside the viewport: a click that hits nothing clears.
xdotool mousemove 90 540
sleep 2
xdotool click 1
sleep 3
import -window root "$OUT/09-cleared-by-empty-click.png"

echo "captured into $OUT"
