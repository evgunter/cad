#!/bin/bash
lbl=$1; shift
cd /home/user/capfork-b/crates/geom-brep
B=$(ls -t /home/user/tgt-capfork-b/release/deps/all-* | grep -v '\.d$' | head -1)
out=/home/user/probe-b/r5/$lbl-pair.log
: > $out
for o in "hyperbola c=1e-2" "hyperbola c=1e-4" "hyperbola c=-1e-4"; do
  env "$@" PROBE_ONLY="$o" PROBE_VAR=plain,near-degenerate-axes PROBE_EPS=1e-6,1e-9 timeout 1500 $B --exact zz_rev4034_probes::probe_pairing_against_truth --nocapture --test-threads 1 >> $out 2>&1
done
echo ALLDONE >> $out
