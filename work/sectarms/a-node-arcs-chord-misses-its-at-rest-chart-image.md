---
id: a-node-arcs-chord-misses-its-at-rest-chart-image
kind: issue
title: A node arc's chord does not certify at rest, the fixed foot schedule's spans too coarse at the node for limb 3
status: review
opened: 2026-11-13
priority: P1
cost: M
branch: sectarms/one-image-function
pr: 7537
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

## The fork

This item, `a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung`
and `a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest`
are one question. Each refusal is on the wall's image of a carrier the
trace certified, never on the carrier:
- the plane lane's degree-1 image sags off a foot path that bends in the
  chart;
- the curved lane's finest uniform rung misses by about 5e-10 of the
  wall's size;
- a node arc's uniform end span is far wider than its node box.

The trace fits the carrier and its wall pcurve together on one shared
parameter (`ssi::fit_branch`, graded toward a node by `Ends::graded`)
and certifies that pair. The join keeps the carrier and its certificate
and drops the pcurve. Rest then images the carrier on the fixed foot
schedule, chosen by the partner's kind, and the mint stores a third
image of its own.

What rest's image of a fitted spline × implicit carrier is, for every
partner kind, node ends and both charts of a spline pair, and what the
wall's face stores for the same edge, is the decision. C2's at-rest
image sentences ("derived from the same foot schedule", PR 6879) and
its NURBS × NURBS sentence ("from the fixed foot schedule") change
with it.

## Decided (Ev, PR 7032, 2026-11-17; fork-log row 282)

Ev took state (1): "(1) makes sense i think! i certainly don't want to
introduce a fake cache like (2) has".

- **One image function.** The image of a carrier on a spline wall is
  one function of the carrier, the wall and the band, and it reads
  nothing about the partner. It is the wall's foot path of the carrier,
  interpolated on the carrier's own knots, node grading included.
  - It refines only by knot insertion where its measured miss asks, and
    refuses typed past a fixed cap.
  - The trace's certificate, the pcurve mint and the at-rest certificate
    all call it. A stored pcurve is only its remembered value.
  - The partner-kind `match` and the uniform 33-to-257-sample climb go.
- **The same rule everywhere.** It holds for every partner kind, for
  node ends and for both charts of a spline × spline pair. The trace and
  rest mint the wall's chart the same way.
- **Projection is made well-posed.** It follows the carrier
  continuously from its start vertex's stored chart point, not point by
  point to the closest point, so it recovers the march's branch on
  folded walls and across closed-wall seams. Imports need this under any
  answer.
- **No stored-fit fallback.** Keeping the trace's own fit is not taken,
  even as a temporary measure. If the function refuses where the march
  did not, that is a defect of the projection: stop and report it.
- **Ratified text.** C2's at-rest image sentences and its NURBS × NURBS
  sentence in `crates/geom-brep/README.md` are rewritten to this. Add
  that a carrier's knot vector is the chain its certificate is built on.

This decides
`a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung`
and `a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest`
too.

## Done when

A chord of a carried node arc certifies at rest, its node end read
about the edge's end vertex.
