---
id: an-ellipse-trimmed-ring-on-a-cylinder-wall-has-no-volume-lane
kind: issue
title: A cylinder wall carrying a ring trimmed by ellipse arcs (a tilted bar through a pipe) has no volume lane: RingOnCurvedFace
status: open
opened: 2026-10-02
---


## What

`topo::mass_properties` refuses a cylinder wall that carries a RING
trimmed by an ellipse arc: `RingOnCurvedFace`
(`crates/topo/src/props.rs:1770`). The cylinder's closed form reads
rings (`geom_brep::props::curved_face_loops`, the chart Green form) but
only rims and rulings; the quadrature lane that reads ellipse trims
takes the outer loop only.

Witness (TANG, PR 3851, while scanning the ring lane's closure): the
unit pipe (`crates/sweep/tests/verbs_germarms.rs`'s `pipe`) and the
brick `x ∈ (−1.1, 1.1)`, `y ∈ (0.2, 0.7)`, `z ∈ (−0.15, 0.15)` turned
about the x axis through `(0, 0.45, 0)` by any of 0.3, 0.7, 1.0, 1.3
rad. The ∩ and bar ∖ pipe build and measure; ∪ and pipe ∖ bar build,
pass every tier, and refuse at the volume backstop with
`VolumeUnmeasured { source: RingOnCurvedFace }` (72 of 72 such ops over
the poses tried, thicknesses down to 1 mm, both tilts, both sides of
the axis).

## The shape of a fix

The quadrature lane's Green form over every loop, as the cylinder's
closed form now does: a ring's contribution is its own `∮` with its own
winding. The closed form's closure checks (`props_loop_closed`,
`props_chart_loops_closed`) are the premise to carry over.
