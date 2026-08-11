#!/usr/bin/env bash
# cold-c.sh — config C cold build (mold + line-tables-only + sccache),
# comparable to build-exp.sh's "A cold (all-targets) 4189s" baseline.
#
# Phase 1 is the FIRST PAYER: empty cache, so it measures mold+thin-debug
# against the 4189 s baseline with sccache contributing nothing.
# Phase 2 is the number that decides how bad the rollout storm is: same
# sources, target/ wiped, cache warm — i.e. exactly what every other lane
# faces on its next build, and what new-lane.sh will cost from now on.
set -uo pipefail
cd "$(dirname "$0")/cad"

LOG=../cold-c.log
: > "$LOG"
say() { echo "$*" | tee -a "$LOG"; }
run() {
  local label="$1"; shift
  local t0 t1 rc
  t0=$(date +%s); "$@" >>"$LOG.raw" 2>&1; rc=$?; t1=$(date +%s)
  say "$(printf '%-36s %5ds  (rc=%d)' "$label" "$((t1 - t0))" "$rc")"
}
stats() { sccache --show-stats 2>/dev/null | grep -E 'Compile requests$|Cache hits +[0-9]|Cache misses|hits rate' | sed 's/^/    /' | tee -a "$LOG"; }

say "config C cold build  (baseline to beat: A cold = 4189s)"

sccache --zero-stats >/dev/null 2>&1
cargo clean >/dev/null 2>&1
run "C1 cold, cache EMPTY"  cargo build --workspace --all-targets
stats
say "    target/: $(du -sh target 2>/dev/null | cut -f1)"

sccache --zero-stats >/dev/null 2>&1
cargo clean >/dev/null 2>&1
run "C2 cold, cache WARM (= new lane)"  cargo build --workspace --all-targets
stats
say "    target/: $(du -sh target 2>/dev/null | cut -f1)"
say "    sccache dir: $(du -sh "$HOME/.cache/sccache" 2>/dev/null | cut -f1)"
say "done"
