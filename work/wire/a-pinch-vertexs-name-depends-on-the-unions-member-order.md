---
id: a-pinch-vertexs-name-depends-on-the-unions-member-order
kind: issue
title: A pinch vertex's name depends on the union's member order: it is named by whichever block's corner reached it last
status: open
opened: 2026-10-02
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
