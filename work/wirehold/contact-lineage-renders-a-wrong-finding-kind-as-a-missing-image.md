---
id: contact-lineage-renders-a-wrong-finding-kind-as-a-missing-image
kind: issue
title: editor-core: ProductError::ContactLineage renders a wrong at-rest finding kind as 'the key bridge has no image for it'
status: parked
opened: 2026-10-13
priority: P4
cost: E
blocked_on: [the-at-rest-census-is-a-check]
---


## Finding

`ProductError::ContactLineage`'s `Display` (`crates/editor-core/src/product.rs`) says the graft's descendant map "has no image for" `{what}` — the key bridge is incomplete. The face sites fit; the site raising "an at-rest finding the census does not raise for a pair" is a wrong finding kind, not a missing image, and the sentence reads ungrammatically there.

Same shape as band's `BodyNotIntact` (PR 5021). Found by that PR's §5 sweep.

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. The sentence is still wrong, but stage 5 C restates the at-rest refusal as a check and stage 4 I retires the declared-record carries that share `ContactLineage`; probably moot. Parked on `the-at-rest-census-is-a-check`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).
