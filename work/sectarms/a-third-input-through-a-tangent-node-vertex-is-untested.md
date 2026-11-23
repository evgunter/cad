---
id: a-third-input-through-a-tangent-node-vertex-is-untested
kind: issue
title: A third input through a result vertex where two faces are tangent (a node from A∘B) is untested
status: open
opened: 2026-11-03
priority: P3
cost: M
refs: [6398]
---


Found while answering Ev on `[ev]` PR 6398 (the node fork).

A boolean has two inputs, and an n-ary union or intersection folds them
two at a time (`crates/topo/src/boolean/nary.rs`, module doc). After
A∘B, a section node such as the two equal cylinders' bisector nodes
becomes a vertex of the result where four edges meet and two faces are
tangent. When a third input C then passes through exactly that vertex,
the general vertex-on-face code takes over: `boolean/reduce.rs`
(`vertex_on_curved_face`, around line 5319) for curved faces, and
`vertex_section.rs` for planes. No row exercises that code on a vertex
where faces are tangent, so it is not known whether such a case builds
or refuses.

Not to be confused with a third face of one of the two inputs passing
through a node, which refuses `NodeUnbuilt{OnBoundary}`
(`boolean/node.rs:140`) and is pinned by
`crates/sweep/tests/bisector_nodes.rs`.

**Done when** a row builds ((A∪B)∪C), and the same with ∩, where C passes
through a bisector node of A∪B. Each must build sound or refuse typed,
and its outcome is pinned.
