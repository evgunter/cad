#!/usr/bin/env bash
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/wm-target CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_DEBUG=0 CARGO_PROFILE_TEST_DEBUG=0
export CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1
export RUSTC_WORKSPACE_WRAPPER=$PWD/scripts/rustc-test-crates-opt0.sh
W=/home/user/wm-work; rm -rf $W/log-F; mkdir -p $W/log-F
env CAD_WM_LOG=$W/log-F cargo nextest run -p sweep -p editor-core --profile wm --ignore-default-filter --no-fail-fast -E "$(cat $W/expr-five.txt)" > $W/out-F.txt 2>&1
echo "FIVE done $(date +%s)" >> $W/five-progress
