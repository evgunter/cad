#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
# Probe's share of the TEST BINARIES (not just the libs) — the archive
# step compiles these, so this is the number that predicts a CI saving.
cd "$(dirname "$0")/.."
tot=0; prb=0; ivl=0; f64c=0
for f in $(ls -S target/debug/deps/ | grep -E '^[a-z_]+-[0-9a-f]{16}$' | head -40); do
  p="target/debug/deps/$f"
  [ -x "$p" ] || continue
  nm -S --defined-only "$p" 2>/dev/null | grep -E ' [Tt] ' | awk '{print $NF}' | rustfilt > /tmp/bt-tb.txt
  n=$(wc -l < /tmp/bt-tb.txt); [ "$n" -eq 0 ] && continue
  a=$(grep -c 'k_stats::Probe' /tmp/bt-tb.txt)
  b=$(grep -c 'interval::Interval' /tmp/bt-tb.txt)
  c=$(grep -c '\bf64\b' /tmp/bt-tb.txt)
  tot=$((tot+n)); prb=$((prb+a)); ivl=$((ivl+b)); f64c=$((f64c+c))
  printf '%-28s syms=%7d  Probe=%6d  Interval=%6d\n' "$f" "$n" "$a" "$b"
done
echo "-----"
printf 'TOTAL syms=%d  Probe=%d (%.1f%%)  Interval=%d (%.1f%%)  f64=%d\n' \
  "$tot" "$prb" "$(echo "scale=2; 100*$prb/$tot" | bc)" "$ivl" "$(echo "scale=2; 100*$ivl/$tot" | bc)" "$f64c"
