---
id: a-unions-vertex-group-size-reads-the-fold-steps-spelling
kind: issue
title: A union's published vertex group reads size 1 in one member order and no group in another
status: dispatched
opened: 2026-10-26
priority: P3
cost: M
---


A union answers the group-size rung for a vertex base from its fold
steps' records, never from the finished body. Whether a published vertex
has a group on record depends on whether some fold step happened to spell
the published name. The fold steps run in member order, so the record
depends on member order too, even though the name does not.

- `crates/editor-core/src/names/groups.rs:220` (`rederived`): only faces
  and seam edges are answered by the groups the union re-derives over the
  finished body. A vertex base falls through to the fold.
- `crates/editor-core/src/names/groups.rs:357` (`FragmentGroups::sizes`):
  a folded base is answered by the latest step whose key collapses to it.
  If no step spells it, the answer is `Some([])`, which means no group.
- The union's end pass re-derives every vertex name from the finished
  body (`emit_union::Flush::meeting`). A fold step's key for that vertex
  (`emit_topo.rs:3158`, `rec.record_by_name`) collapses to the published
  name only where the step happened to spell it the same way.
- `crates/editor-core/src/resolve/mod.rs:2066` (`group_resized`): a prior
  run that read size 1 and a new run that reads no group report
  `GroupResized { was: 1, now: 0 }`. The rung declines only when the
  prior size is 0, so the misreport is one-sided.

## Measured (wire-pairflush, PR 5778, 2026-10-26)

A probe logged `sizes(base)` for every published vertex of every union,
over the editor-core suite. The interval/range drives were not covered.

- **On main (2b875e0f0a):** 1803 (test, published vertex name) pairs read
  both `Some([1])` and `Some([])` across the member orders of one test.
  Examples:
  - `emit_union_flush_names::a_union_cites_only_what_the_finished_body_holds` (302)
  - `emit_union_rim_piece_ranks::*` (294 each)
  - `emit_shared_rim_several::no_order_of_the_probe_corpus_refuses_several_shared_rims` (258)
- **PR 5778 adds instances.** The pair's pinch arm now spells a line of
  several flush edges as `Merged`, as the end pass does. Some fold-step
  pinch keys therefore collapse to the published name and read
  `Some([1])` where main read `Some([])`. That happens in
  `name_invariance` (member and operand orders),
  `union_member_orders_decide_alike` and `union_pinch_member_order`.
  1 is the true count. It shows only in the orders where the pinch arm
  runs, so it is the same class.

## Repair direction

Answer a union's vertex groups from the end pass, as faces and seam
edges are answered (`name_by_parents`'s record): each published vertex is a group of
the entities its name holds. The record then reads the same in every
member order. This moves the `grouped` record of every union vertex from
no group to its count. Nothing published in a name table moves.
