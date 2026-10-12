#!/bin/bash
cd /home/user/review-alt
export CARGO_TARGET_DIR=/home/user/review-alt-target
F='test(/wedge_end_doors/) | test(/tang_split_review_probes::probe_scale_unit/) | test(/tang_split_review_probes::probe_profile_turns/)'
for m in M1 M2 M3 M4 M5 M6; do
  git checkout -q crates/topo/src/split.rs
  python3 /home/user/review-mut/mutate.py $m || { echo "$m: patch failed" >> /home/user/review-mut/summary; continue; }
  cargo nextest run -p sweep --test all --no-fail-fast -E "$F" --no-capture > /home/user/review-mut/$m.log 2>&1
  echo "$m exit $?: $(grep -E '^\s+Summary' /home/user/review-mut/$m.log)" >> /home/user/review-mut/summary
done
git checkout -q crates/topo/src/split.rs
echo MUTDONE >> /home/user/review-mut/summary
