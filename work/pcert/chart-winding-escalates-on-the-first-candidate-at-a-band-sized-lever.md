---
id: chart-winding-escalates-on-the-first-candidate-at-a-band-sized-lever
kind: issue
title: The chart-winding classifier escalates on its first candidate at a band-sized lever, before the candidate that decides
status: open
opened: 2026-10-13
priority: P3
cost: M
needs_ev: true
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

## The question for Ev

One question covers this item and its sibling
(`chart-winding-escalates-on-the-first-candidate-at-a-band-sized-lever`,
`band-scale-torus-geometry-ends-as-a-kernel-defect`): what certification
owes a row whose angular or curvature-scale quantities sit at a
band-sized lever, where today the refusal ends as "a kernel defect;
report it". Two designers weighed it and converged after one round: on
a periodic chart the winding is structure the row stores and `certify`
compares exactly, never decided in metres (the near-apex child row then
certifies); the torus convention is decided where a torus face is
admitted; a closed-form class gate in band means "class not shown" and
the carrier takes the general (projected) lane. The C4 text the answer
changes is in the `[ev]` PR that carries this question.

Ev asked (on that PR) for a general principle behind the third answer,
since booleans decide their in-band cases differently and the lint is a
separate stage. Two more designers converged on one: a reading's
consumer is either a lane choice (it picks a form for one certified
statement, takes a lane that does not rest on the answer, and records
nothing) or a structural act (it changes the result's cells, records a
Zero for the lint and refuses in band). That paragraph goes in D3, with
a sentence in D10.
