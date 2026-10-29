---
id: a-boolean-whose-face-is-tangent-to-a-holes-rim-at-a-vertex-refuses-its-strut-order
kind: issue
title: A boolean whose half-space face is tangent to a hole's rim at a vertex refuses at bool_strut_order: two germs along one tangent are not ordered by how they curve
status: closed
opened: 2026-10-28
priority: P1
cost: M
refs: [a-plane-tangent-to-a-holes-rim-at-a-vertex-refuses-its-germs-as-coincident, germ-orders-about-a-normal-have-two-homes]
branch: splitdoor/boolean-strut-order
closed: 2026-10-29
pr: 5968
---

Found by the sweep of `a-plane-tangent-to-a-holes-rim-at-a-vertex-refuses-its-germs-as-coincident`
(branch `splitdoor/hole-rim-tangent-germs`), the boolean door of the
pose that unit fixed for `split`.

## What

The block `[-2, 2]² × [0, 2]` with a circle hole of radius 1 about the
z axis, its vertex at azimuth π/2 (`holed_block` and
`two_arc_holed_block` in `crates/sweep/tests/a_split_through_a_wrap_edges_vertex.rs`),
turned about x by `t` and booleaned against the brick `z ≤ z₀` whose
top face passes through the turned vertex, the construction of that
file's `a_boolean_through_a_strut_vertex_keeps_one_face_where_the_piece_goes_round`.
At `t = 0.4` and `0.9` the top face leans onto the vertex: its trace on
the block's bottom face is tangent to the hole's rim there, with
material on both sides. A − B, A ∩ B and A ∪ B all refuse
`Escalated { decision: Coincidence(Sectors, Moot) }`, predicate
`bool_strut_order`, margin ~1e-16 rejected as Zero, for the
one-segment hole and the two-arc hole alike, at ε = 1e-9. At
`t = −0.4, −0.9` (leaning away) all three build. `split` builds the
same poses at their closed forms.

## Reading

`boolean/insert.rs` `strut_order` places two germs by their side of
the arrival edge, then orders two in one half-turn by
`(g0 × g1)·n` (`bool_strut_order`) through `decide_nonzero`; its doc:
"a decided zero … of two germs along one direction, refuses: nothing
orders the germs". Here the two germs are the cap's straight trace and
the hole wall's section, tangent at the vertex: an exact tangency of
decidable order. `walks_after` (`bool_shared_cut_order`) returns `None`
at the same tie. The split orders the same pair one order down
(`splitting/insert.rs` `section_chords`, `curving`): each face's
section curvature off the shared tangent toward the plane's quarter
turn of it (`geom_brep::implicit_graph_acceleration`), differenced and
read by `geom_brep::enters_material_order2`. Unmeasured: whether that
descent at `strut_order`'s tie is all the boolean needs, and where the
germs' faces are reached from there.


## Closed (2026-10-29, PR 5968)

The Boolean orders two germs of one pierce that tie at first order by their bends, one order down. `geom_brep::tied_germ_turn` is now the one home shared by the split and the ring. `geom_brep::implicit_section_acceleration` gives the bend in a curved face, and at a cone apex the bend is 0.

A − B, A ∩ B and A ∪ B build at their closed forms at three ε for every pose of this row, except the one-segment hole's single-site class. That class refuses `JoinDesync` in the join and is held on SECTTAIL's row.

FULL single review: APPROVE. It probed 1440 poses and found no wrong volume.
