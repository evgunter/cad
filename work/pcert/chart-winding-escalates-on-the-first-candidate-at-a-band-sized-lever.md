---
id: chart-winding-escalates-on-the-first-candidate-at-a-band-sized-lever
kind: issue
title: The chart-winding classifier escalates on its first candidate at a band-sized lever, before the candidate that decides
status: open
opened: 2026-10-13
priority: P3
cost: M
---


Found on branch `pcert/cone-singular-gate-lever`, when the
loop walk's joint stopped escalating near a narrow cone's apex.

`geom_brep::pcurve_cache`'s chart-winding classifier
(`crates/geom-brep/src/pcurve_cache.rs:3636`) reads the candidates in
`Winding::ALL` order (`Neg, Zero, Pos`, `:3363`), each as
`(slope − β)·arm` (`pcurve_chart_winding`), and returns on the first
escalation. On a cone the arm is `azimuth_lever(surface, v_sup)`
(`:3617`), the row's own lever. A ruling's child row near a narrow
apex has a band-sized lever: in
`topo/tests/cone_apex_sector_joints.rs`'s split at slant `1.5·K·ε` on
a 0.1-rad cone, the arm is `1.5e-9`, so `Neg`'s margin, `1·arm`, is in
the band and the classifier escalates (`PcurveCertifyError::Escalated
{ check: ChartWinding }`). It never reaches `Zero`, whose margin is
exactly 0. The split refuses as `EulerOpError::PcurveSplit` with the
body untouched; row `a_split_near_a_narrow_apex_passes_its_joint`
pins that.

The question is not obviously "take `Zero`": at that lever the three
windings differ by `arm` metres, all inside the band, so `Neg` is not
excluded either. It is the near-pole read's question
(`topo::pcurves::lever_periods`) asked of a winding. A candidate whose
margin decides Zero names a row the samples meet within ε in metres;
whether a winding read that way is sound for the readers of the
winding (the chart polygon, the forward-span check) is what an answer
has to show.

Loud today (an escalation), never a wrong answer.
