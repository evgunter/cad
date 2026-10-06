---
id: cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time
kind: issue
title: the cylinder classifiers decide the axis tilt and the gap one at a time, not their sum
status: open
opened: 2026-10-06
priority: P3
cost: M
---


Found by PR 4118's full review (MINOR-4). This is pre-existing: the PR
moved where the rows are read, not how many are decided.

## What

`crates/geom-brep/src/extent.rs` (module docs) says a reader that
bridges a residue decides the SUM of the position datum at the pivot
and the tilt levered from it, "never the two terms one at a time (each
just inside the band would sum to nearly twice it)".
`topo::boolean::carrier_eq`'s cylinder pair does
(`carrier_cyl_reach`, `crates/topo/src/boolean/carrier_eq.rs`: `apart`
plus the levered tilt in one margin).

The section classifiers do not:

- `plane_cylinder_ruled` (`crates/geom-brep/src/intersect.rs:915`)
  decides `pc_axis_plane_parallel` (the tilt levered at the reach),
  then `pc_parallel_gap` (`r − |gap|` at the foot) separately.
- `cylinder_cylinder_section` decides `cc_axes_parallel`
  (`cylinder_axes_parallel`), then `cc_parallel_gap`
  (`parallel_cylinder_gap`, `intersect.rs:1599`) separately.

Each row can sit just inside its Zero band, so `TangentLine` can be
minted for a pose whose ruling stands up to about `2ε` off one wall
across the reach.

## The shape of a fix

Decide the tangency on one margin that carries both terms, as
`carrier_cyl_reach` does: for example `r − |gap| ± tilt·lever` read as
a bracket, keeping the tilt row only to route between the parallel and
crossing lanes. The witness lane (`locus.rs`) reads the same helpers
and follows.
