---
id: split-keeps-has-two-builders-that-disagree
kind: issue
title: The Split's face Keeps and vertex Keeps are two hand-written builders that disagree on joins and on in-plane edges
status: open
opened: 2026-10-18
priority: P3
cost: M
---


## Finding

From the review of PR 5275 (2026-10-18). `crates/editor-core/src/names/emit_topo.rs`
builds N2's Split `Keeps` twice:

- `name_split_faces` ("Same-side multiplicity") reads a face piece's edges by
  descent alone (`chase_split_edge_to_table`). It goes through neither the
  join covers (`topo::join_covers`) nor `SplitNaming::joined_lineage`, and it
  cites no edge that lies in the tool plane.
- `name_split_edges_vertices` (the `OnToolVertex` arm) reads a corner copy's
  edges through the join covers and `chase_split_lineage`, and it credits an
  in-plane edge through the section chord along it (`chord_within`).

N2 defines one qualifier, so the two should be one builder parameterised by
what it reads: a face's boundary edges, or a vertex's own edges. Whether the
face reading cites an in-plane boundary edge is the question in
`a-split-names-an-operand-edge-in-its-plane-as-a-section-chord`.

Signed (wire-keeps lane).
