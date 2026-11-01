---
id: nurbs-cell-bounds-cert-drops-a-nan-vertex-and-lands-a-nan-end-on-the-first-cell
kind: issue
title: mesh nurbs_cert: NurbsCellGrid::cert drops a NaN vertex through f64 min/max, and cell_lo/cell_hi land a NaN end on the first cell
status: open
opened: 2026-10-09
---


Found by the NURBS span-locator unit's §5 shape sweep (branch
`nurbs/span-locator-refuses-poison`), which made the spline locator
refuse a NaN parameter and every spline window a `ParamRange`. This is
the same shape in a hand-rolled cell locator outside the spline types.

## The finding

`crates/mesh/src/nurbs_cert.rs`:

- `NurbsCellGrid::cert` builds the triangle's UV box with `min3`/`max3`,
  which are `f64::min`/`f64::max` chains. Those DROP a NaN operand, so a
  triangle with one NaN vertex coordinate is boxed over its other two
  vertices.
- `NurbsCellGrid::cell_lo` / `cell_hi` locate a box end with
  `partition_point(|c| *c <= x)` / `partition_point(|c| *c < x)`; both
  predicates are false for a NaN `x`, so a NaN end lands on cell 0, the
  first cell, by the same tie-break the spline locator used to have.
  `NurbsCellGrid::row_of`, the band schedule's v-band locator, reads
  through `cell_lo` too.
- `NurbsFaceBound::cert` reads its extents `au`/`av` with the same
  `min3`/`max3`, so with one NaN vertex they are the other two
  vertices' extents, finite.

So a triangle with a single NaN vertex coordinate gets a finite `Q/4`
certificate read off the cells its two numeric vertices touch: a
certificate for a triangle that is not the one asked about. With every
coordinate NaN the extents are NaN and the certificate is NaN, which
refuses.

## Why it is latent

The UV triangles come from the trimmed lane's own grid and the pcurve
trim polygon; a NaN vertex would need a poisoned pcurve evaluation that
earlier doors refuse. No fixture known to this lane reaches it.

## Repair

Refuse a box with a NaN vertex coordinate where the box is built
(`cert` answering NaN, the certificate's own refusal), so the locator
never sees one; or give the cell locator the spline locator's shape,
`None` for NaN. The NaN-dropping `min3`/`max3` is the sharper half:
`f64::min` laundering a NaN is what the module-level NaN policy in
`geom_core::real` forbids for the scalar `min`/`max`.
