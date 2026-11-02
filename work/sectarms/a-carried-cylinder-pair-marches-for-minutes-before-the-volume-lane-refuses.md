---
id: a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses
kind: issue
title: A skew or unequal-radius cylinder pair the join now carries marches and chords for tens of seconds per operation (debug) before the result gate refuses at the volume lane
status: open
opened: 2026-11-02
priority: P1
cost: M
refs: [a-projected-boundary-on-an-analytic-chart-has-no-volume-lane]
---


Filed by the cylinder × cylinder stage 3 lane (branch
`sectarms/cylinder-join-reads-value`, 2026-11-02).

## What

Since the join reads cylinder pairs on the section value, a skew or
unequal-radius pose no longer refuses `GermFrameUnsupported` at the
frame. The join carries its marched components and cuts its chords, and
the result gate then refuses: the walls are chorded by spline arcs, whose
projected pcurves have no volume lane
(`work/spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane.md`).

Measured locally in the debug profile:

| row | time | before |
|---|---|---|
| `sweep::verbs_germarms2::a_rod_through_a_drum_wall_reaches_the_volume_lane` (four ops) | 187 s | under a second |
| `verbs_germarms2::the_fenced_poses_keep_their_own_outcomes` | 264 s | |
| `tang_circle_cylinder::a_tilted_rod_through_a_rim_reaches_the_volume_lane` | 35 s | |
| `verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose_refuses_typed_or_answers_correctly` | 216 s | |

The rows moved to the slow set.

## Why it matters

The work buys a refusal today: everything the join does is thrown away
at the result gate. Once the volume lane lands, the same cost buys a
body, and it is the march and carry cost the cylinder × sphere row
measured (`work/sectrung/a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build.md`).

## What a fix could be

Measure the split between march, carry and chord, in release, and either
cheapen the dominant step or have the boolean know before the join that
the result gate cannot measure a spline-chorded wall.

## Re-priced P1 (PR 6373 review, 2026-11-02)

Both reviewers measured about 6–7 s per operation in release for every
skew or unequal-radius cylinder boolean, a pose that used to refuse
instantly. That is a user-visible cost on ordinary geometry, so P1.

**The fix:** decide before marching that no volume lane can take the
result.

**Constraint:** the gate must read an existing capability predicate of
the volume lane plus the decided pose; it must not be a new geometric
reading that predicts it. It retires when the volume lane lands
(`spiricarm/a-projected-boundary-on-an-analytic-chart-has-no-volume-lane`).

**Not built in stage 3:** no such capability predicate exists to read.
The volume lane's refusal is raised at quadrature time, per face
(`QuadratureUnsupported`), from the pcurve kind a chord mints. Nothing
upstream of the join can ask "will this face's boundary have a harmonic
form", short of a new reading of what the chord will be.
