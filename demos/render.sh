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
# ONE FREECADCMD PROCESS PER SCENE, IN BOTH LANES (#224 follow-up) —
# AND STILL EXACTLY THAT BY DEFAULT. A warm FreeCAD session that
# rendered many scenes deadlocked partway through — reproducibly on
# this host, at a different scene each time, on an idle box as well as a
# loaded one (freecadcmd at 0% CPU, `wchan = futex_do_wait`): it was the
# session that wedged, not any one scene. That deadlock was ROOT-CAUSED
# in 2026-08 — FreeCAD's notification area re-entering its own
# non-recursive mutex under the offscreen QPA plugin — and
# render_freecad.py disables it before FreeCADGui loads, so the process
# boundary is no longer a workaround for a live bug. It is kept because
# it is what BOUNDS a future hang: CAD_RENDER_BATCH is exactly the dial
# for how much of a pass one hang can cost. Every process runs under a
# wall-clock budget (see SCENE_TIMEOUT) with the process tree killed and
# ONE fresh retry when the budget is exhausted. A budget exhausted TWICE
# fails the pass, loudly, naming the scene and the budget — never a
# silent skip, never a degraded cell.
#
# ...AND, BY DEFAULT, ONE PER PROCESS. CAD_RENDER_BATCH renders that
# many scenes in ONE freecadcmd process, trading FreeCAD's startup (a
# near-constant few seconds, paid once per process) against blast radius
# (a hang costs its whole batch). Default 1 — byte-for-byte today's
# behaviour, down to the log filenames.
#
# ...AND, BY DEFAULT, ONE AT A TIME. CAD_RENDER_JOBS renders that many
# scenes concurrently — still one fresh session per scene, so it does
# not reopen #224; see the knob's own note for why the per-scene budget
# is what concurrency actually trades against. Sequential is the
# default and the only setting the budget note's measurements cover.
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
#
# THIS KNOB STAYS PER-SCENE WHEN ONE PROCESS RENDERS SEVERAL. A batch of
# N scenes gets N x this, so the number means the same thing at every
# batch size and only the process it is applied to changes. N x is
# generous on purpose — a batch pays ONE startup, and startup is most of
# a typical scene — because this budget's job is to BOUND A HANG, not to
# detect a slow scene. The alternative, a per-scene watchdog INSIDE the
# process, was rejected on the root cause: the hang it would have to
# interrupt is a mutex re-entry on FreeCAD's own main thread, which
# never gets back to a bytecode boundary and never releases the GIL, so
# neither a Python signal handler nor a watchdog thread would ever run.
# Only a process outside can kill it, and `timeout` is that process.
SCENE_TIMEOUT="${FREECAD_SCENE_TIMEOUT:-300}"
# Grace between the budget's SIGTERM and SIGKILL to the scene process.
SCENE_KILL_GRACE=5

# ---- how many scenes at once ---------------------------------------
# DEFAULT 1 — strictly sequential, which is what every measurement in
# this file's budget note was taken under. Raising it renders that many
# scenes CONCURRENTLY, each still in its own fresh session under its own
# budget: the #224 finding is that a REUSED session wedges, and nothing
# here reuses one, so concurrency is orthogonal to it. What concurrency
# does interact with is the budget, and in the direction that matters:
# the same note measures a scene at 106 s under contention versus 3-19 s
# idle, so K concurrent scenes on a K-core box push every scene's
# wall-clock toward the contended number. The budget is sized for that
# (300 s is ~3x the worst contended scene ever measured), but a box with
# fewer cores than RENDER_JOBS is asking for the wedge-shaped failure
# that is expensive to tell apart from a real one. Keep it <= the core
# count.
RENDER_JOBS="${CAD_RENDER_JOBS:-1}"
case "$RENDER_JOBS" in
    ''|*[!0-9]*|0) echo "CAD_RENDER_JOBS must be a positive integer, got '$RENDER_JOBS'" >&2; exit 2 ;;
esac

