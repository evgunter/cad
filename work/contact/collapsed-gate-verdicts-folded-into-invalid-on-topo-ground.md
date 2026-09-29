---
id: collapsed-gate-verdicts-folded-into-invalid-on-topo-ground
kind: issue
title: topo: bool_torus_frame_radius and census's dihedral read fold a definite collapsed gate into an Invalid escalation
status: open
opened: 2026-09-29
---

## What

Three collapse gates on this program's ground fold a definite verdict
into an `Invalid` escalation, found by the ENCL lane `encl/collapsed-arm-gates` (`certify-collapsed-arm-gates-route-as-the-decision-they-guard`):

- `topo::boolean::solid_contain` (near line 2135):
  `decide_positive("bool_torus_frame_radius", ..)` guards the division
  by `rho`; near line 2110 `geom::require_ring_torus` (whose
  `torus_tube_positive` / `ring_torus_convention` gates live in
  `crates/geom/src/surfaces.rs`) does the same for a spindle or horn
  torus, which is a definite fact about the surface, not an unreadable
  margin. `boolean::section_cert` (near lines 521 and 562) reads the
  same door with `is_err()`.
- `topo::census` (near line 2044) reads `classify_dihedral` and pushes
  its folded escalation as an undecided candidate, so a collapsed
  arm there reads as an unreadable wedge.
- `topo::boolean::contact_verify` hand-mints `MarginDiag::INVALID`
  escalations at eight sites (near lines 166-645); each wants reading to
  tell a gate's definite verdict from a real poison.

## Shape

Where a gate's definite verdict reaches a person, take the reported door
and end it as its own decision.

## The door that now exists

`geom_core::k_stats::decide_positive_reported` hands a collapse gate's
verdict back: `GateRefusal::Collapsed { sign, margin, .. }` for a
definite non-positive sign (the funnel still records its `Invalid`
escalation, so the log is unchanged) and `GateRefusal::Undecided` for
an in-band or poisoned margin. `geom_brep::recourse::Refused::collapsed`
turns the first into a verdict a `SizedDecision` ends. For the dihedral,
`geom_brep::classify_dihedral_gated` returns `DihedralRefusal::{Arm,
Wedge}`, and `geom_brep::dihedral::LEVER_ARM` is the arm's one
`SizedDecision`. Certification (`CertCheck::TransversalityArm`,
`CertCheck::ParamSpanMeter`) and the tier-3 validator
(`ValidationError::NoDihedralArm`, `WedgeCheck::LeverArm`) route through
them; that is the pattern to follow here.
