#!/usr/bin/env bash
# Fast local test runs: whole-graph opt-1 with debug-assertions and
# overflow-checks still ON (fail-loud postconditions keep their teeth).
#
# opt-2 -> opt-1 ON 2026-08-25, following ci.yml's two archive jobs. This
# script's contract is "reproduce the hosted configuration locally", so it
# tracks that setting rather than holding an opinion of its own. It is also
# the better trade for what this script is FOR: a three-arm sweep that day
# (3489 tests, all green at every level) put opt-1 within 3% of opt-2's
# execution for 58% of its build penalty, and a local loop pays the build
# far more often than CI does. ci.yml's OPT LEVEL note is the argument.
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
# This IS the CI configuration for the two nextest-archive jobs. It was
# opt-2 from #449 (2026-08-12), which reversed the #52/#53 "opt 2 is
# net-slower on CI" verdict once its premises expired — #179/#387 took the
# workspace from 261 test binaries to 14, and test execution grew to ~79%
# of run wall; hosted result: critical path 1065s -> 840s, billed ~137 ->
# ~72 min. It became opt-1 on 2026-08-25, when the same arithmetic was
# finally applied to the level nobody had measured. Full write-ups:
# docs/GENERICS-BUILD-COST.md and docs/perf-data/opt-level/.
#
# THE TIMINGS ABOVE ARE opt-0 -> opt-2 READINGS and are left as taken.
# NOTHING RE-TAKES THEM, and nothing is scheduled to: GENERICS-BUILD-COST
# and perf-data/opt-level/ are dated write-ups, and the CI registers that
# do refresh on a schedule (rebuild latency, tess-budget) do not cover this
# measurement. So no guard can go red when the suite grows out from under a
# ratio — one of these already did, and says so in place two paragraphs up,
# which is the pattern to copy rather than the figure. What this script
# SETS is tracked to ci.yml's opt level and to nothing here, so a drifted
# timing costs a reader accuracy and costs the script nothing.
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
export CARGO_PROFILE_DEV_OPT_LEVEL=1
export CARGO_PROFILE_TEST_OPT_LEVEL=1
exec cargo test "$@"
