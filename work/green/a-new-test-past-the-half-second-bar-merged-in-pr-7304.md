---
id: a-new-test-past-the-half-second-bar-merged-in-pr-7304
kind: issue
title: step-import's a_straight_rim_recognition_declines_is_carried_on_its_column merged in PR 7304 past the new-test 0.5 s bar it should have met
status: open
opened: 2026-11-22
priority: P3
cost: E
---


Found by a designer weighing the fast-set bar (2026-11-22). The row is a plain `fn` in `crates/step-import/tests/an_edge_is_read_on_its_description_whatever_recognition_says.rs`, added by PR 7304, and it reads 1.31-1.72 s on later gates and 1.46 s on the nightly at opt 2. PR 7304's gate should have held it to the new-test 0.5 s bar. Either its run read it under 0.5 s, or the PR merged without a kernel test run or past a red. Read 7304's run record to say which: a hole in the new-test bar would be the finding.
