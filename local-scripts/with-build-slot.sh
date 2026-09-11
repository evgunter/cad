#!/usr/bin/env bash
# with-build-slot.sh — machine-wide build-slot semaphore for agent lanes.
#
#   local-scripts/with-build-slot.sh [-x | --express [SECS]] [-n] [-w SECS] -- <command> [args...]
#
# Replaces the soft "two lanes" convention (and the retired
# cargo-slots.txt registry) with real locks: slot lockfiles under
# ~/.local/share/cad-work/locks/ bound the number of concurrent heavy
# cargo operations machine-wide. Any number of agents may be ALIVE;
# only their builds/batteries queue here.
#
# WIDTH IS 1 BY DEFAULT (a mutex), measured, not assumed: the
# 2026-08-06 slot experiment (two lanes, warm-deps workspace rebuild
# after touching geom-core) measured the concurrent pair at 98s wall
# (-j8 each) and 111s (-j4 each) vs 69s run back-to-back — concurrency
# LOSES ~40% to cache/memory-bandwidth contention on this 8-core box,
# and capping jobs makes it worse (solo -j4 52s vs solo -j8 33s), so
# no CARGO_BUILD_JOBS cap is applied either. RAM was never tight (min
# MemAvailable 5.5 GB at the worst) — serialization is purely a
# throughput win, on top of removing the battery-OOM failure mode.
# Full numbers: ~/.local/share/cad-work/slot-exp-results.md and PR
# #230. If the hardware changes, re-run the experiment and set
# CAD_SLOT_WIDTH=2 to re-widen shared mode.
#
# THAT IS AN EXTRACTION AND NOTHING RE-TAKES IT. The default width is set
# BY the 2026-08-06 numbers, and they are one 8-core box's wall clock on
# one workspace state; no register re-measures them and none could — the
# subject is a developer's machine, which no CI job runs on. Two things
# follow, and the second is the sharper one. The reading ages with the
# hardware AND with the workspace, but a shrinking build only makes
# contention matter less, so width 1 is a floor that stays SAFE as it ages
# rather than one that quietly stops holding — which is why unguarded is
# tolerable here and `CAD_SLOT_WIDTH` is the whole recourse. And the
# evidence is off-repo: `~/.local/share/cad-work/slot-exp-results.md` sits
# on one machine, so for every other reader that citation is a promise
# rather than a source. PR #230 is the half anyone can open; prefer
# re-running the experiment to trusting either.
#
# Modes:
#   (default)  acquire one build slot (with width 1: the mutex) —
#              ordinary builds / fast test runs.
#   -x         acquire BOTH MAIN slots, slot-1 then slot-2 (the express
#              slot is never touched) — full batteries and gate runs.
#              Identical to default at width 1; kept distinct so
#              batteries stay exclusive if the width is ever raised
#              (two concurrent batteries are the documented OOM shape:
#              bare "Terminated" rows).
#   --express [SECS]
#              EXPRESS LANE: acquire the separate express slot instead
#              of a main slot — for jobs that DECLARE a short budget
#              (default 600s, hard max 600s). The declaration is
#              self-enforcing: the command runs under `timeout SECS`,
#              so a job that lied about being short is killed by its
#              own declaration (exit 124). Batteries and default jobs
#              keep the main mutex exactly as before — two concurrent
#              batteries stay impossible (the documented OOM shape).
#              Incompatible with -x (batteries are never express).
#   -n         non-blocking: if the slot(s) are busy, exit 75
#              (EX_TEMPFAIL) immediately instead of waiting. Agents can
#              try -n first and fall back to a blocking call — useful
#              because a blocking wait can eat a Bash tool call's
#              10-minute cap.
#   -w SECS    give up (exit 75) after SECS of waiting. Default: wait
#              forever, printing a status line every 60s naming the
#              current holders.
#
# WHY AN EXPRESS LANE (and not a priority queue): the measured pain
# (#235, #266) is short jobs (a 3-minute clippy, a small suite)
# starving an hour-plus behind a battery holding the width-1 mutex —
# six bounded waits starved during one 90-minute workspace test. A
# priority queue needs a broker or lock-ordering protocol flock can't
# express; a second slot file gets the priority-queue benefit with
# zero queue machinery. Cost-benefit from the #230 numbers:
# concurrency loses ~40% throughput only WHILE both jobs run, so a
# ≤10-min express job overlapping a battery costs the battery a few
# minutes and saves the express job up to the battery's whole
# remaining hold — overwhelmingly net-positive, and death-safe like
# everything flock. The budget declaration is the admission ticket:
# `timeout` enforces it, so the express slot can never silently become
# a second battery slot.
#
# RECORDED FOLLOW-UP (unmeasured): render×cargo contention. Renders
# currently hold the main mutex like any other job; #266 measured
# renders degrading ~25× under cargo contention (median 4s → 56s per
# scene), but the effect of the express split on renders is UNMEASURED
# — a measurement in the #230 style may later give renders their own
# class (e.g. shared render slots that never block on cargo).
#
# Locking is flock(2) on inherited fds: the kernel releases the lock
# when the last holding process exits — even SIGKILL or an OOM kill —
# so a dead agent can never leave a stale lock. Exclusive mode acquires
# slot 1 then slot 2 in FIXED ORDER; shared holders never hold-and-wait
# (they take exactly one), so deadlock is impossible.
#
# Nesting: sets BUILD_SLOT_HELD in the child's environment; a nested
# invocation (e.g. an agent wrapping test-fast.sh, which self-wraps)
# passes straight through instead of double-acquiring. Note a shared
# holder invoking an exclusive-wanting script passes through with only
# its one slot held — don't wrap battery scripts in a shared slot.
#
# The battery scripts self-acquire (ci-local.sh: exclusive;
# test-fast.sh: shared), so anything going through the standard entry
# points is queued automatically; use this wrapper directly for raw
# `cargo build` / `cargo nextest` invocations in lanes.
set -euo pipefail

