---
id: viewer-product-badge-speaks-the-node
kind: unit
title: The product badge and the scene's NoProduct speak the gather's refusal from the session's document
status: review
pr: 3806
opened: 2026-10-02
priority: P3
cost: E
parent: node-labels-are-document-data
refs: [viewer-refusals-speak-the-node]
---


Split from `viewer-refusals-speak-the-node`, parked on PR 3794 (`product-refusals-speak-the-node`) until `ProductError` had `spoken(doc)`. 3794 merged while PR 3806 was in review, and the two reads were small, so 3806 carries them:

- `frame::product_badge(fault, doc)` speaks the gather's refusal from the landed document it was gathered from (`DocSession::landed_pair`), not the committed one: the fault belongs to the landed run, and the committed document may be one an `Open` replaced since.
- `scene.rs`'s `SceneError::NoProduct` holds the `ProductError` beside the `HeldNodes` its sentence names (`held_by`, at `scene::product_body`/`product_of_evaluation`, where the document is in hand). `frame::scene_badge` keeps it until a rebuild succeeds; every landing rebuilds, so a rename that lands replaces the sentence.

Pinned by `node_labels::the_gathers_refusal_speaks_its_nodes`.
