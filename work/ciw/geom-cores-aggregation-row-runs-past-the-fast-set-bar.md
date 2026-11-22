---
id: geom-cores-aggregation-row-runs-past-the-fast-set-bar
kind: issue
title: geom-core's every_suite_file_is_aggregated ran 1.19 s against the fast set's 1.0 s bar on a hosted runner and reded PR 7174, whose diff does not touch it
status: open
opened: 2026-11-20
priority: P4
cost: E
refs: [two-bisector-node-rows-break-the-fast-set-one-second-bar, fast-set-tests-half-a-second-to-0-8-at-opt-0-on-the-slow-runner]
---


Filed by GERM's orchestrator (2026-11-20). On PR 7174's CI run 44747158593,
the first attempt of `test (kernel)` failed on
`geom-core::all every_suite_file_is_aggregated` alone: 1.19 s against the
fast set's 1.0 s bar. All 12509 tests passed, and a re-run of the job was
green. The PR's diff touches no geom-core file. This is one more instance of
the fast-set bar class in this row's refs. The row reads the source tree
(the `every_suite_file_is_aggregated!` macro), so its cost is file I/O and
does not scale with the kernel's arithmetic. Owed: the slow set, or a
cheaper check.
