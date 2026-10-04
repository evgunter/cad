#!/bin/bash
lbl=$1; shift
cd /home/user/capfork-b/crates/geom-brep
B=$(ls -t /home/user/tgt-capfork-b/release/deps/all-* | grep -v '\.d$' | head -1)
out=/home/user/probe-b/r4/$lbl.log
: > $out
env "$@" timeout 1500 $B --exact zz_rev4034_probes::probe_bend_single_detail --nocapture >> $out 2>&1
echo "DONE bend $?" >> $out
for e in 1e-6 1e-9 1e-12; do
  s=$(date +%s.%N)
  env "$@" CAD_TOLERANCE_EPS=$e timeout 1500 $B a_branch_beside_a_close_crossing_is_sampled_by_its_own_curvature --nocapture >> $out 2>&1
  echo "DONE arms $e $? $(echo "$(date +%s.%N)-$s" | bc)" >> $out
done
env "$@" PROBE_ONLY=hyperbola PROBE_VAR=plain,near-degenerate-axes PROBE_EPS=1e-6,1e-9 timeout 3000 $B --exact zz_rev4034_probes::probe_pairing_against_truth --nocapture --test-threads 1 >> $out 2>&1
echo "DONE pairing $?" >> $out
env "$@" PROBE_EPS=1e-6,1e-9 timeout 3000 $B --exact zz_rev4034_probes::probe_straight_then_bend --nocapture --test-threads 1 >> $out 2>&1
echo "DONE stb $?" >> $out
echo ALLDONE >> $out
