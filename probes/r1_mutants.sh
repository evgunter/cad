#!/usr/bin/env bash
# Reviewer r1 mutant battery for PR #3973 at 9f8ab75a24: each mutant is a
# sed over one file; runs the PR's own rows touching it and the r1 oracle
# dump (seed 1, 20 per cell), then restores the file.
set -u
cd "$(dirname "$0")/.."
S=${S:-/tmp/claude-0}
ER=crates/topo/src/boolean/ellipse_roots.rs
CR=crates/topo/src/boolean/circle_roots.rs
IM=crates/geom-brep/src/implicit.rs
CT=crates/topo/src/boolean/circle_torus.rs
run() { # name file sed-expr
  local name=$1 file=$2 expr=$3
  cp "$file" "$S/bak"
  sed -i "$expr" "$file"
  if cmp -s "$file" "$S/bak"; then echo "== $name: SED DID NOT APPLY"; return; fi
  echo "== $name"
  CARGO_INCREMENTAL=0 cargo nextest run -p topo --lib --no-fail-fast ellipse_roots circle_roots circle_torus 2>&1 | grep -E "^\s+(FAIL|SIGABRT)|Summary|error\[" | sort -u | head -12
  if [ "$file" = "$IM" ]; then CARGO_INCREMENTAL=0 cargo nextest run -p geom-brep --lib torus_harmonics 2>&1 | grep -E "FAIL|Summary" | head -3; fi
  CARGO_INCREMENTAL=0 R1SEED=1 R1N=20 cargo nextest run -p topo --lib --run-ignored only r1_dump --no-capture 2>&1 | sed -n 's/^R1ET //p' > "$S/m.jsonl"
  python3 probes/r1_oracle.py "$S/m.jsonl" | grep -A3 FAILURES | cut -c1-200
  cp "$S/bak" "$file"
}
run M1_no_slack_meter "$ER" 's/Some(&meter),/None,/'
run M2_slack_drops_running_error "$CR" 's/let off = (meter.residual)(root).magnitude();/let off = (meter.residual)(root).value.abs();/'
run M3_slack_ceiling_is_floor "$ER" 's/f_per_metre_hi: h.f_per_metre_hi,/f_per_metre_hi: h.f_per_metre_lo,/'
run M4_m4_reads_degree_two "$CR" 's/let fourth_hi = (1..=n_deg).fold/let fourth_hi = (1..=n_deg.min(2)).fold/'
run M5_bernstein_at_two "$CR" 's/let deg = k_of(n_deg);/let deg = k_of(2);/'
run M6_root_cap_four "$CR" 's/if roots.len() > 2 \* n_deg ||/if roots.len() > 4 ||/'
run M7_s_second_harmonic_dropped "$IM" 's/(aa - bb) \* half,/zero,/'
run M8_circle_torus_dropped_harmonics_uncharged "$CT" 's/noise: rounding_charge(h.terms) + dropped,/noise: rounding_charge(h.terms),/'
run M9_ellipse_torus_noise_zero "$ER" 's/noise: rounding_charge(h.terms),/noise: T::zero(),/'
run M10_slack_arc_no_speed "$CR" 's/let arc = speed_hi \* meter.f_per_metre_hi/let arc = meter.f_per_metre_hi/'
run M11_h_axial_first_harmonic_dropped "$IM" 's/\[w.dot(t_axis), a \* u.dot(t_axis), zero, zero, zero\]/[w.dot(t_axis), zero, zero, zero, zero]/'
run M12_terms_drop_h "$IM" 's/terms: p_abs.powi(2) + four_rr \* (s_abs + h_abs.powi(2)),/terms: p_abs.powi(2),/'
