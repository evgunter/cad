---
id: a-run-only-struts-hold-has-no-pose-past-the-unit-row
kind: issue
title: A run only struts hold (its depth sort and the by_strut check) is reached by no pose since a shared vertex leads its pairs, only by one unit row
status: open
opened: 2026-11-11
priority: P3
cost: E
refs: [shared-vertex-crossings-that-tie-or-interleave-are-unprobed]
---


## What

A run that only struts of its own plan hold (`Held { fan: None, .. }`,
`insert::held_by`) mints at its plan's vertex, sorted among that
vertex's struts by its nesting depth (`insert::mint_plans`, the
"a run only struts hold nests deeper than its strut holder"
`debug_assert`), and `insert::mint_directed` checks the walk's holder
against the corner's geometry (`by_strut`, "the walk order and the
corner's geometry disagree on which strut holds a nested strut").

Since a shared vertex's solid leads its pairs' pairing (`insert::pairing_leads`,
PR 6832) no pose reaches that arm. A local probe that panicked on
minting such a run (GAUGE `gauge-svc` lane, 2026-11-11, re-run under
the rebuilt kept-side rule) failed one test across the topo and sweep
`ci` profiles and the sweep's `join_pierce_runs_sweep` slow rows: the
unit row
`insert::tests::a_whole_orbit_run_of_four_survivors_is_spelled_as_its_strut`.
On main the sweep's eight-crossing row
(`join_pierce_runs_sweep::an_eight_crossing_pair_at_a_shared_vertex_builds_in_both_orders`,
then named for its depth-three strut chain) reached it: main paired
in A's walk, so the chain nested in the shared wedge's walk. Paired in
the shared solid's own walk, the pair's runs there are adjacent, and
its nesting moves to the partner's walk, under fans.

The arm stays reachable in principle: a run spelled as the strut its
reverse is (`insert::spelled_reversed`) holds the rest of its plan in
the lead, and a nested matching inside one physical sector nests
under a strut in the other solid.

## Owed

A pose through a public door that mints a run only struts hold, at
depth two or more, pinned at its volume; or a reading that the arm is
unreachable, and its retirement.
