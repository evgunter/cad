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
  confirm it returns `Unsettled(None)` there). geom-brep's refines the homogeneous
  net through the kernel's insertion plans in the scalar (`refined`:
  `plan_inputs::refine_plan` + `CurvePlan::apply_homogeneous`).
- topo's `Patch::extent` reads `value_channel`, NaN for a wide cell, so at
  `Interval` a kept cell would be a leaf at depth 0 (read, not run).
- geom-brep's floor adds the slab bound and an `f64` rounding charge for the cut;
  topo's reads the box alone, uncharged
  (`work/splice/spline-region-decides-off-the-carrier-on-an-uncharged-f64-box-floor.md`).
- The caps drift: geom-brep's walk halves a cell across its longer way up to 96
  times and visits 4096 cells a sample (`CELL_DEPTH`, `CELL_VISITS`); topo's
  quarters up to 48 times and visits 8192 (its `CELL_DEPTH`, `CELL_VISITS`).

Every home of a cell cut or a cell hull a single home would take in (review
fix Y12, PR 6498):

- `geom-brep/src/pcurve_cache/spline_incidence.rs`: `Net::of`, `refined`,
  `Cell::split`, the floor (box and slab) and its `f64` charge;
- `topo/src/boolean/spline_region.rs`: `bezier_cells`, `refine_rows`,
  `Patch::split`, `Patch::hull_gap`;
- `geom-brep/src/patch_bound.rs`: `patch_cells`, `patch_cells_across_creases`,
  `patch_cells_refined` (an `f64` chart's partials per cell);
- `geom-core/src/spline/compose/patch.rs`: `PatchSpans::decompose`, one
  interval channel per cell;
- `geom-brep/src/ssi/enclose.rs`: `NurbsBoxes`;
- the refinement routes: `geom_core::spline::RingRefinement` and
  `refine_plan_homogeneous` + `CurvePlan::apply_certified` (one `Interval`
  channel), and `plan_inputs::refine_plan` + `CurvePlan::apply_homogeneous` (any
  scalar).

Move the cells, the cut and the floor to one home in geom-brep and read them from
both.
