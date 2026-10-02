---
id: viewer-product-badge-speaks-the-node
kind: unit
title: The product badge and the scene's NoProduct speak the gather's refusal from the session's document
status: parked
opened: 2026-10-02
priority: P3
cost: E
blocked_on: [3794]
parent: node-labels-are-document-data
refs: [viewer-refusals-speak-the-node]
---


Split from `viewer-refusals-speak-the-node`, parked on PR 3794 (`product-refusals-speak-the-node`), which gives `ProductError` its `Say` and `spoken(doc)`. Until it lands there is nothing to call.

Two reads, both of the gather's refusal by its `Display` (tags):

- `frame::product_badge` (`fault.to_string()`), over the session's `product_fault`. The badge is re-read every frame from session state, so it speaks from the document that fault was gathered from: the landed run's (`DocSession::landed_pair`), not the committed one, which may be an edit ahead.
- `scene.rs`'s `SceneError::NoProduct` (its `Display`, `write!(f, "{error}")`), raised by `scene::build_from`/`build_landed` from `product(doc, …)`, where `doc` is in hand. Speak it at the raise: `NoProduct` keeps the error and the `HeldNodes` its sentence names (`held_by`), as `FaceFrameFault::Unresolved` does. `frame::scene_badge` keeps the refusal until a rebuild succeeds; every landing rebuilds, so a rename (an edit that lands) replaces the sentence rather than leaving its label standing.
