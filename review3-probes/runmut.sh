#!/bin/bash
cd .
for m in G1_hollow_ignored G2_hollow_nonconvex G3_nesting_reversed G4_fallback_skipped G5_single_keeps_hollow G6_either_side G7_outermost_any; do
  python3 review3-probes/mut.py $m
  R3_OUT=review3-probes/mut_$m.txt R3_RANDOM=2 CARGO_TARGET_DIR=/home/user/r3-target-head timeout 3000 cargo nextest run -p topo --no-fail-fast --run-ignored all -E 'test(/a_vertex_read_again|a_vertex_read_by_two|vtxfac::tests|sectors::tests|r3_probe/)' --no-capture > review3-probes/mutlog_$m.txt 2>&1
  echo "== $m: $(grep -E '^SUMMARY' review3-probes/mutlog_$m.txt)"
  grep -E "^\s+(FAIL) \[" review3-probes/mutlog_$m.txt | sort -u | sed 's/^/   /'
  cp review3-probes/orig/*.rs crates/topo/src/boolean/
done
echo DONE
