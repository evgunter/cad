#!/bin/bash
S=/tmp/claude-0/-home-user-cad/49e1e794-31a7-596f-a5c4-95370c0cbe34/scratchpad
cd $S/head
for m in "$@"; do
  git checkout -q -- crates
  python3 $S/mut.py $m || { echo "MUT $m: apply failed"; continue; }
  out=$(CAD_TOLERANCE_EPS=${EPS:-1e-9} CARGO_TARGET_DIR=$S/tgt-head cargo nextest run --profile ci --ignore-default-filter -p geom-brep -p topo -p editor-core --no-fail-fast -E '(test(/end_root::rows/) | test(/junction_end_rows/) | test(/a_pipe_unions_with_its_elbow::/)) & !test(/review_belt/)' 2>&1)
  echo "MUT $m eps ${EPS:-1e-9}: $(echo "$out" | grep -E 'Summary|error\[' | head -3)"
  echo "$out" | grep -E '^\s+FAIL' | sed 's/.*\] //' | sort -u | sed 's/^/   red: /'
done
git checkout -q -- crates
