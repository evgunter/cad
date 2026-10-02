#!/usr/bin/env bash
# Reviewer probe (PR 3847 dual, lane r1): the door fuzz under each root-
# placement / slack mutant, judged by probes/oracle_r1.py.
set -u
F=crates/topo/src/boolean/circle_roots.rs
declare -A M=(
  [near_far_inverted]='s|select_le_zero(past(hi_value), T::pi() - past(lo_value))|select_le_zero(T::pi() - past(lo_value), past(hi_value))|'
  [old_acos]='s|let half_chord = (lo_value + hi_value).select_le_zero(past(hi_value), T::pi() - past(lo_value));|let half_chord = { let _ = past; (T::zero() - c0 / a1).max(T::zero() - T::one()).min(T::one()).acos() };|'
  [angle_dropped]='s|let slack = radius \* (at_root / slope + phase + rounding_charge(T::tau()));|let slack = radius * (at_root / slope + phase);|'
  [phase_dropped]='s|let slack = radius \* (at_root / slope + phase + rounding_charge(T::tau()));|let slack = radius * (at_root / slope + T::zero() * phase + rounding_charge(T::tau()));|'
)
for name in "${!M[@]}"; do
  cp $F /tmp/claude-0/cr.bak
  sed -i "${M[$name]}" $F
  if cmp -s $F /tmp/claude-0/cr.bak; then echo "$name: PATCH DID NOT APPLY"; continue; fi
  PROBE_OUT=/tmp/claude-0/fz_$name.txt PROBE_N=${PROBE_N:-9000} cargo test -q -p topo --lib door_fuzz_r1 2>&1 | grep -E "^error|test result"
  cp /tmp/claude-0/cr.bak $F
  python3 probes/oracle_r1.py /tmp/claude-0/fz_$name.txt > /tmp/claude-0/or_$name.txt 2>&1
  echo "== $name: $(grep -c '^ARC' /tmp/claude-0/or_$name.txt) roots off the band; err>slack total $(grep -o "ARC_ERR_GT_SLACK': [0-9]*" /tmp/claude-0/or_$name.txt | awk '{s+=$2} END{print s+0}')"
  grep -v '^ARC\|^MISS\|^CERT' /tmp/claude-0/or_$name.txt | grep -o "^([^)]*)\|worst err/slack ([0-9.e-]*" | paste - - | sort -t'(' -k3 -g | tail -3
done
