---
id: a-rank-change-leaves-readers-unsaveable
kind: issue
title: A door that changes a family's rank or a side index's definition admits a document the load door refuses
status: open
opened: 2026-11-10
priority: P2
cost: M
design: true
---

Found by the review of PR 6555 (probes P1, P3, P5; the reviewer's `zz_review_probe.rs`).

`DocEdit::DefineVar` re-defining an index `k` as `within` another index `j` gives every family under `k` a second index. The edit door admits it, and every member read of such a family is left at the old rank:
- a selection's `fillet(xs[2])` (P1): evaluation refuses `MemberOutOfRange`, and `persist::save` refuses `Snapshot(SelectionShape { fault: Input(IndexRank { found: 1, rank: 2 }) })` (`crates/editor-core/src/persist/check.rs` `first_selection_fault`, `crates/editor-core/src/edit.rs` `selection_body_fault`);
- a mate side `pattern[1]` read through its plane pose (P3): the same at save, pinned by `crates/editor-core/tests/select_member_read.rs` `the_load_door_holds_a_side_read_through_a_pose`;
- a node seat's `xs[2]` on main (P5): `save` refuses through `Node::input_fault` (`crates/editor-core/src/node.rs`).

The same shape: `DefineVar` turning a mate side's written index into an expression (`k mod 2`) is admitted, and save refuses `SideIndex { k: 0 }` (same test). Until save, the split's interface gather reads no head for that side (`crates/editor-core/src/mate/member.rs` `head_of` → `None`, `crates/editor-core/src/refactor.rs` crossing loop) and skips its crossing as AQ8 SKIP.

The design question: should every door that changes a family's rank, or a written index's definition, re-check the readers it reaches and refuse, or should rank-stale and index-stale reads stay saveable and refuse at evaluation (as an index out of range does)? Done when one is chosen and the door and the load door agree, pinned by P1, P3 and P5's shapes.
