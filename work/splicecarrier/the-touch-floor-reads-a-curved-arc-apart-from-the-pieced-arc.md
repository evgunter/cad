---
id: the-touch-floor-reads-a-curved-arc-apart-from-the-pieced-arc
kind: issue
title: arc_distance_floor's ArcPieces reads a spiric or spline arc at the carrier's whole speed bound, beside PiecedArc's per-piece reading of the same arc
status: open
opened: 2026-10-26
priority: P2
cost: M
---

Found in the second review of PR 5715 (branch `splice/touch-doors`).

There are two piece readings of "is this point clear of this curved
arc":
- `splitting::containment::ArcPieces` (the floor of
  `arc_distance_floor`, which the touch gate and the cell door read)
  walks the arc on `geom::Curve3::point_walk`. Each piece is a ball
  of `speed_range`'s sup times its half-width: a spiric's sup over a
  whole period, a spline's over its whole control net.
- `splitting::pieced_arc::PiecedArc` (the in-plane walk's boundary
  reading, behind `spiric_arc` and `spline_arc`) bounds each piece
  over its own window: a spiric's `spiric_rate_bounds` over the
  piece's `ρ` and `|sin v|`, a spline's Bézier piece off the blossom.
  Near a spiric's fold that is up to 10⁴ times tighter.

`ArcPieces` is sound but coarser, so a ball near a fold or a long
spline clears later than it could, or not within the budget. Putting
`ArcPieces` on `PiecedArc`'s pieces would give one reading. It moves
certified decisions, so it was not done in that PR's review pass.
`PiecedArc` landed on main during that review. Its reading answers
"on or off within the band", where the floor needs "clear past a
radius", so the fold needs the piece balls shared, not the whole
reading.


Moved from SPLICE to SPLICECARRIER on 2026-10-28, by `git mv` in SPLICE's split along its seams (Track size; Ev, in chat, 2026-10-28).
