---
id: through-and-meet-decide-their-degeneracy-at-unit-norm-unlevered-over-their-readers
kind: issue
title: PoseDef::Through and Meet decide their degeneracy at a unit norm, levered over none of what reads the frame or line they make
status: open
opened: 2026-11-18
priority: P3
cost: M
refs: [a-face-charts-turn-is-levered-over-none-of-what-it-carries]
---


Both designers of the `lever` fork (2026-11-18) noted this independently, off their question, from the code. Not yet run.

## What

A pose construction's degeneracy is decided where the construction is
evaluated, at the unit norm, and is not levered over the reach of what
reads its result:

- `PoseDef::Through` (`crates/editor-core/src/eval/wire/pose.rs:363-377`)
  builds its frame with `RigidMotion::from_aim_and_reference(at, dir,
  position - foot, POSE_UNIT_NORM, b)`. Whether the point stands off the
  axis is decided at `POSE_UNIT_NORM`. A point that is close to the axis
  but decided off it spins the frame by an angle that rounding moves,
  and a body read in that frame far from the axis moves by that angle
  times its reach.
- `PoseDef::Meet` (`:379-397`) decides `na × nb` nonzero with
  `UnitVec3::new(d, POSE_UNIT_NORM, b)` and then divides by `|d|²` for
  the line's foot. For two planes that are nearly parallel, the line's
  direction and foot move by `1/|d|` times their rounding, and nothing
  asks how far from the foot the line is read.

This is the same class as
`a-face-charts-turn-is-levered-over-none-of-what-it-carries`: a band
decided once at the construction, where the `lever` fork (`[ev]` PR
7122) places the band on the carriage over what it moves. If Ev
ratifies 7122, these two constructions are its next readers. Whether
they get the same treatment, or stay decided at the unit norm because
nothing reads them far away, is that build's question.

## Done when

Either a test shows a Through or a Meet that is decided non-degenerate
while a body it carries builds where a placement of that reach refuses,
and the construction is levered over its readers; or a test shows that
no reader of either one reaches far enough for that to happen, and the
row closes with that test cited.
