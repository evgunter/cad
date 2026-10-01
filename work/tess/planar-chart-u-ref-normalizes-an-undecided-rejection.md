---
id: planar-chart-u-ref-normalizes-an-undecided-rejection
kind: issue
title: the planar chart frame's u_ref normalizes an undecided rejection
status: open
opened: 2026-10-01
---


## What

`crates/mesh/src/planar.rs` (the planar chart frame, ~:382) takes
`u_ref = far.reject_from(normal).normalize()`, where `far` is the
boundary vertex farthest from the frame origin. The rejection's
length is not decided: a face whose vertices all lie within the band
of the origin, or whose farthest vertex sits along the normal, gives
a residual that is zero (poison through `reject_from`'s 0/0) or in
the band (a definite direction made of rounding).

`f64` only, and the tessellator may refuse such faces earlier; this
filing did not build a fixture, so the reachability is unmeasured.

## Shape

The Gram–Schmidt step, decided: `UnitVec3::new` on the rejection (or
`OrthoFrame::from_aim_and_reference` against the normal) under a
tessellation K name, refusing typed. Found by the
`linalg/decided-not-minted` sweep's second pass, which looked for
`reject_from(…).normalize()` after the first pattern
(`- a * a.dot(x)).normalize()`) could not see the helper spelling.
