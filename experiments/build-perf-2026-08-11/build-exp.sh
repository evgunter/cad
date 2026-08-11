#!/usr/bin/env bash
# build-exp.sh — local linker/debuginfo A/B, in the #230 slot-experiment style.
#
# Two configs, same tree, same workload, same machine:
#   A  baseline            GNU ld, full DWARF (today's local default)
#   C  mold + thin debug   -C link-arg=-B<mold libexec>, debug=line-tables-only
#
# Three phases per config:
#   cold      cargo clean, then `cargo build --workspace --all-targets`
#   touch-lib touch geom-core/src/lib.rs, rebuild  (100% test-binary invalidation
#             per the bazel-verdict blast-radius table — the worst realistic PR)
#   touch-test touch topo/tests/all.rs, rebuild    (one binary: LINK-dominated,
#             which is what isolates the linker's contribution)
set -uo pipefail
cd "$(dirname "$0")/cad"

MOLD_B="-C link-arg=-B$HOME/.local/libexec/mold"
LOG=../exp.log
: > "$LOG"

say() { echo "$*" | tee -a "$LOG"; }
run() {  # label -- command...
  local label="$1"; shift
  local t0 t1
  t0=$(date +%s)
  "$@" >>"$LOG.raw" 2>&1
  local rc=$?
  t1=$(date +%s)
  say "$(printf '%-28s %5ds  (rc=%d)' "$label" "$((t1 - t0))" "$rc")"
}

phase() {  # config-name
  local cfg="$1"
  say ""
  say "=== config $cfg  RUSTFLAGS='${RUSTFLAGS:-}'  DEBUG='${CARGO_PROFILE_DEV_DEBUG:-full}'"
  cargo clean >/dev/null 2>&1
  run "$cfg cold (all-targets)"   cargo build --workspace --all-targets
  touch crates/geom-core/src/lib.rs
  run "$cfg touch geom-core"      cargo build --workspace --all-targets
  touch crates/topo/tests/all.rs
  run "$cfg touch topo/tests"     cargo build --workspace --all-targets
  say "$cfg target/ size: $(du -sh target 2>/dev/null | cut -f1)"
}

say "build-exp on $(nproc) cores, $(awk '/MemTotal/{printf "%.1f GB", $2/1048576}' /proc/meminfo)"
say "tree: $(git rev-parse --short HEAD) (+ step-import aggregation)"

unset RUSTFLAGS CARGO_PROFILE_DEV_DEBUG CARGO_PROFILE_TEST_DEBUG
phase A

export RUSTFLAGS="$MOLD_B"
export CARGO_PROFILE_DEV_DEBUG=line-tables-only
export CARGO_PROFILE_TEST_DEBUG=line-tables-only
phase C

say ""
say "done"
