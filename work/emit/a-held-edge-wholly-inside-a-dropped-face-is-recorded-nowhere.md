---
id: a-held-edge-wholly-inside-a-dropped-face-is-recorded-nowhere
kind: issue
title: A kept face's edge that lies wholly inside a dropped covered face, both ends at vertex-on-face contacts, is held by no discard row
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

Found while building PR 3753 (emit lane, 2026-10-02). The record is `DiscardRow::held` (`crates/topo/src/boolean/discard.rs`). It is read only where `recl::recl_sectors` meets a covered pair at a vertex both operands hold. The discarded fragment is identified by holding that vertex, or a null-edge copy of it (`HeldInto::stretches`).

At a vertex-on-face contact (`vtxfac::classify_vertex_on_face`), the dropped face holds no vertex at the contact. That leaves nothing topological to tell its fragments apart, so the contact records no held edge.

An edge of the kept face that reaches the dropped face's boundary is recorded where it does. An edge whose two ends both lie inside the dropped face is recorded nowhere. An example: the floor of a pocket in the kept body, lying inside the dropped face, with walls rising from it.

`Obstacles` then sees the dropped face's lost region as bordering nothing along that edge. The obstacle may fail to join a neighbouring discard, so a piece may refuse (`a piece of a face held as several borders no recorded discard between them`) or be named apart from other orders.

No corpus case reaches this. The notch the slab cuts in `b`'s wall in `near` has its bottom edge at two vertex-on-face contacts, but the notch's x 0.501 edge is recorded at a vertex–vertex contact, and that joins the obstacles.

## Next

1. Build the pocket case and measure it.
2. If it refuses or names by order, decide how a fragment is told at a vertex-on-face contact, for example by the pierce ring vertex where surgery inserts one. That is a design question if topology alone cannot say.
