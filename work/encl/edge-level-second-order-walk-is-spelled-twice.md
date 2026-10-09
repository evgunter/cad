---
id: edge-level-second-order-walk-is-spelled-twice
kind: issue
title: the edge-level second-order walk is spelled twice, in must_carry_over_edge and inline in tier 3's check 4, kept in step only by prose
status: dispatched
branch: encl/second-order-walk-one-home
priority: P3
cost: M
opened: 2026-10-09
---

## Finding (ENCL review of PR 4411)

The edge-level second-order walk is spelled twice: once in
`geom_brep::must_carry_over_edge` (`crates/geom-brep/src/dihedral.rs`,
which constructors ask), and once inline in tier 3's check 4, in the
`all_smooth` branch's station loop (`crates/topo/src/validate.rs`,
`match decide("tangent_second_order", margin, band)`, ~`:6584`). Both
walks use the schedule's interior stations through `sample_param`, the
`tangent_second_order` predicate, the sagitta over `folded_lever_arm`,
and let the first non-`Positive` station decide. The two are kept in
step only by prose (the rule's doc says "the stations are the tier-3
must-carry arm's"). PR 4411's defect was drift between them: the rule
gated the reading by `tangent_certificate_lane` while tier 3 did not,
so the rule stored conventional what tier 3 then refused
`SliverDihedral { SecondOrder }`.

## Repair shape

Either tier 3 calls the rule's walk, or both call one shared walker
that returns the per-station verdict. Tier 3 interleaves the material
pairing and `material_cusp_side` into the same loop, so the shared
piece is the station iteration plus the second-order decision, with
tier 3's material reads as a per-station hook. This changes neither
answer; it removes the second spelling.

## Class siblings

- `topo::boolean::contact_verify::tangent_locus_relation` meters
  `Margin::sagitta(|κ_rel| − drift, arm)` under its own predicate
  (`"contact_tangent_second_order"`) over its own schedule
  (`0..CERT_SAMPLES`).
- `replace_face`'s dihedral readers, named by the review. This sweep
  did not find a `classify_dihedral` call in
  `crates/topo/src/replace_face.rs` itself. The `classify_dihedral`
  callers on the paths that reach it are
  `crates/topo/src/boolean/edge_join.rs` (~`:412`) and
  `crates/topo/src/boolean/reduce.rs`, and they need reading before
  this row is scoped.

