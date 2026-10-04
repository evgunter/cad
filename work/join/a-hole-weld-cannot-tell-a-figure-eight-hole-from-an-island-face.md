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
