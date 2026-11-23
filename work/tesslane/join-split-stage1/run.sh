#!/bin/bash
# usage: run.sh <name> <eps|default> [env assignments...]
name=$1; eps=$2; shift 2
out=/home/user/js/$name; rm -rf $out; mkdir -p $out/log
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/join-split-target CARGO_INCREMENTAL=0 JS_LOG=$out/log
[ "$eps" != default ] && export CAD_TOLERANCE_EPS=$eps
for a in "$@"; do export "$a"; done
cargo nextest run -p mesh --no-fail-fast --test-threads 4 > $out/nextest.log 2>&1
echo DONE >> $out/nextest.log
