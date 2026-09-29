---
id: sweep-dihedral-readers-take-the-folded-arm-escalation
kind: issue
title: sweep: extrude, revolve upgrade and must_carry_over_edge read classify_dihedral's folded escalation, so a collapsed arm reads as an unreadable wedge
status: open
opened: 2026-09-29
---

## What

`sweep::extrude` (near lines 943 and 1241), `sweep::revolve::upgrade`
(near line 116) and `geom_brep::must_carry_over_edge`
(`MustCarryVerdict::InBand`, under `"dihedral_arm"` too) read
`classify_dihedral`, which folds a definitely collapsed lever arm into an
`Invalid` escalation. Certification and the tier-3 validator now tell
the arm's decision from the wedge's (the ENCL lane `encl/collapsed-arm-gates` (`certify-collapsed-arm-gates-route-as-the-decision-they-guard`)); these build-time
readers still report a collapsed arm as an undecided wedge.

## Shape

Read `classify_dihedral_gated` where the refusal reaches a person, and
end a collapsed arm through `geom_brep::dihedral::LEVER_ARM` at
`Reading::Build`.

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
