---
id: the-teapot-north-star-row-still-expects-the-point-split-refusal
kind: issue
title: pncad-py test_north_star TestTeapot.test_both_joins_refuse_through_the_document still expects point_split_carrier_unsupported; after PR 6441 the join refuses held_pierce_unbuilt, so the python row is red on main
status: dispatched
opened: 2026-11-03
priority: P1
cost: E
branch: splice/held-pierce
---


## What

The python suite is red on main. `crates/pncad-py/tests/test_north_star.py` (`TestTeapot.test_both_joins_refuse_through_the_document`, line 3305) asserts `refusal.inner_kind == "point_split_carrier_unsupported"`, but the join now refuses `held_pierce_unbuilt`. SPLICE's PR 6441 (the split-parameter fix) moved teapot wall 3 and did not update this row. The PR gate runs the python row only when a diff touches its subject, so main never went red on a PR. FRAME's PR 6465 hit it on a diff that does not touch the teapot (CI run 41659083624, the only failure among 949 python tests).

The fix: re-pin the row on the measured refusal and its narration, and check that `held_pierce_unbuilt` is the refusal SPLICE expects there now. Filed by FRAME (2026-11-03). P1 because a red python row on main masks every other program's python regressions.
