---
id: a-split-through-a-pinch-names-both-tip-copies-alike
kind: issue
title: A split whose plane passes through a pinch vertex refuses Naming(Duplicate): both copies of the vertex on one side are OnToolVertex of it
status: open
opened: 2026-10-08
priority: P1
cost: M
---


## What

A split whose plane passes through a vertex that the cut leaves pinched
(its neighbourhood has two runs on one side, so two pieces of that side
touch there) refuses `NodeErrorKind::Naming(Duplicate)`. The split's
reduction gives the vertex one copy per run (`topo/src/splitting/insert.rs`
`insert_null_edges`), so the pinched side holds two vertices at the one
point, and `name_split`'s vertex pass
(`crates/editor-core/src/names/emit_topo.rs`, the `OnToolVertex` arm,
"the plane passed THROUGH an operand vertex") names each of them
`OnToolVertex { side, of }` with the operand vertex as `of`. Two
entities, one name.

Reproducer: the notched block of `topo/tests/m3_pr3_split.rs`
(`NOTCHED`, a V-notch whose tip `(4, 1)` has both neighbours above
`y = 1`), extruded one unit through the document and split by the plane
`y = 1`. The kernel's split builds (its own suite pins the halves); the
document's split node refuses:

```
Naming(Duplicate { name: StableName { kind: Vertex, node: <split>,
  path: [OnToolVertex { side: Above, of: <extrude>.CapVertex(Start, Piece { role: Leg }) }] } })
```

## Why it matters now

INTENT stage 4 PR B records the split's pinch as a coincidence row
(`DecisionSite::SplitOn`, `topo/src/splitting/mod.rs` `reduce`), and
`coincidence_door.rs` would read that row through the document. With
this refusal no document can reach a pinch split, so the row is pinned
at the kernel only (`topo/tests/m3_pr3_split.rs`
`a_split_records_its_pinch_and_nothing_where_it_only_cuts`).

## Fix shape

The copies are told apart by the run each took (the run's sector face,
or which piece of the side holds it); the name needs that qualifier, or
the pair needs a tie (N2). Which one is a naming decision for the wire
program.
