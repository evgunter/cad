---
id: offset-fit-refuses-every-degree-1-base-direction
kind: issue
title: offset_fit refuses DerivedKnots, a kernel-defect note, on any degree-1 base direction
status: open
opened: 2026-10-10
priority: P2
cost: M
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
