---
id: main-red-1e-12-a-cylinder-wall-through-a-prism-corner-union-escalates
kind: issue
title: main red at 1e-12: a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta's union escalates at props_chart_junction_closed (margin 2.8e-12)
status: open
opened: 2026-11-23
priority: P1
cost: M
refs: [7419]
---

Filed by the CIW orchestrator, 2026-11-23. The red was seen on PR 7419's
gate (run 45062330481, `test (kernel)`, the 1e-12 row):
`sweep::all a_cylinder_wall_through_a_prism_corner_meshes_at_a_coarse_delta`
fails with `the union builds at eps 1e-12`, Escalated/Indeterminate at
`props_chart_junction_closed`, margin 2.8e-12.

The red is inherited, not PR 7419's. That PR changes no `src` that sweep
reads: comments, a deleted probe test, and test files in other crates.
The row ran only because the PR edits sweep test files, which selects
sweep's 1e-12 row. No row named this test. The closest is
`work/nearpoint/a-cylinder-circle-and-a-chart-junction-escalate-in-band-at-1e-12.md`,
whose owner should say whether this is the same escalation. The last two
nightlies were cancelled, so main has no nightly reading for it yet.

Band P1, not P0: the red is at a non-default ε. It is still a red main
that any PR editing a sweep test inherits.

What would close it: the test is green at 1e-12 on main, or it is pinned
to the escalation with the reason and an owning row.
