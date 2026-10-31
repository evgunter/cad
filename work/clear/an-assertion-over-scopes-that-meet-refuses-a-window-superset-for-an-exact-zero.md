---
id: an-assertion-over-scopes-that-meet-refuses-a-window-superset-for-an-exact-zero
kind: issue
title: an AtLeast assertion over min_clearance of scopes that meet refuses WindowSuperset, though the meeting's 0 is the faces' own
status: closed
opened: 2026-10-28
priority: P3
cost: M
branch: clear/tighten
pr: 5998
closed: 2026-10-29
---


## Finding

SELFX made `min_clearance` metric: two scopes that meet read exactly
`[0, 0]` from `topo::clearance::min_separation`, with the shared cell
carried as `MinSeparation::meets` (`crates/topo/src/clearance.rs:1729`,
`Meets` at `:1685`). That 0 is the faces' own value, attained on the
shared cell, not a carrier window's.

An assertion over it cannot see that. `Certified`
(`crates/editor-core/src/measure.rs:658`) is decided from the
expression tree alone, so a bare `min_clearance` is `LowerBoundOnly`.
`decide_assertion` (`:763`) therefore refuses
`UnevaluatedReason::WindowSuperset { endpoint: "upper" }` for
`min_clearance(wall, wall) ≥ 0.3`, though the measure is `[0, 0]`
exactly and the assertion is violated. Pinned by
`crates/editor-core/tests/m10_6_r1_probes_interval.rs:876`
(`a_wall_against_itself_reads_zero_and_its_at_least_bound_refuses_by_name`).

The refusal is conservative: it never reports a wrong verdict, and the
`AtMost` arm still gates.

## Repair shape

Carry the meeting with the measured value through the clearance lane
(today `MinClearanceLane`, which SYMCLEAR replaces with
`topo::ClearanceLane`), and admit the upper end when the scopes meet:
there `m = M = 0`. Then flip the pinned row to `Violated`.

## Closed

Resolved by TIGHTEN (#5998). `WindowSuperset` retired with all of
`Certified`. Two scopes that meet now read the bracket `[0, 0]`, which
is the faces' own: the wall against itself violates its `AtLeast`
bound (`a_wall_against_itself_reads_zero_and_violates_its_at_least_bound`).
