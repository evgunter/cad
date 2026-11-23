#!/usr/bin/env bash
# MEASUREMENT ONLY: the census population under PR 7329 (P) and the mutant (M).
set -u
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/wm-target CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1
export RUSTC_WORKSPACE_WRAPPER=$PWD/scripts/rustc-test-crates-opt0.sh
W=/home/user/wm-work
PKGS=$(awk '{print "-p " $1}' $W/binaries.txt | sort -u | tr '\n' ' ')
echo "BUILD start $(date +%s)" >> $W/progress
cargo nextest run $PKGS --no-run > $W/build.out 2>&1 || { echo "BUILD FAILED" >> $W/progress; exit 1; }
echo "BUILD done $(date +%s)" >> $W/progress
for RUN in ${RUNS:-P M}; do
  rm -rf $W/log-$RUN; mkdir -p $W/log-$RUN
  echo "RUN $RUN start $(date +%s)" >> $W/progress
  if [ "$RUN" = M ]; then MUT="CAD_WM_MUTANT=1"; else MUT="CAD_WM_X=0"; fi
  env CAD_WM_LOG=$W/log-$RUN $MUT cargo nextest run $PKGS --profile wm --ignore-default-filter \
    --no-fail-fast -E "$(cat $W/expr-all.txt)" > $W/out-$RUN.txt 2>&1
  echo "RUN $RUN done $(date +%s) $(grep -E '^\s+Summary' $W/out-$RUN.txt | head -1)" >> $W/progress
done
echo "ALL done $(date +%s)" >> $W/progress
