---
id: a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses
kind: issue
title: A skew or unequal-radius cylinder pair the join now carries marches and chords for tens of seconds per operation (debug) before the result gate refuses at the volume lane
status: open
opened: 2026-11-02
priority: P2
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
