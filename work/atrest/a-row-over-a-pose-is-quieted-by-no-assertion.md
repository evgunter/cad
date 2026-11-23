---
id: a-row-over-a-pose-is-quieted-by-no-assertion
kind: issue
title: A split's ON row is quieted by no assertion, though [intent.checked] says one at its site quiets every unproven coincidence: Gap reads no pose
status: closed
opened: 2026-10-27
priority: P3
cost: M
closed: 2026-10-30
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

**Closed (2026-10-30, folded into `a-gap-over-adjacent-cells-states-nothing-of-the-row`).**
Ev's answer on `[ev]` #6086 holds `[intent.checked]` for a row over a
pose: a measure's reference may be a pose. `Gap` reads a plane, or a
frame as its xy plane, against a vertex, and the row half of the
quieting rule (`checks::at_rest::quiet_row`, `saying`) quiets a split's
ON row under a holding `Gap(tool, vertex) = 0` over its tool read and
its vertex. `atrest_operation_rows_quiet.rs`
`a_splits_on_row_is_quiet_under_its_tool_gap` re-pins the row this
issue cited. A profile junction's row is over pieces, which no
assertion reads; it keeps the construction as its recourse.
