---
id: display-prune-withdrawals-list-instances-in-id-order
kind: issue
title: a display prune's withdrawals list instances in id order, which since ids are digests is no order a reader can follow
status: open
opened: 2026-10-02
---


## What

`DisplayState::prune` (`crates/viewer/src/display.rs`, ~1018) builds
`PruneReport::superseded` by `retain` over `moves`, a
`BTreeMap<RecipeNodeId, Frame>`, and `PruneReport::dropped_hides` by
iterating `hidden`, a `BTreeSet<RecipeNodeId>`. Both lists therefore
come out in id order, and `frame::Withdrawal::all`
(`crates/viewer/src/frame.rs`, ~1370) renders them in that order into
the status sentence a user reads.

Since #3594 an id is a digest, so id order is no order a reader can
follow: when one edit withdraws several instances (a delete that takes
a group, a mate that constrains two probed instances), the sentence
lists them in an order that is neither the tree's nor the authoring
one, and that differs between ε rows.

Found by PLACE's document-order sweep
(`document-order-is-read-off-node-id-comparison-since-ids-are-digests`, closed with PLACE (`docs/doc-ledger/place-leaves-the-tracker.md`)).

## Why it is not a one-line fix there

An instance withdrawn because it was DELETED has no place in the
document the prune runs against, so "document order" does not cover
the whole list. The choice is CHROME's: order the live ones by
`Doc::positions` and put the deleted ones after them, read the order
off the document before the edit, or say the list is a set and render
it so. A row red under reversed ids pins whichever is chosen:
`assembly_display::a_fused_instances_refusal_lists_the_others_in_document_order`
shows a way to make the ids disagree with the document deterministically
(grow the instance count until they do).
