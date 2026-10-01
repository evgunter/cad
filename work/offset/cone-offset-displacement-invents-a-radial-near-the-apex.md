---
id: cone-offset-displacement-invents-a-radial-near-the-apex
kind: issue
title: ConeOffset::displacement normalizes an undecided radial, inventing a direction near the apex
status: open
opened: 2026-10-01
---


## What

`crates/geom-brep/src/offset.rs` `ConeOffset::displacement` takes
`radial = (p - self.apex).reject_from(self.axis).normalize()` and
decides nothing about its length. Its doc covers the apex itself
(`0/0`, poison — D2 row 3, honest). It does not cover a point within
the band of the apex, or within the band of the axis: there the
rejection is a few ε long and `normalize` returns a definite unit
direction made of the point's rounding, which `displacement` then
scales by `d` and hands on as a real offset.

The one caller is `crates/topo/src/replace_face.rs`'s cone transport,
`action.displacement(nappe, mid)` at an edge's midpoint. A generator
edge whose midpoint is near the apex (a short edge ending at it) is
the case that reaches it; this filing did not build that fixture, so
reachability is unmeasured.

## Shape

The Gram–Schmidt step, decided: `UnitVec3::new` on the rejection (or
`OrthoFrame::from_aim_and_reference` against the axis) under an
offset K name, with a typed refusal where it decides zero or
escalates in band, instead of a direction. The value-poison posture at
the exact apex then comes for free. Found by the `linalg`
`decided-not-minted` sweep's second pass (`reject_from(…).normalize()`)
and confirmed in review of PR #3686.
