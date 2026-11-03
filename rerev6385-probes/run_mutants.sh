#!/bin/bash
cd /home/user/wt-head/crates/topo/src
for m in M1_drop M2_flip M3_zero M6_gap_leaves M7_zero_leaves; do
  cp bound_side.rs.orig bound_side.rs
  python3 /home/user/probes/mutants.py $m
  (cd /home/user/wt-head && CARGO_TARGET_DIR=/home/user/tgt-head cargo nextest run --release -p topo --lib --no-fail-fast --status-level fail --final-status-level fail -E 'not test(/rerev6385/)' > /home/user/runs/mut-$m.log 2>&1)
  echo "$m $?" >> /home/user/runs/mut.summary
done
cp bound_side.rs.orig bound_side.rs
echo done >> /home/user/runs/mut.summary
