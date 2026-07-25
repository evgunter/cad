#!/usr/bin/env bash
# Render the demo-tour scenes (demos/out/) to PNGs (demos/renders/).
#
# Primary path (#91 C10): headless FreeCAD importing the tour's OWN
# STEP exports (STL mesh for bodies the analytic STEP subset refuses
# until M5) — every montage image dogfoods the F6 lane. Fallback:
# the numpy+matplotlib STL renderer (zero system deps, and the lane
# that draws OUR tessellation). The montage sheet is composed from
# the per-scene PNGs either way.
#
# The demo-local Python venv (numpy + matplotlib, pinned; both years
# past the repo's 2-week age policy) is created on first run. Prefers
# uv; falls back to python3.
set -euo pipefail
cd "$(dirname "$0")"

VENV=.venv
if [ ! -x "$VENV/bin/python" ]; then
    if command -v uv >/dev/null 2>&1; then
        uv venv --python 3.12 "$VENV"
        uv pip install --python "$VENV/bin/python" \
            'numpy==2.2.6' 'matplotlib==3.10.3'
    else
        python3 -m venv "$VENV"
        "$VENV/bin/pip" install 'numpy==2.2.6' 'matplotlib==3.10.3'
    fi
fi

FREECADCMD="${FREECADCMD:-$HOME/.local/share/cad-work/freecad/squashfs-root/usr/bin/freecadcmd}"
rm -f renders/.freecad_ok
if [ -x "$FREECADCMD" ]; then
    # freecadcmd's Qt teardown can crash AFTER a fully successful
    # render pass (offscreen destructor bug), so success is the
    # sentinel file, not the exit status.
    QT_QPA_PLATFORM=offscreen "$FREECADCMD" render_freecad.py out renders || true
    if [ ! -f renders/.freecad_ok ]; then
        echo "FreeCAD render did not complete — matplotlib fallback" >&2
        "$VENV/bin/python" render.py out renders
    fi
else
    echo "freecadcmd not found at $FREECADCMD — matplotlib fallback" >&2
    "$VENV/bin/python" render.py out renders
fi

exec "$VENV/bin/python" compose_montage.py out renders
