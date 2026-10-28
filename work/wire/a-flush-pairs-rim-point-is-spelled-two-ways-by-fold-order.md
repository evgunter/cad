---
id: a-flush-pairs-rim-point-is-spelled-two-ways-by-fold-order
kind: issue
title: union_pinch_member_order's flush pair spells the point x=0.5 as Vertex@m1 with m0 folded last and Edge@m1 with m2 last: one fact, two spellings (DM4)
status: open
opened: 2026-10-26
priority: P3
cost: M
---


Found by `wire-meetrow` (PR 5798, 2026-10-26), by measurement. Unchanged by that PR: the merge base splits it the same way.

In `flush_onto_an_edge`, the point x=0.5 is recorded as `Vertex@m1` when m0 is folded last, and as `Edge@m1` when m2 is folded last. The lane attributes the split to wire's naming of the merged rim, the rim the fold re-cites. DM4 (`crates/editor-core/REFERENCES.md`) requires one spelling per fact.

To measure: dump the published coincidence rows of `flush_onto_an_edge` in all six member orders and find which re-cite picks the vertex over the edge.
