---
id: the-vertex-alone-plane-leans-along-a-cells-long-metric-side-so-an-anisotropic-pair-splits
kind: issue
title: planes_through's plane at a shared vertex alone is taken across the rectangles' midpoint headings, so on an anisotropic chart it leans along the cell's long metric side and a strictly separable pair splits to the floor
status: open
opened: 2026-11-17
priority: P2
cost: M
refs: [departure-reads-no-curved-corner-off-its-shared-edge]
---

## Finding

`topo::injective`'s `planes_through`, at a vertex the pair shares
alone, builds one plane: `n = unit(hx − hy)`, where `hx` and `hy` are
the headings from the vertex to each presentation's rectangle midpoint
(`heading`). That heading is a midpoint's, not the wedge's. On a chart
whose speeds differ a lot, a cell that is square in the chart is long
and thin on the surface, and the heading points along its long side.

## Evidence

Measured on the review's thin-cylinder fixture (PR 7093's fix pass, a
scratch row not kept). The fixture is two diagonal quadrants at a
vertex alone on a cylinder of radius 0.055, the same pair that
certifies on radius 1. Cells were chart-square, of half-width 0.02
and 0.002.
- Both faces are presented as their quadrants (`Zero::Point`).
- The one plane is `n ≈ (0.046, −0.030, 0.998)`, almost the ruling's
  direction. The quadrant's metric extent along the ruling is 0.020 m,
  against 0.0011 m along the parallel.
- The leaned axis `B` has `z = −0.25`, so `n·B < 0` while `n·A > 0`.
  `sign_on` gives `None` on both faces, and the pair is
  `Split { Departure }` at every half-width tried (0.02, 0.002; the
  review also tried 0.0005 and 0.00005).
- The quadrants are strictly separable, for example by the bisector of
  the two wedges' edge directions.

No body measured refuses on this. The re-posed torus belly
(cylinder radius 0.055) certifies, because its cells reach the vertex
wider along the parallel.

## What a fix needs

A vertex-alone candidate taken from the presentations' wedges rather
than their midpoints. For example, `n` across each face's two corner
directions (the images of the quadrant's axes at the vertex), as the
two-line case already builds one from the edges' directions. Any
direction is exact, so this is a choice of candidate, not a decision.

## Instances in the per-solid census (EMBED trace lane, 2026-11-21)

Traced on main `c2a0c41f40` with the at-rest gate and prints of
`departure`'s separators (`embed/trace-cone-torus-measure`). Two pairs
sharing a vertex alone, both faces presented by a `Zero::Point`, split
to depth 40 with `planes_through`'s one plane signing the plane face and
not the curved one, and no carrier or graph separator:
- `band_open_mixed_chain::open_mixed_chains_chamfer_whole_at_pappuss_volume_at_interval`
  (`Interval`, plane `3v1` against a chamfer's cone `16v1` at `19v1`):
  `planes = [(Some(Positive), None)]`, the cone's cell
  `u ∈ (3.926991, 3.926991 + 9e-13)`, `v ∈ (0.42371, 0.42426)`;
- `band_planar_cut_off_interval::the_box_rows_carve_at_the_certified_scalar_and_bracket_their_closed_forms`
  (`Interval`, plane `3v1` against sphere `10v3` at `16v1`): the same,
  the sphere's cell `v ∈ (−2.2e-15, 0)` by `6.1e-3` in `u`.

Both curved cells are anisotropic by many orders, the shape this row
describes. That the plane leans along the long side is not printed, so
the match is unchecked. Assigned here by shape, 14 tests: the vertex,
plane/cone bucket (4: the chamfers, three at `Interval`), plane/sphere
(9) and sphere/plane (1). Two traced. Valid bodies.
