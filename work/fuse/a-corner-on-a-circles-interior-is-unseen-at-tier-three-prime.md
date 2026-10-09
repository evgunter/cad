---
id: a-corner-on-a-circles-interior-is-unseen-at-tier-three-prime
kind: issue
title: Tier 3′ does not see a vertex resting on a curved edge's interior: unrecorded, a corner on a circle off its vertex reads clean
status: open
opened: 2026-10-07
priority: P3
cost: M
---

## The finding

The census's vertex-granular sweeps read straight edges only
(`crates/topo/src/census.rs:1173`, `edge_is_line`): a vertex resting
on a curved edge's interior is confirmed when an op records it (the
curved confirm lane, `census/curved.rs`), but never found when no op
did. The corner witness pins the gap as expected
(`crates/sweep/tests/snowman.rs:1568`): a cube's corner on a cap
circle 40° from its conventional vertex, with its `(u, E)` record
stripped, reads clean at tier 3′, where the same corner at the
vertex reads `UndeclaredContact`. An op that lost such a record would
ship a body whose touch no tier sees.

## What it needs

A vertex-on-curved-edge sweep: each vertex against the curved edges
whose boxes it meets, read through `census::curved::on_curved_interior`
(which already decides on-carrier and in-span), finding an unrecorded
touch as `UndeclaredContact`. The snowman row's 40° arm then flips to
expect the finding.
