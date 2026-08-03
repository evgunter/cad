#!/usr/bin/env bash
# Render the demo-tour scenes (demos/out/) to PNGs — TWO montage lanes
# (#159: "our tessellation vs FreeCAD"):
#
#   ./render.sh            kernel lane -> renders/montage.png
#       Every cell shows the KERNEL'S OWN tessellation (the tour's STL
#       facets). Primary renderer: headless FreeCAD importing the STL
#       meshes (one warm session, #91 C10). Fallback: the
#       numpy+matplotlib STL renderer (zero system deps).
#   ./render.sh --freecad  FreeCAD/OCC lane -> renders-freecad/montage-freecad.png
#       Every cell is FreeCAD importing the body's OWN STEP export and
#       letting OCC re-tessellate — the reference rendering the kernel's
#       montage can be compared against, cell for cell (same scenes.json
#       cameras/captions/grid). One freecadcmd process PER SCENE with a
#       timeout (bulk imports have stalled before); a scene that fails
#       gets a labeled placeholder cell, never a silent gap or a stall
#       that kills the whole montage. FREECAD_SCENE_TIMEOUT (seconds,
#       default 300) tunes the per-scene budget.
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

if [ "${1:-}" = "--freecad" ]; then
    # ---- FreeCAD/OCC STEP lane ------------------------------------
    if [ ! -x "$FREECADCMD" ]; then
        echo "freecadcmd not found at $FREECADCMD — the --freecad lane" >&2
        echo "has no fallback (its whole point is the OCC reference render)" >&2
        exit 1
    fi
    TIMEOUT="${FREECAD_SCENE_TIMEOUT:-300}"
    RD=renders-freecad
    LOGDIR=out/freecad-logs
    mkdir -p "$RD" "$LOGDIR"
    SCENES=$("$VENV/bin/python" -c 'import json
for s in json.load(open("out/scenes.json")):
    if s.get("montage", True):
        print(s["name"])')
    fails=0
    for name in $SCENES; do
        rm -f "$RD/$name.png" "$RD/$name.fail.txt"
        log="$LOGDIR/$name.log"
        set +e
        # NB: bare keywords, not --flags — freecadcmd's option parser
        # rejects unknown dashed tokens (and --pass drops them too).
        QT_QPA_PLATFORM=offscreen timeout "$TIMEOUT" \
            "$FREECADCMD" render_freecad.py step "scene=$name" out "$RD" \
            >"$log" 2>&1
        rc=$?
        set -e
        # Success is the PNG existing, not the exit status (freecadcmd's
        # Qt teardown can crash after a fully successful render).
        if [ -f "$RD/$name.png" ]; then
            echo "  [$name] rendered (STEP -> OCC)"
        else
            if [ "$rc" -eq 124 ]; then
                reason="freecadcmd timed out after ${TIMEOUT}s (STEP import/render stall)"
            else
                reason="freecadcmd rc=$rc: $(tail -n 2 "$log" | tr '\n' ' ' | cut -c1-160)"
            fi
            printf '%s\n' "$reason" >"$RD/$name.fail.txt"
            echo "  [$name] FAILED — $reason (placeholder cell; log: $log)" >&2
            fails=$((fails + 1))
        fi
    done
    if [ "$fails" -gt 0 ]; then
        echo "$fails scene(s) fell back to placeholder cells" >&2
    fi
    exec "$VENV/bin/python" compose_montage.py out "$RD" \
        --montage=montage-freecad.png \
        '--banner=FreeCAD/OCC render — OCC re-tessellation of the kernel'\''s own STEP exports (compare renders/montage.png: the kernel'\''s facets)'
fi

# ---- kernel-tessellation lane (the original montage) ---------------
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

# The kernel sheet carries its own provenance banner too, so the two
# sheets superimpose exactly — cell for cell AND banner for banner.
exec "$VENV/bin/python" compose_montage.py out renders \
    '--banner=kernel render — the kernel'\''s own certified tessellation (compare renders-freecad/montage-freecad.png: OCC'\''s re-tessellation of the same bodies)'
