---
id: value-decided-vertex-fusions-are-recorded-with-the-undeclared-glue
kind: issue
title: A boolean's vertex fusions decided by a margin are coincidences D10 records; E records them when undeclared touch glues
status: parked
opened: 2026-10-08
priority: P1
cost: M
blocked_on: [booleans-glue-on-zero]
---


## What

The boolean's reduction makes an A vertex and a B vertex one point by a
margin, and the finish fuses them (`PairSite::VertexVertex` → the seam
correspondence → `BooleanNaming::vertex_merges`). On an undeclared
corner-to-corner union the two pieces touch there. D10 says every such
coincidence is recorded at the door; INTENT stage 4 PR B records the
declared one-carrier glue, the split's pinch and the battery's turn, and
none of these (orchestrator ruling on S4-B, 2026-10-08).

The deciding sites, each pushing a `VvContact` into the reduction's
`ContactAcc`:

- `crates/topo/src/boolean/mod.rs` `one_vertex` (`bool_contact_vertex`,
  a magnitude decided Zero), read at `reduce.rs`
  `vertex_on_curved_face_at` (~:3963);
- the containment ladders' `FaceContainment::OnVertex` arms,
  `reduce.rs` ~:1221, ~:1419, ~:1510, ~:3921 and ~:4053 (`push_vv`).

None returns its margin: `one_vertex` answers a `bool`, and the
containment arms read a verdict whose margin is inside `contain.rs`.

## What closing it takes

Thread each site's decided `MarginDiag` out (`one_vertex` returning
the `Decided`, the containment verdicts carrying theirs), and record a
`topo::Coincidence` per A/B vertex identity (`(Vertex, Vertex)`,
`Relation::OnCarrier`, a `DecisionSite` of its own) beside E's Zero
glue. Like any row, it is provable by the door (the same construction
read twice, or C's margin identity at `Sym`). The `ContactRecords` rows
these fusions back cite them through
`contact-records-cite-their-decision`.
