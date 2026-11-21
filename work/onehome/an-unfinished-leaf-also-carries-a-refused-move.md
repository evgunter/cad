---
id: an-unfinished-leaf-also-carries-a-refused-move
kind: issue
title: NodeErrorKind::UnfinishedLeaf also carries a placed operand's refused move, which its name does not say
status: open
opened: 2026-11-21
priority: P3
cost: E
---


Filed by the operands lane (PR 7206, delta review).

`NodeErrorKind::UnfinishedLeaf { input, into, findings }`
(`crates/editor-core/src/eval/mod.rs`) was the refusal of a leaf that did
not finish at its take. Since PR 7206 it also carries a placed
operand's refused move: the subtract (`subtract_from`,
`crates/editor-core/src/eval/wire.rs`) and the fold (`fold_members`)
raise `topo::PlacedBooleanError::Placed` / `verbs::VerbError::Placed`
through `NodeErrorKind::unfinished_in(input: the operand moved, into:
Some(the operand whose frame it is))`. The image is a finished body
moved past the session box, not an unfinished leaf, so the variant's
name says less than it holds. Its doc says both.

The review held the rename (for example to a variant naming the copy
that did not stand in the computing frame) as not worth the churn in
PR 7206. It is a rename of one variant and its `Display` arm, plus the
rows that match it (`product_finishes_once::copies_either_side_of_the_origin_refuse_to_{unite,subtract}`,
`a_turned_discs_union_refuses_on_the_widened_copy`, and the
`UnfinishedLeaf` rows of `work/onehome/an-unfinished-leaf-names-the-copy-that-was-not-turned.md`).
Stage 2 of fork 288, which moves the move out of the door, is the
natural time to decide it.
