#!/bin/bash
# usage: run.sh side eps label [nextest filter args...]
side=$1; eps=$2; label=$3; shift 3
SP=/tmp/claude-0/-home-user-cad/9a2ba13e-681e-54bf-b3ea-c8108b555fe9/scratchpad
if [ $side = head ]; then D=/home/user/cad; else D=/home/user/cad-base; fi
out=/home/user/diff/$label-$side-$eps; rm -rf $out; mkdir -p $out
cd $D
envs="CAD_FUZZ_SEED=0x5eed CAD_DIFF_DIR=$out"
[ $eps != default ] && envs="$envs CAD_TOLERANCE_EPS=$eps"
env $envs CARGO_TARGET_DIR=/home/user/tgt-$side cargo nextest run --no-fail-fast --profile ci "$@" > $SP/run-$label-$side-$eps.log 2>&1
echo "rc=$?" >> $SP/run-$label-$side-$eps.log
