---
id: a-space-other-than-the-world-is-census-checked-only-if-a-new-space
kind: issue
title: The at-rest gate checks the world and each new_space() holding a placed copy, but not a construction's own space: copies placed into one are checked by nothing until the ⊔ operator decides its operands
status: closed
opened: 2026-10-22
priority: P1
parent: assemble-is-an-operator-and-the-product-is-a-slot
closed: 2026-10-26
---



Found by the D′1 review (`a-placement-names-its-target-space`, PR 5592).

The at-rest gate (`crates/editor-core/src/assembly.rs`, `gate_spaces`)
checks the world's copies (`product_recorded`, the world's gather) and
each `new_space()` that holds a placed copy, gathered by itself
(`space_gathers`, `crates/editor-core/src/product.rs`). A construction's
own space is not gathered: its body is no placed copy, and the copies
placed into it are operands of whatever reads them there, which is an
operator's business, not the product's. So two copies placed into a
construction's space that interpenetrate are reported by nothing until
an operator reads them, and then by that operator's own refusal or
result, not by the census.

D′3 makes `⊔` the fourth operator with the census at its door, so the
copies a construction's space holds are checked where they are
combined; `space_gathers` then retires into it.

Done when: two copies placed into a construction's space that clash
are refused by the census at the door of the operator that combines
them, and `space_gathers` is gone.

## Resolved by D′3 (INTENT, 2026-10-26)

D′3 (PR 5636, merged `29110056e8`) moved the census to each `⊔`'s door and retired the per-space gate (`gate_spaces` is gone from `crates/`). Copies meet only where a `⊔` reads them, whatever space they are in, and that `⊔` decides them. A copy no `⊔` and no product reads is an unread body, which is a finding (test 26). Where a construction's own space is shown and not read: `the-viewer-does-not-show-a-constructions-own-space`.
