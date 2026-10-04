#!/bin/bash
B=/home/user/tgt-fitfork-b/release/examples/dome_scratch
export CAD_SCRATCH_FIT_BUDGET=1000000 CAD_SCRATCH_REFINE=1
for e in 1e-6 1e-9 1e-12; do
  CAD_TOLERANCE_EPS=$e DOME_DS=0.05,0.25,1,3 DOME_CUTS=oblique,tilt $B
  CAD_TOLERANCE_EPS=$e DOME_DS=2 DOME_CUTS=oblique,tilt,zcut $B
done
echo DONE
