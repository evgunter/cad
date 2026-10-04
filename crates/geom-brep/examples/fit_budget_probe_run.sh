#!/bin/bash
B=/home/user/tgt-fitfork-b/release/examples/dome_scratch
export CAD_SCRATCH_FIT_BUDGET=1000000 CAD_SCRATCH_REFINE=1
for e in 1e-9 1e-12 1e-13; do
  CAD_TOLERANCE_EPS=$e DOME_DS=1 DOME_CUTS=level,oblique,tilt,zcut $B
done
CAD_TOLERANCE_EPS=1e-12 DOME_DS=2,4 DOME_HALF=4 DOME_CUTS=oblique,zcut $B
echo DONE
