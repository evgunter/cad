#!/bin/bash
B=/home/user/tgt-fitfork-a/release/examples/dome_scratch
OUT=/tmp/claude-0/-home-user-cad/b7f60dd6-dba8-53f8-9cf2-54b2db674bcf/scratchpad/probe-a-out2
mkdir -p $OUT
export CAD_SCRATCH_FIT_BUDGET=1000000 CAD_SCRATCH_REFINE=1
for spec in "1e-12 0.05 oblique" "1e-12 0.5 oblique" "1e-12 1 oblique" "1e-12 1 tilt" "1e-12 3 oblique" "1e-12 3 tilt" "1e-9 0.05 oblique" "1e-9 1 tilt" "1e-9 3 tilt" "1e-13 1 tilt" "1e-13 0.05 oblique"; do
  set -- $spec
  CAD_TOLERANCE_EPS=$1 DOME_DS=$2 DOME_HALF=4 DOME_CUTS=$3 timeout 900 $B > $OUT/dome-$1-$2-$3.log 2>&1
  echo "dome2 $1 $2 $3 exit=$?"
done
echo PROBE2_DONE
