---
id: the-general-offset-door-moves-a-record-two-solids-share-as-one
kind: issue
title: The general offset door keys a moved chart by its surface record, so it refuses to move apart faces of two solids that one record carries
status: open
opened: 2026-11-03
---


A boolean now leaves each glued one-carrier pair's faces on one surface
record (`Body::unify_surfaces`, DESIGN `[coincidence.door]`), so a record
can be worn by faces of two solids: the union of two cubes kissing at a
corner leaves the first cube's top and the second's bottom on one plane
record (`crates/topo/tests/offset_a_corner_kiss.rs`).

`offset_surfaces_together` refuses moving those two faces by separate
moves, `SharedSurfaceKey`: its chart groups are read over every solid the
moves touch (`replace_face.rs`, `ChartGroups::within` over
`Scope::of_solids`), and every moved chart is found by its old key
(`replace_face.rs` `moved_of`, `chart_slots`, and the "two moves on one
surface key" assert in the hook beside them). The refusal's own text says
the shared key is "within its solid", and a corner's charts are one
solid's (`offset_together.rs` module docs, "The unit is the SOLID").

The planar door had the same keying at its corner gather and now reads a
moved plane by its chart and its move (`offset_together.rs`
`MovedPlane::same_move`), which parts the kiss. The general door wants
the same: a chart group and a moved chart's slot per solid and record.
The corner-kiss rows pin the refusal until then.

The same keying meets a revolve's caps. A half-turn revolve now leaves
its two coplanar caps on one record (`sweep::revolve`,
`RevolvedKind::Partial::coplanar_caps`), the two worn at opposite senses.
`crates/sweep/tests/torax_axial.rs`
`torax_the_half_turn_lune_refuses_the_parallel_cap_pair` pins `topo::shell`
refusing at the corner (`TogetherAxialCorner`, "parallel"). It now refuses
`TogetherEdgeDisagreement` at the axis edge: the door moves the one record
by one distance, where the two caps' material sides want it moved two ways.
