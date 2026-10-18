---
id: the-edit-door-takes-a-reach-it-never-reads
kind: issue
title: The edit door takes a MateReach it never reads: apply's reach parameter and the trait retire
status: open
opened: 2026-10-17
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
