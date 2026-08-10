#!/usr/bin/env bash
# Render the demo-tour scenes (demos/out/) to PNGs — TWO montage lanes
# (#159: "our tessellation vs FreeCAD"):
#
#   ./render.sh            kernel lane -> renders/montage.png
#       Every cell shows the KERNEL'S OWN tessellation (the tour's STL
#       facets), drawn by headless FreeCAD importing the STL meshes.
#       Fallback: the numpy+matplotlib STL renderer (zero system deps)
#       — which renders to the GITIGNORED renders-preview/renders/ tree
#       and never to renders/ (see "The fallback is uncommittable").
#   ./render.sh --freecad  FreeCAD/OCC lane -> renders-freecad/montage-freecad.png
#       Every cell is FreeCAD importing the body's OWN STEP export and
#       letting OCC re-tessellate — the reference rendering the kernel's
#       montage can be compared against, cell for cell (same scenes.json
#       cameras/captions/grid).
#
# ONE FREECADCMD PROCESS PER SCENE, IN BOTH LANES (#224 follow-up). A
# warm FreeCAD session that renders many scenes deadlocks partway
# through — reproducibly on this host, at a different scene each time,
# on an idle box as well as a loaded one (freecadcmd at 0% CPU,
# `wchan = futex_do_wait`): it is the session that wedges, not any one
# scene. So no session is reused across scenes, and every scene runs
# under a per-scene wall-clock budget (see SCENE_TIMEOUT) with the
# process tree killed and ONE fresh retry when the budget is exhausted.
# A budget exhausted TWICE fails the pass, loudly, naming the scene and
# the budget — never a silent skip, never a degraded cell.
#
# NOTHING IS WRITTEN TO THE COMMITTED LANE DIRECTORY UNTIL THE PASS
# SUCCEEDS. Every scene renders into an untracked staging tree
# (out/stage-<lane>/); only a completed pass has its frames moved into
# renders/ or renders-freecad/. So any failure — wedge, crash, missing
# FreeCAD — leaves the committed tree byte-for-byte as it was, and a
# half-finished pass can never be mistaken for a whole one.
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

# Hosted is the default renderer; this refuses without the explicit
# preview-only override. See demos/hosted-render-guard.sh.
# shellcheck source=demos/hosted-render-guard.sh
. ./hosted-render-guard.sh
require_hosted_render "demos/render.sh"

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

# ---- per-scene budget ----------------------------------------------
# Wall-clock budget for ONE scene in ONE fresh freecadcmd process,
# FreeCAD's startup included. What it must clear is not the typical
# scene but the worst LEGITIMATE one, and on this workload that is set
# by machine contention, not by the scene: measured over full passes of
# both lanes (2026-08, 8-core box), a scene takes 3-19 s — median 4 s
# in the kernel lane, 7 s in the STEP lane — on an unloaded box, and
# 106 s for the SAME scene at load average 13 (other lanes building;
# startup dominates and it is all CPU contention, memory was never
# tight). So the budget is sized off the contended number, not the idle
# one: 300 s is ~3x the slowest legitimate scene ever measured here and
# still bounds a wedged process at 10 minutes (two attempts) instead of
# the whole night. Raise FREECAD_SCENE_TIMEOUT only for a scene that is
# genuinely that slow — a wedge does not get faster with a bigger
# budget.
SCENE_TIMEOUT="${FREECAD_SCENE_TIMEOUT:-300}"
# Grace between the budget's SIGTERM and SIGKILL to the scene process.
SCENE_KILL_GRACE=5
LOGDIR=out/freecad-logs
# Where an in-flight pass lives. Untracked (out/ is gitignored), and
# holding a directory named after each lane, so a staged frame's path
# — which FreeCAD stamps into it — matches its published one exactly.
STAGE_ROOT=out/stage

# Set by render_scene for its caller.
SCENE_REASON=""     # why the scene failed
SCENE_TIMED_OUT=0   # 1 iff the failure was the budget, twice
SCENE_SECS=0        # wall seconds of the winning (or last) attempt

