---
id: a-leaves-verdict-carries-no-witness-point
kind: issue
title: A bound's Leaves verdict is decided on the leading term of its parting, with no witness point of the bound clearing the band
status: open
opened: 2026-11-01
priority: P1
cost: M
parent: the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading
---


Found by the stage-2 lane (`tangrim/sector-passes-descend`): item 4 of
`the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading`
(the design's item 3, M2), not built in stage 2.

`bound_side::Tie::descend` decides a tied bound's side on the sign of
its parting, `(κ_b·n̂ − II_F(d̂, d̂))·l²/2` (`geom_brep::parting_side`,
predicate `tangent_sector_order2`), at the shorter of the bound's reach
and the face's lever. That is the leading term at a length, not a point
of the bound: the third-order remainder over the reach is unbounded
there, so a `Leaves` verdict need not agree with a finite-reach reader
of the same edge against the same carrier (`reduce.rs` `interior`,
`carrier_cross`, `carrier_touch::off_face`, the conic × quadric root
door) or with the result census. Only the conic-against-a-plane arm
(`geom_brep::conic_bend_side`) lifts a certified point of the edge.

Owed, per the design: the verdict stands only where a witness point of
the bound at its reach (`edge_extent_inf`, or the capped reach) stands
more than the band off the face's carrier on that side; where a root
door covers the edge and carrier kinds, its residual is the witness's
offset. The arms that now decide on the leading term only: a line or a
conic against a quadric or torus face, and a bisector against any face
(its own face's normal section stands in for its bend). The shipped
parting sites (`runs_into`, `crescent`, `corner_holds`, `walls_inside`,
`tangent_relative_side`, `tied_germ_turn`) are the same.
