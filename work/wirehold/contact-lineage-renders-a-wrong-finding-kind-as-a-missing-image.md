---
id: contact-lineage-renders-a-wrong-finding-kind-as-a-missing-image
kind: issue
title: editor-core: ProductError::ContactLineage renders a wrong at-rest finding kind as 'the key bridge has no image for it'
status: closed
opened: 2026-10-13
priority: P4
cost: E
closed: 2026-10-26
---


## Finding

`ProductError::ContactLineage`'s `Display` (`crates/editor-core/src/product.rs`) says the graft's descendant map "has no image for" `{what}` — the key bridge is incomplete. The face sites fit; the site raising "an at-rest finding the census does not raise for a pair" is a wrong finding kind, not a missing image, and the sentence reads ungrammatically there.

Same shape as band's `BodyNotIntact` (PR 5021). Found by that PR's §5 sweep.

## Parked on the D10 hold (2026-10-17, WIRE triage)

Read against the tree at `fd218851f9`. The sentence is still wrong, but stage 5 C restates the at-rest refusal as a check and stage 4 I retires the declared-record carries that share `ContactLineage`; probably moot. Parked on `the-at-rest-census-is-a-check`, the INTENT row whose landing reaches this ground, and moved to WIREHOLD by WIRE's priority-seam cut, keeping its id.

Signed (WIRE orchestrator).

## Closed

2026-10-26 (stage 5 C, `the-at-rest-census-is-a-check`, its trigger):
moot, as the parking note expected. The site raising "an at-rest finding
the census does not raise for a pair" is gone: `ProductError::ContactLineage`
is raised from `product::carry_contacts` alone, whose `what` is "vertex",
"edge", "face" or "cell pair", each an entity the bridge has no image for,
which is what the sentence says.
