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
  splits: 3080, .. }`). Its cavity holds
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
(282 refused), which goes red when the drum holds.

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

## Instance: a sphere's cell beside a cap's circle, in the circle's pose (EMBED, lane lune, 2026-11-18)

The two-arc lune's hollow (`two_arc_lune` in
`crates/sweep/tests/self_intersection_proves_injectivity.rs`), turned
about `x` by 0.3, about `(1, 1, 1)` by 1.0 or about `z` by 1.2, refused
`Uncertified` at the lane's first head (`f35afe01`). The cells were the
cavity's sphere halves beside a cap's circle, read in the circle's pose
(`present_posed`, `crates/topo/src/injective.rs`), where the circle is
a parallel. A cell outside the face, its side a rounding off that
parallel's enclosure, was read `Local` at every depth: no line put it
on the parallel's non-face side, and with no extent in the pose no sheet
said it missed the face. That is this row's class, on a line of a pose
rather than a curve of the stored chart.

**What answers it for a pose:** the posed extent (`posed_extent`,
granted only where every row of the outer loop has an enclosure in the
pose, the lift closes on one sheet without wrapping the azimuth, and its
orientation is certified to make the face the bounded side), read by
`sheets` as a stored extent is. Those cells now read `Missed`
(`Vacuous`): 6 on the plain hollow, 86 over it and the six turns, and
every turn holds (`a_two_arc_lunes_hollow_holds_turned_six_ways`, and a
pinned sweep of 24 rigid motions). It does not answer this row's drum:
the drum's cell is the stored chart's, beside a curve, not a line of a
pose, and its face's extent already exists there.
