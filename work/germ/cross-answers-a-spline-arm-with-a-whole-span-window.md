---
id: cross-answers-a-spline-arm-with-a-whole-span-window
kind: issue
title: curve_face::cross answers a spline arm with one Enclosed window instead of reading its enclosures
status: open
opened: 2026-11-20
---


Found by U3a (PR 7226). C5 (`crates/geom-brep/README.md`) and
`docs/GERM-CROSSING-U3-SPEC.md` §a.2 make a spline arm's enclosures its
method (`composite::crossings`, `spline_face::isolate` and `settle`), its
in-band pieces and floor boxes windows. `curve_face::cross`
(`crates/geom-brep/src/curve_face/cross.rs`, `cross`) is generic over
`T: Decide`, and those readers need certification arithmetic: they are
reached through `crate::NurbsLane` (`certify.rs`), which `Decide` does
not carry. So `cross` answers `Door::SplineComposite` and
`Door::SplineFace` with one `CrossWhy::Pose(PoseRefusal::Enclosed)`
window over the whole span, and the sweep keeps reading the spline arms
through its own `NurbsLane` call.

Consequence: a reader of `cross` (the trace, the census touch confirm)
refuses a spline pair whole where the enclosures would have answered.
The trace refused it before too (`TraceFault::Unsupported`); the census
refuses it typed.

## What would close it

`cross` reading the spline arms where the scalar certifies (a bound on
`NurbsLane`, or a `cross_certified` entry the sweep and the census read),
their windows the enclosures' own; rows that a spline span crossing an
analytic face, and one crossing a spline face, are answered through
`cross`.
