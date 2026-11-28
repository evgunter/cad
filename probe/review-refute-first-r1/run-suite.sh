#!/bin/bash
# usage: run-suite.sh <head|base> <label> <eps|-> <nextest args...>
t=$1; label=$2; eps=$3; shift 3
export CARGO_INCREMENTAL=0 CARGO_PROFILE_DEV_OPT_LEVEL=1 CARGO_PROFILE_TEST_OPT_LEVEL=1 CARGO_PROFILE_DEV_DEBUG=line-tables-only CARGO_PROFILE_TEST_DEBUG=line-tables-only
if [ $t = head ]; then cd /home/user/cad; export CARGO_TARGET_DIR=/home/user/review-refute-target; else cd /home/user/cad-base; export CARGO_TARGET_DIR=/home/user/review-refute-target-base; fi
[ "$eps" != - ] && export CAD_TOLERANCE_EPS=$eps
L=~/.local/share/cad-work/review-refute-first/suite-$t-$label.log
cargo nextest run --build-jobs 2 --profile ci --no-fail-fast "$@" > $L 2>&1
echo "rc=$?" >> $L; echo SUITEDONE >> $L
