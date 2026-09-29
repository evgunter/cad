---
id: part-product-refusal-draws-the-gathers-stage-prefix
kind: issue
title: editor-core: a part with no product draws the gather's own 'product:' stage prefix inside PartFault::PartProduct, which the concision roster's made-up message hides
status: closed
closed: 2026-09-29
pr: 3482
rides_with: product-error-root-arms-re-spell-the-standing
branch: edit/part-product-refusals
opened: 2026-09-29
priority: P3
cost: E
---

## What

An instance whose part has no product for a reason other than a failed
root crosses as `PartFault::PartProduct { kind, message }`
(`crates/editor-core/src/eval/parts.rs`, `product_fault`), whose
`message` is the gather's own `ProductError` rendering
(`crates/editor-core/src/product.rs`, `impl Display for
ProductError`). Every arm of that rendering opens with the stage
prefix `product:`, and `PartProduct`'s `Display` wraps it once more,
so the feature tree draws, for a part whose only root is a frame:

    node N failed: instantiating the part: the referenced document has no product: product: no product root denotes a body — this document has no body product

`test_utils::refusal::problems` flags the `product:` clause as a stage
prefix. The concision roster does not see it: its `Part/PartProduct`
row (`crates/editor-core/tests/refusal_concision_chains.rs`,
`document_arms`) builds the fault with a made-up message ("the
document declares no body root") rather than the gather's own text.
The same holds on `main`.

Found by the review of `edit/part-root-carried-refusal` (PR 3459), its
probe of a part whose root is `Ok` but whose product refuses.

## What would close it

The roster row built from a real `ProductError`'s text, red, and then
the sentence the instance draws rewritten so the gather's stage word
is not repeated inside the part's: the part's wrapper names the class
(`kind`), or the gather's sentence loses its prefix where it is
carried. `the-refusal-shape-guard-has-blind-spots` item 9 already
notes that `PartProduct` states no recourse of its own; the two are
one rewrite.

## Ruled (2026-09-29, EDIT orchestrator) — rides with `product-error-root-arms-re-spell-the-standing`

One unit with its two siblings; the spec is in that row.

## Built (2026-09-29, PR 3482)

The part carries `ProductError::sentence` (no stage word) and states a
recourse by class; the `Part/PartProduct` roster row is built from a
real evaluation. Record in
`product-error-root-arms-re-spell-the-standing`'s `## Built`.
