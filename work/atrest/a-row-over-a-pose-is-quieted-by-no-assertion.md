---
id: a-row-over-a-pose-is-quieted-by-no-assertion
kind: issue
title: A split's ON row is quieted by no assertion, though [intent.checked] says one at its site quiets every unproven coincidence: Gap reads no pose
status: open
opened: 2026-10-27
priority: P3
cost: M
---

A split's ON row (`DecisionSite::SplitOn`, `crates/topo/src/splitting/mod.rs`
`reduce`) is `[Vertex(A), Tool]`: an operand vertex decided on the
cutting plane. Its second cell is the split's tool read
(`coincide::NamedCell::Tool`), which is a pose and not a cell of any
body. A profile junction's row (`NamedCell::Piece`) has the same shape.

Two ratified clauses conflict over it:

- `[intent.checked]` says of every unproven coincidence that "an
  assertion at its site quiets it". `[coincidence.door]` names a
  split's ON verdict among the coincidences the lint checks.
- `[assertions.quiet]` quiets a contact by a `Gap` "taken between the
  finding's two cells". `Gap` admits only a face, an edge or a vertex
  (`MeasurePrimitive::admits`), so no `Gap` is written over a pose.

The code is conservative: the row is reported loud, and its finding
never refuses at any severity (`checks::at_rest::contact_quietable`,
`CheckEvidence::never_refuses`, CHECKS DS6). This is pinned by
`crates/editor-core/tests/atrest_operation_rows_quiet.rs`
`a_splits_on_row_is_loud_and_never_refuses`. The door's own recourse,
a construction that puts the vertex on the plane, still applies.

**The question for Ev:** does `[intent.checked]` hold for a row over a
pose? If it does, `[assertions.quiet]` and `Gap`'s door would need to
read a plane or a pose as a reference, or a measure would need to state
a point on a plane. If it does not, `[intent.checked]` would need to
name the exception.
