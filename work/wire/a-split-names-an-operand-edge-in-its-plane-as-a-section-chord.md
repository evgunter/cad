---
id: a-split-names-an-operand-edge-in-its-plane-as-a-section-chord
kind: issue
title: A split names an operand edge lying in its plane, and the copies of it, as section chords: the edge's own name dies, and a face piece's Keeps cannot cite it
status: open
opened: 2026-10-18
priority: P2
cost: M
---


## Finding

Measured by the `wire-keeps` lane (2026-10-18, building the Split vertex
`Keeps` rule): the `NOTCHED` prism of `crates/topo/tests/m3_pr3_split.rs`,
extruded 1 and split at `y = 1`. The notch tip's lateral edge
`LateralEdge(tip)` lies in the tool plane. Its operand key survives in the
lower half, and the upper half holds one new edge along it per piece
touching the tip. `crates/editor-core/src/names/emit_topo.rs`
`name_split_edges_vertices` names all three by its chord pass
(`chord_faces`, which lists every boundary edge of a section face, operand
edges included), each `SectionEdge { face }` for the operand face across it:
`SectionEdge { Lateral(tip) }` below, `SectionEdge { Lateral(left rim) }`
and `SectionEdge { Lateral(right rim) }` above. Nothing in either half's
table is named `LateralEdge(tip)`, so a reference to that edge through the
split resolves in neither half, although the lower half holds it whole.

(The FORK-PINCHNAME designers expected these edge copies to be carried as
one name twice and refuse `Duplicate`. At this tree they do not: the chord
pass names them for different faces.)

A second consequence: the face `Keeps` lane (`name_split_faces`, "Same-side
multiplicity") reads a piece's boundary edges only by descent
(`chase_split_edge_to_table`), so a face piece bounded by a chord along a
parent boundary edge that lies in the plane does not cite that edge. Two
pieces of one face in one half that differ only there would tie. The
vertex `Keeps` rule (N2, PR 5258) cites such an edge ("an edge lying in the
tool plane included"), and its build reads the chord along it
(`chord_within`); the face rule does not say either way.

## Question

Is an operand edge lying in the tool plane a pass-through (its own name in
the half that holds its key) and its copies `SplitFragment { parent }` +
`Ends`, as N1/N2 would read an intact edge and its pieces, or a section
chord, as today? N2 names neither case. If it changes, N2's face `Keeps`
sentence should say whether an in-plane boundary edge is cited.

Signed (wire-keeps lane).
