#!/bin/bash
# usage: run.sh <mutant>
S=/tmp/claude-0/-home-user-cad/97e1f4be-7435-58f8-b80d-b767cd8658d2/scratchpad
m=$1
cd /home/user/cad
cp $S/mut/$m.rs crates/topo/src/boolean/ops.rs
export CARGO_INCREMENTAL=0
cargo nextest run --profile ci --ignore-default-filter -p topo -p sweep -p editor-core \
  -E '(not test(reach_dual3980) & (default() | test(rest_mate_every_op))) | test(claim2) | test(claim3) | test(claim1_ball)' \
  --no-fail-fast > $S/mut/$m.log 2>&1
grep -E "^\s+(FAIL|TIMEOUT) |Summary|^error" $S/mut/$m.log | sort -u