# One attempt at one scene, in a FRESH freecadcmd process under the
# budget. The attempt runs in its OWN SESSION so the budget covers the
# whole process TREE: `timeout` signals only its direct child, so
# anything a wedged FreeCAD spawned would outlive it; the session's
# process group is swept after a timeout. `setsid -w` is what makes
# the exit status propagate (plain setsid may fork and return 0), and
# the exec'd shell reports the session leader's pid — the pgid to kill.
# Returns 124 iff the budget was exhausted, whatever ended the process.
scene_attempt() {
    local name=$1 lane=$2 mode=$3 log=$4
    local pgidfile="$LOGDIR/$name.pgid" pgid rc=0
    local start=$(date +%s)
    rm -f "$pgidfile"
    set +e
    (
        # This shell's own report of a signalled child is dropped: the
        # caller announces the kill in its own words, and the log must
        # hold NOTHING but the scene's output — its mtime is the
        # scene's last sign of life, which is what tells a stalled
        # process from a slow one.
        exec 2>/dev/null
        RT_GRACE="$SCENE_KILL_GRACE" RT_BUDGET="$SCENE_TIMEOUT" \
        RT_FC="$FREECADCMD" RT_RENDERER="$PWD/render_freecad.py" \
        RT_MODE="$mode" RT_SCENE="$name" RT_LANE="$lane" \
        RT_ROOT="$STAGE_ROOT" RT_PGID="$pgidfile" \
            setsid -w bash -c '
                echo $$ >"$RT_PGID"
                # The scene process runs with the STAGING ROOT as its
                # cwd, so the render directory it is handed is the bare
                # lane name — FreeCAD stamps the saveImage PATH into
                # the PNG (a tEXt "Title" chunk), so a staged frame is
                # byte-identical to a published one only if that path
                # string is identical too. The staging root sits one
                # level under demos/out, so the tour output dir is "..".
                cd "$RT_ROOT"
                exec timeout -k "$RT_GRACE" "$RT_BUDGET" \
                    env QT_QPA_PLATFORM=offscreen \
                    "$RT_FC" "$RT_RENDERER" "$RT_MODE" "scene=$RT_SCENE" \
                    .. "$RT_LANE"
            ' >"$log" 2>&1
    )
    rc=$?
    set -e
    # The CLOCK decides whether the budget was exhausted, not the exit
    # status: `timeout` reports 124 when the scene process died of the
    # SIGTERM it sends, but escalating to SIGKILL means signalling its
    # own process group — so `timeout` dies along with the scene and
    # reports 137 instead. Both are the budget running out.
    if [ "$rc" -ne 0 ] && [ "$(( $(date +%s) - start ))" -ge "$SCENE_TIMEOUT" ]; then
        rc=124
    fi
    if [ "$rc" -eq 124 ]; then
        # Only after a timeout: the group may still hold live children.
        # (Once the group is gone its pgid is reusable, so sweeping a
        # cleanly-exited scene would be aiming at whoever inherited it.)
        pgid=$(cat "$pgidfile" 2>/dev/null || true)
        [ -n "$pgid" ] && kill -KILL -- "-$pgid" 2>/dev/null
    fi
    rm -f "$pgidfile"
    return $rc
}

