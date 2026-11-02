---
id: a-duplicate-freezes-a-mated-copy-into-literal-values
kind: issue
title: Duplicate writes a copy's solved motion back as literal values and drops its mates; it retires in favour of a second placement of the body
status: parked
opened: 2026-11-02
priority: P1
cost: M
design: true
blocked_on: [6353]
---


Found by the PLACING sweep Ev asked for on PR 6281 (2026-11-02): "can you see if anything similar also got missed?" by the INTENT refactor. Read at `3bf15bef9c`; traced in code, not run unless said.

**Ruled.** `[placement.placement]`: "A placed copy's frame is not a variable: it is the construction its bundle states." The Stage-3 helpers ruling: no constructor "reads geometry or writes a number its caller did not write". Also `[intent.stored]`.

**Survives** (sure).
- `DocSession::add_duplicate` (`crates/viewer/src/session.rs`) goes through `duplicate_seats` to `combine::copy_motion` (`crates/viewer/src/combine.rs`). That function reads the evaluation's `motion` for the copy, then `moves_of` writes it as `atan2` and nominal `Formula::literal`s.
- So duplicating a mated copy gives an unmated copy frozen at today's numbers, which no longer follows the source copy's variables or mates.
- Pinned by `crates/viewer/tests/duplicate_beside.rs::a_duplicate_of_a_mated_copy_is_drawn_beside_it_unmated`, whose module doc states the snapshot rule.
- Stale prose: the `DuplicateTool` doc in `combine.rs` ("the step lands in the translation slots of the transform the gesture authors"). Also `crates/viewer/README.md`, *A new body lands unplaced*, which states the snapshot rule and then says "Nothing a place gesture writes reads the copy's geometry".

**Design question.** What a duplicate of a placed copy is. For example, a further copy of the copy slid by the step, or a second placement of the same body whose bundle is the copy's own, with the step added. `a-feature-gesture-re-points-every-copy-of-its-target` already says Duplicate can become a second placement once that row lands. This goes to designers before a lane builds it.

## Ev's direction (2026-11-01, in a session)

The design question above is answered: **Duplicate retires**. It is
superseded by making more than one placement of a body, in the
placement editor (PR #6300). It does not retire automatically with
Transform, so this row tracks it.

**A second placement is made the way the first one is**: pick the
body, and the placement editor (PR #6300) opens its Create door on it.
Picking a copy opens that copy's placement for editing instead
(`MateTool::pick`, `crates/viewer/src/matetool.rs` on
`placing/bundle-editor`). A body that is already placed is
intermediate, so it is not shown by default, but the View pane can
show it and then it is picked like any other body (`crates/viewer/README.md`,
*What the picture shows*). Nothing new is needed to make a second
placement. The retirement's done-when includes a row that does exactly
this: place a body, show it again, pick it, and get a second placement.

**What the retirement still needs to decide (goes to designers):**
- **The coupling** with the parked
  `a-feature-gesture-re-points-every-copy-of-its-target`. Two
  placements of one body share it, so a feature on either takes over
  both. That is why Duplicate makes a body of its own today.

**What goes:** `SessionOp::Duplicate`, `DocSession::add_duplicate`,
`duplicate_seats`, `combine::duplicate_step`/`copy_motion`/
`duplicate_moves`, the `DuplicateTool` and its panel, the duplicate
tests (`crates/viewer/tests/duplicate_beside.rs`), and the duplicate
sentences in the README's "A new body lands unplaced".

## Conflict with PR 6356, to be undone by this unit (2026-11-02, PLACING orchestrator)

PR 6356 (Transform's retirement) landed, in its review fix pass, two behaviours that contradict Ev's direction above. They were adjudicated without this direction in view:
- **Routing:** `matetool::door_on` sends a body exactly one copy places to that copy's Edit door.
- **Refusal:** `SessionOp::Place` on an already-placed body refuses `Refusal::BodyPlaced`, naming its copy.

Under Ev's direction, a placed body shown in the View pane and picked as itself opens the Create door, which makes a second placement. This unit owes that:
- `door_on` routes a body picked as itself (drawn because it is shown) to Create;
- a pick that lands on a copy still routes to that copy's Edit door;
- `BodyPlaced` narrows to whatever case remains genuinely wrong, or retires.

Its done-when row (place, show, pick, get a second placement) fails on main today for exactly this reason.

**Why parked on `[ev]` 6353:** its one open design point, a feature on either of two placements taking over both, is 6353's question. 6353's agreed half (a feature on a copy reads the copy) dissolves the coupling.
