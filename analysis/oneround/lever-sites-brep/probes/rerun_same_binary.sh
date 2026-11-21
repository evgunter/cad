#!/usr/bin/env bash
# Run each named test twice on ONE tree's binaries with the differential's env, and say whether its
# decision log repeats. Usage: rerun_same_binary.sh <ihead|base> <eps> <nextest filter expr>
tree=$1; eps=$2; expr=$3
export CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only CARGO_PROFILE_TEST_STRIP=debuginfo
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_LTO=off CARGO_PROFILE_TEST_LTO=off CARGO_INCREMENTAL=0
export CARGO_TARGET_DIR=/home/user/tgt-$tree RUSTC_WORKSPACE_WRAPPER=/home/user/wt-$tree/scripts/rustc-test-crates-opt0.sh
cd /home/user/wt-$tree || exit 1
for n in 1 2; do
  rm -rf /home/user/diffrun/rerun-$tree-$n; mkdir -p /home/user/diffrun/rerun-$tree-$n
  KLOG_DIR=/home/user/diffrun/rerun-$tree-$n CAD_FUZZ_SEED=0x5eed0000c0ffee01 CAD_TOLERANCE_EPS=$eps \
    cargo nextest run --profile ci --workspace --ignore-default-filter --no-fail-fast -E "$expr" > /home/user/diffrun/rerun-$tree-$n.log 2>&1
done
for f in /home/user/diffrun/rerun-$tree-1/*; do
  b=$(basename "$f"); if cmp -s "$f" "/home/user/diffrun/rerun-$tree-2/$b"; then echo "REPEATS   $b"; else echo "DIFFERS between two runs of one binary: $b"; fi
done
