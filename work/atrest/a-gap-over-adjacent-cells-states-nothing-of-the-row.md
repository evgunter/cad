---
id: a-gap-over-adjacent-cells-states-nothing-of-the-row
kind: issue
title: A Gap = 0 over two cells that share a boundary holds by topology, so it quiets a row while stating nothing of what the row decided
status: open
opened: 2026-10-27
priority: P2
cost: M
---


The contact half of the quieting rule (`checks::at_rest::quiet_contact`)
quiets a row under a holding `Gap = 0` over its two cells, whatever
the row's relation. Since `an-assertion-quiets-an-operations-unproven-row`
it reads an operation's rows too. Some of those rows name two cells of
**one operand that share a boundary**. A `Gap` between such cells
reads zero at every value of the document, because the cells touch by
topology. So the assertion holds without stating anything about the
coincidence the row decided, and it quiets the row anyway.

Evidence, by row kind:

- **`BatteryTurn` (`CommonFoot`)**: the two rim edges meeting at a
  corner (`crates/sweep/src/blend/battery.rs`, the `requested` edges at
  the turn). The row decides that the two bands' feet on the third edge
  are one point. `Gap = 0` over the two edges reads only that they share
  the corner. Pinned by
  `crates/editor-core/tests/atrest_operation_rows_quiet.rs`
  `a_blends_turn_is_quiet_under_its_gap_and_nothing_else`, which
  asserts the adjacency and the quieting for a turned box's eight
  corners.
- **`BatteryJoint` (`Tangent`)**: the two links meeting at a chain
  junction share its vertex. Over arcs, `Gap` refuses a curved edge, so
  the row is unquietable today
  (`gap-reads-no-curved-cell-below-a-face`). Over two line links, `Gap`
  admits it, and `= 0` holds at the shared vertex whether or not the
  links are tangent.
- **`BatterySupportAxis` (`Coaxial`)**: the cap and the wall share the
  band's rim edge. A plane against a cylinder has no `Gap` row, so this
  is unquietable today. Two adjacent coaxial walls of one radius read
  `r − r − 0 = 0` whatever their axes do along the shared edge.
- **Not affected**: a boolean's rows (`PlaneLadder`, `VertexFusion`,
  and the others) and every at-rest row name cells of two different
  operands or arrivals, where a `Gap = 0` states the touch. A split's
  row is unquietable (`a-row-over-a-pose-is-quieted-by-no-assertion`).

Beneath the adjacency there is a wider point. `Gap = 0` states contact,
and a `Tangent`, `Coaxial` or `CommonFoot` row decides more than
contact. `[assertions.quiet]` quiets "a contact", and stage 5 C wrote
the rule over a row's two cells whatever its relation, excluding only
a flush pair.

**The design question, for D10 `[assertions.quiet]`**: does a holding
`Gap = 0` quiet a row whose cells share a boundary, or whose relation
is more than contact? The options include excluding adjacent cells (the
row then unquietable, never refusing), or asking for a measure that
states the row's own relation. This goes to a designer pair and an
`[ev]` PR. Until then the rule quiets these rows as ratified.
