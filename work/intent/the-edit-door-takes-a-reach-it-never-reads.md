---
id: the-edit-door-takes-a-reach-it-never-reads
kind: issue
title: The edit door takes a MateReach it never reads: apply's reach parameter and the trait retire
status: closed
opened: 2026-10-17
closed: 2026-10-18
priority: P2
cost: E
refs: [a-mate-relates-two-poses]
---


INTENT stage 3 C moved the mate fold into the placement's own op
(`crates/editor-core/src/eval/wire.rs`, `fold_with`), which takes its
lever from the copy's own evaluated geometry (`payload_reach`). Since
then nothing reads a `MateReach`:

- `MateReach::reach` (`crates/editor-core/src/mate/reach.rs`) has no
  caller. C dropped the trait's other method, `face_pose`, and
  `FacePoseRefusal` with it. The trait is left with one method nobody
  calls.
- `apply`'s `reach` parameter (`crates/editor-core/src/edit.rs`,
  `apply`/`apply_replayed` and `apply_insert`, each `let _ = reach`)
  is threaded through and unread. The door is structural
  (`admit_mate`, `admit_value`, and stage 3 F's bundle rule
  `admit_bundle_make_up`); F dropped the parameter from the internal
  functions below `apply` (`admit_written_constraint` and the rest).
- `PartReach`, `RefusingReach` and `mate_reach` exist only to fill
  that parameter.

The work: retire the parameter, the trait and its implementations,
then restate every caller. That is mechanical, about 180 files across
editor-core, pncad, pncad-py, viewer and the demos. C kept the
parameter unread rather than widen its diff.

## Closed (2026-10-18)

`apply`, `apply_replayed`, `apply_insert`, `Doc::apply`,
`Recording::start`, `measure` and `resolve::apply_with_names` no longer
take a reach. The `MateReach` trait, `RefusingReach`, `PartReach` and
`mate_reach` are gone from editor-core and from the `pncad::document`
façade. `reach_of`, the one implementation behind them, went with them.
The fold still reads each copy's reach off its own geometry
(`payload_reach` in `eval/wire.rs`). The free functions `part_reach`,
`body_reach` and `ReachRefusal` stay.

The callers are restated across editor-core, pncad, pncad-py, the
viewer, the tour demo and the guide pages. Test fixtures that existed
only to hand a reach in (`constrain_with`, `constrain_through`,
`step_with`) are gone: their callers use `constrain`, `constrain_at`
and `step`. Rows that measured a part's reach through the door
(msolve6's `reaches` and its interval row) now read it off the part's
own body with `part_reach`. Three rows asserted something about the
reach itself, and the door's signature now guarantees each:

- msolve6 and msolve10's counting reaches, which proved that no edit
  asks the store;
- the store-versus-refusing-reach comparisons;
- part_depth_bound's second refusal, which came through the reach.

Those rows are retired or reduced to their remaining claims.

The Python edit doors still take a `resolver=` they no longer read.
Retiring that keyword removes public Python API, so it is filed as
`the-python-edit-doors-take-a-resolver-they-never-read`.

