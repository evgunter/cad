---
id: a-boolean-result-with-an-island-cannot-be-a-boolean-operand
kind: issue
title: A boolean result with an island is two solids, and a boolean refuses a multi-solid operand as JoinDesync rather than typed
status: closed
closed: 2026-10-03
opened: 2026-10-02
priority: P0
cost: H
design: true
---

Filed by the lane that files each island as a solid of its own
(`crates/topo/src/boolean/islands.rs`, `file_islands`; unit
`subtract-of-a-hollow-operand-files-the-island-under-one-solid`).

## Measured

`r = subtract(cube(0..6), shell(cube(2..4), 0.25))` is now two solids
(the island inside B's cavity is a solid of its own). Every further
boolean with `r` as operand A refuses:

- `subtract(r, cube(0.5..1))` (no crossings, the containment
  fallback): `JoinDesync { what: "fallback operand not one solid" }`
  (`crates/topo/src/boolean/ops.rs`, `fallback`'s `carve_kept`);
- `subtract(r, brick((5.5,6.5),(1,2),(1,2)))` (crossing A's outer
  wall, the seamed path): `JoinDesync { what: "operand A is not a
  single-solid body" }` (`crates/topo/src/boolean/finish.rs`,
  `setopfinish`).

On main before the island filing, both ran (`Voided`, one solid;
`Seamed`, one solid), because the island sat under A's solid. The same
holds for `union(hollow A, B inside A's cavity)`, and for a hollow
operand built that way being fed to a later boolean
(`subtract(cube(0..6), union(hollow(1..5, cavity 1.5..4.5), cube(2.5..3.5)))`
refuses `"fallback operand not one solid"`).

## What it is

Two things. (1) The refusal is labelled as a kernel lockstep bug
(`JoinDesync`) where it is a capability gap: the pair boolean takes
single-solid operands only. `shell` of a hollow operand already hands
back `k + 1` solids and meets the same wall, and
`work/pin/rows-do-not-cross-a-boolean-remap.md` and
`work/show/heatsink-placedunion-base-union-unfinished.md` record it from
the instance side. (2) The capability itself: a boolean over a
multi-solid operand, solid by solid, or a typed refusal that names the
missing door. Which of the two the next cut takes is the design
question (`design: true`).

## Closed (PR 3891)

Ev ruled (PR 3901) that booleans, `shell` and `split` take bodies. A
multi-solid operand now enters each pipeline as one multi-shell solid
and its result is sorted back into pieces, so the two-solid result is
the next boolean's operand:
`crates/topo/tests/hollow_island.rs::the_two_solid_result_is_the_next_booleans_operand`
cuts it far from the island (the containment fallback) and through A's
wall (the seamed path), and both build. `SplitFinishError::NotSingleSolid`
is gone; the boolean's internal single-solid checks are now desyncs.
