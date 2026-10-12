#!/bin/bash
# In-band-only reversal, with the matching dumped, against the same probe binary unreversed.
S=/tmp/claude-0/-home-user-cad/4933b454-9319-5389-a7ac-536fed6e8288/scratchpad
export CAD_PROBE_DUMP=1
$S/batt2.sh /home/user/head 1e-6 $S/n2_6 curved_notch_pinch:: join_rc_probes:: join_pierce_runs_sweep::four_pairs_battery
$S/batt2.sh /home/user/head 1e-9 $S/n2_9 join_pierce_runs_sweep::near_tangent_battery
export CAD_PROBE_REVERSE=1
$S/batt2.sh /home/user/head 1e-6 $S/r2_6 curved_notch_pinch:: join_rc_probes:: join_pierce_runs_sweep::four_pairs_battery
$S/batt2.sh /home/user/head 1e-9 $S/r2_9 join_pierce_runs_sweep::near_tangent_battery
cd $S
for d in 6 9; do
  for f in r2_$d/*.log; do
    n=$(basename "$f")
    o=$(diff <(grep -v "finished in\|^thread\|panicked\|^note:\|running for over\|PROBE\|^ *[0-9]*: \|^ *at " "$f" | sort) <(grep -v "finished in\|^thread\|panicked\|^note:\|running for over\|PROBE\|^ *[0-9]*: \|^ *at " "n2_$d/$n" | sort) | grep -c '^[<>]')
    m=$(diff <(grep "PROBE matching" "$f" | sort) <(grep "PROBE matching" "n2_$d/$n" | sort) | grep -c '^[<>]')
    i=$(grep -c "PROBE first_pair mutual=true" "n2_$d/$n")
    echo "eps$d $n outcome_moved=$o matching_moved=$m inband_mutual_rounds=$i"
  done
done > $S/rev2.result
echo ALLDONE >> $S/rev2.result
