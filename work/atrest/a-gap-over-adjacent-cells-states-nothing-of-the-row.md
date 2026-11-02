---
id: a-gap-over-adjacent-cells-states-nothing-of-the-row
kind: issue
title: What assertion could say a tangency, a coaxiality or a blend's common foot? No Gap = 0 does, so those rows are quieted by none
status: closed
opened: 2026-10-27
priority: P0
cost: H
closed: 2026-10-30
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

## The answer (Ev, `[ev]` #6086, merged 2026-10-30; DESIGN `[intent.checked]`, `[placement]`, `[assertions]`, `[assertions.quiet]`, State of the build)

**The rule.** An assertion quiets an unproven row when it says what the row decided, wherever it can say it. That holds even where a construction could also say it (Ev: "fine to allow assertions everywhere for simplicity"), and the finding then still names the construction edit. Only a relation against zero quiets; a `≤ t` bound is a tolerance. Rows no assertion can say keep the construction as their only recourse, are loud, and never refuse: a blend turn's two feet, and two radii written as two variables.

**The build:**
1. **`Gap`'s curved arms read each face's material side** (`crates/editor-core/src/eval/measure.rs` `gap`). Today the cylinder and sphere arms always compute a pin in a bore (`r_out − r_in − d`), so two convex rods touching from outside read −2r. The arms needed:
   - cylinder × cylinder and sphere × sphere, external as well as internal;
   - plane × cylinder and plane × sphere, which have no arm today.
   The role order stays authored (C5), never inferred from values. Opposed `TangentWitness` rows become contacts in `checks::at_rest::contact_cells`. An aligned tangency is a continuation, so a `Gap = 0` over its faces read on their sides quiets it as it does a flush pair.
2. **A measure's reference may be a pose:** a point, an axis or a plane read off a cell, or a split's tool (`MeasureVerb::admits` today allows only a face, an edge or a vertex). A pose coincidence is quiet under a holding `= 0` between poses read off the row's cells or tool:
   - `CoaxialSphere` / `SphereOnTorusAxis`: `Distance(centre, axis)`;
   - the coaxial rows and `BatterySupportAxis`: `Distance(axis, axis)`, one scalar (the kernel's own axis verdict over the cells' lever), not `Angle` plus offset;
   - `CrossingCylinders` `AxesMeet`: `Distance(axis, axis)`, skew arm;
   - `SplitOn`: `Gap(tool plane, vertex)`. This folds in and closes `a-row-over-a-pose-is-quieted-by-no-assertion`.
   The quieting rule's cell match sees a pose as the cell it was read off.
3. **A flush pair is quiet under a `Gap = 0` over its two faces read on their sides** (`[assertions]`' flush sentence as merged).
4. **The recourse is chosen by the row's kind.** `coincide::Recourse::words` never says "assert it" where no assertion can say the row (the turn's feet, written values). It names the construction where one exists. The concrete variable edit is `coincide/an-unproven-finding-names-the-variable-edit-that-would-construct-it`; don't build it here.
5. **`BatteryJoint` carries the operand's verdict** (`[coincidence.booleans]`: a meeting inside one operand is that operand's decision) rather than re-deciding the profile's junction. If that is large, file it as its own row, say so, and leave the joint row loud.
6. **Docs.** DESIGN's State of the build drops the not-built item once 1–4 land, and CHECKS.md and the measure docs follow.

**Rows.**
- Each row kind's assertion quiets it; the opposite-sense and the `≤ t` variants do not.
- Two touching rods read `Gap = 0`, not −2r, and a rod on a plate has an arm.
- The turn's feet and the two radii stay loud with a construction recourse.
- A flush stack-up is quiet under its `Gap = 0`.
- Today's pinned rows flip: `a_tangency_is_quieted_by_no_gap_and_never_refuses`, the `TangentWitness` table row, and `a_splits_on_row_is_loud_and_never_refuses`. Re-pin them and say what moved. `a_blends_turn_is_quieted_by_no_gap_and_never_refuses` stays.

**Review tier:** dual H. Dispatched 2026-10-30 to cloud lane `session_01VyvYnJXePRoMMKnGq2SyqP`, branch `atrest/an-assertion-quiets-what-it-says`. It changes the assertion vocabulary, a measure's semantics and the quieting rule, which every check reads.
