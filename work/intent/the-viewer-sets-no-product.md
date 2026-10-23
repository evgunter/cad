---
id: the-viewer-sets-no-product
kind: issue
title: The viewer sets no product: SessionOp::SetProduct is built and no pane offers it
status: open
opened: 2026-10-23
priority: P2
cost: M
refs: [assemble-is-an-operator-and-the-product-is-a-slot]
---

Found by D′3 (`assemble-is-an-operator-and-the-product-is-a-slot`). A document's product is the body its slot reads (`DocEdit::SetProduct`), and nothing joins it by being placed. The viewer's session has the op: `SessionOp::SetProduct { bodies }` (`crates/viewer/src/session/op.rs:970`, applied at `crates/viewer/src/session.rs:1845`) sets the product to the one body, or inserts a `⊔` of several in the same action. But no pane or gesture offers it. A document authored in the viewer, by place-part or by any construction, has no product until a script or a test sets one, so its export refuses `EmptyProduct`.

The viewer's own scenes set their products (`crates/viewer/src/scene.rs`), and the tests do through `tests/common/mod.rs`'s `world_is_the_product`.

Shape of a fix: a product row in the properties pane (the product's body, or its `⊔`'s members) with a "make the product" action on a selected body or set of bodies. Whether placing a part's first copy also offers to set the product is a UI question for Ev.
