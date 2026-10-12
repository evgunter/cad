---
id: a-touch-and-an-edge-edge-crossing-have-no-ruled-spelling
kind: issue
title: The sense ruling names no spelling for an edge that touches a face without crossing it, nor for two edges crossing; the build keeps Seam for the first and mints EdgeCrossing for the second
status: open
opened: 2026-10-06
priority: P2
cost: M
pr: 4284
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

## Ruled (Ev, PR 4284, 2026-10-08)

Ev approved the recommendation ("i think this makes sense"). The names
README states it (N2 *Vertices*, the flush paragraph, N5's cutters
bullet), and fork-log row 117 records it.

**What to build:**
- **Corner identity.** A vertex that is one of an operand's own vertices
  is named as that vertex. `operand_identity` reads it through every
  operand key the boolean leaves at the point (`operand_vertex_keys`,
  and the contact rows), not only the kept key and its zip partners.
- **A four-valued sense:** `Enters`, `Leaves`, `Outside`, `Inside`.
  - `flipped` swaps `Enters` and `Leaves` and keeps the other two.
  - `CrossingSenses` carries all four.
  - The words read "the touch where E meets F from outside/inside".
- **Every edge x face vertex is a `Crossing`, and every edge x edge
  vertex is an `EdgeCrossing { a, a_sense, b, b_sense }`.** `Seam` stops
  heading them, and the absent-side fallback ("no portion counts as
  outside") goes.
- **A union reads its edge x edge vertices from the finished body**, like
  its edge x face ones. This closes
  `an-edge-edge-crossing-vertex-of-a-union-is-spelled-by-member-order`.
- **The residue: three or more members' edges at one point.** Ev leans
  to A's reading, unsure: read each edge's sense against all the other
  members together. Build it that way only if every `union_pinch` row
  keeps one name in every member order. Otherwise keep today's spelling
  there, say why in the PR, and file the residue.
- **Re-baseline the goldens** (`crossing_slots` and the pinned digests),
  turn the pinned emission refusals that now fuse into builds, and check
  that those orders publish one table.

