---
id: ring-paths-conic-arc-floor-is-a-second-spelling-of-the-covers-near
kind: issue
title: ring_path's ConicArc::distance_floor is a second reading of a conic arc's nearest distance beside the cover's
status: open
priority: P3
cost: C
opened: 2026-11-14
---


`topo` `ring_path::ConicArc::distance_floor` (ring_path.rs, near line
1772) bounds the distance from a point to a conic arc as the circle of
radius `b` over the span less `|a − b|`. The cover's `Arc::near`
(`crates/geom/src/cover.rs`) answers the same question, tighter: the
circle of the mean radius less half the semi-axes' difference, met with
the conic's annulus. `splitting::containment::arc_distance_floor` moved
onto the cover in `cover-kind-match-gate`; ring_path's own reader
(`Margin::of(charge(conic.distance_floor(mid) - half, slack))`, near
line 1923) and its rows (`ring_path/arc_floor_rows.rs`) still read the
second spelling. Read the arc's cover (`crate::cover::arc_cover`) there.
The kind-match gate cannot see it: the method dispatches on no kind.
(boxes-cover7 lane)
