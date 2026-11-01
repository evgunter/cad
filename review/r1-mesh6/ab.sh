#!/bin/bash
# R1 MESH-6 measurement driver. Usage: ab.sh <rounds...> ; env EPSROW optional.
BIN=/root/lanes/mesh-6r1/target/release/deps/all-bbbf0efdcf975416
OUT=/root/lanes/mesh-6r1/review/r1-mesh6/ab
run() { # mode outfile
  local mode=$1 out=$2
  case $mode in
    none)  env CAD_S65_COST=1 "$BIN" issue897_guard_cost_report --nocapture --test-threads=1 ;;
    check) env CAD_S65_COST=1 CAD_S65_CHECK=1 "$BIN" issue897_guard_cost_report --nocapture --test-threads=1 ;;
    *)     env CAD_S65_COST=1 CAD_S65_SKIP=$mode "$BIN" issue897_guard_cost_report --nocapture --test-threads=1 ;;
  esac > "$out" 2>&1
}
if [ "${CENSUS:-0}" = 1 ]; then
  CAD_S65_CENSUS=1 "$BIN" r2_byte_stability_report --nocapture --test-threads=1 > $OUT/census.txt 2>&1
fi
tag=${EPSTAG:-def}
for r in "$@"; do
  for mode in none seam chord both check; do
    run $mode "$OUT/${tag}_r${r}_${mode}.txt"
  done
done
echo done
