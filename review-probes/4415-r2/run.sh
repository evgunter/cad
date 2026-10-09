#!/bin/bash
S=/tmp/claude-0/-home-user-cad/632e7b18-4656-5a40-9f73-5afebc684e10/scratchpad
run() { # tag eps tilts
  for w in base head; do
    (cd $S/$w && CAD_TOLERANCE_EPS=$2 NT_D=$3 $S/tgt-$w/release/examples/near_tangent_census_probe > $S/census/$1.$w.txt 2>&1)
  done
}
run e9new 1e-9 "2e-8,-2e-8,5e-8,-5e-8,1.5e-8,-1.5e-8,6e-9,-6e-9"
run e9pr 1e-9 "3e-7,-3e-7,3e-8,-3e-8,3e-9,-3e-9"
run e12 1e-12 "1e-8,-1e-8,1e-9,-1e-9,1e-10,-1e-10,1e-11,-1e-11"
run e6new 1e-6 "2e-5,-2e-5,5e-5,-5e-5,1.5e-5,-1.5e-5"
echo DONE > $S/census/done
