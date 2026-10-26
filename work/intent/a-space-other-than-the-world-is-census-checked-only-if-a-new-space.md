---
id: a-space-other-than-the-world-is-census-checked-only-if-a-new-space
kind: issue
title: The at-rest gate checks the world and each new_space() holding a placed copy, but not a construction's own space: copies placed into one are checked by nothing until the ⊔ operator decides its operands
status: parked
opened: 2026-10-22
priority: P1
parent: assemble-is-an-operator-and-the-product-is-a-slot
blocked_on: [assemble-is-an-operator-and-the-product-is-a-slot]
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
