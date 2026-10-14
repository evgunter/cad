---
id: clearance-footprint-reads-an-arc-as-its-whole-carrier-ball
kind: issue
title: wall_clearance's footprint box holds an arc edge in its whole carrier ball (a circle arc in its whole circle), so it may read a facing pair whose footprints are apart
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [shell-clearance-footprint-reads-vertices-not-arcs, tilted-read-takes-a-spline-or-spiric-edge-as-its-carrier-ball]
---


Found by PR 4467's sweep. That PR closed the same conservatism in the
tilted read.

`wall_clearance` (`crates/topo/src/shell.rs`) grows each planar face's
footprint box over every curved edge by `carrier_box`. That box is the
projection of `containment::carrier_ball`'s ball, and for a circle or
an ellipse that ball is the WHOLE conic (centre, larger semi-axis), not
the arc. Spiric and spline arcs get one ball for the whole edge.
`shell-clearance-footprint-reads-vertices-not-arcs` (PR 4115) put the
ball there to stop a crossing pair reading as separated, and it is sound
in that direction. In the other direction, a facing pair whose arcs bow
nowhere near each other can have boxes that meet. If its plane gap is
also under `2t`, the pair refuses `WallClearance` though its offsets do
not cross.

No row has been built that reaches it. A candidate is a pair of facing
caps whose short arcs sit on large circles reaching into the other cap's
footprint.

Shape of a fix: the tilted read's `ArcPiece` refinement (PR 4467) bounds
an arc to within about the band, and a conic arc's box is closed form
(its extremes are where the side function is extreme, as
`MovedWall::cut` already splits it). Either gives a box holding the arc
and not the carrier.
