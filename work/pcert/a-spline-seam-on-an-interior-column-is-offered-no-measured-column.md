---
id: a-spline-seam-on-an-interior-column-is-offered-no-measured-column
kind: issue
title: the boundary-iso schedule offers its measured column to a line only, so an Intersection spline seam on an interior column takes General though the seam class certifies a spline's interior column
status: open
opened: 2026-10-14
priority: P3
cost: E
---


Found by the review of PR 5042 (`pcert/wall-seam-column-pick`, its S2),
measured on that branch.

## What happens

`nurbs_iso_derive`'s boundary-iso schedule (`crates/topo/src/pcurves.rs`,
the `iso_schedule` closure, its measured-map step) offers the measured
COLUMN to a `Line` carrier only (`[cu0, cu1, f0.x].filter(|_| line)`).
The reason its comment used to give was that the seam class compares
the image against the chart's boundary net, so a spline's interior or
partial column would refuse structurally. That is not so: the seam
class certifies a spline's column by de Boor collapse at the column's
`u`.
- `sweep` `r1_p2_probes::r1_wall_seam_arm_mints_and_certifies_the_interior_column`
  certifies a spline seam's interior column on a chart widened in `u`.
- A probe on PR 5042's branch split that fixture's spline seam at
  `t = 0.37`: both halves derive `IsoLine` on their restricted maps,
  `PcurveCache::certify` accepts each, and `mint_pcurves` is `Ok`.

So a spline seam stated as a chart image on its neighbour reaches the
exact class (the wall-seam arm states the column), while the same seam
described as an `Intersection` gets no measured column and takes
`General` (#498's executed case).

## Why it is P3

Nothing wrong reaches rest: the `General` image is a certified row at
the honest Fitted grade. The cost is an exact image a description form
does not get.

## Fix

Offer the measured column to a spline carrier too (drop the `line`
filter), measured over the same certification, and pin the
`Intersection` interior-column spline seam taking `IsoLine`.
Re-measure every row that pins #498's `General` image first.
