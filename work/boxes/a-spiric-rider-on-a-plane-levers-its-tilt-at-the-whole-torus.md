---
id: a-spiric-rider-on-a-plane-levers-its-tilt-at-the-whole-torus
kind: issue
title: slit's spiric-on-plane rider levers its frame tilt at the whole torus's R + r by hand, not at the edge's cover
status: open
opened: 2026-11-09
priority: P3
cost: E
---

Found by `cover-levers`' sweep (BOXES, 2026-11-09).

## What

`crates/topo/src/boolean/slit.rs`, the spiric-edge-on-plane rider arm
(`RIDER_SPIRIC_PLANE`), decides whether a spiric edge lies in a plane by
`Margin::of(n.cross(u).norm() * (major_radius + minor_radius))`: the
frame tilt levered by hand at the whole torus's ball, `R + r` about its
centre. Two departures from the cover build's rule:

- the lever is a carrier kind's own region (`[one-way.places]`), not the
  edge's cover (`topo::edge_cover`), which over the edge's span is the
  spiric's window rectangle met with its tube;
- the margin is levered by hand outside `Margin::levered`, which
  `crates/geom-core/README.md` "Float rounding" (the lever's sources)
  leaves to review.

## The shape of a fix

Read the edge's cover's range along the plane's normal, as
`splitting::containment::certify_plane` now reads every edge of a loop
(`cover-levers`): an edge lies in the plane where both ends of that range
decide `Zero`, and no lever is needed. Re-measure the rider rows it
moves.