# ---- how many scenes per freecadcmd process ------------------------
# DEFAULT 1 — one fresh process per scene, which is what this lane has
# done since #224 and what every measurement in the budget note above
# was taken under. At 1 this knob changes NOTHING: the same processes in
# the same order, the same per-scene budget, the same per-scene log
# filenames, the same failure messages.
#
# Above 1, that many CONSECUTIVE scenes (scenes.json order) render in
# ONE freecadcmd process, paying FreeCAD's startup once instead of once
# per scene. Startup is worth paying less often: on the hosted runner a
# scene costs 3-9 s with a MEDIAN of 5 s across scenes of wildly
# different geometric complexity, and a near-constant floor under wildly
# varying work is process + GUI startup, not drawing.
#
# WHAT IT COSTS, AND WHY THE DEFAULT IS NOT "ALL OF THEM". The process
# boundary is what bounds a hang. At batch B a wedge costs B scenes'
# work and up to 2 x B x SCENE_TIMEOUT of wall clock before the pass
# gives up, against 2 x SCENE_TIMEOUT at B=1. That is the whole trade
# and it is linear in B, so a SMALL batch takes most of the startup
# saving while keeping a hang's blast radius to a fraction of a pass.
#
# WHAT IT MUST NOT COST IS THE PIXELS. Scenes in one process share a
# document and a view (render_freecad.py: one warm document, per-scene
# visibility toggling — per-scene document cycling races the offscreen
# view-provider setup, which is a worse bug than the one being avoided),
# so batching is admissible only if a batched frame is BYTE-IDENTICAL to
# an unbatched one. That is this knob's acceptance test, the same one
# CAD_RENDER_JOBS had to pass, and it is asked of a repeat render on ONE
# box — PNG bytes are not comparable across GL stacks.
RENDER_BATCH="${CAD_RENDER_BATCH:-1}"
case "$RENDER_BATCH" in
    ''|*[!0-9]*|0) echo "CAD_RENDER_BATCH must be a positive integer, got '$RENDER_BATCH'" >&2; exit 2 ;;
esac
LOGDIR=out/freecad-logs
# Where an in-flight pass lives. Untracked (out/ is gitignored), and
# holding a directory named after each lane, so a staged frame's path
# — which FreeCAD stamps into it — matches its published one exactly.
STAGE_ROOT=out/stage

# Set by render_batch for its caller. A "batch" is one freecadcmd
# process and the scenes it renders; at CAD_RENDER_BATCH=1 that is one
# scene, and every message below reads exactly as it did before.
BATCH_REASON=""     # why the batch failed
BATCH_TIMED_OUT=0   # 1 iff the failure was the budget, twice
BATCH_SECS=0        # wall seconds of the winning (or last) attempt

