---
id: a-hole-weld-cannot-tell-a-figure-eight-hole-from-an-island-face
kind: issue
title: Joint::Hole reads only 'one ring holds both copies': it cannot tell a figure-8 hole from an island face pinched inside the hole, and weld_pinches now takes it on an unreached path
status: open
opened: 2026-10-04
priority: P2
cost: M
---


## What

Found by PR 4026's dual review (r2 S8, r1 S8).

`boolean/finish.rs` `pinch_site` returns `Joint::Hole` when one ring
of a face holds both vertices. `zip::fuse_by_joint` then `mef`s across
the ring, `kev`s the zero-length joint, and `kfmrh`s the divided-off
face back into the face as a ring. That is right for a figure-8 hole:
two holes that meet at the vertex. But the reading cannot tell that
case from an island face pinched off inside the hole, whose outer loop
the ring also passes at the vertex. There, the `kfmrh` would turn a
face's outer loop into a ring. Review r2's holed-block island poses
never reached it, because the copies landed on separate faces.

`weld_pinches` reaches `pinch_site` too. A ring holding both of two
pierces' copies used to take a `Chord` there (a face made of a hole,
`LoopRoleInverted`). It now takes `Hole`. No row reaches that path in
`weld_pinches`, so the change is unmeasured.

## The shape to give

Decide the ring's sides at the vertex before welding: whether the
divided-off loop bounds material (an island) or a void (a hole). Read
that from the loop's winding about the face normal, as
`join::ring_run_ccw` does. Then build a row that reaches `Hole` in
`weld_pinches`, and one that reaches an island.

## The split direction builds the island shape (PR 4051)

The pinch pre-pass (`zip::split_across`) now crosses an island face
pinched to its hole's ring by `kef`: the island dies into the holed
face, whose ring then passes the point twice. So the island poses that
"never reached" `Joint::Hole` do reach that shape now, by the other
route. Witness: `join_pierce_runs_sweep::an_island_face_pinched_to_its_holes_ring_crosses_and_builds`.
This row's question is unchanged for the weld direction.

## Ruling (Ev, PR 4057, 2026-10-05)

A pinch is one vertex per cone: several vertices on one point key, and
no face crosses between cones. See
`work/join/a-pinch-no-kept-face-can-cross-refuses.md`, "The shape to
give". This row is settled by that unit.
Under the ruling, no loop is split and re-roled by its winding. The welds retire, so the question dissolves. The row closes when the pinch unit lands.
