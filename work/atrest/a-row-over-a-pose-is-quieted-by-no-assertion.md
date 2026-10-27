---
id: a-row-over-a-pose-is-quieted-by-no-assertion
kind: issue
title: A row over a split's tool pose or a profile piece is quieted by no assertion: Gap reads no pose
status: open
opened: 2026-10-27
priority: P3
cost: M
---


A split's ON row (`DecisionSite::SplitOn`, `crates/topo/src/splitting/mod.rs`
`reduce`) is `[Vertex(A), Tool]`: an operand vertex decided on the
cutting plane. Its second cell is the split's tool read
(`coincide::NamedCell::Tool`), a pose and not a cell of any body. A
profile junction's row (`NamedCell::Piece`) has the same shape. `Gap`
admits only a face, an edge or a vertex (`MeasurePrimitive::admits`),
and `[assertions.quiet]` quiets a contact "between the finding's two
cells". So no assertion quiets such a row, and it is reported loud. Its
finding never refuses (`CheckEvidence::never_refuses`, CHECKS DS6), as
pinned by `crates/editor-core/tests/atrest_operation_rows_quiet.rs`
`a_splits_on_row_is_loud_and_never_refuses`.

Quieting one would need `Gap`, or `[assertions.quiet]`'s cell reading,
to take a plane or a pose as one of its references. That is a change to
a ratified clause and to the measure's door, and it is Ev's call if
anyone ever wants it. Nothing asks for it today. A split's ON row
has the door's recourse instead (a construction that puts the vertex
on the plane).
