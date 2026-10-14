---
id: contact-lineage-renders-a-wrong-finding-kind-as-a-missing-image
kind: issue
title: editor-core: ProductError::ContactLineage renders a wrong at-rest finding kind as 'the key bridge has no image for it'
status: open
opened: 2026-10-13
priority: P4
cost: E
---


## Finding

`ProductError::ContactLineage`'s `Display` (`crates/editor-core/src/product.rs`) says the graft's descendant map "has no image for" `{what}` — the key bridge is incomplete. The face sites fit; the site raising "an at-rest finding the census does not raise for a pair" is a wrong finding kind, not a missing image, and the sentence reads ungrammatically there.

Same shape as band's `BodyNotIntact` (PR 5021). Found by that PR's §5 sweep.
