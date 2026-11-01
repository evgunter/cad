---
id: pick-ambiguity-lists-groups-in-id-order
kind: issue
title: pick_for's cross-group Ambiguous refusal lists unplaced spaces and moved instances in id order, which since ids are digests is no order a reader can follow
status: open
opened: 2026-10-02
---


## What

`PickIndex::pick_for` (`crates/viewer/src/pickindex.rs` ~1296) batches
the unmoved parts in a `BTreeMap<Option<RecipeNodeId>, _>` keyed by
unplaced-group root id (`eval.unplaced`), and the moved instances by
iterating `DisplayView::moved_roots`, a `BTreeMap` keyed by root id.
The doc comment (~1269) says group order is "the unmoved batch first,
then moved instances in node order", and that it "is the order the
refusal LISTS them in and decides nothing". Before #3594, "node order"
was insertion order. Now it is digest order, which moves between ε
rows and is no order a reader can follow.

Found by PLACE's document-order sweep (PR 3882). Not fixed there,
because no cheap fixture reaches a cross-group `HitTestError::Ambiguous`
to pin the change with a row red under reversed ids.

## The fix, likely small

Order both batches by the pick index's own root order (`self.parts`,
"root order then payload order"), the world's batch first. Restate the
comment. The row is the hard part: two groups tied on one ray, with
ids against document order (grow the instance count until they are,
as `assembly_display::a_fused_instances_refusal_lists_the_others_in_document_order`
does).
