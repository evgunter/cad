---
id: spline-region-decides-off-the-carrier-on-an-uncharged-f64-box-floor
kind: issue
title: The spline face placement decides a point off its carrier on an uncharged f64 box floor
status: closed
opened: 2026-11-04
priority: P1
cost: M
branch: splice/spline-region-charge
closed: 2026-11-04
pr: 6530
---


Filed by `pcert/spline-chart-incidence` (PR 6498, review fix Y9, R1's MINOR-5).

## What

`topo/src/boolean/spline_region.rs` `read_capped` decides a point **off the
carrier** when every Bézier cell's floor decides past the band (`decide(CARRIER,
Margin::of(cell.hull_gap(q)), band)`). `Patch::hull_gap` is the distance to the
control points' box. At `f64` that box is computed from points cut by
`refine_knots` and de Casteljau (`Patch::split`), and nothing charges their
rounding.

PR 6498's spline incidence test measured the same uncharged `f64` box floor
exceeding a sample's distance from its chart. At band `1e-12`, on patches
`1e2`–`1e5` across or placed up to `1e5` away, it found false `Off`s past the
distance of up to 5.8e-12, the review's probe `probe_corner_tight` (913 of 4000
corner samples), and
`spline_incidence::rows::the_f64_charge_holds_where_its_rounding_is_a_bands_size`
under the `spline-charge-dropped` mutant. Here a false "off the carrier" places
a point outside a face it lies on. The kernel runs ε = 1e-12 (the nightly's
1e-12 row) and admits bodies of that scale, so the case is reachable.

## Fix

Charge the cut's rounding, as `geom_brep::pcurve_cache::spline_incidence` does:
a per-point first-order bound tracked through every blend (`blend`,
`point_rounding`), and `rounding_charge` of the reach for the read. Better,
read both walks through one home (`work/splice/the-spline-chart-cell-walk-is-written-twice.md`).

## Closed (SPLICE, PR 6530, 2026-11-04)

`topo::boolean::spline_region` charges the f64 cut's rounding under both of its readings: the carrier floor and the trim's boxes.
- **What changed.** Each homogeneous control point carries a first-order bound through every blend of the insertion (`refine_plan` + `apply_homogeneous`) and of de Casteljau. `Patch::floor` replaces `hull_gap`: the box gap less the root sum of the worst per-channel bound, and less the reviewed read charge `rounding_charge(2·(reach + |q − c| + |c|))`. The structure_positive weight guard returns poison, and a NaN or infinite bound refuses (no box, a poison floor). `Trim::copy`'s pieces carry the same bound, at every scalar: an Interval piece is charged its half-width, and the iso ends and the Fitted run's ends are charged too. `close_gap` compares homogeneous forms. The walk is split out as `carrier_walk`, which `read_capped` calls.
- **What it fixes.** PCERT's charge is copied, not moved, because a live `pcert/spline-certificate-door` branch edits `spline_incidence.rs`; the P3 row `the-spline-chart-cell-walk-is-written-twice` names this as the second copy. Phase 0's probe falls from 133 false Offs to 0, and a double-double reference bounds the drift at 0.95–0.98 of the bound.
- **Review.** Single FULL review: APPROVE-WITH-FIXES, its MAJOR a test gap. The fix pass gave every charge site a named row, ten mutants in all. No ending moved: topo's slow set (three inherited reds only), sweep `/spline/` at 1e-6 and 1e-12, and the spline-operand census, line-identical with main.
- **Residue filed:**
  - P2: `charttail/the-spline-chart-arms-read-an-uncharged-f64-cut`, `hone/the-spline-arc-pieces-read-an-uncharged-f64-restriction`, `isotail/the-edge-piece-tube-certifies-an-uncharged-f64-restriction`, `pcert/the-projected-frame-box-reads-an-uncharged-f64-restriction`;
  - P3: `the-spline-face-floors-read-charge-is-wider-than-its-read`.
