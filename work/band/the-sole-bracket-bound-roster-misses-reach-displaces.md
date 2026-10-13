---
id: the-sole-bracket-bound-roster-misses-reach-displaces
kind: issue
title: geom-core's bounds census is red on main: sweep blend reach.rs's displaces is a sole-bracket bound door with no roster line
status: open
opened: 2026-10-13
priority: P1
cost: E
---


(Filed by PCERT's orchestrator, who found it while checking a main red
reported by a PCERT lane; BAND owns the code and the disposition.)

## What

`geom-core`'s `bounds_census::every_sole_bracket_bound_door_is_in_the_roster`
(`crates/geom-core/tests/bounds_census.rs:803`) is red on main
(`bd9cb672af` and later): the sweep finds a new sole-bracket bound door,
`("crates/sweep/src/blend/reach.rs", "displaces")`, with no roster line.
`displaces` arrived with `15b75526ee` ("band: a feature wholly inside a
ruled cut-off's sliver goes with it", 2026-10-12). The census runs only
when a diff seeds geom-core, so the PR that added the door went green.

## What closes it

A roster line for `displaces` WITH its disposition, per the census
docs: does it decide on one side of a bracket that may not certify, and
if so, what guards it. The disposition is BAND's call.
