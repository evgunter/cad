---
id: a-lone-cut-piece-of-an-operand-edge-keeps-its-bare-name
kind: issue
title: A lone piece of an operand or member edge a cut shortened keeps the edge's bare name
status: open
opened: 2026-10-12
priority: P3
cost: M
---


## What

N2 names every piece of a divided edge by its line + `Ends`, "a lone
piece on its side of a cut included" (`names/README.md`, "Edge
pieces"). Two rows in `crates/editor-core/tests/emit_curved_rim_chord.rs`
publish a lone piece that is part of its edge under the edge's bare
name. Main does the same (checked with main's `emit_topo.rs`).

- **The circle through 0°** (`a_post_across_the_piece_of_a_notched_circle_through_0_unites`).
  The post x ∈ [0.8, 1.5] swallows the top half of the notched
  cylinder's lateral edge at (1, 0). The union publishes the stretch
  z ∈ [0, 0.5] as `From { read, of: <the lateral edge> }`, bare.
- **The cavity** (`a_cavity_cut_by_the_notched_prism_names_its_rim_pieces`),
  a pair subtract. The slotted box keeps the stretch z ∈ [0.5, 1] of
  the prism's lateral edge at the origin, and publishes it bare.

`within_sources` in that file asserts that a name spelled as a piece is
part of its row. It does not assert the converse (a bare name is all of
its row), because of these two rows.

In the pair emitter, the bare name comes from `whole` in
`emit_topo::name_boolean_edges`'s operand-edge groups. That compares
the edge's end vertices with the operand edge's own, through
`operand_vertex_keys`. Why it reads a cut end as the operand edge's end
here is not established.

## Next

Find why `whole` holds for the cavity's piece, and what makes the union
name its piece bare. Then add the converse assertion to
`within_sources`. Siblings: `a-union-seam-cut-to-one-piece-keeps-its-bare-name`
(a union seam) and `a-second-crossing-by-one-face-renames-the-first-and-its-pieces`.
