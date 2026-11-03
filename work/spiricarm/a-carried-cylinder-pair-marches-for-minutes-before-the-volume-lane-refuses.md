---
id: a-carried-cylinder-pair-marches-for-minutes-before-the-volume-lane-refuses
kind: issue
title: A skew or unequal-radius cylinder boolean marches and chords for tens of seconds per operation (debug), about 6–7 s in release
status: open
opened: 2026-11-02
priority: P1
cost: M
refs: [a-projected-boundary-on-an-analytic-chart-has-no-volume-lane]
---


Filed by the cylinder × cylinder stage 3 lane (branch
`sectarms/cylinder-join-reads-value`, 2026-11-02).

## What

A skew or unequal-radius cylinder pose no longer refuses
`GermFrameUnsupported` at the frame: the join reads the pair on the
section value, carries its marched components and cuts its chords, and
since SPIRICARM's rotational lane (stage 1, branch
`spiricarm/rotational-lane-cylinder`) the result gate measures the
chorded walls, so the operations build. The rows that pinned the
refusal are held SOUND against a sliced oracle now
(`sweep/tests/common/crossed_cylinders.rs`). What is left is the cost:
the march and the carry, which the join pays before it builds, plus the
lane's own reading of the chords.

Measured locally in the debug profile, before the lane, when each run
refused at the volume lane:

| row | time | before |
|---|---|---|
| `sweep::verbs_germarms2::a_rod_through_a_drum_wall_builds_at_its_sliced_volume` (four ops) | 187 s | under a second |
| `verbs_germarms2::the_fenced_poses_keep_their_own_outcomes` | 264 s | |
| `tang_circle_cylinder::a_tilted_rod_through_a_rim_builds_at_its_sliced_volume` | 35 s | |
| `verbs_cylcyl_r1_review_probes::every_reachable_crossing_pose_answers_correctly` | 216 s | |

The rows are in the slow set. With the lane, the tilted rod's ∪ alone
took 17 s in debug, of which the measurement is 3.6 s (its chords hold
39–213 Bézier spans each).

## Why it matters

Both reviewers of PR 6373 measured about 6–7 s per operation in release
for every skew or unequal-radius cylinder boolean, a pose that used to
refuse instantly: a user-visible cost on ordinary geometry, so P1. The
work now buys a body, so the cost is no longer thrown away, but it is the
same march and carry cost the cylinder × sphere row measured
(`work/sectrung/a-ball-and-cylinder-boolean-costs-seconds-per-op-in-an-optimized-build.md`).

## What a fix could be

Measure the split between march, carry, chord and measurement, in
release, and cheapen the dominant step. The earlier fix, deciding before
marching that no volume lane can take the result, retires: the lane
takes it.
