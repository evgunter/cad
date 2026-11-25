---
id: main-red-the-tess-lint-baseline-census-pins-the-triangle-count-before-the-teapot-re-cut
kind: issue
title: Main red: tools/tess-lint baseline_census the_committed_baseline_sizes_this_much pins 428290 triangles, which PR 7780's re-cut baseline (teapot handle, lily leaf) moved to 427802
status: open
opened: 2026-11-25
priority: P1
cost: E
refs: [7780, 7755]
---

Filed by INSIDE's orchestrator (2026-11-25) from PR 7755's CI. The
`tools` job went red on its merge ref after main moved, and was green one
push earlier.

`tools/tess-lint/tests/baseline_census.rs`
`the_committed_baseline_sizes_this_much` asserts `t.triangles ==
428_290`. PR 7780 (`49f4da5b86`, "the teapot's re-cut handle and the
lily's mirrored leaf in the tess-budget baseline") changed
`docs/tess-budget-data/tess-budget-baseline.csv`, and the sweep now reads
427802. The literal was not moved with it. The test's other literals
(`nurbs_triangles` and the rest) were not checked: a hosted run
stops at the first failed assertion.

The fix re-takes every literal in the test from the committed baseline
(`memories/output-stability-as-justification.md`: the number reports
what the baseline holds; it is not a target to preserve), in a PR
whose diff edits the test so the `tools` row runs. Every PR whose
merge ref includes PR 7780 sees the red until then.
