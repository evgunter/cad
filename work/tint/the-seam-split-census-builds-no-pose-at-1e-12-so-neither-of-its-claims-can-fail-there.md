---
id: the-seam-split-census-builds-no-pose-at-1e-12-so-neither-of-its-claims-can-fail-there
kind: issue
title: The seam-split census builds none of its 384 poses at ε 1e-12, so at that row neither of its claims can fail
status: open
opened: 2026-11-14
priority: P4
cost: E
---


Filed by GREEN's default-eps lane (`green/default-eps`, PR 6931,
2026-11-14), from its STYLE review (S10). Older than that PR.

## Measured (the PR's head on main `34cc6b103`, the test's own print)

`sweep::all an_aimed_ray_from_far_reads_a_split_side_right::the_seam_split_census_reads_no_side_inside_out`:

| ε | build | refuse a side on the ellipse's rounding | refuse the split | stop a measure at its floor |
|---|---|---|---|---|
| 1e-9 | 168 | 54 | 158 | 4 |
| 1e-6 | 192 | 0 | 192 | 0 |
| 1e-12 | **0** | 138 | 246 | 0 |

The row claims that no side reads inside out and (since PR 6931) that
the poses whose measure stops at its floor are the ones pinned. A side
can read inside out, or stop a measure, only where the split returns
sides. At 1e-12 no pose does, so the row runs 384 splits and can fail
on neither claim; it reports green at that row every night having
asserted nothing about a side.

Its sibling `a_seam_split_a_hair_off_tangency_reads_no_side_inside_out`
says as much for its own family (a built floor of 0 at 1e-12, all 21
refusing), in a comment.

## Wanted

One of: the census states a built floor per ε row as its sibling does
and `test_utils::loud_skip_marker!`s the row where the floor is 0, so
the log shows the absence; or the 1e-12 row gets poses that build
there (the tilts are fixed lengths against a band that scales with ε,
which is the likely reason none does, not checked).
