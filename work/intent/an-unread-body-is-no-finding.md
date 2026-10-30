---
id: an-unread-body-is-no-finding
kind: issue
title: An unread body is no finding: DESIGN [operations]' 'an unread body is an unread variable, a finding' has no check
status: open
opened: 2026-10-23
priority: P0
cost: M
design: true
refs: [assemble-is-an-operator-and-the-product-is-a-slot]
---

Found by D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`), building test 26 (`docs/INTENT-STAGE3-SPEC.md` §11, row 26: "an unread body is a finding and is not exported").

`docs/DESIGN.md` `[operations]` ("The product", line 1220): "Nothing is derived from which bodies go unread: an unread body is an unread variable, a finding, and a document whose slot reads nothing has an empty product." D′3 builds the slot, so an unread body is not exported (`crates/editor-core/tests/intent_dprime3_assemble_product_slot.rs`, `the_product_is_what_the_slot_reads`). But no check reports it: `CheckId` (`crates/editor-core/src/checks.rs:80–115`) holds `Connectedness`, `Separation`, `ChartCoherence` and `UnprovenCoincidence`, and nothing reads which `Body` variables no reader and no slot reads.

The spec does not settle the check's shape, so this is a question rather than a build:
- Which variables count. A body nothing reads (a construction whose only reader is deleted, a placement's copy the slot does not read)? Every unread variable of any kind, as the sentence says, or bodies alone?
- Its honesty label and default severity (DS6): it is exact, so certified, but is it `Warn` or `Off` by default? A sketch kept beside the product for reference is unread on purpose.
- Its subject: the variable, or the node defining it.

**Re-priced P0 (2026-10-30).** The sentence is D10 text, ratified in `[ev]` D′ (commit 679b4abbf7, 2026-10-22), so D10 is not built until the check exists. D10's umbrella now waits on this row. It is `design: true`, so the designer pair weighs it first.
