---
id: gauge-of-recomputes-the-clusters-per-placement-lookup
kind: issue
title: gauge_of recomputes clusters(doc) on every placement lookup, so an evaluation's placement reads cost O(instances²) on any document with a mate
status: open
opened: 2026-09-19
priority: P3
cost: E
---


(MSOLVE-7 fix pass, 2026-09-19.) `mate::gauge_of` (`crates/editor-core/
src/mate/solve.rs`, the `gauge_of` fn) answers the fast path for a
document with no mates and otherwise recomputes `clusters(doc)` —
`read_mates` (a walk per mate reference) plus `components` over every
instance — to find the one cluster holding `instance`. `Doc::placement`
(`doc.rs`, the `placement` method) goes through it, and the evaluation
reads a placement per instantiate node (`eval/wire.rs`, `wire_instantiate
_part`'s `env.poses.placement(doc, id)`) and per mate node, so one
evaluation of a document with `n` instances and any mate at all pays
`n` cluster computations.

**Measured**, on `n` instances of one unresolved part reference and
ONE mate between the first two (every other cluster a singleton), `n`
calls of `doc.placement(id)` against one `clusters(doc)`:

| n | one `clusters(doc)` | n × `placement` | ratio |
|---|---|---|---|
| 250 | 0.43 ms | 120 ms | 280 |
| 500 | 0.96 ms | 528 ms | 551 |
| 1000 | 2.0 ms | 2.26 s | 1123 |
| 2000 | 4.2 ms | 9.5 s | 2245 |

Quadratic in the instance count, and the whole of it is repeated work:
the partition is one fact about the document. The shape of a fix is
the one the solve already has for its own reads — `solve_document`
computes the clusters once and `SolvedPoses` carries each instance's
gauge (`gauge` map), so a placement read made through `SolvedPoses::
placement` can answer from that map and never ask `gauge_of`; the
`Doc::placement` door then serves only callers with no solve in hand
(the edit door's re-keying, `edit.rs` `SetPlacement`, once per edit).
Not taken in MSOLVE-7: it changes what the evaluation reads a
placement through, outside the unit's fence.
