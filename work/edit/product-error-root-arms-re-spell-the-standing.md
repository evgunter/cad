---
id: product-error-root-arms-re-spell-the-standing
kind: issue
title: ProductError's three root arms re-spell the node standing instead of carrying it
status: open
opened: 2026-09-29
priority: P4
cost: S
rides_with: C6
---

Filed by `edit/node-standing-one-type` (C6's "no usable value"
member), which left this site half done on purpose.

## The finding

The gather reads each root through `Evaluation::usable`
(`crates/editor-core/src/product.rs`, `product_recorded`'s first
pass over the roots), so the
READ is the one door. The REFUSAL is not: `ProductError` still
spells the standing's three arms as three arms of its own —
`UnknownNode { node }`, `RootFailed { node }`,
`RootPoisoned { node, through }` — mapped from the standing by
`impl From<NodeStanding> for ProductError`, with its own three
sentences in `ProductError`'s `Display` ("root N failed to evaluate
(ask `Evaluation::node_error` …)", "root N never ran — poisoned
through failed ancestor M", "root N has no entry in this
evaluation"). Every other door C6 retyped carries the standing as
one payload and renders it through `NodeStanding`'s `Display` under
its own subject.

## Why it was left

`edit/part-root-carried-refusal` was live on the arms' readers when
C6 landed: `eval/parts.rs`'s `product_fault` destructures
`ProductError::RootFailed { node }`, and `ProductErrorKind`'s
`RootFailed`/`RootPoisoned`/`UnknownNode` are the classes the viewer's
badge placement (`crates/viewer/src/frame.rs`) and the binding's tags
(`crates/pncad-py/src/tags.rs`, `product_error_tag`:
`unknown_node`/`root_failed`/`root_poisoned`) read. C6's spec said not
to re-spell `product.rs`'s arms around that lane.

## What a fix would be

One arm, `ProductError::Root(NodeStanding)`, rendered
`product: root {standing}` or similar; `ProductError::kind` derives the
three classes from the standing's kind, so `ProductErrorKind`, the
viewer's badge placement and the Python words need not move;
`product_fault` matches `Root(NodeStanding::Failed { node })`. Then
`From<NodeStanding> for ProductError` goes, and the census row
(`crates/editor-core/tests/node_standing.rs`) is unchanged — the read
already goes through the one door.
