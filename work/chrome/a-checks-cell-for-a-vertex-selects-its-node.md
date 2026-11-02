---
id: a-checks-cell-for-a-vertex-selects-its-node
kind: issue
title: a Checks-window cell naming a vertex selects its node, though a vertex is now a selection
status: open
opened: 2026-11-02
priority: P3
cost: E
---

## Finding

Found by PLACING's `placing/mate-kinds`, which made a vertex a
selection (`Selection::Vertex`, `VertexSelection`,
`crates/viewer/src/session/select.rs`), picked in the viewport under
`PickKinds::WithVertices` and marked by `marks::vertex_segments`.

The Checks window's named cells (`crates/viewer/src/frame.rs`, the
`NamedCell::Entity` arm) still turn a vertex's cell into a selection of
its node — "as a vertex's or a body's does" — because no vertex
selection existed when it was written: the cell reads a drawn body
only for `EntityKind::Face | EntityKind::Edge`, and selects the node
otherwise.

**Done when** a vertex's cell selects the vertex on the drawn body it
names (`Selection::Vertex`), its button saying the pick as a held pick
says one (`PickedName::vertex`), and a row clicks one.
