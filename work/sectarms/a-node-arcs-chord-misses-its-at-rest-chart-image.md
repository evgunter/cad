---
id: a-node-arcs-chord-misses-its-at-rest-chart-image
kind: issue
title: A node arc's chord does not certify at rest, the fixed foot schedule's spans too coarse at the node for limb 3
status: open
opened: 2026-11-13
priority: P1
cost: M
design: true
---

Filed by the `sectarms/spline-node-carry` lane
(`a-spline-implicit-arc-is-carried-to-its-node`), rewritten by its fix
pass with the measured cause.

## What

At trace time an arc to a node is carried to the node's point, its
knots graded toward the node (`ssi::ends::Ends::graded`). Its chart
tube ends in the node's own box (`ssi::certify::chart_tube`,
`node_ends`). That box is twice the chain's end span's reach. The node
reading (`ssi::node::node_box`) must place the node within the band of
the end, and the end span must reach no farther than
`ssi::certify::NODE_BOX_REACH` times the node's settle reach,
`√(NODE_SETTLE_FLOOR·ε′/σ)` (about `8√(εR)`). The carrier is fitted on
chord-length parameters, so its speed is constant along its span (to
`1 + 1e-11`, both reviewers of PR 6897 measured).

At rest, `edge_nurbs::residual_nurbs_limbs` images the carrier over its
whole certificate span (`carrier_net::carrier_span`, for every chord of
it). The image comes from the fixed `PXN_FIT_SAMPLES` schedule, 33 feet
uniform in the carrier's parameter. Its last span is a thirty-second of
the arc, far wider than the node's box allows, so `node_ends` does not
read that end as a node, and limb 3's transverse enclosure straddles
over the windows at the node.

Measured on the fix pass's head, at ε 1e-9 (`EdgeCurve::certify`, the
plane lane):
- every chord of each of the cross saddle's four carried arcs refuses
  `PlaneNurbs(TubeStraddles { boxes: 32 })` (the whole span, either
  half, the last tenth);
- so do the 78° node's four arcs.

At PR 6897's first head the cross's chords certified at rest only
because the node box was read over the end span's reach, which the fix
pass removed as unsound: a carrier past the node, short of it or turning
at it certified the same way.

The first filing's cause, a carrier speed falling toward the node and an
image miss, was wrong. The sweep saddle's
`ImageMiss 1.155e-4` is the plane's chord along the block's side wall
`x = −1`, no node chord: it is
`a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest`.

## The question

C2's at-rest paragraph: "The wall's image of the carrier is derived from
the same foot schedule". One fixed schedule for every carrier. A node
end needs image stations at the node no wider than the node's settle
reach, about `8√(εR)` (a few microns at ε 1e-12), so no uniform
schedule of a size the certificate can afford reaches it. Certifying a
node arc's chord at rest needs a schedule that grades toward a node end,
which that sentence excludes. The image is the pcurve mint's too
(`pcurve_cache::general_image_lane`, which reads no partner).

## Done when

A chord of a carried node arc certifies at rest, its node end read
about the edge's end vertex.
