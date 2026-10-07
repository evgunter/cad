---
id: a-touch-and-an-edge-edge-crossing-have-no-ruled-spelling
kind: issue
title: The sense ruling names no spelling for an edge that touches a face without crossing it, nor for two edges crossing; the build keeps Seam for the first and mints EdgeCrossing for the second
status: open
opened: 2026-10-06
priority: P2
cost: M
design: true
needs_ev: true
branch: emit/ev-touch-spelling
refs: [a-second-crossing-by-one-face-renames-the-first-and-its-pieces]
---


## What

The ruling on `a-second-crossing-by-one-face-renames-the-first-and-its-pieces`
(Ev, PR 4134) names a crossing by its sense and spells the boolean's edge ×
face vertex `Crossing { edge, face, sense }`. Two shapes it does not settle
were reached while building it.

**A touch.** An edge that meets a face of the other operand without entering
or leaving its closed body: an edge ending on the face from outside, or
lying in the closed body on both sides of the vertex. N2's *Vertices* bullet
defines the sense as entering or leaving, so such a vertex has none. The
build keeps the old spelling there, `Seam { edge, face }` (ranked along the
edge as before), and mints `Crossing` only where the edge crosses
(`emit_topo::name_boolean_vertices`, `emit_topo::senses_at`). Reached: the
corpus document `crossing_slots` (an A corner whose lateral edge ends on B's
face from outside), `m9_1_declare_classes::a_wrong_class_declaration_refuses_at_the_op`,
`review_decl_r1::a_merged_row_contact_is_declared_through_its_constituents`,
`emit_seam_junction::a_slab_crossing_a_merged_rim_is_named_by_the_rim_and_the_slab`.
An edge × edge vertex where either edge does not cross is spelled
`Seam { a, b }` the same way.

**Two edges crossing.** N2 says such a vertex "carries each edge's sense
against the other operand's closed body" and names no head. The build mints
`EdgeCrossing { a, a_sense, b, b_sense }` (`role.rs`), in name order in a
union with each sense beside its edge (`canonical.rs`), and ranks a
same-senses group along the A edge as before.

## The question

Whether a touch keeps `Seam`, takes a head of its own, or refuses; and
whether `EdgeCrossing` is the spelling, written into N2 if so. A design
fork, to weigh before anyone changes either.
