---
id: a-cell-whose-side-lies-a-rounding-off-a-curved-edge-reads-local-at-every-depth
kind: issue
title: A cell whose side lies a rounding off a curved edge it does not hold is read by no presentation at any depth, so the collinear-cap drum's own body runs out of depth at f64
status: dispatched
opened: 2026-11-17
priority: P0
cost: M
refs: [the-sliver-concession-reads-inside-the-band-against-the-finished-body-clause]
pr: 7113
branch: embed/lune
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
(282 refused; 210 since PR 7131's frames), which goes red when the drum
holds, and by `a_drum_of_radius_0_7_runs_out_of_depth_off_its_half_circle`,
the same drum at radius 0.7 (below).

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

## 2026-11-18 — the drum family, and the radius-1 drum's holding by alignment that PR 7131 then moved (EMBED, lane rational-window)

Measured with `injective` at f64, the default depth and 65,536 pairs,
main `c5bc653ae` against `embed/rational-window`, where `boundary_of`
reads each edge's box corners off its cover over its span
(`cover::arc_cover`, `Cover::box_in`) instead of one interval
evaluation. Windows printed with an uncommitted print in `window_of`.

- **What moved on the radius-1 drum.** One window: the half-annulus
  `2v1`'s, whose low `u` and `v` ends were `−1.0000000000000004` (the
  outer circle's interval evaluation over its span) and are
  `−1.0000000000000002` (its arc's closed-form extremum, one ulp
  tighter). Halving from that end lands the cells' sides on `−0.5`
  exactly, the half-circle's own bound, where main's window landed them
  on `−0.5000000000000001`, a rounding off it. The drum then holds:
  `Holds`, 18 candidates, 1,030 discharged, 0 refused, 1,012 splits.
  The holding is the engine's own discharge of every pair; nothing about
  this row's cell was read differently.
- **Other radii still meet it.** The same drum built at radius `r`,
  height `h`, ring at `r/2`:

  | `r`, `h` | main | branch | branch, crest exact (M6) |
  |---|---|---|---|
  | 1, 2 | `Budget(Depth)`, 282 refused | holds | holds |
  | 1, 1.3 | 282 | holds | holds |
  | 2, 3 | 282 | holds | holds |
  | 0.7, 2 | 282 | 280 | 282 |
  | 0.3, 0.9 | 234 | 448 | 234 |
  | 1.1, 2.3 | 152 | 436 | 210 |
  | 0.9, 1.7 | 148 | 282 | 140 |
  | 1.3, 2 | 140 | 284 | 140 |

  The last column is PR 7120's review's mutant M6 (on
  `embed/rational-window-review`): the arc cover's crest read as
  `√(p² + q²).min(|p| + |q|)`, exact on the world axes
  (`work/onecopy/an-arcs-cover-reads-its-crest-a-rounding-out-where-the-radius-squared-rounds.md`).
  This row stands without the crest's rounding.

  So the drums whose windows halve onto the half-circle hold, and the
  others refuse as before: the cell this row names is still read by no
  presentation, and a window reaches it whenever its halvings miss the
  curve's bound by a rounding.

**Re-measured 2026-11-21**, main `45023f4e3`, and again on `0cae9969f` with every count the same (with PR 7131, which reads a
stored frame at `Interval` by its polar enclosure summed exactly)
against `embed/rational-window` merged onto it. Every drum of the
family refuses on both, and the radius-1 and radius-2 drums read the
same receipt on both: 210 refused, 3,250 and 3,222 discharged. The
frames moved the windows again, and the alignment the branch's windows
had given at `c5bc653ae` is gone. So the holding above was alignment
alone, and this row's cell is reached by every drum of the family now.

| `r`, `h` | main `45023f4e3` | branch merged onto it |
|---|---|---|
| 1, 2 | 210 refused | 210 |
| 1, 1.3 | 210 | 210 |
| 2, 3 | 210 | 210 |
| 0.7, 2 | 232 | 294 |
| 0.3, 0.9 | 234 | 448 |
| 1.1, 2.3 | 210 | 440 |
| 0.9, 1.7 | 140 | 294 |
| 1.3, 2 | 146 | 288 |

- 2026-11-25 (LATHE one-band lane, PR 7414): the collinear-cap drum
  (`self_intersection_proves_injectivity::the_collinear_cap_drum_itself_runs_out_of_depth_off_its_half_circle`)
  now holds: a full revolve's wall is one band and its caps whole discs,
  so the drum has no half-annulus cell beside a half-circle. That row
  pins `Holds` (its name kept, as the slow set lists it). The radius-0.7
  drum, whose half-circles are drawn by hand, is still this row's
  witness.
