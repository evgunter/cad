---
id: a-gap-over-adjacent-cells-states-nothing-of-the-row
kind: issue
title: What assertion could say a tangency, a coaxiality or a blend's common foot? No Gap = 0 does, so those rows are quieted by none
status: open
opened: 2026-10-27
priority: P0
cost: M
design: true
---

D10 `[assertions.quiet]` quiets a finding only by an assertion that
says the same, "`= 0` for a contact". A `Gap = 0` says that two cells
meet. The contact half (`checks::at_rest::contact_cells`, whose table
classifies each row kind the kernel decides) therefore quiets only rows
whose decision is that their two cells meet:

- a boolean's opposed glue (`PlaneLadder`, `CarrierLadder`,
  `SameOpposite`);
- a vertex fusion (`OnCarrier`, a two-cell `Meet`);
- an at-rest contact.

The other rows decide more than meeting, so no `Gap = 0` says them,
and they are loud and never refuse (CHECKS DS6). Evidence by row kind:

- **`BatteryTurn` (`CommonFoot`)**: the two rim edges meeting at a
  corner (`crates/sweep/src/blend/battery.rs`). The row decides that the
  two bands' feet on the third edge are one point. A `Gap = 0` over the
  two edges holds at every value, because they share the corner, and so
  says nothing about the feet.
  `crates/editor-core/tests/atrest_operation_rows_quiet.rs`
  `a_blends_turn_is_quieted_by_no_gap_and_never_refuses` asserts the
  adjacency, the holding `Gap = 0`, and that the row stays loud.
- **`BatteryJoint` (`Tangent`)**: two links meeting at a chain junction,
  which share its vertex.
- **`BatterySupportAxis` (`Coaxial`)**: the cap and the wall, which share
  the band's rim edge.
- **`TangentWitness` (`SurfaceTangent`)**: two parallel rods externally
  tangent read `r_b − r_p − d = −2r`-ish in either role order (the PR
  5882 review's probe), so no `Gap = 0` over them even holds. A rod on a
  plate is a pair `Gap` has no form for.
  `a_tangency_is_quieted_by_no_gap_and_never_refuses` pins the second.
- **`CoaxialSphere` (`OnCarrier`)** decides a sphere's centre on a
  cylinder's axis. **`CrossingCylinders` (`SameRadius`, `AxesMeet`)**
  decides a radius and an axis meeting.

`[intent.checked]` says of every unproven coincidence that "an assertion
at its site quiets it", so these rows sit outside it until some
assertion says what they decide.

**The question for Ev:** what assertion could say a tangency, a
coaxiality, a common radius or a blend's common foot? Possible answers
include:

- a measure per relation, such as an angle between normals at a point,
  or an axis distance;
- a mate-like statement that makes the row structural;
- an exception named in `[intent.checked]`.

This goes to a designer pair first (`docs/prompts/designer.md`), then
to an `[ev]` PR on D10.

**Re-priced P0 (2026-10-30, INTENT orchestrator).** DESIGN's "State of the build" lists this as D10's one unbuilt item, and D10's umbrella (`work/intent/d10-one-way-to-say-intent-is-unbuilt.md`) now waits on it alone. It is a design question (what a quieting assertion says for a row that decides more than meeting), so the designer pair weighs it first.
