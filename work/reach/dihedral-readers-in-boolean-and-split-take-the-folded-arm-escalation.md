---
id: dihedral-readers-in-boolean-and-split-take-the-folded-arm-escalation
kind: issue
title: topo: the boolean glue and split finish read classify_dihedral's folded escalation, so a collapsed arm reads as an unreadable wedge
status: open
opened: 2026-09-29
---

## What

`topo::boolean::ops` (near line 1532) and `topo::splitting::finish`
(near line 582, `SplitFinishError::DescribeEscalated`) read
`classify_dihedral`, which folds a definitely collapsed lever arm into an
`Invalid` escalation, so a glued or split edge through a cone apex
refuses as an undecided wedge with the poisoned-margin note rather than
as "there is no angle to measure". Found by the ENCL lane `encl/collapsed-arm-gates` (`certify-collapsed-arm-gates-route-as-the-decision-they-guard`).

## Shape

Read `classify_dihedral_gated` and end a collapsed arm through
`geom_brep::dihedral::LEVER_ARM` at `Reading::Build`.

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
