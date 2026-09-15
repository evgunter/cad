---
id: bvh-is-over-has-no-consumer
kind: issue
title: Bvh::is_over has no caller in the tree once the pick index keys by the memo entry
status: open
opened: 2026-09-13
---



## What is there

`Bvh::is_over` (`crates/bvh/src/tree.rs`) exists for one stated
consumer: "A consumer that caches trees answers 'is the cached tree
the tree over these boxes' with this instead of rebuilding to
compare." That consumer was `editor_core::resolve::pick`'s `PickMemo`
tree level, and PERF-10 removed it: the pick memo now serves a whole
`PickTable` under `mesh::StoredPatchId`, which is minted on the far
side of the patch memo's full-key byte comparison, so there is nothing
left to re-derive and compare.

After PERF-10 the only callers in the tree are in
`crates/bvh/tests/determinism.rs`
(`is_over_holds_for_the_input_bits_in_input_order_only`). The door is
sound and its test is real; what it no longer has is a user.

## Why it is filed rather than deleted

`docs/PERF-10-SPEC.md` §2 fences `crates/bvh` ("unchanged unless a
door there is cleaner than in `pick.rs`") and §4 fences its query, so
retiring a public door of that crate is not PERF-10's call.

## What a disposition is

Either: delete `Bvh::is_over` with its test row and the sentence in
its docs that promises a consumer — a public door kept alive by its
own test is the shape `memories/` calls out — or keep it and say in
its docs that it is offered for a tree-caching consumer the tree does
not currently have, so a reader does not go looking for one. The
second is cheap and honest; the first is what the repo usually does.
