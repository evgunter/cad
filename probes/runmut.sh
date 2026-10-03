#!/bin/bash
cd /home/user/cad
S=/tmp/claude-0/-home-user-cad/670d6476-a46f-5d05-ad07-9502b49c2741/scratchpad
export CARGO_INCREMENTAL=0
F='test(/rest_mate|full_turn_bore|mate7a|reach_continuation|sphere|cavity|snowman|kiss|reach_aligned|seam_and_kiss|probe_r2_3980::r2_(ball|multi|results)/)'
for m in "$@"; do
  cp $S/ops_head.rs crates/topo/src/boolean/ops.rs
  python3 $S/mutants.py crates/topo/src/boolean/ops.rs $m
  cargo nextest run --ignore-default-filter -p topo -p sweep --no-fail-fast -E "$F" > $S/out/mut_$m.txt 2>&1
  echo "== $m"; grep -E "^\s+FAIL \[|Summary" $S/out/mut_$m.txt | sed -E 's/\[[^]]*\] *\([^)]*\)//' | sort -u
done
cp $S/ops_head.rs crates/topo/src/boolean/ops.rs
echo ALLDONE
