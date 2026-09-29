---
id: enters-arm-gates-fold-a-collapsed-arm-into-invalid
kind: issue
title: geom-brep: enters_material_arm, tangent_sector_order2_arm and material_wedge_side's discriminant gate fold a definite collapse into an Invalid escalation
status: open
opened: 2026-09-29
---

(ENCL implementer, residue of
`certify-collapsed-arm-gates-route-as-the-decision-they-guard`.)

## What

The remaining collapse gates in `geom-brep` that hand a definite
verdict back folded:

- `enters::enters_material` (`enters.rs`, near line 203,
  `"enters_material_arm"`) and `enters::enters_material_order2` (near
  line 255, `"tangent_sector_order2_arm"`) return `Indeterminate`, so a
  definitely collapsed arm reaches their `topo` callers as a poisoned
  margin.
- `dihedral::classify_material_pairing_as` (`dihedral.rs`, near line
  726) gates the discriminant with `decide_nonzero`; `decide_nonzero`
  keeps the folded return (`GateRefusal::escalation`), and a definite
  zero there reaches the validator as `WedgeCheck::MaterialSide`.

The second pass of that sweep, for gates minted by hand rather than
through the funnel (`MarginDiag::INVALID` in production), found about
thirty sites in `topo` (`chart_region`, `splitting::{order, rules}`,
`sector_shape`, `invalid_margin`, `boolean::{reduce, rim_wedge,
contact_verify, solid_contain, plane_eq}`); `contact_verify` and
`solid_contain` are on CONTACT's row
(`work/contact/collapsed-gate-verdicts-folded-into-invalid-on-topo-ground.md`),
the rest are unread.

## Shape

Where the verdict reaches a person, take
`geom_core::k_stats::decide_positive_reported` (a `decide_nonzero`
sibling if the discriminant gate needs one) and end the gate as its own
decision.

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
