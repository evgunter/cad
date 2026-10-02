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

## A second residue: attaching by the entry vertex alone

Found in review of PR 3753 (re-review, 2026-10-02).

**The gap.** `discard::HeldInto` gives a held stretch to any discarded fragment that holds the edge's entry vertex `at`, or a null-edge copy of it. A fragment can touch `at` without bordering the stretch.

**Where it breaks.** At a reflex vertex, a kept sector of the dropped face can lie between the covered sector and a lost one (covered Q1 | kept Q2 | lost Q3). The lost Q3 fragment would then join the covered region's obstacle through the point `at` alone. That is the M1 failure, in miniature.

**Why nothing hits it today.**
- With axis-aligned boxes, at most three sectors meet at such a vertex, so the rule is exact.
- The breaking shape tried was an L-shaped `a`, `b` = [.5,1]×[.5,1]×[0,1], and `H` = [.3,.7]×[.3,.5]×[.5,2]. The kernel refuses it with `ClassificationInvariant` ("edge-edge membership disagreement"), the reflex-wedge limit documented on `resolve_edge_edge`.

**When to revisit.** Before that limit is lifted, or before non-axis-aligned planar covered pairs are admitted, the attach rule should test "borders the stretch", not "touches `at`". The probe source is in the reviewer's notes: `probe_l_vertex.rs`.
