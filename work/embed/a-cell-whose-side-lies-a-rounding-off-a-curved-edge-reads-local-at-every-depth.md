---
id: a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth
kind: issue
title: A cell whose side lies a rounding off a curved edge it does not hold is read by no presentation at any depth, so the collinear-cap drum's own body runs out of depth at f64
status: open
opened: 2026-11-17
priority: P0
cost: M
refs: [the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause]
---


## Finding

Found by EMBED's lane `embed/conic-arcs` (2026-11-17) while pinning the
drum of `common::latitude_seam::collinear_cap_drum` (`crates/sweep/tests/common/latitude_seam.rs`).
Measured with `injective` (the helper in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`) at f64,
the default depth and 65,536 pairs; the same on main and on the lane's
branch, whose corner reading this shape never reaches.

- **The body.** The drum itself, not its cavity, refuses
  `Budget(Depth { max_cell_depth: 40 })` with 282 cells refused
  (`CellReceipt { candidates: 18, discharged: 2816, refused: 282,
  splits: 3080, .. }`); 564 since the face windows are read off the
  edges' covers (below). Its cavity holds
  (`a_drum_cavitys_cap_disc_on_two_half_circles_holds`).
- **The pair.** The cap's disc `1v3` against the half-annulus `2v1`
  across their half-circle `6v3`, at every depth: the disc presented
  (`Zero::Curve`), the half-annulus `Unpresented::Local`. 1,060 such
  splits, at cell widths from 6.2e-2 down to 9.8e-4.
- **The cell.** For example `u ∈ (−1.1e-16, 0.015625)`,
  `v ∈ (−0.515625, −0.5000000000000001)`: just outside the circle at its
  lowest point, the half-circle at `v = −0.5`, a rounding above the
  cell's top side. The half-annulus's only local edge is `6v3`: the
  ball is not decided clear of the arc's enclosure.
- **The raising site.** `graph_read` (`crates/topo/src/injective.rs`)
  refuses in both columns at step 1's choice of run
  (`reaches.iter().enumerate().find(|(_, g)| inside(g))?`): no run of
  the curve over the columns meets the cell's range on the other axis,
  so `present_curved` returns `Local`, and every smaller cell along that
  side is the same.

This is the curved sibling of the line case `present_plane_corner`'s
sliver concession answers (`chute`'s annulus cells touching their line
from its far side within rounding). The sliver concession is frozen
(`the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause`),
so this row does not adopt it. What a fix needs: a reading of a cell
that the curve passes outside of, by an amount the band decides, as a
cell the curve does not bound (its face's part of the cell is all on one
side, or none of it), which is the sliver row's question for a curve.

The at-rest gate asks every finished body, so it would refuse this
drum at its door. Pinned by
`the_collinear_cap_drum_itself_runs_out_of_depth_off_its_half_circle`
(564 refused), which goes red when the drum holds.

The sliver fork's recommendation (PR 7095: a thin rectangle read rather
than conceded, the departure margin metered at the faces' reach) would
not reach this cell as it stands: `graph_read` refuses at step 1's
choice of run, before any rectangle is built or any margin taken, so
the reading needs a rectangle for a cell the curve passes outside of
first.

## Re-banded P0 (EMBED orchestrator, 2026-11-17)

The at-rest gate (P0) would refuse this drum at its door, and a blocker
takes the band of what waits on it. It is the curved twin of the
question the sliver fork put to Ev (PR 7095), and by this row's own
note that fork's reading does not reach the cell: `graph_read` refuses
at its choice of run, before any rectangle or margin exists.

## 2026-11-18 — the tour's `budfillet` meets it once its windows are read off the edges' covers (EMBED, lane rational-window)

The tour walk (`embed_walk`, f64, the default depth, 65,536 pairs) holds `budfillet` on main `ff2319654` (24 candidates, 3,423 discharged, 3,399 splits). With `embed/rational-window`, which reads each face's boundary box off its edges' covers (`cover::arc_cover`) rather than one evaluation over the span, it refuses `Uncertified { a: 1v11, b: 7v1, shared: Edge(2v5), arm: Departure }` (3,601 discharged, 12 refused, 3,589 splits).
- **What moved.** The base disc `7v1`'s window, `(−2, 0) × (−1, 1)` on main, is `(−2.0000000000000004, 2.2e-16) × (−1.0000000000000002, 1.0000000000000002)`: the circle's cover rounds outward by an ulp where `sin_cos` over the whole turn read `±1` exactly.
- **The cell.** For example `u ∈ (−1.0, −0.9990234375)`, `v ∈ (0.25000000000000006, 0.2509765625000001)`, against the torus `1v11`'s cell at `v ∈ (4.6878, 4.7124)`: the disc reads `Err(Local)` at every depth. The edge `2v5` is the radius-0.25 circle about `u = −1`, so the cell's lower side is one rounding above the circle's top `v = 0.25`. On main's window the dyadic split put that side on `0.25` exactly.

This row's shape, reached by a window an ulp wider.

**The drum moves the same way.** On the same branch the drum refuses
`Budget(Depth)` as before with 564 cells refused, not 282
(`discharged: 4682, splits: 5228, outside: 588, certified: 102`), at
the default ε, 1e-6 and 1e-12 alike. The disc `1v3`'s window was
exactly `(−0.5, 0.5)²` on main and is `±0.5000000000000001` there: each
support of the cover is read in interval arithmetic, so a closed-form
edge's box rounds outward by an ulp or two, and cells now stand a
rounding off the half-circle on both of its sides.
