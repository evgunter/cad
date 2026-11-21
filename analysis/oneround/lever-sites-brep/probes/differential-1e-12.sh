#!/usr/bin/env bash
# The differential's second row, eps 1e-12, on the binaries differential.sh built. NOT the whole
# suite: every test whose decision log shows a moved margin value at eps 1e-9 (compare.py's second
# list, 99 tests, the slow-set ones among them), every test of geom-brep and of step-import, and the
# mesh probe the brief names. The filter is filter-1e-12.txt, written from the 1e-9 comparison.
# (A whole kernel row at 1e-12 was started and stopped at 5 499 of 12 120 tests on the head after 55
# minutes: the sweep suites run several times slower at this eps.)
set -u
export CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only CARGO_PROFILE_TEST_STRIP=debuginfo
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_LTO=off CARGO_PROFILE_TEST_LTO=off CARGO_INCREMENTAL=0
SEED=0x5eed0000c0ffee01; OUT=/home/user/diffrun; eps=1e-12
for tree in ihead base; do
  cd /home/user/wt-$tree || exit 1
  export CARGO_TARGET_DIR=/home/user/tgt-$tree RUSTC_WORKSPACE_WRAPPER=/home/user/wt-$tree/scripts/rustc-test-crates-opt0.sh
  mkdir -p "$OUT/klog-$tree-$eps"
  KLOG_DIR="$OUT/klog-$tree-$eps" CAD_FUZZ_SEED=$SEED CAD_TOLERANCE_EPS=$eps \
    cargo nextest run --profile ci --workspace --ignore-default-filter --no-fail-fast --status-level all \
    -E "$(cat $OUT/filter-1e-12.txt)" > "$OUT/run-$tree-$eps.log" 2>&1
  echo "ran $tree $eps rc=$? $(date -u +%T)" >> "$OUT/status"
done
echo DONE >> "$OUT/status"
