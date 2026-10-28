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
