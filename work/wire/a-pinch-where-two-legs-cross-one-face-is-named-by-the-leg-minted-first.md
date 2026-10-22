---
id: a-pinch-where-two-legs-cross-one-face-is-named-by-the-leg-minted-first
kind: issue
title: A pinch where two members' legs cross one face is named the crossing of the leg minted first, so the authoring order picks the name
status: closed
priority: P2
cost: M
opened: 2026-10-20
closed: 2026-10-20
pr: 5488
branch: wire/union-reads-not-mint-order
---

Filed by WIRE's `wire-nameinv` lane (branch `wire/name-invariance-checks`),
found by `crates/editor-core/tests/name_invariance.rs`
(`which_twin_was_authored_first_moves_no_name`,
`an_authoring_history_moves_no_name`).

## What

In `union_pinch_member_order::pinch` (the plate, `p1` and `p2`; the blocks'
footprints meet at (1.5, 1) on the plate's top), the vertex where both
blocks' vertical legs at (1.5, 1) cross the plate's top is named
`Crossing { edge: <p1's leg>, face: <plate Cap(End)>, sense: Leaves }`
when `p1` is minted before `p2`. Authored `p2` first (`[p2, p1, plate]`,
with the union still listing `[plate, p1, p2]`), it is the crossing of
`p2`'s leg. So are the two leg pieces whose `Ends` cite it: 18 names move
over the five other authoring orders. The twin fixture (two blocks of one
size touching along the line through (1.5, 1) on the plate) moves 6 names
when only the twins' authoring order is swapped.

The union's member order is not the leak: the member-order rows
(`union_pinch_member_order::a_pinch_union_publishes_one_table_in_every_member_order`)
pass, because every document there is authored in one order. The rule
picks one of the two legs by name order, and name order compares node ids,
which order as they were minted.

## What it needs

One name for the point whichever leg was minted first: both legs (an
`EdgeCrossing` run of the two, as the junction rule names wedges meeting
at a vertex), or the junction of the seam lines, as
`a-pinch-vertexs-name-depends-on-the-unions-member-order` proposed.
Unmeasured: the site is likely the vertex-run rule in
`crates/editor-core/src/names/emit_union.rs` (`Flush::meeting`).
