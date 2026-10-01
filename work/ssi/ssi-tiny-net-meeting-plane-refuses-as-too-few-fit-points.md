---
id: ssi-tiny-net-meeting-plane-refuses-as-too-few-fit-points
kind: issue
title: plane_nurbs_ssi on a 1e-2-spread collapsed net that the plane meets refuses Fit(TooFewPoints{have:3,need:4}), a fitting diagnosis for what is a degenerate-wall case
status: closed
opened: 2026-10-01
priority: P1
cost: M
rides_with: ssi-chart-speed-usability-boundary
closed: 2026-10-01
pr: 3694
branch: ssi/chart-floor
---


(SSI orchestrator, from the review of PR 3653, 2026-10-01; measured by
the reviewer at the PR's merge base and at its head, with an identical
result on both.)

`plane_nurbs_ssi` on the collapsed net of `m5_pr7_ssi.rs`'s
zero-speed row, at spread `1e-2`, against a plane that MEETS it
(`z = 0.25·s`, normal `+z`), returns
`Err(Fit(TooFewPoints { have: 3, need: 4 }))`. The intersection exists
and the wall is tiny but honest, so a fit-sample shortfall is the
caller's least useful diagnosis. The neighbouring spreads refuse
differently: `1e-8` gives `ExhaustivenessInconclusive`, and `1e-100`
through `1e-315` escalate `ssi_transversality` as `Invalid`.

Open: what the operation should say about a wall this small relative
to its extent. It might trace it, refuse it as below the model's
resolution (D4 ¶4's model-size range), or name the trace's sample
shortfall in terms of the geometry. The chart-floor unit
(`ssi-chart-speed-usability-boundary`'s Design, part 5) is the
nearest door, so look there first.

## Closed (2026-10-01, PR 3694)

Not a floor case. The 3 cm branch is shorter than the march's longest step at the 1.5 m extent, so it yields 3 samples where the cubic fit needs 4. It refuses `BranchUndersampled` naming the extent, and following that recourse traces and certifies it. The row that pins it pins the refusal kind per spread. The design alternative, having the stepper trace short branches itself, is filed as `ssi-short-branch-step-reads-only-the-callers-extent`.
