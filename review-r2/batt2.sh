#!/bin/bash
# usage: batt2.sh <tree> <eps> <outdir> <filters...>
tree=$1; eps=$2; out=$3; shift 3; mkdir -p "$out"
cd "$tree" || exit 1
bin=target/release/deps/all-75a03b9b4cf81d6d
for t in "$@"; do
  CAD_TOLERANCE_EPS=$eps timeout 3000 $bin --include-ignored --nocapture --test-threads 4 "$t" > "$out/$t.log" 2>&1
  echo "$t rc=$?" >> "$out/summary"
done
echo ALLDONE >> "$out/summary"
