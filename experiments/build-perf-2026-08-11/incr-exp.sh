#!/usr/bin/env bash
# incr-exp.sh — what does `incremental = false` cost the steady-state loop?
#
# I enabled sccache machine-wide with CARGO_INCREMENTAL=0 (sccache cannot
# cache incremental compilations) WITHOUT measuring the tax on the most
# common agent operation: edit a crate, rebuild. This closes that gap.
#
# THE HYPOTHESIS WORTH TESTING: cargo only compiles WORKSPACE/path crates
# incrementally; registry dependencies are non-incremental regardless. If
# that holds, sccache still serves all ~225 deps with incremental ON, and
#   D = sccache + incremental=true
# dominates the config I actually shipped
#   C = sccache + incremental=false
# on both cold builds AND the edit-rebuild loop. Measured, not assumed.
#
# Per config: cold build, then two edit-rebuild rounds of different blast
# radius (geom-core = 100% of test binaries per the bazel-verdict table;
# topo = ~71%). Each touch is done TWICE, because incremental's whole
# point is that the second edit reuses the first one's cached analysis.
set -uo pipefail
cd "$(dirname "$0")/cad"

LOG=../incr-exp.log
: > "$LOG"
say() { echo "$*" | tee -a "$LOG"; }
run() {
  local label="$1"; shift
  local t0 t1 rc
  t0=$(date +%s); "$@" >>"$LOG.raw" 2>&1; rc=$?; t1=$(date +%s)
  say "$(printf '  %-34s %5ds  (rc=%d)' "$label" "$((t1 - t0))" "$rc")"
}
hits() { sccache --show-stats 2>/dev/null | awk '/^Cache hits +[0-9]/{h=$3} /^Cache misses +[0-9]/{m=$3} END{printf "    sccache: %s hits / %s misses\n", h, m}' | tee -a "$LOG"; }

phase() {
  local name="$1" incr="$2"
  export RUSTC_WRAPPER=sccache
  export CARGO_INCREMENTAL="$incr"
  say ""
  say "=== config $name  (CARGO_INCREMENTAL=$incr, sccache on)"

  sccache --zero-stats >/dev/null 2>&1
  cargo clean >/dev/null 2>&1
  run "cold (cache warm from before)" cargo build --workspace --all-targets
  hits

  # Edit-rebuild loop, the operation agents actually repeat all day.
  for round in 1 2; do
    sccache --zero-stats >/dev/null 2>&1
    printf '\n// incr-exp touch %s.%s\n' "$name" "$round" >> crates/geom-core/src/lib.rs
    run "edit geom-core, round $round" cargo build --workspace --all-targets
    hits
  done
  for round in 1 2; do
    sccache --zero-stats >/dev/null 2>&1
    printf '\n// incr-exp touch %s.%s\n' "$name" "$round" >> crates/topo/src/lib.rs
    run "edit topo, round $round" cargo build --workspace --all-targets
    hits
  done
  git checkout -- crates/geom-core/src/lib.rs crates/topo/src/lib.rs
  say "    incremental/: $(du -sh target/debug/incremental 2>/dev/null | cut -f1 || echo none)"
  say "    target/:      $(du -sh target 2>/dev/null | cut -f1)"
}

say "incremental tax experiment — is CARGO_INCREMENTAL=0 costing the edit loop?"
say "load at start: $(cut -d' ' -f1-3 /proc/loadavg)"

phase C 0
phase D 1

say ""
say "load at end: $(cut -d' ' -f1-3 /proc/loadavg)"
say "done"
