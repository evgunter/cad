---
id: describe-maps-every-chart-pcurve-escalation-to-chart-image
kind: issue
title: certify's describe re-wraps every chart_pcurve escalation as CertCheck::ChartImage, so an azimuth-headroom escalation ends as a defect rather than by ParamWinding
status: open
opened: 2026-10-12
priority: P3
cost: E
---

(Found by ENCL's PR 4874 sweep for readers that flatten an escalation's decision into a generic arm. That PR fixed the same shape at `pcurve_cache` `ssi_refusal`, where `FittedEscalated` now carries its `SsiDecision`. The topo siblings are `work/cleave/chart-pcurve-and-props-escalations-lose-their-check-at-split-join-and-point-in-solid.md`.)

## What

`PcurveCertifyError::Escalated { check, .. }` ends by its `PcurveCheck` (`PcurveCertifyError::ending`, `PcurveCheck::recourse`):
- `AzimuthPeriod` and `TubePeriod` by `CertCheck::ParamWinding`: sized, move the geometry or tighten;
- `ParamSpan` by `CertCheck::ParamSpan`;
- the chart winding checks by `Unsized::Defect`;
- the residual checks by `Unsized::Fit`.

`geom_brep::certify` `describe`, the `chart_pcurve(..).map_err` (about certify.rs:2801), turns every check into `CertifyError::Escalated { check: CertCheck::ChartImage, sample: NOT_A_SAMPLE, cause }`. `CertCheck::ending` routes `ChartImage` to `Unsized::Defect` (about certify.rs:922). So an azimuth-headroom escalation ends sized at the pcurve door and in the defect ending through the edge certifier. One decision ends two ways.

## Question

Carry the `PcurveCheck` through, as `CertifyError::PlaneNurbs` carries its refusal whole, and end through `PcurveCertifyError::ending`. Or state why `ChartImage`'s ending is the one the user acts on there.