LOCK_DIR="${CAD_SLOT_DIR:-$HOME/.local/share/cad-work/locks}"
# Shared-mode width: how many one-slot holders may run concurrently.
# 1 (mutex) per the 2026-08-06 experiment — see header. Max 2.
WIDTH="${CAD_SLOT_WIDTH:-1}"
case "$WIDTH" in
  1|2) ;;
  *) echo "with-build-slot: CAD_SLOT_WIDTH must be 1 or 2, got '$WIDTH'" >&2; exit 2 ;;
esac
MODE=shared
TRY=0
WAIT_MAX=0   # 0 = forever
EXPRESS_MAX=600
EXPRESS_SECS=$EXPRESS_MAX
WANT_X=0
WANT_EXPRESS=0

while [ $# -gt 0 ]; do
  case "$1" in
    -x) MODE=exclusive; WANT_X=1; shift ;;
    --express)
      MODE=express; WANT_EXPRESS=1; shift
      if [ $# -gt 0 ] && [[ "$1" =~ ^[0-9]+$ ]]; then EXPRESS_SECS="$1"; shift; fi
      ;;
    -n) TRY=1; shift ;;
    -w) WAIT_MAX="${2:?-w needs seconds}"; shift 2 ;;
    --) shift; break ;;
    *)  echo "usage: with-build-slot.sh [-x | --express [SECS]] [-n] [-w SECS] -- <command> [args...]" >&2; exit 2 ;;
  esac
