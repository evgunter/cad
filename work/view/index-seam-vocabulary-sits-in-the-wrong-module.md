---
id: index-seam-vocabulary-sits-in-the-wrong-module
kind: issue
title: evalseam and pick import each other because the index seam's trait lives beside the evaluation seam rather than beside its payload
status: open
opened: 2026-09-05
---


## What

Found by VIEW-6b's style review (S3).

`crates/viewer/src/evalseam.rs` imports `crate::pick::{PickIndex,
PickIndexError}`; `crates/viewer/src/pick.rs` imports
`crate::evalseam::{Generation, IndexDone, IndexRequest, IndexService,
InlineIndexer}`. Both are vocabularies under
`crates/viewer/README.md`'s module rule, which the cycle does not
break — but it is a cycle, and the evaluation seam beside it has none:
it depends on the kernel and on nothing in this crate.

The asymmetry has a cause. **`IndexService`'s whole payload lives in
`pick`** — the request is what `PickIndex::build` takes and the answer
is what it returns. The trait is in `evalseam` only because the OTHER
seam is, and because that file is where this crate agreed to keep its
threads.

## Why it is not obviously a bug

The threads rule is real and was worth keeping: one file owning every
join handle and channel is what makes "no source change above this
boundary" checkable. Moving `IndexService` into `pick` would put a
worker thread in a module whose job is answering questions about a
tessellation.

So the choice is between the cycle, a third module for the index seam
alone, and hoisting the payload types. It is a placement question with
three defensible answers, which is why it is filed rather than fixed
inside a unit that had a ruling to build.