# Render ONE scene, isolated and budgeted, retried ONCE if the budget
# was the thing that failed. A crash is NOT retried: it is deterministic
# and the caller's business. Success is the PNG existing, not the exit
# status — freecadcmd's Qt teardown can crash after a fully successful
# render (offscreen destructor bug).
render_scene() {
    local name=$1 lane=$2 mode=$3
    local stage="$STAGE_ROOT/$lane"
    local log="$LOGDIR/$name.log" attempt start rc stalled
    SCENE_REASON=""; SCENE_TIMED_OUT=0; SCENE_SECS=0
    for attempt in 1 2; do
        start=$(date +%s)
        rc=0
        scene_attempt "$name" "$lane" "$mode" "$log" || rc=$?
        SCENE_SECS=$(( $(date +%s) - start ))
        if [ -f "$stage/$name.png" ]; then
            echo "  [$name] rendered in ${SCENE_SECS}s (attempt $attempt)"
            # The frame is good (chunk framing and CRCs are checked
            # downstream), but the process still had to be killed:
            # FreeCAD stalled AFTER the render. Never silent — this is
            # the wedge showing itself where it costs nothing.
            if [ "$rc" -eq 124 ]; then
                echo "  [$name] NOTE — the frame was written, then the process stalled past the ${SCENE_TIMEOUT}s budget and was killed (log: $LOGDIR/$name.log)" >&2
            fi
            return 0
        fi
        if [ "$rc" -eq 124 ]; then
            # Cheap wedge-vs-slow signal, recorded not acted on: a scene
            # that is merely slow keeps writing to its log, a wedged
            # session goes silent. Both outcomes are the same failure.
            stalled=$(( $(date +%s) - $(stat -c %Y "$log") ))
            SCENE_REASON="freecadcmd exceeded the ${SCENE_TIMEOUT}s per-scene budget (silent for the last ${stalled}s)"
            SCENE_TIMED_OUT=1
            if [ "$attempt" -eq 1 ]; then
                echo "  [$name] TIMED OUT after ${SCENE_TIMEOUT}s — process tree killed, retrying once in a fresh process (silent for the last ${stalled}s)" >&2
                continue
            fi
        else
            SCENE_TIMED_OUT=0
            SCENE_REASON="freecadcmd rc=$rc: $(tail -n 2 "$log" | tr '\n' ' ' | cut -c1-160)"
        fi
        return 1
    done
    return 1
}

# A scene that exhausted the budget twice ends the pass. Loud, named,
# and with the committed tree untouched — the same contract as the
# absent-FreeCAD arm: a pass that did not finish publishes nothing.
wedged() {
    local name=$1 rd=$2
    local stage="$STAGE_ROOT/$rd"
    cat >&2 <<EOF

================================================================
 RENDER WEDGED — scene '$name'
   exhausted the ${SCENE_TIMEOUT}s per-scene budget TWICE, each time
   in a fresh freecadcmd process whose tree was then killed. Two
   fresh processes stalling on one scene is not a slow scene.
   log: demos/$LOGDIR/$name.log
 THE PASS FAILS HERE. No montage is composed, and
   demos/$rd/
 is left exactly as committed: this pass rendered into
   demos/$stage/
 which is untracked. Re-run to retry. Raise FREECAD_SCENE_TIMEOUT
 only if the scene is genuinely that slow; a wedge is not.
================================================================

EOF
    exit 1
}

# Scene names in scenes.json order. "montage" restricts to the cells
# that reach a contact sheet (the STEP lane renders only those).
scene_names() {
    "$VENV/bin/python" -c 'import json, sys
only_montage = len(sys.argv) > 1 and sys.argv[1] == "montage"
for s in json.load(open("out/scenes.json")):
    if s.get("montage", True) or not only_montage:
        print(s["name"])' "$@"
}

# Wall-time summary of the pass — the per-scene distribution is the
# evidence a budget is sized right (or has drifted).
scene_stats() {
    printf '%s\n' "$@" | "$VENV/bin/python" -c 'import sys
t = sorted(int(x) for x in sys.stdin.read().split())
n = len(t)
print(f"{n} scene(s): median {t[n // 2]}s, max {t[-1]}s, total {sum(t)}s"
      if n else "no scenes")'
}

