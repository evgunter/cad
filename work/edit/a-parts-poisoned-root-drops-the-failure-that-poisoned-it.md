---
id: a-parts-poisoned-root-drops-the-failure-that-poisoned-it
kind: issue
title: editor-core: a part whose product root was poisoned crosses as PartProduct with no carried refusal, so the failure that poisoned it is lost at the seam
status: review
pr: 3482
rides_with: product-error-root-arms-re-spell-the-standing
branch: edit/part-product-refusals
opened: 2026-09-29
priority: P3
cost: E
---


## What

`product_fault` (`crates/editor-core/src/eval/parts.rs`) carries the
failed root's own refusal only for `ProductError::RootFailed`. A root
that never ran because an ancestor inside the part failed is
`ProductError::RootPoisoned { node, through }`
(`crates/editor-core/src/product.rs`), and it crosses as
`PartFault::PartProduct { kind: RootPoisoned, message }`: the sentence
"product: root N never ran — poisoned through failed ancestor M" and no
refusal. The nested evaluation dies with the resolution, so the
failure at `through`, which is the reason the part has no body, is
lost at the seam, and the instance row's traceback
(`crates/viewer/src/tree.rs`, `carried_lines`) has nothing to draw.

This is the ordinary shape of a broken part whose root is a transform
or a boolean over the node that failed (`viewer/tests/common/mod.rs`'s
`broken_document` is one: its extrude fails and its root, a transform,
is poisoned).

Found by `edit/part-root-carried-refusal`, whose
`PartFault::PartRootFailed { node, refusal }` carries the refusal for
the `RootFailed` arm only, as spec'd.

## What would close it

The poisoned root crosses carrying the failure at `through`, typed:
the same `PartRootFailed { node: through, refusal }` (the failed node
the author repairs), or an arm naming both the root and `through`. The
Python `part_product` tag and the tag inventory move with it
(`crates/pncad-py/src/tags.rs`), announced to LIB.

## Ruled (2026-09-29, EDIT orchestrator) — rides with `product-error-root-arms-re-spell-the-standing`

One unit with its two siblings; the spec is in that row.

## Built (2026-09-29, PR 3482)

A new arm, `PartFault::PartRootPoisoned { root, through, refusal }`,
tag `part_root_poisoned`; the traceback ends at `through`. Record in
`product-error-root-arms-re-spell-the-standing`'s `## Built`.
