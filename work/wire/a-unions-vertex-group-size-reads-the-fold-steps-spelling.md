---
id: a-unions-vertex-group-size-reads-the-fold-steps-spelling
kind: issue
title: A union's published vertex group reads size 1 in one member order and no group in another
status: closed
opened: 2026-10-26
priority: P3
cost: M
closed: 2026-11-01
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

## Findings (wire-unionvertex, 2026-10-28)

**Decision.** A union vertex group is one member corner's copies. The
end pass (`emit_union`'s `cite_member_edges`) forms one group per
member-corner set, under the corner's name, as the pair boolean groups
an operand corner's copies. Only a corner's copies are qualified, so no
other union vertex forms a group. The one exception is a row the end
pass keeps verbatim with the fold's fragment qualifier: the fold's
`Tied` rows at a member corner, see
`a-unions-tied-fold-rows-at-a-member-corner-bypass-the-corner-rule`.
Such a row is a member of the group the fold step formed, so
`cite_member_edges` puts its base in `Rederived::by_fold`, and that base
alone keeps the fold reading. The fold reading for those bases was
order-free over the suite: 0 disagreeing pairs. Without that exception,
51 such bases read `Some([2])` → `Some([])`.

**Probe** (env-gated hook after `FragmentGroups::folded`, not
committed; editor-core suite minus the interval, range and solve
drives). (test, published vertex) pairs reading both `Some([1])` and
`Some([])` across member orders: **1806 on main, 0 on the branch**.
Pairs reading any two values: 1827 → 0.

**Dump.** Every union evaluation's published table has the same digest
on both sides: 7284 (test, table) digests, all equal. Of 739,479
(test, table, name, query) `sizes` lines, 51,688 moved, all of them
`SIZE` of a bare vertex name:

| Before | After | Lines |
|---|---|---|
| `Some([1])` | `Some([])` | 50,637 |
| `Some([])` | `Some([1])` | 844 |
| `None` | `Some([])` | 132 |
| `Some([0])` | `Some([1])` | 60 |
| `Some([1, 1])` | `Some([])` | 15 |

No face, edge or qualified-vertex base moved.

**Witness.** `resolve_group_membership::a_unions_corner_held_twice_then_once_reads_two_to_one_in_every_member_order`.
On main it reads `GroupResized { was: 2, now: 0 }` in the orders whose
fold joins the corner away, and `now: 1` in the others. On the branch it
reads `now: 1` in all 24 orders.

## Closed

2026-11-01 (WIRE orchestrator): PR 5930 merged. A union records its vertex groups over the finished body: one group per member corner, holding its copies. The order-dependent probe went from 1806 (test, vertex) pairs to 0. Published tables are identical. A fold-tied row at a member corner still reads the fold's record (`Rederived::by_fold`), until the corner row retires that exception. The witness covers all 24 member orders and is red on main in 16 of them.
