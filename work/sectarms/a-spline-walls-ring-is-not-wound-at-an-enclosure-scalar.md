---
id: a-spline-walls-ring-is-not-wound-at-an-enclosure-scalar
kind: issue
title: A ring on a spline wall is not wound at a scalar whose points are wider than one value: the paths' feet are sought on the value channel
status: open
opened: 2026-11-12
priority: P4
cost: M
---


Filed by the `sectarms/spline-ring-winding` lane.

`ring_path::SplineWall` seeks the chart parameters of a path's ends by
`NurbsSurface::project` on the wall's value channel
(`SpanLocate::value_channel`), since the projection's own door asks
`CertifiedBounds` and the ring lane is `Decide`. A point whose
coordinates are wider than one value has no value there, so it has no
foot: the closing chord's midpoint then reads
`JoinError::RingIslandUnread { kind: Nurbs }`, typed, and the ring is
not wound. Nothing reaches it today, because a spline wall's section
is carried at `f64` alone.

The foot's parameters are structure and the distance that admits them
is read at the run's scalar (`split_ring_path_wall_foot`), so a seed
taken from an enclosure's middle would serve; what is missing is a
door from a `Decide` scalar to that middle.
