---
id: a-discard-border-inside-a-merged-face-or-joined-edge-reads-as-no-border
kind: issue
title: A discard's bordered stretch whose end settles inside a merged face or a joined edge is read as no border
status: open
opened: 2026-10-09
priority: P3
cost: M
design: true
---

`BooleanNaming::settler` settles each result vertex, fused or not, on
the live cell its point lies in (`crates/topo/src/boolean/ops.rs`): the
vertex it finally fused into, or the edge the output stage's join made
of it, or the face the merge glued over it. The naming layer reads a
discard's `bordered` stretches through it
(`crates/editor-core/src/names/borders.rs`, `Obstacles::record`): a
stretch with an end inside a joined edge lies along that edge where the
other end lies on it (`stretch_through_joins`), and otherwise along no
edge, so the discard borders nothing there; an end inside a face lies
on no edge.

What the typed cell could say instead: that the border runs inside face
`f` (the merge glued the faces on its two sides, so nothing of a piece
lies along it), or inside edge `e`. Whether `Borders` should cite such a
stretch, and as what, changes what the names say, so it is a design
question for the naming layer and not a kernel fix.

Measured on the lattice-brick corpus
(`crates/topo/tests/fused_into_live_cells.rs`, which pins the count: 1000
bricks against `[0, 2]³` under all three operations, every coplanar
pair declared): 2448 of 27,000 bordered stretches have an end, fused or
not, that settles inside an edge or a face.
