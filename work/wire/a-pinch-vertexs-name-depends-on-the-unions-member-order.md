---
id: a-pinch-vertexs-name-depends-on-the-unions-member-order
kind: issue
title: A pinch vertex's name depends on the union's member order: it is named by whichever block's corner reached it last
status: closed
opened: 2026-10-02
priority: P1
cost: E
closed: 2026-10-18
pr: 5270
branch: wire/union-fold-pinch-row
---


## What

In the pinch union of
`work/tang/a-pinch-union-refuses-ring-homing-in-one-member-order.md`
(plate, `p1`, `p2`; the two blocks' footprints meet at (1.5, 1, 1) on the
plate's top), the vertex at the pinch gets a different name depending on
the order of the union's members. The body is the same in all six orders
(`crates/editor-core/tests/union_pinch_member_order.rs`). Measured on
main before the TANG fix, with
`emit_boolean_vertex_keys::named_geometry`:

- `[p1, plate, p2]` and `[plate, p1, p2]` name it
  `Seam{plate Cap(End), p2 LateralEdge(Leg)}`;
- `[p2, plate, p1]` and `[plate, p2, p1]` name it
  `Seam{plate Cap(End), p1 LateralEdge(Leg)}`.

Whichever block reaches the plate second meets the vertex the first
block left there, so the vertex keeps the operand identity the earlier
fold step gave it. Since the TANG fix, the blocks-first orders fuse the
two pierces into one vertex with two incident A edges. They name it
through the seam-junction arm of `name_boolean_edges`
(`crates/editor-core/src/names/emit_topo.rs`), which gives a third
spelling. The face, edge and vertex count is 101 names in every order;
only this vertex's spelling moves.
A name that is the same in every order needs one rule for the pinch: the
junction of its seam lines, or both legs.

## Measured order-free (2026-10-17, FORK-PINCHNAME)

Both designers measured this row against today's main, independently, and
found it overtaken by PR 4881's vertex-run rule (built in
`emit_union::Flush::meeting`) and TANG's PR 5178. The row's own fixture
(`union_pinch_member_order::pinch`) publishes one table, 99 rows, in all six
member orders; `through` (6/6), `two_pinches` and `corner_holes` agree at the
pinch, as does a variant where the blocks' corners meet on the plate. What is
left is the pin: assert one name table over `orders(3)` in
`tests/union_pinch_member_order.rs` (and the three-block fixture over
`orders(4)`), then close. Re-priced `E`, design cleared.

Signed (WIRE orchestrator).

## Closed (2026-10-18)

PR 5270 pins it: `union_pinch_member_order::a_pinch_union_publishes_one_table_in_every_member_order` asserts one name table over every member order of four fixtures, and goes red with `emit_union::Flush::meeting` switched off (the row's original defect). Merged on the orchestrator's read (test-only).

Signed (WIRE orchestrator).
