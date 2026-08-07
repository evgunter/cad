#!/usr/bin/env bash
# Fast local test runs: whole-graph opt-2 with debug-assertions and
# overflow-checks still ON (fail-loud postconditions keep their teeth).
# Measured 2026-07-21: full workspace suite 75s -> 4.9s (807 tests),
# interval config 7.3s (920 tests). First run pays a one-time rebuild
# (~7-9 min cold); after that the opt-2 artifacts are cached and only
# your diff recompiles. Deliberately NOT the CI configuration: on CI the
# per-push rebuild of changed core crates at opt 2 costs more than the
# test-time it saves (#52/#53 timings).
#
# Usage: scripts/test-fast.sh [cargo test args...]
#   e.g. scripts/test-fast.sh --workspace
#        scripts/test-fast.sh --workspace --features interval
#        CAD_TOLERANCE_EPS=1e-9 scripts/test-fast.sh --workspace
set -euo pipefail
# Queue through the machine-wide build-slot semaphore (width-1 mutex
# by default — measured faster than concurrent; see with-build-slot.sh).
if [ -z "${BUILD_SLOT_HELD:-}" ]; then
  exec "$(dirname "$0")/with-build-slot.sh" -- "$0" "$@"
fi
export CARGO_PROFILE_DEV_OPT_LEVEL=2
export CARGO_PROFILE_TEST_OPT_LEVEL=2
exec cargo test "$@"
