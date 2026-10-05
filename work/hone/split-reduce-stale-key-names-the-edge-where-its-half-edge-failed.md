---
id: split-reduce-stale-key-names-the-edge-where-its-half-edge-failed
kind: issue
title: split's crossing classifier refuses StaleKey naming the edge, which resolves, where a half-edge failed
status: closed
opened: 2026-10-01
priority: P4
cost: E
refs: [stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body]
closed: 2026-10-04
pr: 4029
---

(TOPO fix pass on PR 3621, from its review's NOTE-2.)

## What

`splitting::classify::insert_crossings`
(`crates/topo/src/splitting/classify.rs`, the snapshot loop's `start`
reads, ~:506 and ~:511) reads each edge's two half-edges' `start` and
refuses `SplitReduceError::Euler(EulerOpError::StaleKey { key:
EntityId::Edge(edge_key) })` where one does not resolve. The edge came
from the arena's own snapshot, so it resolves: the half-edge
(`edge.he_plus`, `edge.he_minus`) is what failed. The render, "euler op
requires edge EdgeKey(…), which does not resolve", is false there, and
it names a record a reader can find live.

## Repair shape

Name the half-edge that failed (`EntityId::HalfEdge(edge.he_plus)` /
`he_minus`). Only a torn bijection reaches it, so it belongs with the
torn half of the split
`stale-key-and-not-same-edge-answer-for-a-callers-key-and-a-torn-body`
proposes, whichever lands first.

## Closed 2026-10-04

PR 4029. `insert_crossings` (`crates/topo/src/splitting/classify.rs`,
the snapshot loop's `start` closure) reads each half-edge through
`live::linked` with the edge as the holder and `"he_plus"` /
`"he_minus"` as the link, so a half-edge that does not resolve panics
"edge …'s he_plus names half-edge …, which does not resolve" (D2 row
4): the failed half-edge is named, and no typed refusal claims the live
edge failed.
