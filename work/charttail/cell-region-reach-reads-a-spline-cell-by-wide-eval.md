---
id: cell-region-reach-reads-a-spline-cell-by-wide-eval
kind: issue
title: cell_region's reach evaluates a spline chart at the cell's two wide intervals, which encloses a shallow cell by the whole line, so no NURBS cell clears its boundary
status: open
opened: 2026-10-28
---


## Finding

`cell_region` in `crates/topo/src/cell_region.rs` bounds a cell's patch
by `surface.eval(span(u), span(v))`, the carrier evaluated at the cell's
two intervals. On a spline chart that is `NurbsSurface::eval` at wide
arguments, whose basis recurrence keeps every dependency between the
interval arguments and whose weight sum can straddle zero. Measured by
NURBSWIN on a swept wall (debug build), the enclosure diameter of a cell
of side `2^-k` of the domain about its centre:

- `eval` at intervals: `inf, inf, inf, 2.1e6, 2.04, 0.618, 0.264, 0.125, …`
- `Surface::rect_enclosure` (NURBSWIN's door): `4.72, 2.51, 1.28, 0.64, 0.32, …`

So the reach of a shallow spline cell is infinite and no boundary edge
clears it: a shallow spline cell reads `Undecided`, and a deep one reads a reach several times its own. `cell_region` also cost
5–20 ms a cell on that wall (the spline edges' arc floors and the
spline region door). The clearance engine no longer asks `cell_region`
about a spline face (an iso-bounded NURBS window IS its face), so this
reaches only `cell_region`'s other callers today, which are tests.

The fix is one line: read the reach through
`Surface::rect_enclosure(u, v)`, which is `eval` on analytic carriers.

Filed by NURBSWIN (CLEAR), 2026-10-28.
