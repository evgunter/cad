---
id: a-wall-holding-two-pierces-refuses-at-its-own-ring
kind: issue
title: A cylinder wall a ruling pierces twice refuses HeldPierceUnbuilt at the ring its own first held pierce minted
status: dispatched
branch: tang/two-held-pierces-one-wall
opened: 2026-10-15
priority: P3
cost: M
refs: [a-pierce-point-inside-a-curved-face-of-the-piercing-operand-refuses-unbuilt]
---

## What

Found by the full review of PR 5155.

**The pose.** The rod joined with a square prism whose edge is one of the rod's rulings, against a slab whose top and bottom that edge pierces, both inside the rod's wall. The slab spans z ∈ (0.8, 1.2).

**What happens.** Every op refuses `HeldPierceUnbuilt { placed: false }`, either operand first, at all three turns (`crates/sweep/tests/a_pierce_held_in_a_curved_face.rs:662`, `a_wall_the_ruling_pierces_twice_refuses_typed`). A slab the ruling pierces once, at its top only or at its bottom only, builds.

**The mechanism (reviewer: likely).**

1. The first held pierce mints a ring vertex in the wall (`vtxfac::ring_vertex_in`, `crates/topo/src/boolean/vtxfac.rs:1390`).
2. The second point's trim read then meets a ringed cylinder face.
3. Both of the following read a ringed cylinder face as unread, so the second point refuses as not placed:
   - `curved_interior_placement` (`crates/topo/src/boolean/contain.rs:945`);
   - `wall_outline` (`crates/topo/src/boolean/solid_contain.rs:1412`).

The refusal is typed, and main refused this pose too, since main refused every curved held face. Its `Display`, "cannot read that face's outline there", is true, but the Boolean put the ring there itself.

## The shape to give

Two possibilities:

- read every held pierce of a face before minting any of them;
- read the trim of a face whose only rings are the held pierces' own lone ring vertices, which bound no area.

## Siblings that may share the root (unmeasured)

The same ringed-wall reading also stops two other poses:

- **The pocketed wall.** The pocket's ring makes it refuse `placed: false` (`a_ringed_wall_holding_the_pierce_refuses_typed_and_pocketing_last_builds`, line 597). That ring is a real hole, so the refusal is honest there.
- **The lying cylinder over its whole span.** Folds that ring its wall before the second prism comes refuse `CurvedPierceUnsupported` (`a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`, line 536).
