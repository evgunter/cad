#!/bin/bash
lbl=$1; shift
cd /home/user/capfork-b/crates/geom-brep
B=$(ls -t /home/user/tgt-capfork-b/release/deps/all-* | grep -v '\.d$' | head -1)
out=/home/user/probe-b/r5/$lbl.log
: > $out
t() { local s=$(date +%s.%N); "$@"; echo "TIME $(echo "$(date +%s.%N)-$s" | bc)" >> $out; }
for m in a_semicircle_too_short_to_march_whose_cubic_misses_refuses_by_its_length a_straight_marched_branch_takes_the_fits_four_samples; do
  for e in 1e-6 1e-9 1e-12; do
    echo "M5 $m ε $e" >> $out
    t env "$@" CAD_TOLERANCE_EPS=$e timeout 900 $B $m --nocapture >> $out 2>&1
  done
done
echo "DONE m5" >> $out
t env "$@" timeout 1500 $B --exact zz_rev4034_probes::probe_bend_single_detail --nocapture >> $out 2>&1
echo "DONE bend" >> $out
for e in 1e-6 1e-9 1e-12; do
  echo "ARMSRUN ε $e" >> $out
  t env "$@" CAD_TOLERANCE_EPS=$e timeout 1500 $B a_branch_beside_a_close_crossing_is_sampled_by_its_own_curvature --nocapture >> $out 2>&1
done
echo "DONE arms" >> $out
for o in "hyperbola c=1e-2 " "hyperbola c=1e-4 " "hyperbola c=-1e-4 "; do
  env "$@" PROBE_ONLY="$o" PROBE_VAR=plain,near-degenerate-axes PROBE_EPS=1e-6,1e-9 timeout 1500 $B --exact zz_rev4034_probes::probe_pairing_against_truth --nocapture --test-threads 1 >> $out 2>&1
done
echo "DONE pairing" >> $out
env "$@" PROBE_EPS=1e-6,1e-9 timeout 3000 $B --exact zz_rev4034_probes::probe_straight_then_bend --nocapture --test-threads 1 >> $out 2>&1
echo "DONE stb" >> $out
echo ALLDONE >> $out
