#!/usr/bin/env bash
# The reviewer's differential (C3): merge base b5c3fb4b against the frozen head 37da28e8, each with
# the same decision logger (klog.patch.py), one fuzz seed, eps 1e-9 and 1e-12, the slow set included.
# Usage: differential.sh  (expects /home/user/wt-ihead and /home/user/wt-base patched)
set -u
SEED=0x5eed0000c0ffee01
OUT=/home/user/diffrun
for tree in ihead base; do
  cd /home/user/wt-$tree || exit 1
  export CARGO_TARGET_DIR=/home/user/tgt-$tree
  cargo nextest run --no-run --profile ci --workspace > "$OUT/build-$tree.log" 2>&1 || { echo "BUILD FAILED $tree" >> "$OUT/status"; exit 1; }
  echo "built $tree $(date -u +%T)" >> "$OUT/status"
done
for eps in 1e-9 1e-12; do
  for tree in ihead base; do
    cd /home/user/wt-$tree || exit 1
    export CARGO_TARGET_DIR=/home/user/tgt-$tree
    mkdir -p "$OUT/klog-$tree-$eps"
    KLOG_DIR="$OUT/klog-$tree-$eps" CAD_FUZZ_SEED=$SEED CAD_TOLERANCE_EPS=$eps \
      cargo nextest run --profile ci --workspace --ignore-default-filter --no-fail-fast --status-level all \
      > "$OUT/run-$tree-$eps.log" 2>&1
    echo "ran $tree $eps rc=$? $(date -u +%T)" >> "$OUT/status"
  done
done
echo DONE >> "$OUT/status"
