#!/bin/bash
# run.sh <tag> : runs every battery with bin/all-<tag>, bin/r1-<tag>; env passed through
S=/tmp/claude-0/-home-user-cad/707d27b5-8085-5dd9-aec5-03555aca97a3/scratchpad
T=$1; O=$S/out/$T; mkdir -p $O; cd $S/H/crates/sweep
A=$S/bin/all-${2:-$T}; R=$S/bin/r1-${2:-$T}
t() { $A "$1" --exact --ignored --nocapture --test-threads 1 > $O/$2.txt 2>$O/$2.err; echo "$2 rc=$?" >> $O/status; }
t join_pierce_runs_sweep::a_pinchs_cones_share_one_point_key row
t join_pinch_cones_r2_probes::r2_pinched_operand_battery r2p
t join_pinch_cones_r2_probes::r2_pinch_cones_battery r2c
$R dbl > $O/dbl.txt 2>$O/dbl.err; echo "dbl rc=$?" >> $O/status
R4_KEYS=1 $R dbl3 > $O/dbl3.txt 2>$O/dbl3.err; echo "dbl3 rc=$?" >> $O/status
R4_KEYS=1 $R two > $O/two.txt 2>$O/two.err; echo "two rc=$?" >> $O/status
R4_KEYS=1 $R cyl > $O/cyl.txt 2>$O/cyl.err; echo "cyl rc=$?" >> $O/status
t join_pierce_runs_sweep::pierce_runs_battery pierce
t join_pierce_runs_sweep::pinch_runs_battery pinch
for k in 0 1 2 3; do RCW_SHARD=$k/4 t join_rc_probes::rc_wide_battery rcw$k; done
for s in multi pair x4 stair3 islnotch nt; do $R $s > $O/$s.txt 2>$O/$s.err; echo "$s rc=$?" >> $O/status; done
echo DONE >> $O/status
