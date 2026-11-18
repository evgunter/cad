---
id: the-solid-angle-winding-is-copied-across-three-test-files
kind: issue
title: the solid-angle winding of a planar body is copied in three test files (topo's fin crown, sweep's m5_pr12_fix_pass and strut_cover_on_cylinder_pairs) with no shared home
status: open
opened: 2026-11-17
priority: P4
cost: E
---



## Finding

Found by the review of EMBED's winding-rule unit (`embed/winding-rule`,
PR 7071), 2026-11-17, while sweeping `crates/topo/tests` for hand folds of
the sense bit.

`crates/topo/tests/the_fin_crown_winds_as_a_solid.rs` computes a planar body's
winding number about a point as a sum of signed solid angles
(Van Oosterom–Strackee: `solid_angle`, `faces`, `winding`). Its own doc on
`faces` says the winding "has two more copies (`m5_pr12_fix_pass.rs`,
`strut_cover_on_cylinder_pairs.rs`) and no shared home yet". Both of those are
in `crates/sweep/tests/`. No row covered it. ONEREAD's
`test-files-copy-small-vector-helpers` cites the fin crown only for its small
vector helpers.

## What closing it takes

One home in `test-utils`, read by the three files, or a statement in each of
why its copy differs.
