---
id: a-product-faults-recourse-cannot-be-said-apart-from-its-story
kind: issue
title: a product fault's recourse cannot be said apart from its story, so the toolbar prints it beside the product door's controls
status: open
opened: 2026-11-23
priority: P3
cost: E
---


Found by OFFER's offers unit (`work/offer/a-refusal-offers-no-action-in-the-viewer.md`, item 4: where an offer is drawn as a control, the message's prose recourse for that same act is not printed again beside it).

For an empty, stranded or out-of-world product the viewer draws the product door as controls beside the toolbar's badge ("Add <body> to the product", "Move <body> into the world"; `viewer::recourse::offers`, `product_offers`). The badge is `ProductError::spoken` (`crates/editor-core/src/product.rs`, `fmt_said`), whose `EmptyProduct`, `StrandedProduct` and `ProductOutsideWorld` arms end on `Recourse("set the product to a body in the world")` / `("… a live body")`, the act the controls are.

There is no form of the sentence without that clause, so the viewer prints it beside the controls. A `spoken_beside_its_recourse(doc)` for those three arms (the story alone, its stage word kept) would let the badge say it once; `CheckFinding::said_beside_its_assertion` is the shape.
