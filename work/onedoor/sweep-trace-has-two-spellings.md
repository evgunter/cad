---
id: sweep-trace-has-two-spellings
kind: issue
title: the sweep trace has two spellings - boolean SweepTrace over (EdgeKey, FaceKey) and census SweepPairs over (EntityId, EntityId)
status: closed
opened: 2026-09-13
priority: P1
cost: E
closed: 2026-11-03
pr: 6433
branch: onedoor/one-spelling-trio
---


## The finding

The boolean sweep's differential trace is `SweepTrace` over
`(EdgeKey, FaceKey)` (`crates/topo/src/boolean/reduce.rs:94`, examined
and accepted pairs of one sweep direction); the census's is
`SweepPairs` over `(EntityId, EntityId)` plus `CensusTrace`, one
`SweepPairs` per sweep (`crates/topo/src/census.rs`, PERF-12). The
`EntityId` form generalises the other, and the comparators the two
suites share (restriction of the examined sequence, lost accepted
pairs) live on `SweepPairs` only; `m5_pr8_bvh_diff`'s `missing_pairs`
is the same comparator spelled on `SweepTrace`. Neither type owns the
idealized/realized trace vocabulary. Left as two spellings by PERF-12's
fix pass on the orchestrator's instruction (not this PR).

## What a fix is

One trace type over `(EntityId, EntityId)` with the comparators, the
boolean's `SweepTrace` becoming it (its keys lift into `EntityId`), and
both differential suites reading the one.

## Closed (PR 6433, 2026-11-03)

There is now one type, `topo::SweepPairs<P>`, in its own module (`sweep_trace.rs`), and neither consumer hosts it. It is generic over the pair rather than lifted to `(EntityId, EntityId)`, so the boolean keeps its typed keys.

Its fields are private. The writers on the type keep `accepted` a subsequence of `examined` by construction. Building that showed that `settle_deferred` recorded accepted pairs it never recorded as examined; it now records both.

The comparators are the only spelling. `is_restriction_of` is asserted in both BVH-diff suites and in `s16_box_soundness`.
