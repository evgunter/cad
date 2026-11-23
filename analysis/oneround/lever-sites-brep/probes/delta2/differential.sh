#!/usr/bin/env bash
# Second delta: merge base 91e24317 against the frozen head bb12af18, the decision logger with margin
# hashes, ci.yml's build profile, one fuzz seed. Scope: geom-brep and step-import, their slow sets
# included, eps 1e-9 then 1e-12.
set -u
export CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only CARGO_PROFILE_TEST_STRIP=debuginfo
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_LTO=off CARGO_PROFILE_TEST_LTO=off CARGO_INCREMENTAL=0
OUT=/home/user/diffrun; PK="-p geom-brep -p step-import"
for tree in ihead base; do
  cd /home/user/wt-$tree || exit 1
  export CARGO_TARGET_DIR=/home/user/tgt-$tree RUSTC_WORKSPACE_WRAPPER=/home/user/wt-$tree/scripts/rustc-test-crates-opt0.sh
  cargo nextest run --no-run --build-jobs 2 --profile ci $PK > "$OUT/build-$tree.log" 2>&1 || { echo "BUILD FAILED $tree" >> "$OUT/status"; exit 1; }
  echo "built $tree $(date -u +%T)" >> "$OUT/status"
done
for eps in 1e-9 1e-12; do for tree in ihead base; do
  cd /home/user/wt-$tree || exit 1
  export CARGO_TARGET_DIR=/home/user/tgt-$tree RUSTC_WORKSPACE_WRAPPER=/home/user/wt-$tree/scripts/rustc-test-crates-opt0.sh
  mkdir -p "$OUT/klog-$tree-$eps"
  KLOG_DIR="$OUT/klog-$tree-$eps" CAD_FUZZ_SEED=0x5eed0000c0ffee01 CAD_TOLERANCE_EPS=$eps \
    cargo nextest run --profile ci $PK --ignore-default-filter --no-fail-fast --status-level all > "$OUT/run-$tree-$eps.log" 2>&1
  echo "ran $tree $eps rc=$? $(date -u +%T)" >> "$OUT/status"
done; done
echo DONE >> "$OUT/status"
