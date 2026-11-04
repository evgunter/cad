#!/bin/bash
cd /home/user/cad
export CARGO_TARGET_DIR=/tmp/claude-0/tgt
export R1_STRICT=1
F='test(the_football_revolves) | test(review_6497_r1_probes) | test(near_horn_ladder) | test(one_segment_circle_on_or_against) | test(door_roster::a_sheet_apex) | test(door_roster::a_horn) | test(cert_spindle)'
for m in "$@"; do
  python3 /tmp/claude-0/mut/mutate.py $m apply || { echo "$m APPLY FAILED"; continue; }
  echo "=== $m" 
  cargo nextest run -p sweep -p topo -p mesh --no-fail-fast -E "$F" 2>&1 | grep -E "^\s+(PASS|FAIL|SIGABRT|TIMEOUT)|Summary|^error" 
  python3 /tmp/claude-0/mut/mutate.py $m revert || { echo REVERT FAILED; git checkout crates; }
done
echo ALLDONE
