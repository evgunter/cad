#!/usr/bin/env bash
# Temporary investigation script (NOT for merge): battery 3.
# Re-tests the #52/#53 decision (opt-0 on CI) now that its two premises
# have changed: #179/#387 took the workspace from 261 to 14 test binaries,
# and test EXECUTION has grown to a large share of CI run wall. The share
# is NOT restated here — `local-scripts/test-fast.sh`'s header is its one
# home, beside the rest of that argument and beside the note saying which
# half of it a register re-takes. A second copy of a number nothing checks
# drifts from the first in silence, which is the whole subject of #681.
# Uses exactly local-scripts/test-fast.sh's configuration: whole-graph
# opt-2 with debug-assertions and overflow-checks still ON.
set -u
cd "$(dirname "$0")/.."

t() { local label="$1"; shift; local s e
  s=$(date +%s.%N); "$@" >/tmp/bt-opt-last.log 2>&1; local rc=$?; e=$(date +%s.%N)
  printf '%-56s %8.1fs  (rc=%d)\n' "$label" "$(echo "$e - $s" | bc)" "$rc"; }

echo "### COMPILE cost: opt-0 (CI today) vs opt-2 (test-fast.sh config)"
cargo clean >/dev/null 2>&1
t "A1. clean build --all-targets            opt-0"  cargo build --workspace --all-targets
cargo clean >/dev/null 2>&1
t "A2. clean build --all-targets --features interval opt-0" \
   cargo build --workspace --all-targets --features interval

export CARGO_PROFILE_DEV_OPT_LEVEL=2
export CARGO_PROFILE_TEST_OPT_LEVEL=2
cargo clean >/dev/null 2>&1
t "A3. clean build --all-targets            opt-2"  cargo build --workspace --all-targets
cargo clean >/dev/null 2>&1
t "A4. clean build --all-targets --features interval opt-2" \
   cargo build --workspace --all-targets --features interval
unset CARGO_PROFILE_DEV_OPT_LEVEL CARGO_PROFILE_TEST_OPT_LEVEL

echo
echo "### EXECUTION cost: same suite, both opt levels (binaries prebuilt above)"
cargo clean >/dev/null 2>&1
cargo build --workspace --all-targets >/dev/null 2>&1
t "B1. cargo test --workspace               opt-0"  cargo test --workspace
grep -hoE "test result: .*" /tmp/bt-opt-last.log | tail -3

export CARGO_PROFILE_DEV_OPT_LEVEL=2
export CARGO_PROFILE_TEST_OPT_LEVEL=2
cargo build --workspace --all-targets >/dev/null 2>&1
t "B2. cargo test --workspace               opt-2"  cargo test --workspace
grep -hoE "test result: .*" /tmp/bt-opt-last.log | tail -3
