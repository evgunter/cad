---
id: cone-singular-gate-decides-distance-where-the-lever-is-distance-times-sin-alpha
kind: issue
title: singular_at decides a cone vertex off the apex by its distance, while the joint's lever is the distance times sin α
status: open
opened: 2026-10-12
priority: P2
cost: M
refs: [periodic-branch-floor-remains-at-three-sites]
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
