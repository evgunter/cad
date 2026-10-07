#!/bin/bash
cd /home/user/cad
export CARGO_TARGET_DIR=/home/user/rev2-target
for m in M0_baseline M1_parity_too_few M2_nested_as_apart M3_ignore_on_depth2 M4_pair_intersection M5_ignore_met M6_first_partner_only M7_ignore_on M8_either_side M9_pairs_declined; do
  git checkout -q -- crates/topo/src/boolean/vtxfac.rs
  python3 /home/user/rev2-mut/mutate.py $m || { echo "$m PATCHFAIL" >> /home/user/rev2-mut/summary; continue; }
  cargo nextest run -p topo --no-fail-fast -E 'test(/a_vertex_read_by_two_sector_passes::(a_touching|a_vertex_in_several|a_vertex_crossing)/) | test(review2_oracle) | test(/vtxfac::tests/)' --no-capture > /home/user/rev2-mut/$m.topo.log 2>&1
  cargo nextest run -p editor-core --no-fail-fast -E 'test(/touch_reread_rows::a_vertex_read_twice/)' > /home/user/rev2-mut/$m.ec.log 2>&1
  echo "== $m" >> /home/user/rev2-mut/summary
  grep -E "^\s+(PASS|FAIL) \[" /home/user/rev2-mut/$m.topo.log /home/user/rev2-mut/$m.ec.log | sed 's/.*\(PASS\|FAIL\) \[[^]]*\] *([0-9/]*) */\1 /' >> /home/user/rev2-mut/summary
  grep -E "^TOTAL" /home/user/rev2-mut/$m.topo.log >> /home/user/rev2-mut/summary
  grep -E "error\[|error: could not" /home/user/rev2-mut/$m.topo.log | head -3 >> /home/user/rev2-mut/summary
done
git checkout -q -- crates/topo/src/boolean/vtxfac.rs
echo ALLDONE >> /home/user/rev2-mut/summary
