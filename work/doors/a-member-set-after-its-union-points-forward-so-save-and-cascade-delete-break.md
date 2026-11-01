---
id: a-member-set-after-its-union-points-forward-so-save-and-cascade-delete-break
kind: issue
title: A member SetMembers adds after its union points forward, so save and cascade delete break
status: closed
opened: 2026-10-07
closed: 2026-10-08
priority: P2
cost: M
refs: [a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added, 4244]
---


Raised by PR 4244's review. Main behaves the same way: this is not
from the ordinal ids, though that PR's docs now name it.

**What breaks.** The `SetMembers` door
(`crates/editor-core/src/edit.rs:5668`) checks only that each member is
live and that the DAG stays acyclic (`check_acyclic`, `:5706`). So a
union can take, as a member, a block minted *after* it: an input whose
id is greater than its consumer's. Three things assume an input's id is
always less than its consumer's (id order is topological), and each one
breaks:

1. **`save` refuses `ForwardInput`.** The load door
   (`crates/editor-core/src/persist/check.rs:1547`, `input >= id`)
   refuses the document the edit door just accepted. The author cannot
   save their work.
2. **`cascade_delete_order(b2)` misses the union.**
   (`crates/editor-core/src/edit.rs:5267`.) Deleting the late member
   `b2` returns `[t, b2]`. The single forward pass in id order never
   reaches the union, which sits before `b2`. The first `DeleteNode`
   then refuses `DeleteWouldDangle`.
3. **`split` deletes in reverse id order.**
   (`crates/editor-core/src/refactor.rs:3184`.) It rests on the same
   premise, so a cut holding such a union and its late member would try
   to delete the member first.

**Where it is stated.** `Doc`'s docs, `Doc::ids`, `cascade_delete_order`
and `SnapshotError::ForwardInput` each state the premise with this
exception and cite this row.

**Shape of a fix (a design choice).** Either the `SetMembers` door
refuses a member minted after its union (the insert door's own rule,
which makes the premise true again), or the three readers stop leaning
on id order and walk the edges. This row's sibling
`a-union-that-becomes-flush-later-can-only-be-deleted-and-re-added` is
about the same door's recourse. A refusal here narrows that recourse
further, so the two should be decided together.

## Closed

Dissolved by INTENT stage 2 unit B (`operands-are-reads`, PR 4342),
which reads the dependency relation instead of id order everywhere this
row names. The load door's `ForwardInput` retired (a cycle over reads
refuses `SnapshotError::ReadCycle`); `cascade_delete_order` closes over
readers in schedule order, and a delete no longer refuses
`DeleteWouldDangle` (its readers strand, typed); `split`'s closure
reads `Doc::upstream_of`. The row is
`crates/editor-core/tests/intent_s2_b_reads.rs::a_forward_member_saves_loads_and_cascades`,
the same as the closed
`recipe/set-members-admits-a-forward-member-the-save-validator-refuses`.
