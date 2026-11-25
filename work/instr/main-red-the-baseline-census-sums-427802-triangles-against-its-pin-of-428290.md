---
id: main-red-the-baseline-census-sums-427802-triangles-against-its-pin-of-428290
kind: issue
title: main red: tess-lint's baseline_census::the_committed_baseline_sizes_this_much reads 427,802 triangles against its pin of 428,290 since the teapot and lily re-cut
status: open
opened: 2026-11-25
priority: P1
cost: E
---


## Finding

Seen by EMBEDWALK's lane on PR 7772 (2026-11-25), whose `tools` job ran
`tools/tess-lint/tests/baseline_census.rs`'s
`the_committed_baseline_sizes_this_much` and failed
`427802 != 428290` ("triangles over the whole sweep"; the pin is
`428_290`). PR 7772 has no diff under `tools/` or
`docs/tess-budget-data/`. The committed baseline moved on main with
`49f4da5b86` ("demos: the teapot's re-cut handle and the lily's mirrored
leaf in the tess-budget baseline [render]", 2026-11-25) and the census's
pin did not. So every PR whose change filter selects the `tools` job is
red on it until the pin (and whatever else the census sums from the
re-cut rows) is re-taken.

Fix: re-derive the census from the committed CSV, the way the census's
own runbook says, and say what moved. Owed by the change that re-cut the
baseline.

Filed by the EMBEDWALK orchestrator.
