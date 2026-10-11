---
id: surgery-carry-keeps-a-divided-cells-record-on-the-piece-that-kept-its-key
kind: issue
title: The surgery carry keeps a record on a divided cell on the piece that kept its key, not on the piece that holds the touch
status: open
opened: 2026-10-11
priority: P1
cost: M
refs: [topo-surgery-verbs-drop-a-bodys-contact-records]
---

## What

`ContactRecords::carried_through` (`crates/topo/src/boolean/records.rs`)
maps each record's cells through the op's lineage, and every surgery
verb's carry (`SplitResult::carry_contacts`, `Shelled::carry_contacts`,
`Blended::carry_contacts`, `OffsetOutcome::carry_contacts`,
`MergeCoplanarOutcome::carry_contacts`) reads a cell that SURVIVES under
its key as itself. A cell the op divided keeps its key on one piece:
`Body::split_edge` keeps the parent's key on the first child
(`Provenance::SplitEdge`), a chord `mef` keeps the face's key on one
fragment (`SplitNaming::face_fragments`), and the shell rim surgery keeps
a divided seam's key on its first piece (`RimNaming::seam_pieces`).

So a record whose touch lies on another piece lands on the wrong one.
It is loud, not silent: the census then finds the record stale and the
touch undeclared. But it is a false refusal of a body that is fine.
D1's record paragraph (`docs/DESIGN.md`) already rules the edge case:
"an edge split moves a `(vertex, edge)` record onto the piece the vertex
rests on, and an `(edge, edge)` record onto every pair of pieces whose
interiors still meet, by the split's own lineage". The carry does not
build that rule yet, and D1 says nothing yet for a divided face.

Measure first: a split plane through a recorded vertex–face record's
face, with the vertex on the fragment that did not keep the key, then
`validate_pseudomanifold` with `SplitResult::carry_contacts`' side.
