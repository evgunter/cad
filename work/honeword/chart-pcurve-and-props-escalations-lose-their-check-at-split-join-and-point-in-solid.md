---
id: chart-pcurve-and-props-escalations-lose-their-check-at-split-join-and-point-in-solid
kind: issue
title: chord_join and solid_contain re-wrap a chart_pcurve or props escalation without its check, so it ends by the wrapper's decision
status: open
opened: 2026-10-12
priority: P3
cost: E
---

(Found by ENCL's PR 4874 sweep for readers that flatten an escalation's decision into a generic arm. That PR fixed the same shape at `pcurve_cache` `ssi_refusal`. The geom-brep sibling is `work/issues/describe-maps-every-chart-pcurve-escalation-to-chart-image.md`. These files are shared by cleave and hone ground (`work.py territory`).)

## What

`PcurveCertifyError::Escalated { check, .. }` ends by its `PcurveCheck` (`PcurveCertifyError::ending`): `AzimuthPeriod` and `TubePeriod` by `CertCheck::ParamWinding` (sized), the chart winding checks by `Unsized::Defect`, the residual checks by `Unsized::Fit`. `PropsError::Escalated` ends by its own check likewise. Three topo sites drop the check:

- `topo::chord_join`, the run-edge `chart_pcurve(..).map_err` (about chord_join.rs:2399): becomes `SplitJoinError::Escalated { face, diag }`.
- `topo::boolean::solid_contain`, the torus-chart walk (about solid_contain.rs:2434): becomes `PointInSolidError::Escalated { face, diag }`.
- `topo::boolean::solid_contain` `refused`, over `MassPropsError::Face { source: PropsError::Escalated { cause, .. } }` (about solid_contain.rs:5905): becomes `PointInSolidError::Escalated { face, diag }`.

`PointInSolidError::Escalated`'s `Display` "names no decision": it ends in the unnamed placement lever, which is to move the query point. So a stored boundary's map-residual escalation is told to move the witness.

## Question

Per site, there are two options:
- carry the check, as `FittedEscalated` now carries its `SsiDecision`, and end through the inner refusal's `ending`;
- or state at the site why the wrapper's decision is the one the user acts on. The point-in-solid witness may be the right lever where the walk can retry another witness.

## Slate triage (2026-10-26, CLEAVE)

- Moved to HONEWORD (`work/honeword/`) by `git mv`, id and body unchanged: whose charter is split and boolean refusal text, payloads and endings. CLEAVE measured 270.5 budget points against 30 and split on its priority seam (Ev, in chat, 2026-10-26).
