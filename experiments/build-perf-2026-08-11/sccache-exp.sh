#!/usr/bin/env bash
# sccache-exp.sh — does sccache pay for itself on this box?
#
# THE QUESTION: sccache is content-addressed and shared across lanes, so
# it should make a NEW LANE's cold build (new-lane.sh clones fresh, and
# each of ~10 lanes carries its own target/) nearly free. But it requires
# CARGO_INCREMENTAL=0, which taxes the steady-state edit-rebuild loop
# inside a lane. This measures both sides of that trade.
#
# Baseline for phases 1-2 is config C's `cold` row from build-exp.sh
# (mold + line-tables-only, no sccache); for phase 3 it is C's
# `touch geom-core` row. Runs in config C so only sccache varies.
set -uo pipefail
cd "$(dirname "$0")/cad"

export RUSTFLAGS="-C link-arg=-B$HOME/.local/libexec/mold"
export CARGO_PROFILE_DEV_DEBUG=line-tables-only
export CARGO_PROFILE_TEST_DEBUG=line-tables-only
export RUSTC_WRAPPER=sccache
export CARGO_INCREMENTAL=0
export SCCACHE_CACHE_SIZE=20G

LOG=../sccache-exp.log
: > "$LOG"
say() { echo "$*" | tee -a "$LOG"; }
run() {
  local label="$1"; shift
  local t0 t1
  t0=$(date +%s); "$@" >>"$LOG.raw" 2>&1; local rc=$?; t1=$(date +%s)
  say "$(printf '%-34s %5ds  (rc=%d)' "$label" "$((t1 - t0))" "$rc")"
}
stats() { sccache --show-stats | grep -E 'Compile requests|Cache hits|Cache misses|hits rate' | sed 's/^/    /' | tee -a "$LOG"; }

sccache --stop-server >/dev/null 2>&1 || true
rm -rf "$HOME/.cache/sccache"
sccache --start-server >/dev/null 2>&1 || true

say "sccache experiment (config C + sccache, CARGO_INCREMENTAL=0)"

cargo clean >/dev/null 2>&1
run "1 cold, cache EMPTY (populating)" cargo build --workspace --all-targets
stats

# The new-lane scenario: same sources, no target/ dir, warm shared cache.
sccache --zero-stats >/dev/null 2>&1
cargo clean >/dev/null 2>&1
run "2 cold, cache WARM (= new lane)"  cargo build --workspace --all-targets
stats

# The steady-state tax: no incremental, so this is a full recompile of
# geom-core's reverse closure served from cache only where inputs match.
sccache --zero-stats >/dev/null 2>&1
touch crates/geom-core/src/lib.rs
run "3 touch geom-core (no incr)"     cargo build --workspace --all-targets
stats

say "cache dir: $(du -sh "$HOME/.cache/sccache" 2>/dev/null | cut -f1)"
say "target/:   $(du -sh target 2>/dev/null | cut -f1)"
say "done"
