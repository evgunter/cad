---
id: a-sphere-or-torus-pair-extent-skips-the-face-box-rule
kind: issue
title: carrier_pair's face ball takes a sphere or torus face's whole carrier, bypassing the face box rule its other kinds read
status: parked
opened: 2026-11-02
priority: P1
cost: E
blocked_on: [the-region-an-entity-occupies-is-one-cover]
---


Found by the sweep of the torus-window closed-form unit
(`boxes/torus-window-closed-form`), 2026-11-02.

## What

`boolean/carrier_pair.rs` `face_ball` (the consumed extent
`pair_extent` reads) matches the surface kind before it reads the
kernel's face box rule: a sphere or torus face takes
`geom_brep::ExtentBall::of_carrier`, the whole carrier's ball (`r`, or
`R + r`, about the centre, "whatever the trim"), and only the other
kinds take the ball around `census::face_reach`. That is a second
answer to "what region can this face occupy", kept beside the rule.
Since the rule's sphere arm reads the chart rectangle
(`boxes::sphere_reach`) and its torus arm the chart rectangle's closed
form (`boxes::ring_window_extent`), the ball around `face_reach` is
far tighter for a small patch (a 22° fillet arc of a 5 m ring), while
for a face that keeps the whole carrier the carrier's ball is tighter
than the ball around its box. Which ball a face gets is decided by its
kind, not by which is smaller.

## Siblings, not this row

The same shape answers a different question at three more sites, each a
radial reach from a point over the whole carrier rather than a box:
`side_reader.rs` `beyond`, `editor-core` `mate/reach.rs` `face_reach`,
and `geom_brep::ExtentBall::of_carrier` itself (read by `face_ball`).
Whether the radial reach should read the rule too is the survey's call.

## The shape of a fix

Read the rule for every face, and where the carrier's own ball is the
smaller keep it as a meet rather than a kind match (both enclose the
face). Re-measure the pair levers it moves.
