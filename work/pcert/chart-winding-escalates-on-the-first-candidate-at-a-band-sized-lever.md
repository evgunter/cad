---
id: chart-winding-escalates-on-the-first-candidate-at-a-band-sized-lever
kind: issue
title: A cone rim's winding is still decided in metres at the rim's lever, so a rim at a band-sized radius escalates at pcurve_chart_orientation
status: open
opened: 2026-10-13
priority: P3
cost: M
---


Found on branch `pcert/cone-singular-gate-lever`, when the
loop walk's joint stopped escalating near a narrow cone's apex.

Before PCERT B2a, `geom_brep::pcurve_cache`'s chart-winding classifier
read the candidate windings in the order `−1, 0, 1`, each as
`(slope − β)·arm` (`pcurve_chart_winding`), and returned on the first
escalation. On a cone the arm was `azimuth_lever(surface, v_sup)`, the
row's own lever. B2a removed the classifier: windings are stored and
compared exactly. What stays is the rim's orientation sign, below. A ruling's child row near a narrow
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

## The answer

Ev on `[ev]` PR 5113 (2026-10-14/15): decision 1 accepted (windings are structure the row stores and `certify` compares exactly); decision 3 replaced by the D3 principle (a class gate is a lane choice: in band or definite it takes a lane whose certificate does not rest on the class, records nothing, never ends as a defect; where no lane exists yet it refuses lever-only with "not yet", never a tolerance offer); the edge join records its class Zeros (D10). Decision 2 (the torus convention at admission) is split out to `a-torus-faces-convention-is-decided-where-the-face-is-admitted`, waiting on the import-validation conversation.

## Narrowed (PCERT B2a's review, 2026-10-17)

PCERT unit B2a (branch `pcert/windings-b2a`, PR 5244) carries the
answer's first decision for what a row STORES: a harmonic row's angle
channel holds its phase and its windings (`geom_brep::Channel::Angle`,
an `i32`), certification compares the stored windings with the
derivation's exactly (`PcurveCertifyError::WindingMismatch`), and the
classifier, its `pcurve_chart_azimuth_affine` and `pcurve_chart_winding`
decisions and the harmonic lane's `pcurve_azimuth_period` gate are gone.
The near-apex RULING above now certifies
(`topo/tests/cone_apex_sector_joints.rs`), but only because a ruling's
winding is 0 by construction: nothing at its lever decides it.

What is left is the DERIVATION's reading of a winding. A cone rim's
derived winding is the sign of its orientation `a × b · n̂` over the
rim's radius (`pcurve_chart_orientation`, `Margin::over_lever(orient,
rho)` in `derive_class`'s cone rim arm), and both B2a reviewers executed
a rim, not a ruling, at radius `ρ ≈ 0.5·K·ε`: at `5e-9` (`f64` and
`Interval`) and at `1e-8` (`Interval` only) it escalates
`Escalated { check: ChartWinding, predicate: pcurve_chart_orientation }`,
in the mint (`chart_pcurve`) and again in certify's check 4, which
re-derives the image (probes on `review/B2a-r1` and `review/B2a-r2`).
So a winding is still decided in metres at a band-sized lever, on the
rim; the answer's first decision ("never decided in metres") is not
yet true of the derivation.

The question this item now carries: what names a rim's winding at a
radius the band cannot resolve. The carrier's own orientation relative
to the chart's axis is a sign of a unit-vector product, not a length,
so one candidate is to decide it unlevered (`a × b · n̂` normalised by
`|a × b|`), which is a dimension question for the predicate audit; the
other is the D3 lane choice (in band, the class gate takes the
projected lane and records nothing).
