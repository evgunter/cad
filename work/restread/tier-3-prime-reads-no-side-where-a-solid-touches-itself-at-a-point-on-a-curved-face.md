---
id: tier-3-prime-reads-no-side-where-a-solid-touches-itself-at-a-point-on-a-curved-face
kind: issue
title: Tier 3′ reads no side where a solid touches itself at a point a curved face of it passes
status: open
priority: P2
cost: H
opened: 2026-10-21
---

Found by `tier-3-prime-reads-no-side-for-a-face-touching-a-vertex-of-its-own-solid`'s
lane (branch `restread/point-touch-winding`).

## What

The self-touch reading (`sweep_self_touch_points`,
`crates/topo/src/census.rs`) gathers the solid's whole link at a point
where it touches itself (`Link::at_point`). `Sector::of` reads planar,
line-bounded faces only. A curved face through the point refuses
`TouchUnread::Unreadable`, and so the whole reading does.

## Measured (2026-10-21, on the lane's branch, default ε)

Reading every such point and refusing the unread ones
`BoundaryCrossesItselfUndecided { why: Unreadable }` newly refuses 19
sweep rows, 34 findings in all, every one curved. They are a ruling or
a rim lying on a wall, a pierce held in a cylinder wall, the π-seam
kiss rows, the bisector bores, the rest-zip tangent sites, the curved
notch pinch, the drum corner, `verbs_ga_r2_probes`, and the tube on a
ball. With a curved link left unread instead, which is main's
behaviour at these points, all 19 pass.

## The fix

A curved face's sector at the point is its tangent cone there, a
half-plane or plane of the tangent plane, bounded by its edges'
tangents. The winding reader needs the curved sheet's side read off
its tangent plane only where the sheets separate to first order. The
cross-solid analysis has the same gap (`TouchUnreadable`).
