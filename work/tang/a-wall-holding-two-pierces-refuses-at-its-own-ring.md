---
id: a-wall-holding-two-pierces-refuses-at-its-own-ring
kind: issue
title: A cylinder wall a ruling pierces twice refuses HeldPierceUnbuilt at the ring its own first held pierce minted
status: closed
branch: tang/two-held-pierces-one-wall
opened: 2026-10-15
closed: 2026-10-15
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

## Closed (2026-10-15, TANG)

**The mechanism, confirmed.** When the second point is read, the wall's one ring is the first held pierce's strut loop: two null halves. `curved_interior_placement` reads any ring as unread.

**The fix: every pierce's held faces are read before any is minted.**

- `vtxfac::faces_holding_pierce` now takes every plan of the operand. It reads each one (`held_reads`), writing nothing, and only then mints the ring vertices (`mint_held`).
- This keeps one reader of whether a face holds a point: the trim, at rest, as the operand gave it. The other shape would have taught both trim readers that a ring of null halves bounds nothing, a second notion of the face's region beside the one the readers already hold.
- The order of the minted pierces is unchanged.

**The row.** `a_wall_the_ruling_pierces_twice_builds_in_every_op_and_order` builds:

- every op, either operand first;
- both join orders;
- every member order of the fold, carried;
- at the three turns.

Each passes tier 3 and tier 3′, matches its closed-form volume, and matches the material at the probes.

**What still refuses.** The pocketed wall still refuses `placed: false`: its ring is a real hole.

**`wall_outline` (`solid_contain.rs:1412`) is unchanged.** The mechanism named it as a second ringed-wall reader. The fix keeps the scaffolding ring away from both readers instead of teaching either of them about it, so neither changes.

**The rest of the pass:**

- Two pierces of one face at one point refuse `HeldPierceUnbuilt { placed: true }` before anything is written. No row reaches the pose, so no answer to it is checked. In band of one point, they escalate `VertexOnVertex`.
- A definite refusal at any face of any pierce outranks an escalation at another, as in the pierce pass this serves.

**The lying-cylinder sibling** has a different root, and this does not close it. Over the cylinder's whole span, the folds that ring the wall before the second prism comes still refuse `CurvedPierceUnsupported`. There the ring is the first prism's footprint, a real hole in the wall at rest.

## Note (2026-10-24, CHART)

Both siblings above now build. PR 5664 reads a cylinder or cone face's
region from its edges, rings included
(`topo/src/boolean/ruled_region.rs`). The pocketed wall's row is now
`a_ringed_wall_holding_the_pierce_builds_either_operand_first_and_pocketed_last`.
The lying cylinder over its whole span builds in every member order of
`a_plane_face_holding_the_pierce_over_a_cylinder_builds_in_every_op_and_order`,
and each such body is sound against the row's closed forms.
