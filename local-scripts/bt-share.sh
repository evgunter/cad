#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
# Attribute each crate's LLVM IR lines to the scalar instantiation named in
# the symbol, and to error-Display impls, from cargo-llvm-lines reports.
printf '%-26s %10s %10s %10s %10s %10s %10s\n' report TOTAL f64 Probe Interval Display other
for f in /tmp/bt-llvm/*.txt; do
  tot=$(awk '/\(TOTAL\)/ {print $1; exit}' "$f")
  [ -z "$tot" ] && continue
  # data rows: "  <lines> (a%, b%)   <copies> (c%, d%)  <name>"
  body=$(grep -vE '\(TOTAL\)|Lines|-----' "$f")
  f64=$(printf '%s\n' "$body" | grep -E '::<f64>|<f64>|, f64>|f64\]' | awk '{s+=$1} END {print s+0}')
  prb=$(printf '%s\n' "$body" | grep 'k_stats::Probe' | awk '{s+=$1} END {print s+0}')
  ivl=$(printf '%s\n' "$body" | grep 'interval::Interval' | awk '{s+=$1} END {print s+0}')
  dsp=$(printf '%s\n' "$body" | grep 'fmt::Display>::fmt' | awk '{s+=$1} END {print s+0}')
  oth=$((tot - f64 - prb - ivl - dsp))
  printf '%-26s %10d %10d %10d %10d %10d %10d\n' "$(basename "$f" .txt)" "$tot" "$f64" "$prb" "$ivl" "$dsp" "$oth"
done
