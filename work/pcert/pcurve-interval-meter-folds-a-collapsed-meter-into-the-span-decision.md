---
id: pcurve-interval-meter-folds-a-collapsed-meter-into-the-span-decision
kind: issue
title: geom-brep: pcurve_cache's param_rate_gate refuses a collapsed carrier meter under PcurveCheck::ParamSpan with the poisoned-margin note
status: open
opened: 2026-09-29
---

## What

`geom_brep::pcurve_cache`'s `param_rate_gate` (`pcurve_cache.rs`, near
line 2588) gates the carrier's metered extent with
`decide_positive("pcurve_interval_meter", ..)`, and both callers
(`map_err(span_escalated)`, near lines 3417 and 3958) report its
refusal as `PcurveCertifyError::Escalated { check: PcurveCheck::ParamSpan }`.
A definite non-positive meter (a spline whose speed floor vanishes or
turns negative) is therefore a poisoned `ParamSpan` margin, ending in
the span's lever plus "an unreadable or collapsed margin may indicate a
kernel bug worth reporting". It is the pcurve twin of certification's
`nurbs_span_meter`, which found this in the ENCL lane `encl/collapsed-arm-gates` (`certify-collapsed-arm-gates-route-as-the-decision-they-guard`).

## Shape

Give the meter a `PcurveCheck` of its own, routed as certification's
`CertCheck::ParamSpanMeter` is (`crates/geom-brep/src/certify.rs`).

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
