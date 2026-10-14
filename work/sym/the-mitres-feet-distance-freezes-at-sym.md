---
id: the-mitres-feet-distance-freezes-at-sym
kind: issue
title: A box mitre's feet readings freeze at Sym with every variable a symbol, so the turn's margin is not the zero polynomial
status: open
opened: 2026-10-10
priority: P1
cost: H
---

Found by INTENT stage 4 C+D (`carriers-compare-in-canonical-form`),
measuring every box mitre as Ev asked (2026-10-06: "output definitions
plus rung 3").

The coincidence door replays a document at `Sym<f64>` with every
continuous variable a symbol (`editor_core::analysis::var_env_symbolic`),
in a deferring session (`geom_core::sym::deferred`), and discharges a
row's deciding margin afterwards. On a box 2 × 1.5 × 1 with both rims
filleted (r = 0.1), all 8 `BatteryTurn` rows
(`sweep::blend::battery::turn_at`, `fillet3_turn_isosceles`) fail to
discharge:

- the margin is `max(|cos₁ − cos₂|·arm, |y₂ − y₁|)`; its early form is
  `max(0, sqrt(?a + ?b + ?c))`. The angle term is the zero polynomial,
  so the isosceles fact is an identity; the feet distance (each foot a
  `line_meet` of a band's trimline with the third edge's line) freezes
  into three opaque atoms;
- the margin's DAG has 1563–1566 distinct nodes and 19 parameter symbols;
  the session froze 268 nodes;
- discharge costs 0.11–19.9 s per row, 53 s for the eight
  (`SymRetry::none()`), or 0.2–27.8 s and 80 s under the drive's
  `kept_atom` ladder. Release build.

The two feet are one function of the dihedral, so at an isosceles turn
they are one point as an identity. Done when the eight rows discharge
as theorems at the door, or when the reason they cannot is a measured
limit recorded here. The probe is `probe_mitre_at_sym` in
`crates/editor-core/tests/coincidence_door.rs` (ignored; run with
`--ignored --nocapture`, release).

The door pins the eight rows unproven
(`a_filleted_box_records_one_unproven_turn_per_corner`). The disc's joints
are a sibling of a different kind, a cost rather than an identity:
`the-blend-joints-prove-only-past-the-door-s-budget`.

## 2026-10-12 — the verdict reads the feet alone (overrun step 1)

`a-non-isosceles-turn-overruns-past-the-mitre` step 1 replaced
`fillet3_turn_isosceles` with two readings, both lengths
(`turn_verdict`): the gap between the feet along the third edge,
`(y₁ − y₂)·L̂` (`fillet3_turn_feet_gap`), and the overrun chord
(`fillet3_turn_overrun_chord`, a norm), recorded jointly. The angle
term is gone, and with it the half the algebra closed: on the same
box, replayed at Sym, the gap renders as a sum of three frozen atoms
and the chords as `max(sqrt(…), sqrt(…))` over frozen atoms, so the
eight rows stay unproven. The gap takes no square root now, yet its
three atoms (the feet's `line_meet`s) still freeze.
