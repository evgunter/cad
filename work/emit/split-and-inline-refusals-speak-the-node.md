---
id: split-and-inline-refusals-speak-the-node
kind: unit
title: Split and inline refusals speak the node with its label, from the document each names it in
status: open
opened: 2026-10-01
priority: P2
cost: M
parent: node-labels-are-document-data
---


Split from `kernel-door-refusals-beyond-edit-speak-the-node` (its PR spoke `RootFault`, a forwarded name's minting node inside `EditError` through `SpokenName`, and `Maintenance`'s strand and orphan rows). The rule is DESIGN.md Band 1, "Node labels", and the choice of document is the one on `edit::written`'s doc comment.

## What to speak

- `SplitError` (`refactor.rs`), 12 node fields and 4 names (`Box<StableName>`). `split` holds the source document, so every node is held there or absent: `UnknownCutNode` is `SpokenNode::absent`, the rest `source.spoken(..)`. The names speak through `Doc::spoken_name`.
- `InlineError` (`refactor.rs`), 6 node fields and 4 names. `inline` holds the host and the referenced part document. `UnknownNode`, `NotAnInstance` and `InstanceConsumed` name host nodes. `UnplaceableFrame`'s root and the names (`InstanceBodyNameReferenced`, `ForeignInstanceName`, `NameOnDroppedStep`, `StrandedPartName`) are in the part's ids, so they speak from the part document. `StrandedPartName`'s `missing` is absent from it by definition.
- The Python payloads (`pncad-py/src/py/refactor.rs`, the `E::…` projection) keep the full id through `SpokenNode::id` and the name text through `SpokenName::name`.

## A defect to fix on the way

`SplitError`'s `Display` prints the cut and kept ends of `TornGroup`, `OperandSeveredFromMate` and `SeveredEdge` as `root.0`/`instance.0` (the raw `u64`, in decimal) under `{cut}`/`{kept}`, beside the same ids printed as tags. No test pins the text (`grep 'is cut'` finds only the template). Speaking each end as a `SpokenNode` fixes it.
