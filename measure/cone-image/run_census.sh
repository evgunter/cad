#!/bin/bash
# usage: run_census.sh <outdir>   (reads census-cone.txt T lines)
. ~/.local/share/cad-work/cone-image/env.sh
W=~/.local/share/cad-work/cone-image
out=$1; mkdir -p $out
cd /home/user/cad
grep '^T ' $W/census-cone.txt | while read _ bin test rest; do
  pkg=${bin%%::*}; name=$(echo $test | tr ':' '_')
  if [ "$bin" = "mesh" ]; then sel="package(mesh) & binary(mesh) & test(=$test)"; else sel="package($pkg) & binary(all) & test(=$test)"; fi
  rm -rf $out/gl-$name; mkdir -p $out/gl-$name
  CV_TRACE=1 CAD_GATE_LOG=$out/gl-$name timeout 900 cargo nextest run -p sweep -p mesh --profile ci --ignore-default-filter --no-capture -E "$sel" > $out/$name.log 2>&1
  echo "$test $(grep -a -E '^ +(PASS|FAIL|TIMEOUT)' $out/$name.log | head -1 | awk '{print $1}')" >> $out/summary.txt
done
echo DONE >> $out/summary.txt
