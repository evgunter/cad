#!/usr/bin/env bash
# Delta differential, eps 1e-12, on the binaries delta/differential.sh built. NOT the whole suite:
# every test whose margins moved at 1e-9 (compare.py's second list), all of geom-brep and step-import,
# and the inherited red the dispatch names. The filter is filter-1e-12.txt.
set -u
export CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only CARGO_PROFILE_TEST_STRIP=debuginfo
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_LTO=off CARGO_PROFILE_TEST_LTO=off CARGO_INCREMENTAL=0
OUT=/home/user/diffrun; PK="-p geom-brep -p step-import -p topo -p sweep -p mesh -p verbs -p step-export -p stl"
for tree in ihead base; do
  cd /home/user/wt-$tree || exit 1
  export CARGO_TARGET_DIR=/home/user/tgt-$tree RUSTC_WORKSPACE_WRAPPER=/home/user/wt-$tree/scripts/rustc-test-crates-opt0.sh
  mkdir -p "$OUT/klog-$tree-1e-12"
  KLOG_DIR="$OUT/klog-$tree-1e-12" CAD_FUZZ_SEED=0x5eed0000c0ffee01 CAD_TOLERANCE_EPS=1e-12 \
    cargo nextest run --profile ci $PK --ignore-default-filter --no-fail-fast --status-level all -E "$(cat $OUT/filter-1e-12.txt)" > "$OUT/run-$tree-1e-12.log" 2>&1
  echo "ran $tree 1e-12 rc=$? $(date -u +%T)" >> "$OUT/status"
done
echo DONE12 >> "$OUT/status"
