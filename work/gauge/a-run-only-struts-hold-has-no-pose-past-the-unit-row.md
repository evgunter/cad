---
id: a-run-only-struts-hold-has-no-pose-past-the-unit-row
kind: issue
title: A run only struts hold (its depth sort and the by_strut check) is reached by no pose since the In-arc pairing, only by one unit row
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

Under the In-arc pairing
(`shared-vertex-crossings-that-tie-or-interleave-are-unprobed`, PR
6832) no pose reaches that arm. A local probe that panicked on
minting such a run (GAUGE `gauge-svc` lane, 2026-11-11) failed one test
across the topo and sweep `ci` profiles: the unit row
`insert::tests::a_whole_orbit_run_of_four_survivors_is_spelled_as_its_strut`.
Before the change the sweep's eight-crossing row
(`join_pierce_runs_sweep::an_eight_crossing_pair_at_a_shared_vertex_builds_in_both_orders`,
then named for its depth-three strut chain) reached it at the shared
vertex. There the eight-crossing pair's four runs are now In struts,
none holding another, and its nesting is fans in the partner's walk.

The arm stays reachable in principle: a run spelled as the strut its
reverse is (`insert::spelled_reversed`) holds the rest of its plan in
the lead, and a nested matching inside one physical sector nests
under a strut in the other solid.

## Owed

A pose through a public door that mints a run only struts hold, at
depth two or more, pinned at its volume; or a reading that the arm is
unreachable, and its retirement.
