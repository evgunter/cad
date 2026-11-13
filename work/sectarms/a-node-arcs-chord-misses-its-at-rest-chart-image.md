---
id: a-node-arcs-chord-misses-its-at-rest-chart-image
kind: issue
title: A node arc's chord misses its at-rest chart image: the fixed foot schedule cannot follow a carrier graded toward its node
status: open
opened: 2026-11-13
priority: P1
cost: M
---


Filed by the `sectarms/spline-node-carry` lane
(`a-spline-implicit-arc-is-carried-to-its-node`), which carries a
spline × implicit arc to its node at trace time and stops here.

## What

At trace time a node arc's carrier is fitted through the march's states
and through samples inside the node's box settled onto the section, each
half the last one's distance from the node, and its knots are graded
toward the node (`ssi::ends::Ends::graded`, from `NODE_GRADE_FIRST`);
limb 3's chart tube is graded the same way and ends in the node's own box
(`ssi::certify::chart_tube`, `node_ends`). The carrier's speed in its
parameter therefore falls geometrically toward the node.

At rest the edge certificate (`edge_nurbs::residual_nurbs_limbs`) images
the edge's carrier on the wall from the fixed `PXN_FIT_SAMPLES` foot
schedule, uniform in the carrier's parameter (`chart_image`, or the
mint's climb `general_chart_image` beside a curved partner), and C2
says so: "The wall's image of the carrier is derived from the same foot
schedule". A degree-1 or climbed image on uniform stations cannot follow
a carrier whose speed falls by orders of magnitude over the last stretch,
so limb 2 bounds `|S(P(t)) − C(t)|` far past the band near the node.

The witness: `sweep` `a_saddle_cut_by_its_tangent_plane` (the block
under `y = x·z` against the half-space `y ≥ 0`, a node at the origin).
Every op, either way round, now reaches the join with the four arcs
carried to the node, and refuses at the at-rest certificate of the first
chord to the node:
`Join(BooleanJoin(Euler(Certification { error: PlaneNurbs(Limb { limb: ImageMiss, margin: 1.155e-4 }) })))`.

## The question

The image is the pcurve mint's too (`pcurve_cache::general_image_lane`,
which has no partner and so cannot read the node), so the two must
change together. Two shapes:

- the foot schedule takes the carrier's own knots inside the edge's span
  as stations wherever they are finer than its uniform stations: no
  partner is needed, and the mint and the certificate stay one image,
  but every fitted carrier's at-rest image moves;
- the edge certificate decides each end a node by the node's own reading
  about the end vertex (as limb 3 does, `ssi::node::holds_node`), and only
  then takes the carrier's knots there: no other image moves, but the
  mint must be told the node, or its image differs from the one the
  certificate read.

Either changes C2's "the same foot schedule", which is ratified text.

## Done when

A chord of a carried node arc certifies at rest, and the saddle row
moves past the at-rest certificate.
