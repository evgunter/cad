---
id: revert-writes-each-start-through-an-unproven-next
kind: issue
title: revert writes every half-edge's new start as start(next(he)), so a live-but-foreign next or a foreign start moves onto another half-edge's start through Ok
status: review
opened: 2026-09-30
priority: P3
cost: E
branch: topo/revert-starts
pr: 3562
refs: [revert-anchors-trust-a-torn-next-or-prev]
---

## What

Found by the receipt of `revert-anchors-trust-a-torn-next-or-prev`
(PR 3546), which made `Body::revert` (`crates/topo/src/revert.rs`)
prove the two anchors it writes: each vertex's new `emanating` and
each loop's new `first`. The third map its precondition pass reads is
not proven: every half-edge's new start is `half_edge_end(he)`
(`crates/topo/src/body.rs`), which is `start(next(he))`, written with
nothing comparing it to the edge's other end, `start(mate(he))`.

A live-but-foreign `next` at `he` gives `he` the foreign half-edge's
start in the result, and a foreign `start` at `he` is written as the
start of `prev(he)`. Either way the fault the source carried at one
half-edge's derived end (or stored start) comes out as a stored start
elsewhere, and a validator reads it as a different fault.

## Measured

A probe (not committed) on the anchor-proven head: every single tear
of `declined_cube` and of `ops_holed_box`, then `revert`, comparing
the kinds of `validate`'s errors on the result with those on the torn
source. No `Ok` carries a `kill_anchor_faults` fault the tear did not
plant, under any tear kind. Counts are `Ok` results with an error kind
the source did not have:

| tear | `declined_cube` | `ops_holed_box` | new kinds |
| --- | --- | --- | --- |
| `next` | 336 of 576 | 1440 of 2304 | `OrbitForeignMember`, `SplitVertexOrbit` |
| `start` | 56 of 192 | 240 of 768 | `OrbitForeignMember` |
| `prev` | 432 of 576 | 1728 of 2304 | `EdgeNotAntiparallel`, `LoopCycleOverrun`, `VertexOrbitOverrun`, `UnreachableHalfEdge`, `SplitVertexOrbit` |
| edge slot | 24 of 288 (each slot) | 45 and 49 of 1152 | `VertexOrbitOverrun`, `SplitVertexOrbit` |

The `next` and `start` rows are the start map's. The `prev` rows are
mostly the `next`↔`prev` swap handing the torn link to the field the
validator's cycle walk reads, which is the same link reported under a
new name; a probe comparing kinds cannot tell the two apart, so the
row counts both. The source is tier-1-invalid in every case; the
review_d18 posture ("a torn body is entitled to `Ok(garbage)` and to a
typed error") covers the result as garbage, and the anchor row's
reasoning is that a reversal should not write new faults from a
live-but-wrong link.

## The shape to give

Either `revert`'s precondition pass proves each new start (each
half-edge's end is its mate's start, one comparison per half-edge
beside the `End` check) and refuses `RevertError::Corrupt` naming the
link, or its docs say that a torn `next` or `start` is carried onto
another half-edge's start, and why. The `prev` rows want a measurement
that tells a renamed fault from a written one before either answer.
