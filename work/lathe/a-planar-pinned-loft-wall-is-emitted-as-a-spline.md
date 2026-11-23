---
id: a-planar-pinned-loft-wall-is-emitted-as-a-spline
kind: issue
title: a loft wall beside a pinned corner that is a planar triangle is emitted as a NURBS wall, not a plane
status: open
opened: 2026-11-22
priority: P3
cost: M
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint]
---


A lead raised by the collapsed-column design fork (design-fork row 296)
and confirmed by the `charttail/collapsed-column` lane. A
two-section loft of a square turned about one of its edges
(`crates/sweep/tests/pinned_corners_build.rs`,
`the_two_section_wedge_builds_as_its_prism`) is a triangular prism.
The two walls beside the hinge are planar triangles, each a ruled wall
between two segments that share a corner and lie in one plane. The loft
emits them as `Surface::Nurbs` walls with a collapsed side, not as
planes. That is not wrong: the body is valid at rest and measures its
closed form. It is a description the kernel's own analytic kind could
state more exactly, as it does for extrude's walls.

## Fix

Decide whether the loft recognizes a planar wall (a ruled wall between
coplanar section segments) and states it as a plane. That is a
prefer-intrinsic question for the loft's walls in general, not only
beside a pinned corner.
