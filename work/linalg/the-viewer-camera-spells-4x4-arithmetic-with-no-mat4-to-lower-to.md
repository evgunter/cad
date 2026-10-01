---
id: the-viewer-camera-spells-4x4-arithmetic-with-no-mat4-to-lower-to
kind: issue
title: the viewer camera spells 4x4 products and a perspective divide by hand, and geom-core has no Mat4 for an array door to lower to
status: open
opened: 2026-10-01
priority: P3
cost: M
design: true
---


Split out of `geom-core-linalg-has-no-array-doors` by the lane that
landed its array doors (`linalg/doors`), which left this half alone on
purpose: it is not a missing conversion but a missing type.

## What is there

`crates/viewer/src/camera.rs` works in `[[f64; 4]; 4]` and `[f64; 4]`
throughout: `Camera::view_matrix`, `Camera::projection_matrix` and
`Camera::view_projection` return the column-major array; `project`
and `cursor_projection` spell the matrix-vector product and the
perspective divide; the file-private `mul` spells the 4x4 product as
`(0..4).map(|k| a[k][row] * b[col][k]).sum()`. The VGEOM sweep that
found it (`work/vgeom/viewer-array-lowered-vector-ops-escaped-the-hand-rolled-sweep.md`,
closed) recorded it as "no door exists at all".

## The question

`crates/geom-core/src/linalg.rs` declares `Mat3` and `Affine3` and no
4x4 type, and its module docs say the layer adds on consumer demand.
The camera is a consumer, but of a PROJECTIVE map, which is not an
affine map with an extra row: a perspective divide has no place in the
affine/linear split the module is built on, and the viewer narrows the
result to `f32` for the GPU, so the generic `T: Real` story does not
obviously apply either. So the fork is:

- a `Mat4`/projective type in `geom-core` (with its own fixed
  association order, D9) and array doors in the idiom `Mat3` now has
  (`from_cols_array`/`to_cols_array`/`cols`); or
- a viewer-local type that owns the camera's arithmetic once, leaving
  the kernel's linear algebra 3-D; or
- no type, the three hand spellings kept and each said once.

Nothing is wrong at the sites today; the cost is a hand-spelled 4x4
product, where a transposed index is invisible in review.
