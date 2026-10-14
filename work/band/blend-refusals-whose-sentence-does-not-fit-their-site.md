---
id: blend-refusals-whose-sentence-does-not-fit-their-site
kind: issue
title: blend: a face that cannot be boxed refuses BodyNotIntact, and a missing certified carrier refuses UnsupportedGeometry with a shape recourse
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## Finding

Two blend sites whose refusal sentence does not fit what they met, left by PR 5021's classification:

- `reach.rs`, after `FaceBoxes::of(body, band)`: the box builder refuses a cylinder carrier whose axis has no decided length, and the site renders it `BodyNotIntact`/`Breach::Disagrees` ("a face's certified box"). A degenerate stored surface is neither a key, a walk nor a structural disagreement; it is an undecidable geometry read, closer to `UnsupportedGeometry` or a certification refusal.
- `UnsupportedGeometry` appends `FILLET3_GEOMETRY_RECOURSE` (plane, cylinder, cone, sphere supports with line and circle edges) to sites that met an edge with NO certified carrier (`surgery.rs` `stored_piece`/`metered_piece` reads, `open/end_face.rs` "an end face's rim carries no certified carrier"): the recourse does not address a missing certificate.
