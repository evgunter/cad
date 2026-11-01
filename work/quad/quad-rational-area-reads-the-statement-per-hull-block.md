---
id: quad-rational-area-reads-the-statement-per-hull-block
kind: issue
title: quad: the rational area rule reads its pads and hull per hull block; per area cell they are 1.5-7x narrower and cost 7-20 s a face in a debug build
status: open
opened: 2026-10-22
priority: P3
cost: M
---


## Found (QUAD, `quad/rational-quotient-statement`, 2026-10-22)

`crates/geom-brep/src/props/quad/rational.rs` `rational_patch_face` reads the
area rule's pads `G_u`, `G_v` and its hull of `g = |S_u×S_v|` off the
quotient statement once per hull block (`HULL_BLOCKS_PER_SPAN`
per knot span), not per area cell (`AREA_CELLS_PER_SPAN` = 32 a span).
Measured on the same branch, debug build, area width:

| carrier | main | per hull block (shipped) | per area cell |
|---|---|---|---|
| quarter cylinder, one span | 0.1687 | 0.0660 | 0.0154 |
| two-span quarter cylinder | 0.1221 | 0.0554 | 0.0078 |
| `rational_wall` (cubic × linear) | 0.1237 | 0.0504 | 0.0325 |

The per-cell reading cost 7 s (quarter cylinder) to 20 s (quarter
torus) per face in a debug build: 4096 restrictions of the degree
`(4p, 4q)` slope grids and the `(3p, 3q)` area grid. Per hull block it
is 64 restrictions. Levers, untried: restrict along `u` once per cell
column and along `v` per cell (the grids are tensor products); halve a
block's grids into its cells where the cells are its dyadic halves.

At this width the area bracket is a few percent of the area, and the
convergence meter reads only its midpoint, so no carrier's verdict
waits on it today.

Also seen there: on every carrier measured the per-cell HULL of `g`
was narrower than the per-cell pad. The pad bounds `|∂_d |c||` by
`|∂_d c|`, which carries the turning of the normal as well as the
change of its length; on a cylinder the turning is most of it.

## Re-checked on main `9267d3ae` (QUAD tracker sweep, 2026-10-27)

Still holds. `rational_patch_face`
(`crates/geom-brep/src/props/quad/rational.rs:360`) still reads
`measure.area_bounds` once per hull block (`:456`). The area cells
look their block up by `advance_block` (`:469`–`470`) under
`AREA_CELLS_PER_SPAN` (`props/quad/area.rs`).
