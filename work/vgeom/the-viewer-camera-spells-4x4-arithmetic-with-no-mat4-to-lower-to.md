---
id: the-viewer-camera-spells-4x4-arithmetic-with-no-mat4-to-lower-to
kind: issue
title: the viewer camera spells 4x4 products and a perspective divide by hand, and geom-core has no Mat4 for an array door to lower to
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [camera-project-answers-with-a-screen-position-for-a-projection-that-overflowed]
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

## The design (a designer pair converged, 2026-10-01)

LINALG ran two designers on the problem independently. Both landed on
the same answer: **no `Mat4` anywhere.** The camera never needed a
general 4×4. It needs:

- **A typed view.** `Camera::view() -> Affine3<f64>` is the rigid
  world→view map. It is built exactly as `Rᵀ`, `Rᵀ·(origin − eye)`,
  with `R = Mat3::from_cols(right, up, −forward)`, and never through
  `Affine3::inverse`.
- **A viewer-local perspective value.** It holds three numbers:
  `sx = cot(fov/2)/aspect`, `sy = cot(fov/2)` and `near`. It is
  formed once, through `Camera::perspective(aspect) -> Result<…>`,
  and that is the one place an aspect is refused. Its doors:
  - `project(view_point)` does the divide. It refuses when `w ≤ 0`
    or the result is non-finite, and it is where
    `camera-project-answers-with-a-screen-position-for-a-projection-that-overflowed`
    lands.
  - The inverse on direction (`ray_through`'s algebra), so `project`
    and `ray_through` stop spelling the frustum twice.
  - The GPU matrix as a column map over `Affine3::cols()`. Column `j`
    is `[sx·cⱼ.x, sy·cⱼ.y, near·wⱼ, −cⱼ.z]`. No general product and no
    index sum.
- **Deleted:** the file-private `mul`, `view_matrix` and
  `projection_matrix`. `cursor_projection` is untouched: it edits the
  f32 GPU matrix per column, which was settled by `vgeom/f32-seam`.
- **Rejected:**
  - A geom-core `Mat4`/projective type. No kernel consumer exists; the
    divide needs a comparison that `Real` does not carry; and it would
    represent non-projections where a projection is meant.
  - An external matrix crate. It is f32, and would compute before the
    f64→f32 seam.
- **Siblings for the same lane in `camera.rs`.** `eye()`, `forward()`
  and `ray_through`'s sums spell `Point3 + Vec3·s` by hand. That is
  VGEOM's hand-vector class, and the rewrite should take them too.
- **Bits.** Some f32 GPU entries may flip a zero's sign, and
  `project`'s outputs may move in their last ulps. Re-baseline both and
  say what moved.

Moved from `work/linalg/` when LINALG closed. The work is entirely in
`crates/viewer/src/camera.rs`.
