#!/usr/bin/env bash
# The differential's second row: eps 1e-12, the kernel crates that reach the converted sites
# (geom-brep, step-import, topo, sweep, mesh, verbs, step-export, stl), the slow set included,
# on the binaries differential.sh built. editor-core, pncad, viewer and the tools are NOT in this row.
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
    -E 'package(geom-brep) | package(step-import) | package(topo) | package(sweep) | package(mesh) | package(verbs) | package(step-export) | package(stl)' \
    > "$OUT/run-$tree-$eps.log" 2>&1
  echo "ran $tree $eps rc=$? $(date -u +%T)" >> "$OUT/status"
done
echo DONE >> "$OUT/status"