done
[ $# -gt 0 ] || { echo "with-build-slot: no command given" >&2; exit 2; }

if [ "$WANT_X" -eq 1 ] && [ "$WANT_EXPRESS" -eq 1 ]; then
  echo "with-build-slot: -x is incompatible with --express — a battery is never a short job; take the main mutex" >&2
  exit 2
fi
if [ "$WANT_EXPRESS" -eq 1 ]; then
  if [ "$EXPRESS_SECS" -lt 1 ] || [ "$EXPRESS_SECS" -gt "$EXPRESS_MAX" ]; then
    echo "with-build-slot: --express budget ${EXPRESS_SECS}s out of range (1..${EXPRESS_MAX}s hard max) — long jobs take the main mutex" >&2
    exit 2
  fi
fi

# Already inside a slot (nested invocation): pass through — but an
# express declaration still self-enforces its budget even when nested.
if [ -n "${BUILD_SLOT_HELD:-}" ]; then
  if [ "$MODE" = express ]; then
    exec timeout --kill-after=10 "$EXPRESS_SECS" "$@"
  fi
  exec "$@"
fi

mkdir -p "$LOCK_DIR"

# A dangling core.hooksPath means the pre-push fmt hook is silently OFF —
# git says nothing when the configured directory is missing. Warn loudly;
# the fix is `git config core.hooksPath local-scripts/hooks` (or re-create
# the lane with new-lane.sh). The 2026-08-11 scripts/->local-scripts/
# migration self-heal that lived here was retired on schedule; see
# docs/LOCAL-BUILD-PERF.md §6 and this file's git history.
if git rev-parse --git-dir >/dev/null 2>&1; then
  _hp=$(git config core.hooksPath 2>/dev/null || true)
  if [ -n "$_hp" ] && [ ! -d "$_hp" ]; then
    echo "with-build-slot: WARNING core.hooksPath=$_hp does not exist — the pre-push fmt hook is NOT running (fix: git config core.hooksPath local-scripts/hooks)" >&2
  fi
fi

# IF A COMPILER-CACHE DAEMON IS EVER ADDED TO THE BUILD PATH, PRE-START IT
# HERE — before the lock fds are opened below. A daemon auto-started by a
# slot-wrapped `cargo build` inherits fds 7/8/9 and keeps the flock held
# forever after its parent dies, wedging the machine-wide mutex behind a
# holder file naming a dead pid (the fd-inheritance lock leak, memories/
# agent-lane-operations.md). sccache was briefly the machine rustc-wrapper
# on 2026-08-11 and needed exactly that guard; it was reverted the same day
# (it forces CARGO_INCREMENTAL=0, which measured far slower on the
# edit-rebuild loop — the figures and their disposition live in
# local-scripts/setup-build-env.sh, once), so the guard is gone
# with it rather than left running a daemon nothing uses.

# Slot fds: express -> fd 7, slot 1 -> fd 8, slot 2 -> fd 9. Opened
# once; flock -n per attempt. The fds (and locks) are inherited across
# the final exec and by the command's children, so the slot frees only
# when the whole process tree is gone.
exec 7>"$LOCK_DIR/express.lock" 8>"$LOCK_DIR/slot-1.lock" 9>"$LOCK_DIR/slot-2.lock"

# Holder files are best-effort REPORTING, never correctness (flock is
# the truth). #235 measured them lying: a dead pid's holder file
# polluted every busy message for a day. So on print we verify the
# recorded pid is alive (dead => flock on that slot is free — the
# kernel released it) and compute the hold duration from the recorded
# epoch.
#
# Every claim below is RELATIVE TO THE READER'S SEAT and STATES ONLY
# VERIFIED FACTS. Reader-relative because each line is read by the one
# waiter it is printed for, and a claim about a slot that waiter's loop
# never polls invites acting on a slot the request cannot take.
# Fact-only because a dead recorded pid does NOT mean the flock is
# free: the lock fds are inherited, so a child of a dead holder can
# hold the flock with no record (the daemon note above the lock fds) —
# and these lines print exactly when this request just FAILED to take
# every slot it is currently trying, so "free — safe to acquire" is
# provably wrong at its own print site whenever it matters.
shared_polls_slot2() {
  # THE width test, single home: a shared request polls slot-2 iff the
  # width admits a second one-slot holder. The acquire loop and the
  # status annotations must both consult this, or the two spellings
  # drift and the status lines go back to describing slots their
  # reader never polls.
  [ "$WIDTH" -ge 2 ]
}
polls_slot() {  # slot-name -> 0 iff THIS request's acquire loop ever polls it
  case "$1" in
    express) [ "$MODE" = express ] ;;
    slot-1)  [ "$MODE" = shared ] || [ "$MODE" = exclusive ] ;;
    slot-2)  [ "$MODE" = exclusive ] || { [ "$MODE" = shared ] && shared_polls_slot2; } ;;
    *) return 1 ;;
  esac
}
# Exclusive mode acquires its two slots in sequence; PHASE names the
# slot its wait loop is currently trying, so a status line can tell
# "just found busy" from "not attempted yet". The other modes try all
# their slots on every iteration.
PHASE=1
tried_slot_now() {  # slot-name -> 0 iff the current wait loop just failed it
  case "$MODE" in
    express)   [ "$1" = express ] ;;
    shared)    [ "$1" = slot-1 ] || { [ "$1" = slot-2 ] && shared_polls_slot2; } ;;
    exclusive) [ "$1" = "slot-$PHASE" ] ;;
    *) return 1 ;;
  esac
}
describe_holder() {  # holder-file -> annotated description on stdout
  local f="$1" slot line pid epoch now dur wc note=""
  slot=$(basename "$f" .holder)
  line=$(cat "$f" 2>/dev/null) || return 0
  [ -n "$line" ] || return 0
  pid=$(sed -n 's/^pid \([0-9][0-9]*\) .*/\1/p' <<<"$line")
  epoch=$(sed -n 's/.*(@\([0-9][0-9]*\)).*/\1/p' <<<"$line")
  if [ -n "$epoch" ]; then
    now=$(date +%s); dur=$((now - epoch))
    note=" [held $((dur / 60))m$((dur % 60))s]"
  fi
  if [ -n "$pid" ] && [ "$pid" = "$$" ]; then
    note="$note [held by this request itself (earlier exclusive slot)]"
  elif [ -n "$pid" ] && ! kill -0 "$pid" 2>/dev/null; then
    if tried_slot_now "$slot"; then
      note="$note [record is stale (pid $pid dead), yet this $MODE request just found $slot busy: an unrecorded process holds the flock — the inherited-fd leak; see the daemon note above the lock fds]"
    elif polls_slot "$slot"; then
      note="$note [record is stale (pid $pid dead); this $MODE request polls $slot but has not tried it yet]"
    else
      wc=""; [ "$MODE" = shared ] && wc=" at width $WIDTH"
      note="$note [record is stale (pid $pid dead); this $MODE request never polls $slot$wc]"
    fi
  elif ! polls_slot "$slot"; then
    # A live holder of a slot this request never polls is context, not
    # a wait cause — say so, or the reader misattributes the wait.
    note="$note [not a slot this $MODE request polls — not what blocks it]"
  fi
  printf '%s: %s%s' "$slot" "$line" "$note"
}
holders() {  # best-effort names of current holders, for wait messages
  local f s out=""
  for f in "$LOCK_DIR"/slot-*.holder "$LOCK_DIR"/express.holder; do
    [ -s "$f" ] || continue
    out="$out${out:+; }$(describe_holder "$f")"
  done
  # This prints only after the request failed to take every slot it is
  # currently trying, so a just-tried slot with no record is a real
  # blocker nobody wrote down — name it, or the whole message can be
  # about slots that do not matter to the reader.
  for s in slot-1 slot-2 express; do
    tried_slot_now "$s" || continue
    [ -s "$LOCK_DIR/$s.holder" ] && continue
    out="$out${out:+; }$s: blocks this $MODE request; no holder on record"
  done
  echo "${out:-none on record}"
}
note_holder() {  # slot-name (slot-1 | slot-2 | express)
  local slot="$1"; shift
  echo "pid $$ since $(date +%H:%M:%S) (@$(date +%s)): $*" > "$LOCK_DIR/$slot.holder" 2>/dev/null || true
}

