---
id: value-decided-vertex-fusions-are-recorded-with-the-undeclared-glue
kind: issue
title: A boolean's vertex fusions decided by a margin are coincidences D10 records; E records them when undeclared touch glues
status: closed
opened: 2026-10-08
priority: P1
cost: M
closed: 2026-10-10
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

## Released by INTENT stage 4 E (`intent/s4-e-glue-on-zero`) (2026-10-10)

E records the face pairs it glues: ladder-decided `Rest`/continuation rows and witness-verified `Tangent`/`Seam` rows. It does not record vertex fusions. `one_vertex` still answers a `bool`, and the containment arms still keep their margins. Recording them is this row's work, and it is dispatchable.

## Closed (2026-10-10, with `contact-records-cite-their-decision`)

B2 records the reduction's vertex identities. `one_vertex` answers the
margin it decided (`one_vertex_at`), the containment doors answer the
margin of a boundary verdict (`contfp_decided`,
`curved_face_placement_decided`, through `EdgeContact::On`,
`ConicHit::On` and `SpiricHit::On`), and the planar and curved endpoint
lanes keep their residual's `Decided`. Each push into the reduction's
`ContactAcc` carries its pending decision; the result's carry emits a
`Coincidence { site: VertexFusion, relation: OnCarrier }` for exactly
the decisions a surviving record cites (D1: a decision whose record
dies placed topology), cells read back to the input edge a minted
vertex was split from. Ruled with the orchestrator on B2.
