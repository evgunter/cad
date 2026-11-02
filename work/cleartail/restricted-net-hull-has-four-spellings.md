---
id: restricted-net-hull-has-four-spellings
kind: issue
title: the hull of a NURBS patch's control net restricted to a parameter rectangle is spelled four times, and none of the other three serves rect_enclosure as it stands
status: open
opened: 2026-10-28
---


## Finding

"The hull of a patch's control net, or of the net restricted to a
parameter rectangle" has four spellings:

- `restricted_hull` in `crates/geom/src/surfaces/nurbs.rs`, behind
  `NurbsSurface::rect_enclosure` (NURBSWIN): the window's homogeneous
  net restricted to an ARBITRARY sub-rectangle of one knot cell
  (`SpanRow::restrict`, along `u` then `v`), dehomogenized, hulled.
- `Grid::restricted_along` in `crates/geom-core/src/spline/derivative.rs`:
  the same restriction, by `split_restrict`, on a cell block's Bernstein
  grid of the second-quotient numerators. It is private, and takes
  rows already in Bézier form on the cell, so it runs after a cell
  decomposition.
- `PatchSpans::cell_hull` in
  `crates/geom-core/src/spline/compose/patch.rs`: the hull of a WHOLE
  knot cell's Bernstein coefficients, from `PatchSpans::decompose` of
  the whole net. There is no sub-rectangle, and decomposing the whole
  net per query is the cost a per-cell enclosure cannot pay.
- `patch_bound::window_hull` (`TensorNet::window_hull`) in
  `crates/geom-brep/src/patch_bound.rs`: the hull of the B-spline
  window, unrestricted, which does not narrow below one knot cell.

**The obstacle** to folding `rect_enclosure` onto any of them: only
`Grid::restricted_along` restricts to a sub-rectangle, and it is a
private step of the derivative statement over a pre-decomposed cell
block. The other two do not restrict at all, so their enclosure stops
narrowing at a knot cell, which is exactly the defect
`rect_enclosure` fixes. One home would be a public "restrict a span
window's tensor net to a sub-rectangle" door in `geom-core::spline`,
which `rect_enclosure` and `Grid::restricted_along` both read.

Filed by NURBSWIN (CLEAR), 2026-10-28.
