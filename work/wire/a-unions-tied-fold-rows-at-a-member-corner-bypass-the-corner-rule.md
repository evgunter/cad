---
id: a-unions-tied-fold-rows-at-a-member-corner-bypass-the-corner-rule
kind: issue
title: A union's Tied fold rows at a member corner keep the fold's own Borders name and bypass name_corner_copies, so a corner the fold half-tied is spelled two ways
status: dispatched
opened: 2026-10-26
priority: P3
cost: M
---


Found by PR 5535's re-review (2026-10-26), by inspection, with a probe count.

`emit_union.rs` (~1706, the `one` match): a `Tied` fold row is never checked against `member_vertices`, so it keeps the fold's `Fragment(Borders)` name, and `name_corner_copies` (PR 5535) sees only what the fold left `Unique`. The probe hit 16 such rows at member corners in `union_member_orders_decide_alike::a_vertex_row_is_the_unions_once_in_every_member_order`.

**Mixed case (no witness).** Two copies that the fold tied, plus one it named `Unique`. The lone `Unique` copy takes the bare corner name, while the other two keep the fold's fragment names. If the union's own split then mints a name equal to the fold's tied name, the result is a typed `Duplicate`.

**Witness recipe.** Build a union that nests that pinch union as a member, plus a third member that divides one candidate's region at the corner. The same recipe would also witness PR 5535's new tied multi-copy path, which has no witness yet.

## Findings (wire-unionvertex, 2026-10-28; not taken)

**Witness without a new fixture.** `union_member_orders_decide_alike`'s
`prism_and_cube` (and `pinch_and_far`) hold the L-prism's reflex top
corner as one fold `Tied` row in every member order. The name is
`From(prism, CapVertex(End, ..)) / Borders([From(cube, Cap(Start))])`,
over two vertices. The union's own corner rule never sees those
vertices.

**Routing them through the corner rule does not fall out of the same
pass.** I tried it in `cite_member_edges`: split a `Tied` vertex row's
candidates, send those at a member corner (`Flush::member_vertices`) to
`corners`, and keep the rest as the row's tie. That made
`name_invariance::an_operand_order_moves_no_name` refuse on
"pinch and cap / intersect" and "pinch and far / union" with
`Naming(Emission "a copy of a corner held as several meets no recorded
discard between them")`. The union's `member_corner_walls` →
`Obstacles::split_corner` finds no discard between the two copies,
which the fold had told apart only well enough to tie them. So the
repair needs the union's corner walls to read what the fold read there.
That is more than a routing change. The mixed case (two tied copies
plus one `Unique`) has no witness yet, and whether it can `Duplicate`
is still open.

**Interim.** Since this PR, a union's vertex group is formed over the
finished body. The bases of these kept fold rows are the one exception:
they are left to the fold's record (`Rederived::by_fold`). When this
row lands, the corner rule names those copies, `by_fold` goes empty,
and `Rederived::by_fold` can be retired.

**Assumption this row retires.** `Rederived::by_fold` reads the
fold's record for the base of a `Tied` fold row at a member corner.
That reading is order-free only if every member order's fold ties the
corner. Whether the fold ties it can depend on order, and then one order
reads the fold and another the union's own corner group. The suite
holds no such order (0 disagreeing pairs), but that is a measurement of
the suite, not a proof. Once this row's routing lands, `by_fold`
empties and the assumption goes with it. The site is commented in
`emit_union`'s `cite_member_edges`.
