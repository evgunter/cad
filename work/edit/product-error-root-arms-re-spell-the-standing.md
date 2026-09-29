---
id: product-error-root-arms-re-spell-the-standing
kind: issue
title: ProductError's three root arms re-spell the node standing instead of carrying it
status: spec
branch: edit/part-product-refusals
opened: 2026-09-29
priority: P4
cost: E
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
to re-spell `product.rs`'s arms around that lane. That lane merged
(PR 3459) before C6's fix pass, so the row is unblocked; the fix pass
left it here because the review did not rule on it. `product_fault`
now also reads `eval/parts.rs`'s `NodeResult` (the census's line for
that file), and a `Root(NodeStanding)` arm would not change that read.

## What a fix would be

One arm, `ProductError::Root(NodeStanding)`, rendered
`product: root {standing}` or similar; `ProductError::kind` derives the
three classes from the standing's kind, so `ProductErrorKind`, the
viewer's badge placement and the Python words need not move;
`product_fault` matches `Root(NodeStanding::Failed { node })`. Then
`From<NodeStanding> for ProductError` goes, and the census row
(`crates/editor-core/tests/node_standing.rs`) is unchanged — the read
already goes through the one door.

## Ruled and spec'd (2026-09-29, EDIT orchestrator) — one unit with its two siblings, branch `edit/part-product-refusals`

This row, `a-parts-poisoned-root-drops-the-failure-that-poisoned-it` and
`part-product-refusal-draws-the-gathers-stage-prefix` are **one unit**.
They are the same seam, `eval/parts.rs`'s `product_fault` over
`product.rs`'s `ProductError`, seen from three sides.

**Tier:** single review (one Opus FULL). It changes public refusal payloads
and a Python tag's carried value.

**It depends on D366** (PR #3469). D366's `NodeErrorClass` projects every
`PartFault` arm, so this unit branches from `origin/main` after #3469
merges and extends the class and its witness census for any arm it adds.

1. **ProductError carries the standing.** `ProductError`'s root arms carry
   `NodeStanding` as one payload. `ProductErrorKind`, the viewer's badge
   placement (`viewer/src/frame.rs`) and `product_error_tag` keep their
   classes and their tag words (`unknown_node`, `root_failed`,
   `root_poisoned`), with the tag table pinned against main. The three
   hand sentences go.
2. **A poisoned root carries the failure that poisoned it.**
   - A part whose product root was poisoned crosses carrying the
     refusal at `through`, typed. `through` is the node the author
     repairs, so it goes through the same carried-refusal machinery
     #3459 built (`carried_chain`, `CarriedLine`), and the viewer draws
     it as a level of the traceback.
   - Choose the arm shape: reuse `PartRootFailed { node: through, … }`
     with the root named, or add an arm naming both. Say which and why.
   - Use `viewer/tests/common/mod.rs`'s `broken_document` as the
     witness.
3. **No doubled stage word, and a recourse.**
   - Build the `Part/PartProduct` concision row from a real
     `ProductError`'s text (red today).
   - The instance's sentence must not draw the gather's own
     `product:` inside the part's wrapper.
   - `PartProduct` states a recourse (the shape guard's item 9 notes it
     has none).
4. **Python.** The `part_product` tag and the carried `__cause__` move
   with it; announce the change to LIB.

**Rows** (red on `origin/main`, then green):
- a poisoned-root part's traceback ends at the failing node;
- the `PartProduct` row with real text passes `problems`;
- the product tag table is unchanged;
- the class census covers any new arm.
