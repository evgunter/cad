#!/bin/bash
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/review-target
for m in $@; do
  git checkout -q -- crates/topo/src/injective.rs crates/topo/src/clearance.rs
  python3 review/cone-image/mutate.py $m || { echo "$m APPLYFAIL" >> review/cone-image/mutants.sum; continue; }
  cargo nextest run -p topo -p sweep --profile ci --ignore-default-filter --no-fail-fast --success-output never --failure-output final     -E 'test(/injective::/) | (binary(all) & test(/a_cut_cones_wall|a_cap_cut_under|a_body_through_a_cut_cones|a_needle_and_a_near_flat|review_a_block_in_the_crescent/))' > review/cone-image/mut-$m.log 2>&1
  echo "== $m: $(grep -E '^ +Summary' review/cone-image/mut-$m.log)" >> review/cone-image/mutants.sum
  grep -E '^ +FAIL \[' review/cone-image/mut-$m.log | sort -u >> review/cone-image/mutants.sum
  grep -E '^error' review/cone-image/mut-$m.log | head -3 >> review/cone-image/mutants.sum
done
git checkout -q -- crates/topo/src/injective.rs crates/topo/src/clearance.rs
echo ALLDONE >> review/cone-image/mutants.sum
