#!/usr/bin/env bash
# Temporary investigation helper (NOT for merge).
# Workspace-wide census: which concrete `Real` scalars actually appear in
# codegen'd symbol names, per compilation unit.
#   usage: bt-scalars.sh [profile] [glob]
cd "$(dirname "$0")/.."
prof="${1:-debug}"; glob="${2:-*}"
printf '%-42s %8s %9s %9s %9s %9s %9s\n' unit syms f64 Interval Dual Probe RingIntv
for f in target/$prof/deps/$glob; do
  case "$f" in *.rlib|*.rmeta) ;; *) continue ;; esac
  [ "${f##*.}" = rmeta ] && continue
  n=$(basename "$f")
  nm -S --defined-only "$f" 2>/dev/null | grep -E ' [Tt] ' | awk '{print $NF}' | rustfilt > /tmp/bt-sc.txt
  syms=$(wc -l < /tmp/bt-sc.txt)
  [ "$syms" -eq 0 ] && continue
  f64=$(grep -c '\bf64\b' /tmp/bt-sc.txt)
  iv=$(grep -c 'interval::Interval' /tmp/bt-sc.txt)
  du=$(grep -c 'dual::Dual' /tmp/bt-sc.txt)
  pr=$(grep -c 'k_stats::Probe' /tmp/bt-sc.txt)
  ri=$(grep -c 'RingInterval' /tmp/bt-sc.txt)
  printf '%-42s %8d %9d %9d %9d %9d %9d\n' "${n%%-*}" "$syms" "$f64" "$iv" "$du" "$pr" "$ri"
done
