---
id: the-spline-chart-cell-walk-is-written-twice
kind: issue
title: The spline chart's Bézier cell walk is written twice, in geom-brep and topo
status: open
opened: 2026-11-04
priority: P3
cost: M
---


Filed by `pcert/spline-chart-incidence`. `geom-brep/src/pcurve_cache/spline_incidence.rs`
(a carrier's incidence on a spline chart) and `topo/src/boolean/spline_region.rs`
(a point against a spline face) each cut the chart into Bézier cells and walk them
by a hull lower bound and a chart-point upper bound. Differences worth one home:

- topo's `bezier_cells` refines through `NurbsCurve3::new` + `refine_knots`, which
  refuses the wide weights the first pass leaves at `Interval` (met when
  geom-brep's port of it did; topo's own walk was not run at `Interval` to
  confirm it returns `Unsettled(None)` there); geom-brep's inserts in the homogeneous net
  directly (`inserted`).
- topo's `Patch::extent` reads `value_channel`, NaN for a wide cell, so at
  `Interval` a kept cell would be a leaf at depth 0 (read, not run).
- geom-brep's floor adds the slab bound and an `f64` rounding charge for the cut;
  topo's reads the box alone, uncharged.

Move the cells, the cut and the floor to one home in geom-brep and read them from
both.
