---
id: clearance-cells-are-decided-in-model-space
kind: unit
title: Clearance decides a cell Inside/Outside/Undecided in model space off the face's edges on their carriers and the point-in-face doors; ChartBound, MetredBound, the ring lifts, the span check and clearance::chart_arms retire; hi reads only Inside cells and on-face stations
status: closed
opened: 2026-10-24
priority: P1
cost: H
pr: 5673
branch: chart/clearance-cells
closed: 2026-10-24
---


## The decision (CHART orchestrator, 2026-10-24, on two converged designers)

A face's region is read off the face's edges **on their own carriers**, as
the kernel's point-in-face doors already read it (`[intrinsic]`, and
`boolean::sphere_region` / `torus_region`: "a pole is a point like any
other"). It is not a polygon minted on the chart. Clearance's question about
a cell (a chart rectangle) is a point-in-face question plus a separation,
so the chord polygon in `chart_bound.rs` was a second reading of a question
the kernel already answers (`[one-way.readings]`).

The verdict, a `topo` door, answers `Inside`, `Outside` or `Undecided`:
1. **Boundary-free.** No non-seam boundary edge meets the cell's surface
   patch, decided in metres. The separation is **carrier-aware**: a line's
   distance, an arc's carrier ball, the readings `splitting::containment`
   holds. It is never box against box, because the 3-D box of a long
   diagonal edge covers every cell beside it.
2. **Witness.** A connected patch with no edge crossing it is wholly in or
   wholly out. One station of the patch, placed by the face's point-in-face
   door, says which.
3. Anything else is `Undecided`, and the cell is kept.

The clearance engine drops `Outside` cells. `hi` reads only cells
certified `Inside` and stations the walk placed `In`, `OnEdge` or
`OnVertex`. `ChartBound`, `MetredBound`, the ring lifts, the span check and
`clearance::chart_arms` are deleted. `chart_region`'s area and overlap,
which is a different question (two regions on one chart), and the loop
walk are untouched.

## What it closes

- `chart-boundary-refuses-pole-crossing-sphere-loops`: no azimuth is read,
  so a pole needs no polar chart.
- `round-holes-get-no-chart-bound-benefit`: a cell inside the hole meets
  no edge, and its witness reads `Outside`. The row's own lune fix bought
  nothing, because a two-half-arc hole's chord polygon has no interior.
- `clearance-window-cone-sphere-torus`: the same rule on every kind with
  a point door. The arms stop being a prerequisite.
- `exact-region-cells-for-lower-bound-only`: its premise is withdrawn.
  `hi` needs on-face witnesses, not exact cells.
  **`Certified::LowerBoundOnly` still retires only once CLEAR's
  `min-separation-tightening-crosses-the-drive` closes**: every cell verdict
  inside `min_separation` is a decision on the interval lane only, and that
  lane split is what refuses the drive. This unit lands the clearance
  sweep's half, and the measure's half waits on that row.

## Prerequisite

`curved-face-containment-answers-none-on-a-ringed-face`: the cylinder and
cone point doors are chart trims with inventories, and they answer `None`
on any ringed face. Clearance today tightens a holed cylinder wall through
the ring lifts, so switching before those doors read rings would regress
it. `[one-way.readings]` rules out keeping the chord polygon as a fallback.

## Not decided here

The per-cell cost of the sphere and torus doors at clearance's cell counts
is unmeasured. The unit measures it and reports it; the cost does not change
the design.

## Owed at the deletion (from PR 5656's lane)

`chart_bound`'s outer-span check (`OuterSpansPeriod`) is the premise of the
chord polygon's ring lifts (`RING_SHIFTS`), and `ring_path`'s argument cites
it. W2 does not need it: its own predicate keeps no span bound, by the ruling
on PR 5657. Before deleting the check, give `ring_path`'s cite a new home, or
show that the argument it supports no longer needs it.
Also deleted with `chart_bound.rs`: PR 5656's `ChartEdge::Envelope::reach`
and its tests (`t5c_…_refuses_at_f64`, `an_arc_walls_envelopes_reach_finitely_at_f64`).
That PR's span check refuses on a poisoned margin. Whatever takes over the
premise must refuse on poison too.

## Closed

What landed (PR 5673) is the clearance SWEEP's half of the decision.

- `topo::cell_region` decides a cell `Inside`, `Outside` or `Undecided`
  off its face's edges on their own carriers
  (`splitting::containment::arc_distance_floor`, the one reading the
  boolean's touch gate shares), its carrier's singular set, and the
  face's point door; `topo::cell_region::station_region` places one
  station.
- The sweep drops `Outside` cells, and a `Violated` witness admits only
  stations placed `In`, `OnEdge` or `OnVertex`: a witness is a pair of
  face points.
- `ChartBound`, `MetredBound`, the ring lifts, the span check,
  `chart_boundary`, `chart_edge`, `clearance::chart_arms` and the root
  cut are deleted; `loop_windings` is the loop walk's public reading.

What waits on `min-separation-tightening-crosses-the-drive` (CLEAR): the
measure's half. `Inside` has no production consumer yet; `hi` reading
only `Inside` cells and placed stations lands in `min_separation` once
its decisions no longer cross the drive on one lane, and
`Certified::LowerBoundOnly` retires then. That row's note says so.

Residue: `cell-region-reads-an-ellipse-arc-as-its-whole-conic` is closed
by this PR's review pass (an elliptic edge is read through its arc's
floor); the per-cell costs are in the PR body.


## Moved (2026-10-26)

CHART folded into CHARTTAIL (Ev, in chat, 2026-10-26). Kept although
closed: it is the design record of the model-space cell verdict, and
CLEAR's `min-separation-tightening-crosses-the-drive` builds the
measure's half of this design (`## Note (CHART, 2026-10-24)` there).
