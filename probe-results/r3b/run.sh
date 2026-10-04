#!/bin/bash
# usage: run.sh <label> <env...>
lbl=$1; shift
cd /home/user/capfork-b/crates/geom-brep
B=/home/user/tgt-capfork-b/release/deps/all-3d38e34683cb3fa5
out=/home/user/probe-b/r3b/$lbl.log
: > $out
env "$@" PROBE_ONLY=hyperbola PROBE_VAR=plain,bent,near-degenerate-axes PROBE_EPS=1e-6,1e-9 timeout 3000 $B --exact zz_rev4034_probes::probe_pairing_against_truth --nocapture --test-threads 1 >> $out 2>&1
echo "DONE pairing $?" >> $out
env "$@" timeout 1500 $B --exact zz_rev4034_probes::probe_bend_single_detail --nocapture >> $out 2>&1
echo "DONE bend $?" >> $out
for e in 1e-6 1e-9 1e-12; do
  s=$(date +%s.%N)
  env "$@" CAD_TOLERANCE_EPS=$e timeout 1500 $B a_branch_beside_a_close_crossing_is_sampled_by_its_own_curvature --nocapture >> $out 2>&1
  echo "DONE arms $e $? $(echo "$(date +%s.%N)-$s" | bc)" >> $out
done
echo ALLDONE >> $out
