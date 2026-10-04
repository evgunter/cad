#!/bin/bash
# Fit-budget fork round 2, lane A: per-round refused margins (monotone?) and
# behaviour past the f64 floor with the budget lifted.
B=/home/user/tgt-fitfork-a/release/examples/dome_scratch
OUT=/tmp/claude-0/-home-user-cad/b7f60dd6-dba8-53f8-9cf2-54b2db674bcf/scratchpad/probe-a-out
mkdir -p $OUT
export CAD_SCRATCH_FIT_BUDGET=1000000 CAD_SCRATCH_REFINE=1
for e in 1e-9 1e-12 1e-13; do
  for d in 0.05 0.5 1 3; do
    for c in oblique tilt zcut; do
      CAD_TOLERANCE_EPS=$e DOME_DS=$d DOME_HALF=4 DOME_CUTS=$c timeout 900 $B > $OUT/dome-$e-$d-$c.log 2>&1
      echo "dome $e $d $c exit=$?"
    done
  done
done
# past the floor: ulp-scale eps
for e in 1e-14 1e-15; do
  for c in level oblique zcut; do
    CAD_TOLERANCE_EPS=$e DOME_DS=1 DOME_HALF=4 DOME_CUTS=$c timeout 900 $B > $OUT/dome-$e-1-$c.log 2>&1
    echo "dome $e 1 $c exit=$?"
  done
done
# the rational walls 3eps off the edge (18 rounds at 1e-12 on main)
until grep -q TEST2_EXIT /home/user/tgt-fitfork-a-build.log; do sleep 10; done
T=$(ls /home/user/tgt-fitfork-a/release/deps/all-* | grep -v "\.d$" | head -1)
for e in 1e-12 1e-9; do
  CAD_TOLERANCE_EPS=$e timeout 1200 $T a_rational_walls_plane_three_eps_off_its_edge --nocapture --test-threads 1 > $OUT/rational-$e.log 2>&1
  echo "rational $e exit=$?"
done
echo PROBE_DONE
