#!/usr/bin/env bash
# Render the demo-tour scenes (demos/out/) to PNGs — TWO montage lanes
# (#159: "our tessellation vs FreeCAD"):
#
#   ./render.sh            kernel lane -> renders/montage.png
#       Every cell shows the KERNEL'S OWN tessellation (the tour's STL
#       facets). Primary renderer: headless FreeCAD importing the STL
#       meshes (one warm session, #91 C10). Fallback: the
#       numpy+matplotlib STL renderer (zero system deps) — which
#       renders to the GITIGNORED renders-preview/renders/ tree and
#       never to renders/ (see "The fallback is uncommittable" below).
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
# THE FALLBACK IS UNCOMMITTABLE (#221). A matplotlib fallback frame
# once reached a committed montage cell silently, because the fallback
# wrote the same directory FreeCAD writes. Two layers now:
#   * ROUTING — the fallback renders (and composes its own sheet) into
#     renders-preview/<lane-dir>/, which .gitignore excludes. On
#     fallback this script touches NOTHING under renders/, and says so
#     loudly on stderr;
#   * GUARD — check_render_provenance.py runs over the committed lane
#     directory in both lanes, after the stamp strip and BEFORE the
#     montage is composed, so a sheet is never composed from a cell
#     set that is not certified FreeCAD-authored. `set -e` makes a
#     violation abort the run.
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
    # FreeCAD stamps the wall clock into every PNG it writes, so an
    # unchanged re-render still shows up dirty in `git status`. Drop
    # those two ancillary chunks (see strip_png_stamps.py) BEFORE the
    # montage step, so the sheet is composed from the same bytes that
    # get committed.
    "$VENV/bin/python" strip_png_stamps.py "$RD"
    # Provenance guard before the sheet is composed: every cell that
    # goes on it must carry FreeCAD's signature chunks.
    "$VENV/bin/python" check_render_provenance.py "$RD"
    exec "$VENV/bin/python" compose_montage.py out "$RD" \
        --montage=montage-freecad.png \
        '--banner=FreeCAD/OCC render — OCC re-tessellation of the kernel'\''s own STEP exports (compare renders/montage.png: the kernel'\''s facets)'
fi

# ---- kernel-tessellation lane (the original montage) ---------------
RD=renders
# The fallback's own tree, mirroring the lane structure one level down
# (renders-preview/renders/ here; the --freecad lane has no fallback —
# its whole point is the OCC reference render — so
# renders-preview/renders-freecad/ never appears). Gitignored, so a
# fallback frame cannot be committed even by `git add -A`.
PREVIEW="renders-preview/$RD"

# Matplotlib fallback: preview tree only, and LOUD about it. Ends the
# run (exec) — the committed sheet is not recomposed either, because
# recomposing it from a stale/partial cell set is exactly the silent
# corruption #221 hit.
fallback() {
    mkdir -p "$PREVIEW"
    cat >&2 <<EOF

================================================================
 MATPLOTLIB FALLBACK — this is NOT the committed render
   reason:  $1
   writing: demos/$PREVIEW/  (gitignored preview tree)
 demos/$RD/ is left untouched, montage.png included: a fallback
 frame must never be committable (#221). If FreeCAD ran and
 crashed mid-pass, demos/$RD/ may hold a PARTIAL FreeCAD pass —
 check \`git status\` before committing anything.
 To update the committed sheet, fix FreeCAD (FREECADCMD=...) and
 re-run.
================================================================

EOF
    "$VENV/bin/python" render.py out "$PREVIEW"
    exec "$VENV/bin/python" compose_montage.py out "$PREVIEW" \
        '--banner=PREVIEW ONLY — matplotlib fallback (FreeCAD unavailable); NOT the committed sheet in demos/renders/'
}

rm -f "$RD/.freecad_ok"
if [ -x "$FREECADCMD" ]; then
    # freecadcmd's Qt teardown can crash AFTER a fully successful
    # render pass (offscreen destructor bug), so success is the
    # sentinel file, not the exit status.
    QT_QPA_PLATFORM=offscreen "$FREECADCMD" render_freecad.py out "$RD" || true
    if [ ! -f "$RD/.freecad_ok" ]; then
        fallback "FreeCAD render did not complete (no $RD/.freecad_ok sentinel)"
    fi
else
    fallback "freecadcmd not found at $FREECADCMD"
fi

# Same wall-clock strip as the STEP lane.
"$VENV/bin/python" strip_png_stamps.py "$RD"
# Provenance guard before the sheet is composed (see the header).
"$VENV/bin/python" check_render_provenance.py "$RD"

# The kernel sheet carries its own provenance banner too, so the two
# sheets superimpose exactly — cell for cell AND banner for banner.
exec "$VENV/bin/python" compose_montage.py out "$RD" \
    '--banner=kernel render — the kernel'\''s own certified tessellation (compare renders-freecad/montage-freecad.png: OCC'\''s facets)'
