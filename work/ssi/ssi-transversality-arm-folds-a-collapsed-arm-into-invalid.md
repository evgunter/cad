---
id: ssi-transversality-arm-folds-a-collapsed-arm-into-invalid
kind: issue
title: geom-brep: ssi march's ssi_transversality_arm gate hands a definitely collapsed arm back as an Invalid escalation
status: open
opened: 2026-09-29
---

## What

`geom_brep::ssi::march` (`march.rs`, near line 368) gates the
transversality lever arm with `decide_positive("ssi_transversality_arm", ..)`
and maps the refusal to `SsiError::Escalated`, so a march that reaches a
point where the folded arm definitely vanishes (a cone apex) reports an
invalid-margin escalation rather than "there is no angle to measure
here". Found by the ENCL lane `encl/collapsed-arm-gates` (`certify-collapsed-arm-gates-route-as-the-decision-they-guard`) in its sweep of the collapse gates.

## Shape

Take `decide_positive_reported` and give the collapsed arm its own
`SsiError` arm, or route through `geom_brep::dihedral::LEVER_ARM`.

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
