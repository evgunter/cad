---
id: a-cone-with-its-apex-against-its-copy-refuses-normal-at-cone-apex
kind: issue
title: A cone with its apex, booleaned with an identical copy, refuses NormalAtConeApex at the apex vertex before the On verdict
status: open
opened: 2026-10-29
priority: P2
cost: M
---


Found by the sweep on `splitdoor/curved-copy` (the class: a curved body against a coincident copy).

## What

The triangle `(0,0) (1,0) (0,1)` revolved fully about `y` (a cone with its apex on the body) against an identical copy refuses every op, both orders, at ε = 1e-9:

`NormalAtConeApex { operand: A, face: FaceKey(1v1), .. }`

The cone faces are settled one carrier with their copies (the carrier ladder's cone arm), so the boolean reaches the vertex classification. There the apex of A lies on the apex of B, and the sector lane asks the cone face's tangent plane at that vertex (`boolean/sectors.rs`, the `VertexFace::Apex` arm), which the apex does not have. A frustum, a ring frustum and the lantern against their copies build (`crates/sweep/tests/curved_copy_against_itself.rs`), so the apex vertex is what refuses.

Probe: `curved_copy_against_itself::curved_copy_sweep` (ignored), the "cone with apex" row.

## Where to look

Whether a vertex lying on a vertex of the copy, every face about each settled one carrier with a face about the other, needs a sector reading at all, or can be answered by the `On` verdict as the section pass's settled pairs are (`boolean::ops::OnDeferred`). Sibling row: `boolean-readers-at-a-cone-apex-vertex-escalate-under-their-own-reading`.
