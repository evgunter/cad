#!/usr/bin/env bash
# Temporary investigation script (NOT for merge): post-gate IR census.
# BEFORE (measured on main, 2026-08-12, release --lib, default features):
#   geom-brep 106,897 lines / 1,742 copies   (Probe attribution 21,342 = 20.0%)
#   topo       95,033 lines / 1,358 copies   (Probe attribution 14,779 = 15.6%)
# Predicted after: ~85,555 and ~80,254.
set -u
cd "$(dirname "$0")/.."
OUT=/tmp/bt-llvm-after
mkdir -p $OUT
for c in geom-brep topo geom-core; do
  echo "=== $c --lib --release, default features (probe OFF) ==="
  cargo llvm-lines -p "$c" --lib --release > "$OUT/$c.off.txt" 2>"$OUT/$c.off.err"
  grep "(TOTAL)" "$OUT/$c.off.txt"
  echo "  residual Probe symbols: $(grep -c 'k_stats::Probe' "$OUT/$c.off.txt")"
done
echo
for c in geom-brep topo; do
  echo "=== $c --lib --release --features probe (probe ON) ==="
  cargo llvm-lines -p "$c" --lib --release --features probe > "$OUT/$c.on.txt" 2>"$OUT/$c.on.err"
  grep "(TOTAL)" "$OUT/$c.on.txt"
done
