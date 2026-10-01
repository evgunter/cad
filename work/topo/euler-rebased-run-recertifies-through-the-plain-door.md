---
id: euler-rebased-run-recertifies-through-the-plain-door
kind: issue
title: certify_rebased_run re-certifies a moved run through the lane-free door, so an M7-8 edge in the run refuses NurbsLaneUnsupported at a scalar that holds the lane
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [graft-recertifies-through-the-narrow-lane]
---


Found by CLEAVE's plane x NURBS lane unit while sweeping for generic
operations that re-certify an edge carrier through the lane-free door.
**Unexecuted; this is a reading of the code.**

`Body::certify_rebased_run` (`crates/topo/src/euler.rs`, the fan
site's re-basing gate) re-certifies every edge of a run about to start
at a moved vertex through `EdgeCurve::recertify`, which supplies no
plane x NURBS lane. An edge of the M7-8 class (an `Intersection` of a
plane and a described NURBS wall) in that run therefore refuses
`CertifyError::NurbsLaneUnsupported` before any check of the edge runs,
at `f64` too, where `AtRestPolicy::nurbs_lane()` holds the lane and the
at-rest validator re-derives the same certificate. The gate then reads
that refusal as a non-endpoint certification failure.

It is the transform's split (closed by the CLEAVE unit that made
`transform_rigid` read `T::nurbs_lane()`) at an Euler door, and it
belongs with the mint-door collapse (`Body::set_edge_curve` reading the
policy), which raises the same bound: `certify_rebased_run` is
`T: Decide` under the Euler surface.

**What to establish first:** whether any public operation reaches a
fan site on a vertex of an M7-8 edge (an `mev` fan on a body whose
wall edges were attached through `set_edge_curve_nurbs_lane`). If none
does, the row closes with a sentence at the gate.
