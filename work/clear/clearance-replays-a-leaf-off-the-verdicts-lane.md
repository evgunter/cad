---
id: clearance-replays-a-leaf-off-the-verdicts-lane
kind: issue
title: clearance_with replays a certified leaf at plain Interval whatever lane the drive certified it on
status: closed
opened: 2026-10-27
priority: P1
cost: E
closed: 2026-10-28
pr: 5928
---


## Finding

Both symbolic-tier designers found this independently, by reading the
code (not run; CLEAR orchestrator, 2026-10-27).

`clearance_with` (`crates/editor-core/src/clearance.rs`) answers a
bound question over a drive's certified leaves through
`clearance_over`. It rebuilds each leaf as
`let ev: Evaluation<Interval> = evaluate(…)`, at plain `Interval`,
whatever lane the drive certified that leaf on
(`ParamBoxVerdict::lane()`). `report.rs`'s `replay_leaf` replays on
the drive's own lane. Its docs say a replay on the wrong lane "would
quietly move certified mass".

The drive's up-front guard (`DriveRefusal::SymbolicClearanceUnsupported`)
catches a `min_clearance` *measure*, not this query.
`DriveConfig::default()` has the symbolic tier on.
- On today's ε-wide boxes the two rebuilds agree.
- On a wide leaf certified at `Sym<Interval>`, the plain `Interval`
  rebuild is the one E12 showed refuses. So the query should refuse a
  leaf the drive certified (likely, not run). The refusal is loud, not
  unsound.

## Fix

Evaluate on `verdict.lane()` and hand the generic door the `Body<T>`.
It rides with `symbolic-tier-and-clearance-engine`, whose decision
makes the door generic. A row that drives a wide box with the tier on
and asks the clearance query pins it.
