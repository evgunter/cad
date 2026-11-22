---
id: a-vertex-only-a-spline-wall-fixes-refuses-not-yet-supported
kind: issue
title: A vertex only a spline wall's root fixes refuses not-yet-supported
status: open
opened: 2026-11-18
priority: P3
---


Found by `importgate/build` (PR 7128), the fix pass's ruling m1.

## What

The build fixes a vertex its description fixes by a root of a surface
around it along a built carrier through it (`rooted` in
`crates/step-import/src/build.rs`, through `topo::root_along`). Two
roots have no construction:

- a spline wall's root along any carrier: the wall has no implicit form,
  so Newton reads a sine that is no number (`CornerVerdict::Graze` with a
  NaN sine);
- any surface but a plane along a spline carrier
  (`CornerVerdict::Unsupported`).

Where every root asked at a vertex is one of these, the vertex refuses
`StepImportError::NotYetSupported` naming this row (`ROOT_LANE`) rather
than taking its printed point's foot. The unit row
`build::tests::a_vertex_only_unsupported_roots_fix_refuses_naming_the_root_lane`
is a wall meeting two cylinders. No file of the committed corpus reaches
it at PR 7128's head. The conic quarter torus's knot-split corner did
before the build read it as an end of a wall's whole iso (`iso_end`).

## What would close it

A root of a spline wall along a carrier (the wall's point-inversion
along the curve, certified), and an analytic surface's root along a
spline carrier, in `topo::root_along`.
