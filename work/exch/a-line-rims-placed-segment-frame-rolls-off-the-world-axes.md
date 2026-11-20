---
id: a-line-rims-placed-segment-frame-rolls-off-the-world-axes
kind: issue
title: adopt::line_frame keeps a PlacedSegment frame whose roll is picked off the world axes
status: open
opened: 2026-11-20
priority: P3
cost: E
---


Found by the sweep of
`recognize-normalizes-without-a-length-decision-and-cannot-mint-the-witness`.

`crates/step-import/src/adopt.rs`, `line_frame`: the `PlacedSegment`
placement for a Nurbs-adjacent line rim is
`RigidMotion::gram_schmidt` of the line's direction and the coordinate
axis of its smallest component. The frame is kept, in the edge's
`MappedCurve` description, and its roll — the y/z columns — is a
function of the world axes, not of the inputs, which D10
`[placement.frame]` says a minted reference never is. No segment point
reads the roll (every sketch point has `y = 0`), so nothing the body
answers moves with it today; what moves is the stored placement under
a rigid motion of the file, which a reader of the description would
see.

The inputs hold a reference: the rim's NURBS neighbour face, whose
chart normal at the rim is perpendicular to the line. Taking it means
`mapped_self_description` is handed the surface it is adopting
against, or the roll is stated as part of the description class's
contract (`geom_brep::MappedSource::PlacedSegment`) as unread.
