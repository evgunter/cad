---
id: the-mitres-feet-distance-freezes-at-sym
kind: issue
title: A box mitre's feet distance freezes at Sym with every variable a symbol, so the isosceles margin is not the zero polynomial though its angle term is
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

The door pins the eight rows unproven, each with an assertion as its
recourse because the residual keeps opaque atoms
(`a_filleted_box_records_one_unproven_turn_per_corner`) and pins the
angle term's identity apart
(`a_box_mitres_angle_term_is_the_zero_polynomial`). The disc's joints
are a sibling of a different kind, a cost rather than an identity:
`the-blend-joints-prove-only-past-the-door-s-budget`.
