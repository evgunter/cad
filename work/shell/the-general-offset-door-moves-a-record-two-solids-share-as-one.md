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
`RevolvedKind::Partial::coplanar_caps`), split by the axis edge. The
witness is `crates/sweep/tests/torax_axial.rs`
`torax_the_half_turn_lune_refuses_at_its_axis_edge`.

2026-11-04: `lune(0.3, π)` is three faces: the sphere wall on its own
record, and the two caps on one plane record, both at sense `true`, so
they share one material side. `shell` at `t = 0.05` refuses
`ShellError::Face { TogetherEdgeDisagreement { edge, gap } }` at the
axis edge, whose two sides are the two caps on that one record. The gap
is `t` exactly: the edge's minted curve stands `t` off one of the moved
surfaces it separates (`offset_axial.rs`, the `offset_axial_edge_on_surface`
check). The axial door has a same-record seam arm, which translates a
plane's seam by the chart's rigid move (`mint_carrier`), so the gap
says the edge's two sides reach the door as charts that moved apart.
Which chart, and why, is unread.

The refusal is fail-loud. Every in-scope edge's minted carrier is
checked at its midpoint against both moved surfaces before its spec is
written. A gap past the band refuses, and a gap in band escalates. No
path returns a body with the edge left behind. A hollow half ball is
buildable, so this is the door's defect.

The meridian-pair arm's parallel-caps `TogetherAxialCorner` refusal is
left as it is. No public build reaches it any more: two caps containing
the axis that are parallel are coplanar, and a revolve now leaves
coplanar caps on one record. Whether to make it a
`ClassificationInvariant`, or to pin it from an arm-level Body, is this
program's call. The arm's other `TogetherAxialCorner` refusals keep
their witnesses (`crates/sweep/tests/shell_curved_mouth.rs`
`opening_a_tangent_dome_refuses_two_equidistant_roots`, the tie).