# Move a completed pass from staging into the committed lane directory.
# Only reached when every scene rendered, so the lane directory goes
# from one whole pass to the next, never through a partial one.
publish() {
    local stage=$1 rd=$2 f name
    mkdir -p "$rd"
    for f in "$stage"/*.png; do
        name=$(basename "$f" .png)
        rm -f "$rd/$name.fail.txt"
        mv -f "$f" "$rd/$name.png"
    done
    for f in "$stage"/*.fail.txt; do
        [ -e "$f" ] || continue
        name=$(basename "$f" .fail.txt)
        # The scene produced no frame this pass, so any frame left from
        # an earlier one goes: the sheet must show the labeled failure,
        # not a stale cell that no longer corresponds to anything.
        rm -f "$rd/$name.png"
        mv -f "$f" "$rd/"
    done
}

if [ "${1:-}" = "--freecad" ]; then
    # ---- FreeCAD/OCC STEP lane ------------------------------------
    if [ ! -x "$FREECADCMD" ]; then
        echo "freecadcmd not found at $FREECADCMD — the --freecad lane" >&2
        echo "has no fallback (its whole point is the OCC reference render)" >&2
        exit 1
    fi
    RD=renders-freecad
    STAGE="$STAGE_ROOT/$RD"
    rm -rf "$STAGE"
    mkdir -p "$STAGE" "$LOGDIR"
    fails=0
    times=()
    for name in $(scene_names montage); do
        if render_scene "$name" "$RD" step; then
            times+=("$SCENE_SECS")
        else
            # A wedge ends the pass; a scene that genuinely cannot be
            # imported or rendered costs one labeled cell, as before.
            if [ "$SCENE_TIMED_OUT" -eq 1 ]; then
                wedged "$name" "$RD"
            fi
            printf '%s\n' "$SCENE_REASON" >"$STAGE/$name.fail.txt"
            echo "  [$name] FAILED — $SCENE_REASON (placeholder cell; log: $LOGDIR/$name.log)" >&2
            fails=$((fails + 1))
        fi
    done
    echo "STEP lane: $(scene_stats "${times[@]}") [budget ${SCENE_TIMEOUT}s/scene]"
    if [ "$fails" -gt 0 ]; then
        echo "$fails scene(s) fell back to placeholder cells" >&2
    fi
    # FreeCAD stamps the wall clock into every PNG it writes, so an
    # unchanged re-render still shows up dirty in `git status`. Drop
    # those two ancillary chunks (see strip_png_stamps.py) BEFORE the
    # frames are published, so the sheet is composed from — and the
    # lane directory holds — the same bytes that get committed.
    "$VENV/bin/python" strip_png_stamps.py "$STAGE"
    publish "$STAGE" "$RD"
    # Provenance guard before the sheet is composed: every cell that
    # goes on it must carry FreeCAD's signature chunks.
    "$VENV/bin/python" check_render_provenance.py "$RD"
    exec "$VENV/bin/python" compose_montage.py out "$RD" \
        --montage=montage-freecad.png \
        '--banner=FreeCAD/OCC render — OCC re-tessellation of the kernel'\''s own STEP exports (compare renders/montage.png: the kernel'\''s facets)'
fi

# ---- kernel-tessellation lane (the original montage) ---------------
RD=renders
STAGE="$STAGE_ROOT/$RD"
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
 frame must never be committable (#221). Whatever FreeCAD managed
 to render this pass is in demos/$STAGE/ (untracked), not in the
 lane directory — so a partial pass cannot masquerade as a whole
 one. To update the committed sheet, fix FreeCAD (FREECADCMD=...)
 and re-run.
================================================================

EOF
    "$VENV/bin/python" render.py out "$PREVIEW"
    exec "$VENV/bin/python" compose_montage.py out "$PREVIEW" \
        '--banner=PREVIEW ONLY — matplotlib fallback (FreeCAD unavailable); NOT the committed sheet in demos/renders/'
}

if [ ! -x "$FREECADCMD" ]; then
    fallback "freecadcmd not found at $FREECADCMD"
fi
rm -rf "$STAGE"
mkdir -p "$STAGE" "$LOGDIR"
times=()
for name in $(scene_names); do
    if render_scene "$name" "$RD" mesh; then
        times+=("$SCENE_SECS")
    else
        # This lane has no placeholder cells: a scene it cannot render
        # is a lane it cannot certify, so it goes to the preview tree
        # whole rather than committing a hole. A wedge is louder still.
        if [ "$SCENE_TIMED_OUT" -eq 1 ]; then
            wedged "$name" "$RD"
        fi
        fallback "scene '$name': $SCENE_REASON"
    fi
done
echo "kernel lane: $(scene_stats "${times[@]}") [budget ${SCENE_TIMEOUT}s/scene]"

# Same wall-clock strip as the STEP lane, before publishing.
"$VENV/bin/python" strip_png_stamps.py "$STAGE"
publish "$STAGE" "$RD"
# Provenance guard before the sheet is composed (see the header).
"$VENV/bin/python" check_render_provenance.py "$RD"

# The kernel sheet carries its own provenance banner too, so the two
# sheets superimpose exactly — cell for cell AND banner for banner.
exec "$VENV/bin/python" compose_montage.py out "$RD" \
    '--banner=kernel render — the kernel'\''s own certified tessellation (compare renders-freecad/montage-freecad.png: OCC'\''s facets)'