# One attempt at one BATCH of scenes, in a FRESH freecadcmd process
# under the batch's budget. The attempt runs in its OWN SESSION so the
# budget covers the whole process TREE: `timeout` signals only its
# direct child, so anything a wedged FreeCAD spawned would outlive it;
# the session's process group is swept after a timeout. `setsid -w` is
# what makes the exit status propagate (plain setsid may fork and
# return 0), and the exec'd shell reports the session leader's pid —
# the pgid to kill.
# Returns 124 iff the budget was exhausted, whatever ended the process.
batch_attempt() {
    local bid=$1 lane=$2 mode=$3 log=$4 budget=$5 name
    shift 5
    local pgidfile="$LOGDIR/$bid.pgid" pgid rc=0 scenes=""
    local start=$(date +%s)
    # One `scene=` keyword per scene. render_freecad.py renders them in
    # scenes.json order whatever order they arrive in, so this string
    # cannot become a second, disagreeing definition of scene order.
    for name in "$@"; do scenes="$scenes scene=$name"; done
    rm -f "$pgidfile"
    set +e
    (
        # This shell's own report of a signalled child is dropped: the
        # caller announces the kill in its own words, and the log must
        # hold NOTHING but the scene's output — its mtime is the
        # scene's last sign of life, which is what tells a stalled
        # process from a slow one.
        exec 2>/dev/null
        RT_GRACE="$SCENE_KILL_GRACE" RT_BUDGET="$budget" \
        RT_FC="$FREECADCMD" RT_RENDERER="$PWD/render_freecad.py" \
        RT_MODE="$mode" RT_SCENES="$scenes" RT_LANE="$lane" \
        RT_ROOT="$STAGE_ROOT" RT_PGID="$pgidfile" \
            setsid -w bash -c '
                echo $$ >"$RT_PGID"
                # The scene process runs with the STAGING ROOT as its
                # cwd, so the render directory it is handed is the bare
                # lane name, matching the published one. That WAS a
                # byte-identity requirement — FreeCAD stamps the
                # saveImage path into the PNG as a tEXt "Title" chunk —
                # and is now only tidiness: strip_png_stamps.py drops
                # Title as of 2026-08-22, so the bytes of a frame no
                # longer record where it was written. NOTE: this comment
                # sits inside a single-quoted bash -c body, so it must
                # not contain an apostrophe. The staging root sits one
                # level under demos/out, so the tour output dir is "..".
                cd "$RT_ROOT"
                # RT_SCENES unquoted: it is one `scene=NAME` keyword
                # per scene and scene names are identifiers by
                # construction (see the SC2086 disables below).
                # shellcheck disable=SC2086
                exec timeout -k "$RT_GRACE" "$RT_BUDGET" \
                    env QT_QPA_PLATFORM=offscreen \
                    "$RT_FC" "$RT_RENDERER" "$RT_MODE" $RT_SCENES \
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
    if [ "$rc" -ne 0 ] && [ "$(( $(date +%s) - start ))" -ge "$budget" ]; then
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

# Render ONE BATCH of scenes, isolated and budgeted, retried ONCE if the
# budget was the thing that failed. THE WHOLE BATCH IS RETRIED, frames
# and all: every frame is rewritten from the same inputs to the same
# path, so re-rendering a scene that already succeeded is idempotent and
# costs only its own seconds — and it keeps the retry a plain "run that
# process again" rather than a second, subtly different code path that
# only ever executes after a wedge. A crash is NOT retried: it is
# deterministic and the caller's business (batch_status splits it).
# Success is every PNG existing, not the exit status — freecadcmd's Qt
# teardown can crash after a fully successful render (offscreen
# destructor bug).
render_batch() {
    local bid=$1 lane=$2 mode=$3
    shift 3
    local stage="$STAGE_ROOT/$lane"
    local log="$LOGDIR/$bid.log" attempt start rc stalled name missing
    local n=$# budget=$(( SCENE_TIMEOUT * $# )) what
    [ "$n" -eq 1 ] && what="scene '$1'" || what="$n scenes"
    BATCH_REASON=""; BATCH_TIMED_OUT=0; BATCH_SECS=0
    for attempt in 1 2; do
        start=$(date +%s)
        rc=0
        batch_attempt "$bid" "$lane" "$mode" "$log" "$budget" "$@" || rc=$?
        BATCH_SECS=$(( $(date +%s) - start ))
        missing=""
        for name in "$@"; do
            [ -f "$stage/$name.png" ] || missing="$missing $name"
        done
        if [ -z "$missing" ]; then
            echo "  [$bid] rendered $what in ${BATCH_SECS}s (attempt $attempt)"
            # The frames are good (chunk framing and CRCs are checked
            # downstream), but the process still had to be killed:
            # FreeCAD stalled AFTER the last render. Never silent — this
            # is the wedge showing itself where it costs nothing.
            if [ "$rc" -eq 124 ]; then
                echo "  [$bid] NOTE — the frames were written, then the process stalled past the ${budget}s budget and was killed (log: $LOGDIR/$bid.log)" >&2
            fi
            return 0
        fi
        if [ "$rc" -eq 124 ]; then
            # Cheap wedge-vs-slow signal, recorded not acted on: a scene
            # that is merely slow keeps writing to its log, a wedged
            # session goes silent. Both outcomes are the same failure.
            stalled=$(( $(date +%s) - $(stat -c %Y "$log") ))
            BATCH_REASON="freecadcmd exceeded the ${budget}s budget (${n} x ${SCENE_TIMEOUT}s), no frame for:${missing} (silent for the last ${stalled}s)"
            BATCH_TIMED_OUT=1
            if [ "$attempt" -eq 1 ]; then
                echo "  [$bid] TIMED OUT after ${budget}s — process tree killed, retrying the whole batch once in a fresh process (silent for the last ${stalled}s)" >&2
                continue
            fi
        else
            BATCH_TIMED_OUT=0
            BATCH_REASON="freecadcmd rc=$rc: $(tail -n 2 "$log" | tr '\n' ' ' | cut -c1-160)"
        fi
        return 1
    done
    return 1
}

# ---- running the scenes --------------------------------------------
# render_batch reports through globals, which a background job cannot
# hand back, so the concurrent path routes the same facts through a
# PER-SCENE status file — per scene, not per batch, so everything
# downstream still reasons about scenes. Two lines, because the reason
# is free text:
#
#   line 1:  ok|fail|wedged <seconds> <batch id>
#   line 2:  the reason (empty on success)
#
# <seconds> is the whole batch's wall clock and <batch id> names the
# process that produced it (and its log, $LOGDIR/<batch id>.log), so the
# reader can attribute one process's time once rather than once per
# scene. At CAD_RENDER_BATCH=1 the batch id IS the scene name and both
# fields mean what they always meant.
#
# The parent reads them in SCENES.JSON ORDER, so the pass's report, its
# failure, and which scene ends it are all identical whatever order the
# scenes actually finished in. Only the progress chatter interleaves.
#
# A FAILED BATCH IS SPLIT, which is what keeps a batch from widening a
# failure. One scene FreeCAD cannot draw kills the process it is in, so
# in a batch of 5 it would take the four innocent scenes down with it —
# four placeholder cells (STEP lane) or a whole-lane matplotlib fallback
# (kernel lane) caused by a scene that renders perfectly well. So a
# non-timeout failure re-runs each frameless scene of that batch ALONE,
# one process each: the guilty scene fails exactly as it does today,
# with its own log under its own name, and the innocent ones render.
# The split costs one extra process per frameless scene and is only ever
# reached on a failure. A WEDGE is NOT split: a budget exhausted twice
# ends the pass regardless, so splitting would only spend another
# 2 x budget per scene to reach the same exit.
batch_status() {
    local bid=$1 lane=$2 mode=$3
    shift 3
    local stage="$STAGE_ROOT/$lane" name kind=ok secs reason
    if render_batch "$bid" "$lane" "$mode" "$@"; then
        for name in "$@"; do
            printf 'ok %s %s\n\n' "$BATCH_SECS" "$bid" >"$LOGDIR/$name.status"
        done
        return
    fi
    [ "$BATCH_TIMED_OUT" -eq 1 ] && kind=wedged || kind=fail
    # Snapshot: a split re-enters render_batch and overwrites these.
    secs=$BATCH_SECS; reason=$BATCH_REASON
    for name in "$@"; do
        if [ -f "$stage/$name.png" ]; then
            printf 'ok %s %s\n\n' "$secs" "$bid" >"$LOGDIR/$name.status"
        elif [ "$kind" = wedged ] || [ "$#" -eq 1 ]; then
            printf '%s %s %s\n%s\n' "$kind" "$secs" "$bid" "$reason" \
                >"$LOGDIR/$name.status"
        else
            batch_status "$name" "$lane" "$mode" "$name"
        fi
    done
}

# Sets ST_KIND / ST_SECS / ST_BID / ST_REASON from a scene's status
# file. A missing file is a bug in this script, not a scene outcome —
# say so rather than silently treating it as either.
ST_KIND=""; ST_SECS=0; ST_BID=""; ST_REASON=""
read_status() {
    local st="$LOGDIR/$1.status"
    [ -f "$st" ] || { echo "internal: no status file for scene '$1'" >&2; exit 3; }
    { read -r ST_KIND ST_SECS ST_BID; read -r ST_REASON; } <"$st"
}

# Render every named scene, RENDER_BATCH scenes per process, at most
# RENDER_JOBS processes in flight. `wait -n` starts the next process the
# moment ANY running one finishes rather than draining the whole wave,
# so a single slow process cannot idle the other slots. At
# RENDER_JOBS=1, RENDER_BATCH=1 this is a plain sequential loop — the
# same order, the same one-process-at-a-time, as before either knob
# existed.
#
# Batches are CONSECUTIVE runs of the scene list, so a batch's scenes
# are adjacent in scenes.json and in every report built from it; the
# scene that ends a wedged pass is still the earliest failing one in
# scenes.json order.
#
# ONE BEHAVIOUR DOES CHANGE ABOVE K=1: a wedge no longer ends the pass
# where it happens. Sequentially the loop reaches the bad scene and
# exits; concurrently the scenes already in flight are let finish
# first, so a wedged pass costs a batch's wall clock rather than
# stopping dead. It is bounded either way (every scene has the same
# budget) and the OUTCOME is identical — same scene named, same exit,
# committed tree equally untouched — so this is a cost, not a hole.
PASS_SECS=0   # wall clock of the whole render loop, set by render_all
BATCH_INDEX=0 # numbers the multi-scene batches, for their log names

# Start one batch in the background, once a job slot is free. A batch of
# ONE is identified by its scene's name — so at CAD_RENDER_BATCH=1 (and
# for a split, and for a lane's odd last batch) the log is
# $LOGDIR/<scene>.log exactly as it has always been; only a genuinely
# multi-scene process gets a batchNN name, which is also the name every
# message about it uses.
launch_batch() {
    local lane=$1 mode=$2 bid
    shift 2
    BATCH_INDEX=$(( BATCH_INDEX + 1 ))
    if [ "$#" -eq 1 ]; then bid=$1; else bid=$(printf 'batch%02d' "$BATCH_INDEX"); fi
    while [ "$(jobs -rp | wc -l)" -ge "$RENDER_JOBS" ]; do wait -n; done
    batch_status "$bid" "$lane" "$mode" "$@" &
}

render_all() {
    local lane=$1 mode=$2 name start
    local -a batch=()
    shift 2
    start=$(date +%s)
    BATCH_INDEX=0
    rm -f "$LOGDIR"/*.status
    for name in "$@"; do
        batch+=("$name")
        if [ "${#batch[@]}" -ge "$RENDER_BATCH" ]; then
            launch_batch "$lane" "$mode" "${batch[@]}"
            batch=()
        fi
    done
    [ "${#batch[@]}" -gt 0 ] && launch_batch "$lane" "$mode" "${batch[@]}"
    wait
    PASS_SECS=$(( $(date +%s) - start ))
}

# A scene that exhausted the budget twice ends the pass. Loud, named,
# and with the committed tree untouched — the same contract as the
# absent-FreeCAD arm: a pass that did not finish publishes nothing.
wedged() {
    local name=$1 rd=$2 bid=${3:-$1}
    local stage="$STAGE_ROOT/$rd" unit="scene '$name'" budget=$SCENE_TIMEOUT
    if [ "$bid" != "$name" ]; then
        # A batched process: the budget was the batch's, and the scene
        # named is the earliest one in it that never got a frame.
        unit="batch '$bid' (${RENDER_BATCH} scenes), no frame for scene '$name'"
        budget="$(( SCENE_TIMEOUT * RENDER_BATCH ))"
    fi
    cat >&2 <<EOF

================================================================
 RENDER WEDGED — $unit
   exhausted the ${budget}s budget TWICE, each time
   in a fresh freecadcmd process whose tree was then killed. Two
   fresh processes stalling on the same work is not slow work.
   log: demos/$LOGDIR/$bid.log
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
# that reach a contact sheet (the STEP lane renders only those). The
# shared reader does the walk and owns what "on the montage" means, so
# this lister cannot disagree with the composer about it -- and it runs
# as a SCRIPT, like every other consumer, rather than as an inline
# `python -c` that would reach the module through the current
# directory instead of its own.
scene_names() {
    "$VENV/bin/python" manifest.py --scene-names out "$@"
}

# Wall-time summary of the pass — the distribution is the evidence a
# budget is sized right (or has drifted). $1 names what is being
# counted: one PROCESS's wall clock is one datum, which is a scene at
# CAD_RENDER_BATCH=1 and a batch above it. Counting it per scene would
# quietly restate a batch's seconds as each of its scenes' — the same
# number reported N times — and the budget is applied per process, so
# the process is the honest unit.
scene_stats() {
    local unit=$1
    shift
    printf '%s\n' "$@" | "$VENV/bin/python" -c 'import sys
unit = sys.argv[1]
t = sorted(int(x) for x in sys.stdin.read().split())
n = len(t)
print(f"{n} {unit}(s): median {t[n // 2]}s, max {t[-1]}s, total {sum(t)}s"
      if n else f"no {unit}s")' "$unit"
}

# What scene_stats is counting, and what the lane line calls it.
STATS_UNIT=scene
BATCH_NOTE=""
if [ "$RENDER_BATCH" -gt 1 ]; then
    STATS_UNIT=batch
    BATCH_NOTE=" x ${RENDER_BATCH} scene(s)/process"
fi

# Collect one time per PROCESS. Scenes share a status line with the rest
# of their batch, so the same seconds must not be counted once per
# scene: SEEN_BIDS is the set of batches already counted.
SEEN_BIDS=" "
count_time() {
    case "$SEEN_BIDS" in
        *" $1 "*) return ;;
    esac
    SEEN_BIDS="$SEEN_BIDS$1 "
    times+=("$2")
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
    names=$(scene_names montage)
    # shellcheck disable=SC2086  # scene names are identifiers, by construction
    render_all "$RD" step $names
    for name in $names; do
        read_status "$name"
        case "$ST_KIND" in
            ok) count_time "$ST_BID" "$ST_SECS" ;;
            # A wedge ends the pass; a scene that genuinely cannot be
            # imported or rendered costs one labeled cell, as before —
            # batch_status re-ran it alone to make sure the cost is one
            # cell and not its whole batch.
            wedged) wedged "$name" "$RD" "$ST_BID" ;;
            *)
                printf '%s\n' "$ST_REASON" >"$STAGE/$name.fail.txt"
                echo "  [$name] FAILED — $ST_REASON (placeholder cell; log: $LOGDIR/$name.log)" >&2
                fails=$((fails + 1))
                ;;
        esac
    done
    # The "total" is summed PROCESS wall clock, so above RENDER_JOBS=1
    # it exceeds the pass's own wall clock — both are printed rather
    # than one being quietly redefined.
    echo "STEP lane: $(scene_stats "$STATS_UNIT" "${times[@]}"), ${PASS_SECS}s wall at ${RENDER_JOBS} job(s)${BATCH_NOTE} [budget ${SCENE_TIMEOUT}s/scene]"
    if [ "$fails" -gt 0 ]; then
        echo "$fails scene(s) fell back to placeholder cells" >&2
    fi
    # FreeCAD stamps the wall clock and the output path into every PNG
    # it writes, so an unchanged re-render still shows up dirty in
    # `git status` and the same pixels at two paths are two files. Drop
    # those three ancillary chunks (see strip_png_stamps.py) BEFORE the
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
names=$(scene_names)
# shellcheck disable=SC2086  # scene names are identifiers, by construction
render_all "$RD" mesh $names
for name in $names; do
    read_status "$name"
    case "$ST_KIND" in
        ok) count_time "$ST_BID" "$ST_SECS" ;;
        # This lane has no placeholder cells: a scene it cannot render
        # is a lane it cannot certify, so it goes to the preview tree
        # whole rather than committing a hole. A wedge is louder still.
        wedged) wedged "$name" "$RD" "$ST_BID" ;;
        *) fallback "scene '$name': $ST_REASON" ;;
    esac
done
echo "kernel lane: $(scene_stats "$STATS_UNIT" "${times[@]}"), ${PASS_SECS}s wall at ${RENDER_JOBS} job(s)${BATCH_NOTE} [budget ${SCENE_TIMEOUT}s/scene]"

# Same wall-clock strip as the STEP lane, before publishing.
"$VENV/bin/python" strip_png_stamps.py "$STAGE"
publish "$STAGE" "$RD"
# Provenance guard before the sheet is composed (see the header).
"$VENV/bin/python" check_render_provenance.py "$RD"

# The kernel sheet carries its own provenance banner too, so the two
# sheets superimpose exactly — cell for cell AND banner for banner.
exec "$VENV/bin/python" compose_montage.py out "$RD" \
    '--banner=kernel render — the kernel'\''s own certified tessellation (compare renders-freecad/montage-freecad.png: OCC'\''s facets)'
