---
id: chart-pcurve-escalations-lose-their-check-at-three-wrapping-doors
kind: issue
title: Three doors re-wrap a chart_pcurve or props escalation without its check, so it ends by the wrapper's decision rather than its own
status: open
opened: 2026-10-12
priority: P3
cost: E
---

(Found by the `unsized-fit-reads-a-rejected-zero-on-the-undecided-arm-as-the-last-resort` lane's sweep for readers that flatten an escalation's decision into a generic arm. That lane fixed the same shape at `pcurve_cache` `ssi_refusal`, where `FittedEscalated` now carries the `SsiDecision`.)

## What

`PcurveCertifyError::Escalated { check, .. }` ends by its `PcurveCheck` (`PcurveCertifyError::ending`, `PcurveCheck::recourse`): `AzimuthPeriod` and `TubePeriod` by `CertCheck::ParamWinding` (sized: move the geometry, or tighten), `ParamSpan` by `CertCheck::ParamSpan`, the chart winding checks by `Unsized::Defect`, the residual checks by `Unsized::Fit`. Three doors re-wrap it and drop the check:

- `geom_brep::certify` `describe`, the `chart_pcurve(..).map_err` (about certify.rs:2801): every check becomes `CertifyError::Escalated { check: CertCheck::ChartImage, .. }`, which ends `Unsized::Defect` (`CertCheck::ending`, about certify.rs:922). An azimuth-headroom escalation ends sized at the pcurve door and in the defect ending here.
- `topo::chord_join`, the run-edge `chart_pcurve(..).map_err` (about chord_join.rs:2399): becomes `SplitJoinError::Escalated { face, diag }`.
- `topo::boolean::solid_contain`, the torus-chart walk (about solid_contain.rs:2434), and `refused` over `MassPropsError::Face { source: PropsError::Escalated { cause, .. } }` (about solid_contain.rs:5905): both become `PointInSolidError::Escalated { face, diag }`, whose ending names no decision (the unnamed placement lever, `PointInSolidError`'s `Display`).

So one decision's escalation ends one way at its own door and another way through the wrapper, against "each decision ends by its own kind, the same at every door".

## Question

Per site: carry the check (as `CertifyError::PlaneNurbs` carries the plane × NURBS refusal whole, or as `FittedEscalated` now carries `SsiDecision`) and end through the inner refusal's `ending`, or state at the site why the wrapper's decision is the one the user acts on (the point-in-solid door's witness may be the right lever there).
