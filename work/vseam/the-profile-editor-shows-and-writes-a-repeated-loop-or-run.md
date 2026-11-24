---
id: the-profile-editor-shows-and-writes-a-repeated-loop-or-run
kind: unit
title: The profile editor shows a repeated loop or run as its written steps and writes each k from a pattern gesture
status: parked
opened: 2026-11-22
priority: P1
cost: M
parent: a-loop-cannot-repeat-inside-a-profile
blocked_on: [a-chain-repeats-a-run-of-its-steps-with-each]
---


The viewer half of `work/repeat/a-loop-cannot-repeat-inside-a-profile.md`
(Ev, PR 7263). It is filed on VSEAM, which holds the viewer's session
vocabulary and SELFREF's editor pick unit. It is parked on REPEAT's
run unit, so that the loop and run forms both exist when it is built.

## Done when

- The profile editor (`crates/viewer/src/pane/profile.rs`
  `edit_profile_ui`, `step_rows_ui`; `crates/viewer/src/drafts.rs`
  `ProfileEdit`, `DoorLoops`) shows a braced loop or run as its written
  steps, once, with the index and count it repeats over. It does not
  show N copies.
- A pick on any copy in the sketch opens that copy's written step
  ("editing it edits every copy"). The pick maps a drawn segment back
  through the written-address map
  (`work/repeat/profile-readers-read-kernel-steps-through-the-written-address-map.md`),
  never by kernel position.
- A gesture wraps selected loops, or a selected run of steps, in
  `each k { … }` over a new or existing index. Today's body
  `PatternTool` (`crates/viewer/src/combine.rs`; `SessionOp::AddPattern`)
  writes `k = index(N)` and a placement, and is the model. Which gesture
  is the implementer's to propose, in the viewer's register.
- `crates/viewer/src/session.rs` `carry_unmoved` and the `SlotId::Profile`
  rebuilds (34 viewer sites, mostly `app.rs`, `pane/profile.rs`,
  `sketch.rs` and `drafts.rs`) keep working: written addresses do not
  move, which is what the REPEAT units guarantee.

## Check first

`drafts.rs` holds the editor's working loops as flat kernel steps
(`DoorLoops.loops: Vec<Vec<Step<f64>>>`, `ProfileEdit.loops: Vec<Vec<HeldStep>>`).
A block has no kernel-step form, so the draft model needs a written
form for it. If that means re-basing the draft on `LoopProgram`, size
it before building and report if it exceeds M.
