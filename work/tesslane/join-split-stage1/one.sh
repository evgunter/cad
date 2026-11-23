#!/bin/bash
# one.sh <name> <filter> [env...]
name=$1; f=$2; shift 2
out=/home/user/js/$name; rm -rf $out; mkdir -p $out/log
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/join-split-target CARGO_INCREMENTAL=0 JS_LOG=$out/log
for a in "$@"; do export "$a"; done
cargo nextest run -p sweep --test all --no-fail-fast -E "$f" > $out/nextest.log 2>&1
echo DONE >> $out/nextest.log
