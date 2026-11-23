#!/bin/bash
cd /home/user/wt/instr
export CARGO_TARGET_DIR=/home/user/tgt/instr
A='test(a_three_quarter_turn_cone_frustum_hollows) | test(r2_a_conical_wedge_hollows_with_hyperbolic_meridian_edges) | test(geometry_door_rows) | test(cover_reach_rows) | test(/solid_contain/) | test(/cone_door|partial|outline|point_in_solid/)'
B='test(/offset_derive/) | test(/offd/)'
C="$(cat /home/user/mut/minr.filter) | test(/implicit::/)"
D="$(cat /home/user/mut/link.filter)"
E='test(/session_box/) | test(/mate/) | test(/box/)'
run(){ m=$1; f=$2; SFG_MUT=$m cargo nextest run --profile ci -p geom-brep -p topo -p editor-core -p sweep --no-fail-fast -E "$f" > /home/user/mut/$m.log 2>&1; echo "$m $(grep -E '^\s+Summary' /home/user/mut/$m.log)" >> /home/user/mut/summary.txt; }
run NONE_A "$A"
for m in M1 M2 M3 M4 M5 M6 M9 U2 U3 U13 pos_cone_ray neg_cone_ray; do run $m "$A"; done
run NONE_B "$B"
for m in M7 M8 U4_small U4_nan U5_small U5_nan U5_max; do run $m "$B"; done
run NONE_E "$E"
for m in U14 U15; do run $m "$E"; done
run NONE_C "$C"
for m in M10 U8c U8s U8t; do run $m "$C"; done
run NONE_D "$D"
for m in U9 U10; do run $m "$D"; done
echo DONE >> /home/user/mut/summary.txt
