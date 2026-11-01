---
id: cylinder-axis-rows-decide-tilt-and-gap-one-at-a-time
kind: issue
title: the cylinder and cone×cylinder classifiers decide the axis tilt and the gap one at a time, not their sum
status: closed
opened: 2026-10-06
priority: P2
cost: M
closed: 2026-10-07
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

- `plane_cylinder_ruled` (`crates/geom-brep/src/intersect.rs:921`)
  decides `pc_axis_plane_parallel` (the tilt levered at the reach),
  then `pc_parallel_gap` (`r − |gap|` at the foot) separately.
- `cylinder_cylinder_section` decides `cc_axes_parallel`
  (`cylinder_axes_parallel`), then `cc_parallel_gap`
  (`parallel_cylinder_gap`, `intersect.rs:1640`) separately.

- `cone_cylinder_section` decides `coc_axes_parallel` (the tilt
  levered at the extent from the apex), then `coc_coaxial` (the apex's
  distance from the cylinder's axis) separately, and serves the coaxial
  circles when both read Zero.

Each row can sit just inside its Zero band, so `TangentLine` can be
minted for a pose whose ruling stands up to about `2ε` off one wall
across the reach, and the coaxial circles for a cylinder whose axis
stands up to `d + θ·extent`, about `1.8·zero` in PR 4231's review
differential, off the cone's.

**P2, not P3** (PR 4231's review, MINOR-2): the served side of each of
these is a verdict in the band. PR 4231 read the cone arm at the apex
and serves more poses than main did, which raised the measured exposure
3.4×.

## The shape of a fix

Decide the served verdict on one margin that carries both terms
(`d + θ·extent` on the served side), as
`carrier_cyl_reach` does: for example `r − |gap| ± tilt·lever` read as
a bracket, keeping the tilt row only to route between the parallel and
crossing lanes. The witness lane (`locus.rs`) reads the same helpers
and follows.

## Outcome (2026-10-07)

Each section row that serves a verdict on a position datum beside a
term its routing row admitted decides it across the reach
(`decide_across`, `crates/geom-brep/src/intersect.rs`): the zero side
on `|datum| + swing` (the farthest a consumed point of the served
ruling or circle stands off), each definite side on the datum shrunk
toward zero by the swing (its `_floor` row), a straddle escalating
through the gate. The routing row keeps its own lever. The swing is
each row's own: the tilt levered at the reach (`pc_parallel_gap`,
`pt_spiric_two_ovals`, `pt_cap_gap`), the axes' distance's exact range
over the reach (`cc_coaxial`, `cc_parallel_gap`,
`tangent_locus_internal_gap`, `coc_coaxial`), or the apex gap
(`pn_apex_section`). `pt_axis_plane_gap` reads its gap alone: the tilt
turns the meridian circles along the torus. Rows:
`axis_rows_read_as_one_sum`; the differential:
`one_sum_differential`. The sweep's siblings are filed on the slates
whose ground they land on, and the levers the differential found
over-long on this one.
