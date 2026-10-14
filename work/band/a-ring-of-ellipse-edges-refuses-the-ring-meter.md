---
id: a-ring-of-ellipse-edges-refuses-the-ring-meter
kind: issue
title: blend: a support ring with an ellipse edge refuses the ring carry-through meter, though the outer-boundary walk bounds the same carrier
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## Finding

`ring_pieces` (`crates/sweep/src/blend/surgery.rs`) reads every ring of
a blend's support face as its pieces, and refuses a piece that is not a
line or circle with `UnsupportedGeometry` ("a ring edge's carrier is
neither a line nor a circle, the ring edges the clearance check
covers"). Its two consumers differ in what they could meter:

- `support_boundary_clearance` (a closed rim's supports) already reads
  an outer-boundary ellipse, spiric or NURBS edge through `boxed_reach`,
  a certified bound whose refusal says `bounded: true`. A ring's edges
  join the same walk, so the refusal there is the filter's, not the
  meter's.
- arm (a) of `ring_clearance_pass` (open links) reads each piece's low
  reach along the inward unit through `piece_along`, which has no
  ellipse arm; it needs one (a certified bound along a direction, as
  `boxed_reach` gives along an axis).

Reached by: a unit cube holed by a radius-0.15 bore at `(0.3, 0.5)` and
cut by `common::tilted_bore()` shifted 0.3 in `x` (an ellipse ring on
the top face). Filleting the hole's top rim refuses at r = 0.05 and at
r = 0.1 although the ellipse lies well clear of the trim. Pinned by
`ring_carry_through_by_piece::an_elliptical_ring_beside_a_hole_rim_refuses_unmetered`,
and on the open-link arm by
`review_fillet_e2_probes::the_geometry_recourse_reaches_the_front_door_at_an_elliptical_ring`.
Not a regression: the reading before polygonal rings refused it too.

## What the taker owes

Let a ring's ellipse, spiric and NURBS pieces through on the rim arm,
read by `boxed_reach` like the outer boundary's, and give arm (a) a
certified bound along its inward unit for the same carriers, or say at
`ring_pieces` why a ring edge cannot take the bound an outer edge does.
Re-witness both rows above, and the recourse row
`blend_recourse_followability::the_geometry_recourse_names_a_ring_and_an_order_that_builds`.
