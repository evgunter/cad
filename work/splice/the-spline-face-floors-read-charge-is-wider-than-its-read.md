---
id: the-spline-face-floors-read-charge-is-wider-than-its-read
kind: issue
title: The spline face placement's floor charges its read at the points' reach, wider than the read rounds
status: open
opened: 2026-11-04
priority: P3
cost: E
---



Filed by `splice/spline-region-charge`. Its fix pass narrowed this term, and the review asked for the reviewed term back. The narrowing is filed here, unbuilt.

## What

`topo/src/spline_region.rs` `Patch::floor` charges its read
`rounding_charge(2·(reach + |q − c| + |c|))`, which is `geom_brep`'s spline
incidence's term. The read's own rounding is narrower:

- the move `q − c` rounds each coordinate by `u·|q_c − c_c|`;
- each axis difference rounds by `u` of itself, and is exact (Sterbenz) against a face near `q`;
- the squares, sum and root round by a few `u` of the gap.

The points' quotients belong to the cut's bound (`point_rounding`, at least `u·|P_c|` per point).
So `rounding_charge(|q − c| + gap)` charges the read twice over. The reviewed
term also charges `|c|`, which the read never rounds at (every coordinate is
held about `c`), and the points' reach, which the cut's bound already holds.

The wide term costs Offs at the band's edge: a point within
`16u·(reach + |c|)` past the band reads unsettled where it could read Off. At
`1e5`, that is about `2e-11`, two escalations at the 1e-12 row.

## Fix

Narrow it to `rounding_charge(|q − c| + gap)`. It needs its own review, with:
- a row measuring the read's drift (the moved point and the gap against double-double);
- the composition row `a_cells_floor_is_lowered_by_its_cut_and_its_read` re-sized to the new term.

`geom_brep`'s spline incidence charges the same term; one home for both is
`work/splice/the-spline-chart-cell-walk-is-written-twice.md`.
