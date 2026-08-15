#!/usr/bin/env bash
# Temporary investigation script (NOT for merge): battery 3b.
# Execution rows only — the compile rows (A1-A4) already ran.
#
# Uses `cargo nextest run`, NOT `cargo test`: several suites need
# process-per-test isolation (`Tolerance` is a OnceLock, so
# geom-core's tolerance_init suite fails under cargo test's
# one-process-many-threads model). CI runs nextest for this reason.
#
# NOTE ON TRANSFER: nextest here uses 8 threads; CI runners have 2.
# Absolute seconds do NOT transfer to CI — the opt-0 : opt-2 RATIO is
# the number this battery is for.
set -u
cd "$(dirname "$0")/.."

t() { local label="$1"; shift; local s e
  s=$(date +%s.%N); "$@" >/tmp/bt-opt2-last.log 2>&1; local rc=$?; e=$(date +%s.%N)
  printf '%-50s %8.1fs  (rc=%d)\n' "$label" "$(echo "$e - $s" | bc)" "$rc"
  grep -hoE "Summary \[.*" /tmp/bt-opt2-last.log | tail -1
}

echo "### EXECUTION: cargo nextest run, opt-0 (CI today) vs opt-2"

unset CARGO_PROFILE_DEV_OPT_LEVEL CARGO_PROFILE_TEST_OPT_LEVEL
cargo nextest run --workspace --no-run >/dev/null 2>&1
t "C1. nextest --workspace                opt-0"  cargo nextest run --workspace --no-fail-fast
cargo nextest run --workspace --features interval --no-run >/dev/null 2>&1
t "C2. nextest --workspace --features interval opt-0" \
   cargo nextest run --workspace --features interval --no-fail-fast

export CARGO_PROFILE_DEV_OPT_LEVEL=2
export CARGO_PROFILE_TEST_OPT_LEVEL=2
cargo nextest run --workspace --no-run >/dev/null 2>&1
t "C3. nextest --workspace                opt-2"  cargo nextest run --workspace --no-fail-fast
cargo nextest run --workspace --features interval --no-run >/dev/null 2>&1
t "C4. nextest --workspace --features interval opt-2" \
   cargo nextest run --workspace --features interval --no-fail-fast
