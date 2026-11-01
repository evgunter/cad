---
id: the-products-recourse-names-a-door-only-a-tree-click-reaches
kind: issue
title: The product's recourse says 'set the product' and its only door is on a Place row the viewport never selects
status: open
opened: 2026-10-31
priority: P1
cost: M
---


Reported by Ev (chat, 2026-10-31): the viewer said "product: the
product reads a body that is gone. Recourse: set the product to a live
body", and Ev could not find how to set the product.

## What happens

The sentence is `ProductError::StrandedProduct`'s
(`crates/editor-core/src/product.rs:359`; `EmptyProduct` and
`ProductOutsideWorld` beside it, `:352` and `:365`, name the same
recourse). Its recourse is text: no affordance comes with it.

The one door that sets the product is `product_ui`
(`crates/viewer/src/pane/properties.rs:591`), whose "Add to the
product" button (`:602`) is drawn only when the selected node is a
world copy — a `Place` into the world or an instance
(`crate::world::world_copy`, `crates/viewer/src/world.rs:294`).

The selection it reads is `Selection::node`
(`crates/viewer/src/session/select.rs:205`), which for a viewport face
or edge pick answers the FEATURE that minted the entity
(`FaceSelection::feature`, `:64`) — for a name whose minting is
known, the extrude, not the `Place` that `create_placed`
(`crates/viewer/src/session.rs`) inserts beside it. So clicking the body you can see never shows the button. It shows
only after a tree click on the `Place …` row, which a person has no
reason to know is the copy the product reads.

Nothing adds a new body to the product either: `create_placed` places
it in the world and leaves the product slot alone (the slot is written
only by `DocEdit::SetProduct`, `crates/editor-core/src/edit.rs:8668`),
so a deleted product body strands it for good, and a document with no
product raises `EmptyProduct`, both with the same unreachable recourse.

## What would close it

Some reachable door from the place the sentence is read: the
recourse offered as an action (the standing line naming the world
copies it could read, or "set the product to the selected body"),
and/or `product_ui` drawn for a viewport pick by reading the drawn
copy (`Selection::seat_node`, the node whose drawn body the ray met)
rather than the minting feature. Which of those is the design is
CHROME's call; whether a document's first body should join the product
on creation is a D10 `[operations]` question and goes to Ev.
