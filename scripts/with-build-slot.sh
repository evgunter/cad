#!/usr/bin/env bash
# with-build-slot.sh — machine-wide build-slot semaphore for agent lanes.
#
#   scripts/with-build-slot.sh [-x] [-n] [-w SECS] -- <command> [args...]
#
# Replaces the soft "two lanes" convention (and the retired
# cargo-slots.txt registry) with real locks: TWO slot lockfiles under
# ~/.local/share/cad-work/locks/ bound the number of concurrent heavy
# cargo operations machine-wide (10 GB WSL2 RAM ceiling — see
# memories/agent-lane-operations.md). Any number of agents may be ALIVE;
# only their builds/batteries queue here.
#
# Modes:
#   (default)  acquire ONE slot — ordinary builds / fast test runs.
#              Two may run at once; each defaults CARGO_BUILD_JOBS=4 so
#              a concurrent pair shares the 8 cores fairly (override by
#              exporting CARGO_BUILD_JOBS yourself).
#   -x         acquire BOTH slots (exclusive) — full batteries and gate
#              runs. Two concurrent full batteries have OOM-killed tests
#              before (bare "Terminated" rows); exclusivity is cheaper
#              than a wasted, misleading run.
#   -n         non-blocking: if the slot(s) are busy, exit 75
#              (EX_TEMPFAIL) immediately instead of waiting. Agents can
#              try -n first and fall back to a blocking call — useful
#              because a blocking wait can eat a Bash tool call's
#              10-minute cap.
#   -w SECS    give up (exit 75) after SECS of waiting. Default: wait
#              forever, printing a status line every 60s naming the
#              current holders.
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
MODE=shared
TRY=0
WAIT_MAX=0   # 0 = forever

while [ $# -gt 0 ]; do
  case "$1" in
    -x) MODE=exclusive; shift ;;
    -n) TRY=1; shift ;;
    -w) WAIT_MAX="${2:?-w needs seconds}"; shift 2 ;;
    --) shift; break ;;
    *)  echo "usage: with-build-slot.sh [-x] [-n] [-w SECS] -- <command> [args...]" >&2; exit 2 ;;
  esac
done
[ $# -gt 0 ] || { echo "with-build-slot: no command given" >&2; exit 2; }

# Already inside a slot (nested invocation): pass through.
if [ -n "${BUILD_SLOT_HELD:-}" ]; then
  exec "$@"
fi

mkdir -p "$LOCK_DIR"
# Slot fds: slot 1 -> fd 8, slot 2 -> fd 9. Opened once; flock -n per
# attempt. The fds (and locks) are inherited across the final exec and
# by the command's children, so the slot frees only when the whole
# process tree is gone.
exec 8>"$LOCK_DIR/slot-1.lock" 9>"$LOCK_DIR/slot-2.lock"

holders() {  # best-effort names of current holders, for wait messages
  cat "$LOCK_DIR"/slot-*.holder 2>/dev/null | paste -sd';' - || true
}
note_holder() {  # slot-number
  echo "pid $$ since $(date +%H:%M:%S): $*" > "$LOCK_DIR/slot-$1.holder" 2>/dev/null || true
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
if [ "$MODE" = shared ]; then
  # Poll both slots instead of blocking on one: blocking on slot 1
  # while slot 2 frees first would serialize needlessly.
  while :; do
    if try_slot 8; then HELD=1; break; fi
    if try_slot 9; then HELD=2; break; fi
    wait_tick
  done
  note_holder "$HELD" "shared: $*"
else
  while :; do try_slot 8 && break; wait_tick; done
  note_holder 1 "exclusive(1/2): $*"
  while :; do try_slot 9 && break; wait_tick; done
  note_holder 1 "exclusive: $*"; note_holder 2 "exclusive: $*"
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

export BUILD_SLOT_HELD="$MODE"
if [ "$MODE" = shared ]; then
  # Fair CPU split when two shared slots run concurrently (8 cores).
  # Tuned by the 2026-08-06 slot experiment; override by pre-setting.
  export CARGO_BUILD_JOBS="${CARGO_BUILD_JOBS:-4}"
fi
exec "$@"
