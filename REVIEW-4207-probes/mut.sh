#!/bin/bash
S=/tmp/claude-0/-home-user-cad/707d27b5-8085-5dd9-aec5-03555aca97a3/scratchpad
O=$S/out/X; mkdir -p $O; cd $S/H/crates/sweep
for m in "R4_SHARE=skip" "R4_SHARE=largest" "R4_CLASS=first" "R4_CLASS=multi"; do
  n=${m//=/_}
  env $m $S/bin/all-X join_pierce_runs_sweep::a_pinchs_cones_share_one_point_key --exact > $O/row-$n.txt 2>&1; echo "row $n rc=$?" >> $O/status
  env $m $S/bin/all-X join_pierce_runs_sweep:: > $O/suite-$n.txt 2>&1; echo "suite $n rc=$?" >> $O/status
  env $m $S/bin/r1-X dbl > $O/dbl-$n.txt 2>&1; echo "dbl $n rc=$?" >> $O/status
  env $m $S/bin/all-X join_pinch_cones_r2_probes::r2_pinched_operand_battery --exact --ignored --nocapture > $O/r2p-$n.txt 2>&1; echo "r2p $n rc=$?" >> $O/status
done
R4_DIST=1 $S/bin/r1-X dbl > $O/dist-dbl.txt 2>&1; echo "dist dbl" >> $O/status
R4_DIST=1 $S/bin/all-X join_pinch_cones_r2_probes::r2_pinched_operand_battery --exact --ignored --nocapture > $O/dist-r2p.txt 2>&1; echo "dist r2p" >> $O/status
R4_DIST=1 R4_KEYS=1 $S/bin/r1-X two > $O/dist-two.txt 2>&1; echo "dist two" >> $O/status
R4_DIST=1 R4_KEYS=1 $S/bin/r1-X cyl > $O/dist-cyl.txt 2>&1; echo "dist cyl" >> $O/status
R4_DIST=1 $S/bin/all-X join_pierce_runs_sweep::pierce_runs_battery --exact --ignored --nocapture > $O/dist-pierce.txt 2>&1; echo "dist pierce" >> $O/status
echo DONE >> $O/status
