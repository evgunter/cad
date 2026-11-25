---
id: a-saddle-cut-by-its-tangent-plane-leaves-its-nodes-four-chord-ends-unpaired
kind: issue
title: A saddle cut by its tangent plane refuses UnpairedLooseEnds at its node: the join does not pair the four chord ends that meet there
status: open
opened: 2026-11-23
---


Found by the `sectarms/one-image-function` lane, which moved this run's
stop here.

## What

The saddle block `−2 ≤ y ≤ x·z` cut by the half-space `y ≥ 0`
(`crates/sweep/tests/a_saddle_cut_by_its_tangent_plane.rs`,
`a_saddle_cut_by_its_tangent_plane_refuses_at_its_node_under_every_op`):
the pair is a spline × plane section with a node at the origin, and the
join carries its four arcs to the node. Until the wall's image of a
carrier was built on the carrier's own knots, every op refused at the
at-rest certificate of the plane's chord along the side wall `x = −1`
(`SsiLimb::ImageMiss`), before any chord to the node was read.

Those chords now certify, the node arcs' among them, and every op, either
way round, refuses in the join:
`BooleanError::Join(JoinError::UnpairedLooseEnds { count: 4 })`
(`topo::boolean::join`). The four chord ends that meet at the node's
vertex are left unpaired.

## Done when

The saddle cut by its tangent plane builds at its closed forms (the
block `8`, its part above the plane `1/2`), which
`work/splice/a-saddle-cut-by-its-tangent-plane-builds-at-its-closed-form.md`
asks for, or refuses at a typed stop past the pairing.
