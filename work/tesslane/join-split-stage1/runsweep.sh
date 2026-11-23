#!/bin/bash
name=$1; eps=$2; shift 2
out=/home/user/js/$name; rm -rf $out; mkdir -p $out/log
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/join-split-target CARGO_INCREMENTAL=0 JS_LOG=$out/log
[ "$eps" != default ] && export CAD_TOLERANCE_EPS=$eps
for a in "$@"; do export "$a"; done
mods=$(cat /home/user/js/sweep_mods.txt)
cargo nextest run -p sweep --test all --no-fail-fast --test-threads 4 -E "test(/^($mods)::/)" > $out/nextest.log 2>&1
echo DONE >> $out/nextest.log
