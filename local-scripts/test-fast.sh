#!/usr/bin/env bash
# Fast local test runs: whole-graph opt-2 with debug-assertions and
# overflow-checks still ON (fail-loud postconditions keep their teeth).
# Measured 2026-08-12: full workspace suite 558.7s -> 130.0s (2791 tests,
# 4.30x), interval config 691.7s -> 301.6s (3001 tests, 2.29x). First run
# pays a one-time rebuild (~7 min cold); after that the opt-2 artifacts
# are cached and only your diff recompiles.
#
# SUPERSEDES the original 2026-07-21 row ("75s -> 4.9s, 807 tests", 15x).
# That figure is stale, not wrong-at-the-time: the suite has since grown
# to 2791 tests and 558.7s at opt-0 — 3.5x the test count but 7.4x the
# wall — so the tests added since are less compute-bound than the ones
# that motivated it. Quoting 15x today overstates the win by ~3.5x.
#
# This IS now the CI configuration for the two nextest-archive jobs
# (#449, 2026-08-12): the #52/#53 "opt 2 is net-slower on CI" verdict was
# reversed once its premises expired — #179/#387 took the workspace from
# 261 test binaries to 14, and test execution grew to ~79% of run wall.
# Hosted result: critical path 1065s -> 840s, billed ~137 -> ~72 min.
# Full write-up: docs/GENERICS-BUILD-COST.md.
#
# Usage: local-scripts/test-fast.sh [cargo test args...]
#   e.g. local-scripts/test-fast.sh --workspace
#        local-scripts/test-fast.sh --workspace --features interval
#        CAD_TOLERANCE_EPS=1e-9 local-scripts/test-fast.sh --workspace
set -euo pipefail
# Queue through the machine-wide build-slot semaphore (width-1 mutex
# by default — measured faster than concurrent; see with-build-slot.sh).
if [ -z "${BUILD_SLOT_HELD:-}" ]; then
  exec "$(dirname "$0")/with-build-slot.sh" -- "$0" "$@"
fi
export CARGO_PROFILE_DEV_OPT_LEVEL=2
export CARGO_PROFILE_TEST_OPT_LEVEL=2
exec cargo test "$@"
