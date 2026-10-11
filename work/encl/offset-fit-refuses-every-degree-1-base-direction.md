---
id: offset-fit-refuses-every-degree-1-base-direction
kind: issue
title: offset_fit refuses DerivedKnots, a kernel-defect note, on any degree-1 base direction
status: closed
closed: 2026-10-11
branch: encl/offset-fit-degree-1
opened: 2026-10-10
priority: P2
cost: M
pr: 4616
---

Found by the NURBS delta review of PR 4485. It is the same on main
through the old `derived_knots`, so it predates that PR.

In `crates/geom-brep/src/offset_fit.rs:2396-2405`, the first-partial
nets come from `TensorCoeffs::derivative_u/v()`, and `None` maps to
`PatchBoundError::DerivedKnots`. `KnotVector::derivative` is `None` at
every degree 1, because the derived vector would have degree 0. So any
base face with a degree-1 direction, such as a ruled or bilinear patch,
refuses here with a note that calls it a kernel defect, even though the
face is valid. Its first partial in that direction is per-span constant
and is representable as a `Loose::Const` level, as the quad lanes
already do.

The fix is either to carry the degree-0 partial (the `Loose` structure
from `props/quad.rs`) or to give the refusal a recourse that does not
call a valid face a defect. Start with a red row on a bilinear base.

## Measured

The premise does not hold on main. `Composite::build` raised every
degree-1 base direction to degree 2 at `f64` before taking any partial,
and the door's C¹ gate (`patch_bound::check_direction`) refuses a
degree-1 direction with an interior knot as `Degree1Crease` before the
composite is reached, so `DerivedKnots` was unreachable from a degree-1
base. A bilinear, a rational bilinear and a ruled base (degree 1 in `u`)
all fitted and certified through `fit_offset_at` and
`certify_offset_at` on main.

What the elevation did cost is the sibling row
`offset-fit-composite-elevates-a-rational-direction-at-f64`: the raised
net's control points are rounded (on a rational bilinear with weights
`1, 0.6` along a ruling, the middle point lands up to 8.5e-17 off the
exact one), so the composite was about a neighbour of the base. The fix
drops the elevation: a degree-1 direction's partial is the span's
constant, decomposed on the direction's own vector, and the base's
channels are raised in the ring instead, exact in ℝ.

