#!/bin/bash
# Runs the PR's row (rigid_map_near_eps_plane_nurbs) and main's row
# (copied as probe_old_rigid) under each env-selected mutant at each ε.
B=${B:-target/debug/deps/all-8e49f722ad6642a7}
T=the_certificate_re_derives_within_rounding_under_the_map
run() { # $1 MUT $2 LEAK $3 eps $4 module
  out=$(MUT=$1 MUT_LEAK=$2 CAD_TOLERANCE_EPS=$3 $B "$4::$T" --exact --nocapture 2>&1)
  if echo "$out" | grep -q "test result: ok"; then v="green: $(echo "$out" | grep -o 'limb 1 within.*\|moves by up to.*' | head -1)"
  else v="RED: $(echo "$out" | grep -A1 "panicked at" | tail -1 | cut -c1-170)"; fi
  echo "$1 leak=$2 eps=$3 $4: $v"
}
for spec in "none 0" "L1 0" "NOZ 0" "XLEAK 1e-3" "XLEAK 1e-6" "SCALE 1e-3" "T_FOLD 0" "T_L1 0" "T_LINF3 0" "T_ZLEAK 1e-3" "T_ZLEAK 1e-2"; do
  set -- $spec
  for eps in 1e-12 1e-9 1e-6; do
    for mod in rigid_map_near_eps_plane_nurbs probe_old_rigid; do run $1 $2 $eps $mod; done
  done
done
