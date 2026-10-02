#!/usr/bin/env bash
# Reviewer mutants (PR 3847 dual, lane r1). Each edits circle_roots.rs in
# place, runs the touched rows at the three eps rows, reports which went
# red, and restores the file.
set -u
F=crates/topo/src/boolean/circle_roots.rs
EXPR='((package(topo) & test(circle_sphere)) | (package(topo) & test(circle_cylinder)) | (package(sweep) & test(snowman::)) | test(factored_extremes)) & !test(_r1::)'
declare -A M=(
  [bound_x0]='s|let at_root = (hi_value \* lo_noise - lo_value \* hi_noise) / swing;|let at_root = T::zero() * ((hi_value * lo_noise - lo_value * hi_noise) / swing);|'
  [phase_dropped]='s|let slack = radius \* (at_root / slope + phase + rounding_charge(T::tau()));|let slack = radius * (at_root / slope + T::zero() * phase + rounding_charge(T::tau()));|'
  [angle_dropped]='s|let slack = radius \* (at_root / slope + phase + rounding_charge(T::tau()));|let slack = radius * (at_root / slope + phase);|'
  [near_far_inverted]='s|select_le_zero(past(hi_value), T::pi() - past(lo_value))|select_le_zero(T::pi() - past(lo_value), past(hi_value))|'
  [far_share_dropped]='s|let at_root = (hi_value \* lo_noise - lo_value \* hi_noise) / swing;|let at_root = lo_noise.max(hi_noise) * T::zero() + (hi_value * lo_noise) / swing;|'
)
for name in "${!M[@]}"; do
  cp $F /tmp/claude-0/cr.bak
  sed -i "${M[$name]}" $F
  if cmp -s $F /tmp/claude-0/cr.bak; then echo "$name: PATCH DID NOT APPLY"; continue; fi
  for eps in 1e-9 1e-6 1e-12; do
    out=$(CAD_TOLERANCE_EPS=$eps cargo nextest run -p topo -p geom-brep -p sweep -E "$EXPR" --no-fail-fast 2>&1)
    red=$(echo "$out" | grep -E '^\s+FAIL ' | sed -E 's/.*FAIL \[[^]]*\] //' | sort -u | tr '\n' ';')
    sum=$(echo "$out" | grep -E '^\s+Summary' | sed 's/  */ /g')
    echo "$name eps=$eps :: $sum :: RED: ${red:-none}"
  done
  cp /tmp/claude-0/cr.bak $F
done
