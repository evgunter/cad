---
id: cone-singular-gate-decides-distance-where-the-lever-is-distance-times-sin-alpha
kind: issue
title: singular_at decides a cone vertex off the apex by its distance, while the joint's lever is the distance times sin α
status: closed
branch: pcert/cone-singular-gate-lever
opened: 2026-10-12
closed: 2026-10-13
priority: P2
cost: M
refs: [periodic-branch-floor-remains-at-three-sites, near-pole-read-tries-only-the-images-own-orbit-point, chart-winding-escalates-on-the-first-candidate-at-a-band-sized-lever]
---


Found by PR 4875's review (MINOR-2, class note). Pre-existing.

`topo::pcurves::singular_at` decides whether a joint's vertex is on a
cone's singular set as `|p − apex|` (`pcurve_loop_pole_joint`). The
joint's lever, though, is the vertex's distance from the axis
(`joint_arm`), which on a cone is `|p − apex|·sin α`. On a narrow cone
the two part. At `α = 0.01`, a vertex `1e-8` from the apex decides
`Off`, so `decide_joint` takes `AzimuthRule::Marks` and never the
`MarksNearPole` fallback, while its lever is `1e-10`, under the band.
Every mark then reads Zero (a continuous joint answers
`PinMiss::Discontinuity`) or in band (escalates).

`chord_join::nearest_branch` serves the same configuration through
`pcurves::lever_periods`, which reads the near-pole gap wherever the
marks miss (row
`chord_join::nearest_branch_rows::a_narrow_cone_joint_clear_of_the_apex_takes_its_own_image`).
`decide_joint` does not yet.

What would close it: decide the singular gate on the lever (the
vertex's axial distance), or give `AzimuthRule::Marks` the near-pole
read as `lever_periods` does. Either changes what the loop walk mints
near a narrow apex, so it wants its own rows.

## Closed

By the second route: `AzimuthRule::Marks` takes the near-pole read
where its marks miss, through `pcurves::lever_periods` (now given the
twin), the same read `MarksNearPole` and `chord_join::nearest_branch`
take. `singular_at` stays 3-D incidence, so no reset is written off
the singular set and the fence's refusal kind is unchanged. At a lever
with room nothing moves: the read is asked only where a mark missed.
`Marks` and `MarksNearPole` now differ only in the error on a miss
(the marks' own, or the incidence's escalation), and that error is
read (a decided discontinuity refuses as decided), so they stay two.

Rows:
- `pcurves::stretch_meter::a_narrow_cone_loop_joint_clear_of_the_apex_takes_its_own_image`
  is the item's configuration;
- `cone_apex_sector_joints::a_split_near_a_narrow_apex_passes_its_joint`
  restates the old escalation row: the joint now decides, and the
  child's chart winding refuses;
- `pcurves::turn_miss::a_turn_whose_joint_misses_refuses` is restated:
  a period's gap at a tenth of ε decides, and it escalates in the band.

Each goes red under a mutant restoring the old `Marks` arm.

Filed: `near-pole-read-tries-only-the-images-own-orbit-point` and
`chart-winding-escalates-on-the-first-candidate-at-a-band-sized-lever`
(both P3, pcert).