try_slot() {  # fd -> 0 if acquired
  flock --nonblock "$1"
}

WAITED=0
wait_tick() {
  if [ "$TRY" -eq 1 ]; then
    echo "with-build-slot: busy ($(holders)) — exiting 75 (-n)" >&2
    exit 75
  fi
  if [ "$WAIT_MAX" -gt 0 ] && [ "$WAITED" -ge "$WAIT_MAX" ]; then
    echo "with-build-slot: still busy after ${WAITED}s ($(holders)) — exiting 75 (-w $WAIT_MAX)" >&2
    exit 75
  fi
  if [ $((WAITED % 60)) -eq 0 ]; then
    echo "with-build-slot: waiting for a slot (${WAITED}s; holding: $(holders))" >&2
  fi
  sleep 5
  WAITED=$((WAITED + 5))
}

HELD=""
if [ "$MODE" = express ]; then
  # Express jobs contend ONLY for the express slot; they never touch
  # the main mutex, so a battery and an express job run concurrently.
  while :; do try_slot 7 && break; wait_tick; done
  note_holder express "express(${EXPRESS_SECS}s): $*"
elif [ "$MODE" = shared ]; then
  # Poll the slot(s) within WIDTH instead of blocking on one: at
  # width 2, blocking on slot 1 while slot 2 frees first would
  # serialize needlessly.
  while :; do
    if try_slot 8; then HELD=1; break; fi
    if shared_polls_slot2 && try_slot 9; then HELD=2; break; fi
    wait_tick
  done
  note_holder "slot-$HELD" "shared: $*"
else
  while :; do try_slot 8 && break; wait_tick; done
  note_holder slot-1 "exclusive(1/2): $*"
  PHASE=2
  while :; do try_slot 9 && break; wait_tick; done
  note_holder slot-1 "exclusive: $*"; note_holder slot-2 "exclusive: $*"
fi

# Belt-and-suspenders OOM guard: if available memory is unusually low
# (something outside the slot system is eating RAM), wait briefly for it
# to recover rather than launching into a likely OOM kill.
MEM_FLOOR_KB=2000000
for _ in 1 2 3 4 5 6; do
  avail=$(awk '/MemAvailable/{print $2}' /proc/meminfo)
  [ "$avail" -ge "$MEM_FLOOR_KB" ] && break
  echo "with-build-slot: MemAvailable ${avail}kB < ${MEM_FLOOR_KB}kB — waiting 30s for recovery" >&2
  sleep 30
done

# No CARGO_BUILD_JOBS cap: the slot experiment measured -j4 strictly
# worse both solo (52s vs 33s) and paired — full -j inside the slot,
# serialization between slots, is the fast configuration.
# UNGUARDED, and unguardably so: this pair is a reading on the local box
# (docs/LOCAL-BUILD-PERF.md §0), and nothing in this repo runs this script
# — hosted CI never takes a build slot, so no gate can go red when the
# ordering flips on different core counts. Re-take it on the box before
# reinstating a cap; the absence of a cap is not evidence the pair still
# holds.
export BUILD_SLOT_HELD="$MODE"
if [ "$MODE" = express ]; then
  # Self-enforcing budget: the job runs under its own declared timeout,
  # so a job that lied about being short is killed by its declaration
  # (exit 124). timeout(1) stays the fd-holding parent, so the express
  # flock frees when the whole tree is gone, as with the main slots.
  exec timeout --kill-after=10 "$EXPRESS_SECS" "$@"
fi
exec "$@"
