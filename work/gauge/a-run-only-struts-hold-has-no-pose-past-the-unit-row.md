---
id: a-run-only-struts-hold-has-no-pose-past-the-unit-row
kind: issue
title: A run only struts hold (its depth sort and the by_strut check) is reached by no pose since the shared-vertex reconcile stopped turning runs, only by one unit row
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

Filed new by PR 6832 (GAUGE, lane `gauge-svc`, 2026-11-11). Under
that PR no pose reaches the arm. A local probe that panicked on
minting such a run failed one test across the topo and sweep `ci`
profiles and the sweep's `join_pierce_runs_sweep` slow rows: the unit
row
`insert::tests::a_whole_orbit_run_of_four_survivors_is_spelled_as_its_strut`.
The PR's reviewer found the same with the lead forced to A as well, so
which solid leads is not what moved it.

On main the sweep's eight-crossing row
(`join_pierce_runs_sweep::an_eight_crossing_pair_at_a_shared_vertex_builds_in_both_orders`,
then named for its strut chain) reached it. There the reconcile turned
the eight-crossing pair's two fans, leaving its strut chain
`[2,5] ⊃ [3,4]` at the shared vertex for `hang_at_shared` to re-read:
a run held only by a strut. PR 6832 retires turning and
`hang_at_shared`, and a pair takes its other start whole instead, so
that chain no longer forms. This is read from main's row docs, not
traced.

The arm stays reachable in principle: a run spelled as the strut its
reverse is (`insert::spelled_reversed`) holds the rest of its plan in
the lead, and a nested matching inside one physical sector nests
under a strut in the other solid.

## Owed

A pose through a public door that mints a run only struts hold, at
depth two or more, pinned at its volume; or a reading that the arm is
unreachable, and its retirement.
