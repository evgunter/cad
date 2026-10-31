---
id: infer-outer-reads-an-arcs-sag-off-a-sample-polygon
kind: issue
title: step-import's infer_outer tests one probe per ring against the other ring's per-edge sample polygon, so a probe in an arc's sag reads Out and the face refuses NoUniqueOuter
status: open
opened: 2026-10-01
priority: P3
cost: E
---


Filed from the review of PR 3660 (CLEAVE), whose sweep looked for
regions read off a polygon through sample points where the boundary
bears arcs.

## The site

`step-import/src/chart.rs` `infer_outer` decides which STEP bound is
the outer one. It charts each ring's samples
(`entities.rs` `ring_samples`, `PER_EDGE = 16` points per edge), then
for each candidate tests ONE probe per other ring (that ring's first
chart sample) with `contains(&candidate.poly, probe)`, which is a
polygon test over the candidate's samples. When the probe sits in an
arc's sag, between the arc and the polygon through its 16 samples,
it reads Out. The candidate then fails `contains_all`, and when no
candidate holds every other ring the face refuses
`OuternessRefusal::NoUniqueOuter`.

## Posture

It refuses typed; I know of no input where it gives a wrong answer.
(A probe within the eps budget of the sample polygon refuses
`UndecidableAtEps` first, also typed.) Hence P3: it is a completeness gap on
imported faces whose hole hugs a curved outer bound.

## What would close it

Read the probe against the candidate's boundary on its own curves
(as `topo`'s `point_in_carrier_loop` does), or densify within the sag
bound, or test a probe chosen off the sag. Unmeasured on a real file:
a minimal repro is a planar face whose hole's first sample lies
within the sag of a 16-sample outer arc.
