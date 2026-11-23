---
id: python-raises-a-pose-in-another-space-as-a-space-mismatch
kind: issue
title: pncad-py raises PoseRefusal::OtherSpace (a copy placed into another space has no world pose) as an EvaluationError of kind space_mismatch, a refusal it is not
status: open
opened: 2026-10-22
priority: P3
refs: [5613]
---



Found by the D′1 review (`a-placement-names-its-target-space`, PR 5592).

`SolvedPoses.placement` asks the kernel's `SolvedPoses::placement`,
which refuses a copy placed into a space other than the world with
`PoseRefusal::OtherSpace { placement, space }`: the copy has a pose in
that space (`SolvedPoses::relative`) and none in the world's. The
binding (`crates/pncad-py/src/py/mate.rs`, `pose_err`) raises it as
the `EvaluationError` a node reading two spaces carries,
`NodeErrorKind::SpaceMismatch { a: space, b: WORLD }`, kind
`space_mismatch` (`crates/pncad-py/tests/test_world_pins.py`,
`test_a_copy_placed_into_another_space_has_no_world_pose`). Nothing
mismatched: the placement evaluated, and the question asked of it has
no answer in the world's coordinates. A caller telling "this node
failed" from "you asked for the wrong space" cannot.

The binding's exception words are minted from typed classes
(`crates/pncad-py/src/errors.rs`, `ErrorClass`) and inventoried by the
binding census, so the fix is a word of its own on a class that owns
it, not a string at the raise site.

Done when: `SolvedPoses.placement` of a copy in another space raises a
refusal whose word says the copy is placed into another space and
names that space, and no `space_mismatch` is raised for it.

## D′2 re-read (2026-10-23)

This stays open. D′2 (`a-mate-reads-only-geometry-and-a-value-charts-on-the-target-space`) does not touch the pose door: `PoseRefusal::OtherSpace` and `pose_err` (`crates/pncad-py/src/py/mate.rs:856`) are as this row describes. The fix is a word on an `ErrorClass` of its own, and it shares nothing with D′2's chart.

## Slate triage (2026-10-28, INTENT)

Moved from `work/intent/` by `git mv`, id and body unchanged: residue of INTENT's D10 build that belongs to BIND's charter (the Python bindings). INTENT keeps D10's core (G and its riders, the stage-3 umbrellas, the carry, the refactor rows, the roots rename); Ev asked for INTENT's slate to be cut to its charter and its residue handed on (in chat, 2026-10-28).
