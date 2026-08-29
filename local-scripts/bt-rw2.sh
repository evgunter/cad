#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
# Is rw2_probes' 1425 s in the probe-lane run a regression or machine noise?
# It does not use the Probe scalar (prose only), so the feature should not
# touch it. Runs the SAME test on both lanes back to back, same conditions.
set -u
cd "$(dirname "$0")/.."
export CARGO_PROFILE_DEV_OPT_LEVEL=2 CARGO_PROFILE_TEST_OPT_LEVEL=2
F='rw2_probes::probe_round_trip_bit_identity_and_reorder'

echo "### default lane (probe OFF)"
cargo nextest run -p step-import --no-fail-fast -E "test(=$F)" 2>&1 | tail -3
echo
echo "### probe lane (probe ON)"
cargo nextest run -p step-import --features probe --no-fail-fast -E "test(=$F)" 2>&1 | tail -3
echo
echo "### load at end"
uptime
