---
id: a-datum-mated-on-a-datum-at-zero-levers-its-pole-over-nothing
kind: issue
title: A datum mated on a datum at zero values refuses Backwards: its pole is levered over an arm of nothing
status: open
opened: 2026-11-23
priority: P2
cost: M
design: true
---

DESIGN `[placement.datums]` says a mate may read a datum at all-zero
values. A datum mated on another datum, both at their space's origin,
by a plane or an axis mate, refuses `MateFault::Backwards` at both
senses. At the unflipped sense its recourse names the orientation
already written (`orient` = `write` = identity), so it contradicts
itself. Point and frame mates land. Red row:
`crates/viewer/tests/datum_review_a.rs`
`a_datum_mates_on_a_datum_at_zero_by_every_kind` (ignored, pointing
here).

The cause is in the kernel, not the viewer's near side:

- A datum folds with a reach of 0, because it copies nothing.
  `arm_of` (`crates/editor-core/src/mate/solve.rs:1259`) adds to that
  the mates' poses' distances from their origins, which are 0 here. So
  the arm is 0.
- `RigidMotion::least_turn_minted`
  (`crates/geom-core/src/linalg/rigid_motion.rs:1882`) decides the pole
  on `Margin::levered(chord, arm)`. At arm 0 that margin is 0 whatever
  the chord, so it reads `Sign::Zero`, which is `TurnError::Opposed`.
  Two equal directions are therefore "opposed".
- `Chart::read_as` maps that to `ChartFault::Backwards`
  (`crates/editor-core/src/mate/value.rs:400`), whose `write` is the
  orientation nearest the one written: the same one.

A kernel probe gives the same verdict with no viewer involved: datum B
on datum A through `Xy` or `Z`, at identity, both senses.

What a datum's turn is levered over is the open question already filed
for the face chart's fold
(`face-chart-folds-into-the-datum-once-a-datum-levers-its-pole`): a
datum carries nothing, so it has no reach of its own. Options:

1. A datum's pole is decided at each reader over what that reader
   carries (7151's carriage rule, generalised). Reading a datum from
   another datum still carries nothing, so this case needs its own
   answer.
2. A floor on the arm, for example the band's own length scale. This
   keeps the verdict well-defined at zero, but it is a length nothing
   in the document states.
3. At arm 0, decide the pole on the chord alone, unlevered: nothing is
   carried, so no rounding of the turn moves a point. This mixes a
   dimensionless margin into a length band.

This touches `[placement.datums]` and `[placement.values]`, so it is a
designer pass, not a lane's call.
