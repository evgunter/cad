---
id: a-pair-boolean-names-an-edge-bearing-meeting-as-a-seam-junction
kind: issue
title: A pair boolean names a pinch, and a point both operands' edges run through, as a seam junction where N2 names it by its lines
status: closed
closed: 2026-10-13
opened: 2026-10-13
priority: P2
cost: M
---


## What

Found in review of the vertex-run build (branch `emit/vertex-run`).
N2 names a vertex where two or more lines meet as an `EdgeCrossing`
run of those lines, and one line meeting one face as its `Crossing`.
The pair boolean still names two edge-bearing shapes as a seam
junction, the run of the seam lines' `Seam` segments
(`emit_topo::name_boolean_vertices`, the junction arm):

- a pinch, where several edges of one operand pierce a face of the
  other at one point (`(n, 0)` or `(0, n)` operand edges);
- a point that edges of both operands run through, one of them on one
  side (`(1, n)` or `(n, 1)`).

A seam junction with no operand edge (`(0, 0)`) is not this row: N2
counts a seam as a line only at a union, so a pair boolean's junction
of seams has no other spelling.

## Measured (a trial conversion on the branch, reverted)

The conversion counted each side's flush edges once (straight edges on
one line), named one line and one face of the other operand as the
`Crossing`, and named two or more lines as their run, each with its
sense from `senses_at`. Over the full editor-core suite:

- 125 vertices took the new arm, all in union fold steps: `(2, 0)` ×65,
  `(3, 0)` ×30, `(4, 0)` ×24, `(0, 2)` ×5 and `(0, 3)` ×1. The union
  re-reads every such vertex from the finished body, so no published
  name or golden moved.
- One new refusal: the plate less the three wedges' union
  (`union_pinch_member_order::a_subtract_against_the_wedges_union_names_their_meeting_alike_in_every_member_order`,
  member order `[2, 1, 0]`): "a crossing's edge has no piece the
  boolean classified at the crossing". A line at the point had no
  `edge_classes` row there, so it had no sense to carry.

## Next

Find which of the union's edges at that point has no classified piece.
Either it is a line N2 should not list there, because it ends at the
point as an operand vertex the corner rule missed, or the boolean owes
its row. Then convert the arm. The `(1, 0)` shape with several faces of
the other operand and no edge of it has no N2 spelling for a pair, and
needs one before the arm can close.

## Closed

Branch `emit/pair-meeting-run`.

**The refusal's root cause.** The line with no classified piece was a
copy of an operand vertex. The zip splits a pinch per cone
(`topo::boolean::zip::split_cones`), minting a new vertex on the split
vertex's point for each cone it moves. Nothing recorded the split, so
the copy read as no operand vertex: the corner rule missed it, and the
union's vertex, which P − U holds as that copy, was read as a meeting of
lines, one of which has no `edge_classes` row there because it ends at
the point. The boolean now records each split
(`BooleanNaming::cone_splits`, `(split, new)` in result keys before the
fusions). The emitter reads a copy as the vertex it copies
(`emit_topo::Fused::origin`), and a discard's stretch ending at a split
vertex is read at its copies (`Settler::settle_cones`). The topo change
only records; no geometry or topology moves. A B discard's dead boundary
edges now join obstacles in B's clone keys (`borders.rs`), which the
copies' `Borders` reading needed.

Evidence: with `Fused::origin` forced to the identity, P − U refuses
again with this row's message: "a crossing's edge has no piece the
boolean classified at the crossing". It does so for three wedges in
member order `[2, 1, 0]`, and for four wedges in `[3, 2, 1, 0]`. The
four-wedge case is the refusal WIRE's row recorded on main before any
conversion, so the two refusals share this cause.

**The conversion landed.** At a seam vertex with operand edges where
two or more seam lines meet (`emit_topo::name_boolean_vertices`), each
side's edges are counted as lines, flush straight edges once as the
least (`flush_lines`). One line and one face of the other operand is
that line's `Crossing`, and two or more lines are their `EdgeCrossing`
run, each line with its sense. A junction with no operand edge,
`(0, 0)`, keeps the seam-junction spelling. One line meeting several
faces of the other operand has no N2 spelling for a pair. It refuses
as a missing rule (`SeamVertexParentage`) and is filed as
`a-pair-boolean-has-no-spelling-for-one-line-meeting-several-faces`
(design). No test reaches it.

**Measured** over the full editor-core suite: 124 vertices take the
converted arm, all in union fold steps. Each is a pinch whose edges are
one line meeting one face, so each is a `Crossing`:
`(2, 0)` ×65, `(3, 0)` ×30, `(4, 0)` ×24 and `(0, 2)` ×5. No golden
moved. P − U and U − P now name the wedges' meeting point only by the
union's vertex, carried, in every member order of three and four wedges
(`union_pinch_member_order::a_subtract_against_the_wedges_union_names_their_meeting_alike_in_every_member_order`,
which now asserts that every name at the point is carried and runs four
wedges).
