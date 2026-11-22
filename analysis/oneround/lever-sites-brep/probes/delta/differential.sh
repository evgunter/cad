#!/usr/bin/env bash
# Delta differential: merge base 3900eb46 against the frozen head c65b9292, the same decision logger
# (klog.patch.py: counts AND a hash of every margin), ci.yml's build profile, one fuzz seed.
# Scope: the kernel packages below, the slow set included, eps 1e-9; then eps 1e-12 over
# geom-brep and step-import (the crates the delta's own changes live in) and whatever filter-1e-12.txt adds.
set -u
export CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only CARGO_PROFILE_TEST_STRIP=debuginfo
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_LTO=off CARGO_PROFILE_TEST_LTO=off CARGO_INCREMENTAL=0
SEED=0x5eed0000c0ffee01; OUT=/home/user/diffrun
PK="-p geom-brep -p step-import -p topo -p sweep -p mesh -p verbs -p step-export -p stl"
for tree in ihead base; do
  cd /home/user/wt-$tree || exit 1
  export CARGO_TARGET_DIR=/home/user/tgt-$tree RUSTC_WORKSPACE_WRAPPER=/home/user/wt-$tree/scripts/rustc-test-crates-opt0.sh
  cargo nextest run --no-run --build-jobs 2 --profile ci $PK > "$OUT/build-$tree.log" 2>&1 || { echo "BUILD FAILED $tree" >> "$OUT/status"; exit 1; }
  echo "built $tree $(date -u +%T)" >> "$OUT/status"
done
for tree in ihead base; do
  cd /home/user/wt-$tree || exit 1
  export CARGO_TARGET_DIR=/home/user/tgt-$tree RUSTC_WORKSPACE_WRAPPER=/home/user/wt-$tree/scripts/rustc-test-crates-opt0.sh
  mkdir -p "$OUT/klog-$tree-1e-9"
  KLOG_DIR="$OUT/klog-$tree-1e-9" CAD_FUZZ_SEED=$SEED CAD_TOLERANCE_EPS=1e-9 \
    cargo nextest run --profile ci $PK --ignore-default-filter --no-fail-fast --status-level all > "$OUT/run-$tree-1e-9.log" 2>&1
  echo "ran $tree 1e-9 rc=$? $(date -u +%T)" >> "$OUT/status"
done
echo DONE9 >> "$OUT/status"
