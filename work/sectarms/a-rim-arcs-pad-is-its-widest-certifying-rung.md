---
id: a-rim-arcs-pad-is-its-widest-certifying-rung
kind: issue
title: A spline x implicit rim arc's pad is its chart window's whole image, so two arcs within about two rungs carry neither
status: closed
closed: 2026-11-25
opened: 2026-11-12
priority: P1
cost: M
branch: sectarms/rim-pad-cells
pr: 7499
---



Filed by the `sectarms/join-reads-spline-implicit` lane; rewritten by
its fix pass with the true cause.

## What

A rim arc's pad (`CarriedComponent::pad`, `spline_implicit::pad_of`)
bounds every point of the chart tube's windows, imaged on the wall,
from the carrier. The windows are the wall pcurve's hull over each of
its spans, padded by `pad_u` and `pad_v`
(`ssi::certify::chart_tube_windows`), and limb 3 holds the section in
them one arc. A pcurve of one span gives one window over the whole
arc.

As first read, the pad imaged each window whole, halved its box three
times and read the parts against 9 carrier samples over the window's
span, so it floored near an eighth of the window's span, not near the
rung: on the carry rows' wall against a rod along `x` of radius `0.07`
(`s = 1`), the rung is `0.0444` m and the pad read `0.254` m. Each of
two arcs' pads then reached across to the other, and a site on either
read on both: a desync. The witness:
`loft_prism_at(&[0.0, 0.2, 0.4])` ∪ `ball_poled_z(0.21, (0.3, 0.95,
0.2))` aborted `JoinDesync { "a site lies on two components of its
section" }` in union, subtract and swapped union, at the default ε and
at 1e-12; on the carry rows' wall, rods of radius `0.07·s` and
`0.10·s` desynced each arc's own witness at `s = 0.01, 1, 10`.

The fix pass halves each window into cells until each cell's image is
no wider than two rungs, each imaged on its own and read against eight
carrier points per cell (`spline_implicit::PAD_CELL_RUNGS`), so the
pad now reads about 1.1 rungs on those walls, and the carry refuses
`CarryRefusal::PadsMeet` where two fitted arcs' carriers stand no
farther apart than both pads and both bands (`spline_implicit::apart`).
The prism and ball now refuse `GermSectionUnbuilt` naming `PadsMeet`
(pads `0.092` m, gap `0.080` m).

What remains is valid input refused:

- **The window is a whole span's hull.** The pad is at least that
  hull's width across the carrier imaged on the wall. For a pcurve of
  one span that bends in the chart, that is the arc's whole bend,
  however narrow the rung.
- **Two arcs within about two rungs carry neither.** On the carry rows'
  wall at `s = 1`, rods of radius `0.09` and `0.095` refuse `PadsMeet`
  (radius `0.09`: arcs `0.180` m apart, pads `0.098` m at the rung
  `0.089` m), and the prism and ball above refuse. So does editor-core's
  wavy loft against a box face (`emit_topo`'s
  `wavy_loft::a_union_across_a_nurbs_edge_refuses_where_two_arcs_pads_meet`:
  arcs `1.25` m apart, pads `0.91` and `0.94` m), which at the
  reviewed head reached the join and stopped at rest.
- **The reach is half the arc**: the witness's reach is the farthest
  its window's image stands from it, so for one window about half the
  wall across.

## Remedy

The refusal is at the carry, which refuses a section whose arcs' pads
meet anywhere, though a true point of the section that reads on one arc
alone is read right (it lies within its own arc's pad). Refusing at the
site read instead, only where a site reads on two arcs whose pads meet,
would let such runs on; it needs a typed site refusal the join maps
with the pair's faces. Either way the pads are what to narrow:

Windows over sub-spans: refine the pcurve and read limb 3's one-arc
proof over the narrower windows (a sub-arc spanning the same end
slices is the arc), or certify a narrower chain after the wide one, so
each window's image is about its rung across. PR 6829's per-box
narrowing toward a node is the same move for node arcs.

## The fork

Measured by the implementing lane (2026-11-16): a pad read off the
certifying rung's windows is at least about one rung however the
windows are cut (whole, per cell, per sub-span or per box), because the
tube pads every window by the rung imaged across the carrier
(pad/rung 1.10 on every rod at s = 0.01, 1 and 10). So the Remedy
above cannot meet the done-when. The pad reads closeness off the
region limb 3 uses for isolation, and never consults the partner.
C5's carry sentence ("its pad the chart tube's windows imaged on the
wall by its chart speeds", PR 6862) says exactly that, so a pad read
another way changes it. How a rim arc's pad is bounded, and how far
it is refined, is the decision.

## Decided (Ev, PR 7045, 2026-11-17; fork-log row 280)

Ev took designer A's answer ("let's do (A)").

- **Read the pad off the tube's cells,** cut to the knot rectangle,
  keeping only the cells the partner's form does not clear of the band.
  Each cell is read against the whole carrier, not only its own span's
  points.
- **Refine until the carry's question resolves.** Cells are halved
  until every pair of arcs stands apart, or a pair stops narrowing.
  This is PR 5574's rule: a stop set by the predicate, not by a fixed
  share of the rung.
  - The refinement carries a **work bound**, and refuses typed past it.
  - **Sites are read in the same surviving cells that the apartness
    check separates.**
- **The change is in the carry only.** Limb 3, the certificate and rest
  are unchanged, and `PadsMeet` is kept.
- **C5's carry sentence and C2's node-end clause** in
  `crates/geom-brep/README.md` are rewritten to say this.
- The marched-loop door's reader is a later step.

## Done when

Two rim arcs 1.2 rungs apart are carried, each point reading on its
own arc alone, and a bending one-span arc's pad stands within two
rungs.

## Built (`sectarms/rim-pad-cells`)

The carry reads a fitted arc on its cells
(`pair_section::spline_implicit::cells`): each tube window cut to the
knot rectangle, halved to cells a rung wide, the cells the partner's
form clears of the band dropped, each read against the whole carrier.
The section's arcs are held apart on those cells, each widened by the
band's escalation along every axis, as a site's read widens them. The
cells of two arcs that share a point so are halved until none do, or
the loop stops as `crate::descent` stops one, where the pair refuses
`PadsMeet` naming the stop: a stall (two cycles of halvings that clear
no half and part none, or nothing left to halve) or a work bound of
sixteen halvings a cell of the section's first reading (`2¹¹` at
least). A site is read in the same cells (`CarriedComponent::boxes`).

- The witnesses moved to later stops. The rods of radius `0.09` and
  `0.095` are carried, as is every rod from `0.045` up, two rungs apart
  at the nearest. The prism and ball complete the join and stop at the
  inside/outside test (at `ε = 1e-12`, at the chord's image). The wavy
  loft stops at its chord's image at rest. Each is carried on its first
  reading: no witness on record needs a refinement round.
- On a wall whose chart quickens, arcs `0.11` rung apart are carried.
- A bending one-span arc's pad reads `0.88` rung.
- The certifying rung follows the gap on a wall of even speed, so its
  arcs stand two rungs apart or more; refinement is exercised by a row
  at a band the row names.

Left open, each filed: the overlapping windows' sweep
(`a-rim-arcs-overlapping-tube-windows-are-each-cut-to-cells`) and the
marched-loop door (SECT's
`a-point-is-read-on-a-marched-loop-by-distance-not-by-its-tube`, with
this lane's note).
