---
id: set-members-admits-a-forward-member-the-save-validator-refuses
kind: issue
title: SetMembers accepts a member inserted after the union, and the saved document then refuses ForwardInput: insertion order is not topological by construction
status: closed
opened: 2026-10-07
closed: 2026-10-08
---


## Finding

`DocEdit::SetMembers` (`crates/editor-core/src/edit.rs`, the `SetMembers`
arm of `apply`) checks the offered list for liveness, DM5 distinctness,
the list floor and acyclicity (`check_acyclic`), and nothing about
position. So a union inserted over `[a, b]` accepts `SetMembers { [a, b,
c] }` where `c` was inserted after the union. The document is acyclic
and evaluates (the schedule is Kahn's algorithm over `inputs()`,
`eval/schedule.rs`). But `persist::save` runs the shared snapshot
validator, whose input walk (`persist/check.rs`, the arm raising
`SnapshotError::ForwardInput`) refuses any input whose position in
`Doc::order` is at or after its consumer's, on the stated ground that
"insertion order is topological by construction — a forward ref means a
tampered file". The edit door produces a document the save door refuses.

Reproduced on main (two cubes, a union of them, a third cube, the
`SetMembers` above, then `save`): the edit is accepted and `save` returns
`Snapshot(ForwardInput { node: <the union>, input: <the third extrude> })`.

## What it is evidence of

The invariant is false, not merely unguarded: `SetMembers` has moved
edges both ways since DOCM-3, so `Doc::order` has been presentation
order since then, and `ForwardInput` reads a topological fact off a
positional one. Two more readers of the same false invariant:
`cascade_delete_order` (`edit.rs`) walks `Doc::order` once forward on
the ground that it is topological, so a union whose forward member is
doomed is missed when the union precedes it; and
`EditError::DeclaredNameNotUpstream` is stated as "before the node in
document order". Stage 2 of INTENT (`docs/INTENT-STAGE2-SPEC.md`, PR
4216) turns every operand into a read and FORK-4 asks whether reads may
be re-pointed, which makes the gap general; its fix is the same whichever
way FORK-4 goes.

## The fix

One dependency relation (the spec's `Doc::upstream`, or `inputs()` until
it lands) answers every "before" question: the load validator refuses a
cycle over that relation (the state no door produces) and drops the
positional `ForwardInput`; `cascade_delete_order` closes over consumers
rather than one pass; `DeclaredNameNotUpstream` asks the relation. A row
pins the forward `SetMembers` round-tripping through `save`/`load`.

Found by the FORK-4 designer lane (`design/intent-s2-fork4-A.md`).

## Closed

By INTENT stage 2 unit B (`operands-are-reads`, branch
`intent/s2-b-reads`), on the fix this row states. `Doc::upstream` is
the one dependency relation: the load door's positional `ForwardInput`
(and `DanglingInput`) retired, and a cycle over reads refuses
`SnapshotError::ReadCycle`; `cascade_delete_order` closes over readers
in schedule order; `DeclaredNameNotUpstream` asks the relation alone (a
name minted by a node the carrier does not read, directly or through
what it reads). The row is
`crates/editor-core/tests/intent_s2_b_reads.rs::a_forward_member_saves_loads_and_cascades`.
