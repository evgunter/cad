---
id: a-drawn-vertex-with-no-name-is-left-out-of-the-mark-table
kind: issue
title: an unnamed drawn vertex is left out of the vertex mark table silently, where an unnamed edge is counted
status: closed
closed: 2026-11-02
opened: 2026-11-02
priority: P3
cost: E
---

## Finding

Found by the review of PR 6284 (S7). `pickindex::drawn_vertices`
(`crates/viewer/src/pickindex.rs`) builds the index's vertex table from
`NodePick::boundary_end_names`, and an end whose lookup answers
`UnnamedEntity` (the naming layer's bug report) is skipped: nothing is
counted or kept. The edge window counts the same refusal
(`PickIndex::edge_names_in`'s `EdgeNamesRefused`), so a body some of
whose edges are unnamed says so, and a body some of whose vertices are
unnamed does not. The pick door is loud for the vertex under the cursor
(`PickError::VertexName`), but a vertex never aimed at is invisible to
every reader of the table.

**Done when** the index holds the vertices it could not name as it
holds the edges (a count and the first refusal), and a row with a
seeded unnamed vertex reads it.

## Closed

The index's vertex table (`Vertices`, `crates/viewer/src/pickindex.rs`)
keeps, per drawn body, the naming layer's refusal for each vertex it
could not name, once a vertex, and `PickIndex::vertex_names_refused`
answers the count and the first refusal (`VertexNamesRefused`, the edge
window's `EdgeNamesRefused` shape). Row:
`pickindex::tests::a_bodys_unnamed_vertex_is_counted_and_its_refusal_kept`.
