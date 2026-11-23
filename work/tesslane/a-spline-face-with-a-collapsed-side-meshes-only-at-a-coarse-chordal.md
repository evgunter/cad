---
id: a-spline-face-with-a-collapsed-side-meshes-only-at-a-coarse-chordal
kind: issue
title: a spline face with a collapsed side meshes only at a coarse chordal: the trimmed lane's certificate reads the side's whole span as one triangle
status: open
opened: 2026-11-22
priority: P1
cost: H
refs: [a-nurbs-wall-with-a-collapsed-boundary-column-has-no-loop-joint]
---


Filed by CHARTTAIL's `charttail/collapsed-column` lane. A loft can now
build a wall with a collapsed side: a pinned corner is one vertex, and
the walls beside it are triangles of their charts, collapsed at a
boundary column (`geom_brep::chart_singularity::singular_points`,
`Singularity::Side`). The mesher meshes them only at a coarse chordal.

## Measured

On the bodies of `crates/sweep/tests/pinned_corners_build.rs`, through
`mesh::tessellate`:

- **The two-section wedge** (a square turned 0.5 rad about its edge
  x = −1). Its triangle walls are planar. At chordal 0.1 it meshes. At
  1e-2 it refuses `CertificateExceeded { bound: 0.0165 }`, and at 1e-3
  `bound: 0.0052`.
- **One corner pinned** (turned about the diagonal through it). At 1e-2
  `bound: 0.0117`, at 1e-3 `bound: 0.0038`.
- **A hinge in one slab of three.** At 1e-2 `bound: 0.0103`, at 1e-3
  `bound: 0.0032`.

## Why, as far as the lane read

`crates/mesh/src/trimmed.rs`'s spline arm reads no singular vertex
(`singular_vertices` runs on an `AnalyticRow` only). The polygon's
side along the collapsed column is one UV chord from `(0, 1)` to
`(0, 0)` between two copies of the pole vertex, with no point between
them. The CDT's triangles on it span the whole column in `v`, and
`NurbsFaceBound::cert` (`nurbs_cert.rs`, `Q/4` over the triangle's UV
box) bounds them by that span whatever their 3-D size, which is zero
along the column. The analytic arms already treat a pole row as a side
of the polygon (`singular_row_shape`, `pole_side_columns`, the
identified-vertex fan census).

## Fix

Read a spline face's collapsed sides from the home, keep the pole
vertex at each `v` the polygon walks the side through (the analytic
pole rows' shape), and size the side's points as a pole row's are.
Until then the GUI renders a pinned loft only at a coarse chordal.
