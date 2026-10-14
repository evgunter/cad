---
id: pcurve-interval-meter-refuses-under-the-span-decision
kind: issue
title: geom-brep: pcurve_cache's param_rate_gate refuses a collapsed or undecided carrier meter under PcurveCheck::ParamSpan, the decision it guards
status: open
opened: 2026-09-29
---

## What

`geom_brep::pcurve_cache`'s `param_rate_gate` (near line 3648) gates
the carrier's metered extent with
`decide_positive("pcurve_interval_meter", ..)`, and its caller (near
line 5990, `map_err(span_escalated)`) reports every refusal as
`PcurveCertifyError::Escalated { check: PcurveCheck::ParamSpan }`. A
meter decided zero or negative (a spline whose speed floor vanishes or
turns back) therefore reads as an undecided span, in the span's lever
("not vanishingly short"), not as a meter that is not there. It is the
pcurve twin of certification's `nurbs_span_meter`, which ENCL routes as
`CertCheck::ParamSpanMeter` / `CertifyError::SpanMeterCollapsed`.

## Shape

Give the meter a `PcurveCheck` of its own and a definite variant read
through `Refused::rejected`, as `certify.rs`'s NURBS span arm does.

## The door that exists

A gate's rejection carries the margin the classifier decided, tagged
with the sign it refused (`geom_core::MarginDiag::rejected_sign`;
`geom_brep::recourse::Refused::rejected` reads it as a verdict). A
dihedral or `enters_material` escalation is a `geom_brep::LeverEscalation`:
its `rung` names the arm or the reading, and
`LeverEscalation::collapsed_arm` hands back the arm's verdict where the
gate decided it (ENCL, `work/encl/certify-collapsed-arm-gates-route-as-the-decision-they-guard.md`,
PR 3431). Certification (`CertCheck::TransversalityArm`,
`CertifyError::ArmCollapsed`, `CertCheck::ParamSpanMeter`) and the
validator (`WedgeCheck::Arm`, `ValidationError::NoDihedralArm`) route
through them.
