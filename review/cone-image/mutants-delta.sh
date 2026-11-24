#!/bin/bash
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/review-target
F='test(/injective::/) | test(/clearance::extent_rows/) | (binary(all) & test(/self_intersection_proves_injectivity::(a_cut_cone|a_cap_cut|a_body_through_a_cut|a_needle_and_a_near_flat|a_flap_through)/))'
for m in $@; do
  git checkout -q -- crates/topo/src
  if [ $m != BASE ]; then python3 review/cone-image/mutate-delta.py $m || { echo "$m APPLYFAIL" >> review/cone-image/m2.sum; continue; }; fi
  cargo nextest run -p topo -p sweep --profile ci --ignore-default-filter --no-fail-fast --success-output never --failure-output final -E "$F" > review/cone-image/m2-$m.log 2>&1
  echo "== $m: $(grep -a -E '^ +Summary' review/cone-image/m2-$m.log)" >> review/cone-image/m2.sum
  grep -a -E '^ +FAIL \[' review/cone-image/m2-$m.log | sort -u >> review/cone-image/m2.sum
  grep -a -E '^error' review/cone-image/m2-$m.log | head -3 >> review/cone-image/m2.sum
done
git checkout -q -- crates/topo/src
echo ALLDONE >> review/cone-image/m2.sum
