---
id: an-exact-length-selector-misses-a-loft-walls-sub-arc-pieces
kind: issue
title: An exact-length selector over a loft wall or rim (seg(tag(LoftWall)), seg(tag(RimEdge).side(Start))) misses the wall's sub-arc pieces, whose names carry a trailing SubArc(q)
status: open
opened: 2026-11-23
priority: P2
cost: E
---

Found by LATHE's review of `lathe/skin-sub-arc-walls`
(`work/lathe/the-skin-cuts-an-arc-wall-at-each-sub-arc-joint.md`).

## What happens

The loft now cuts a segment's wall at each sub-arc joint and names every
piece of a cut segment after the segment, with a trailing segment:
`[LoftWall(..), SubArc(q)]` for the wall, rim and section edge of piece
`q`, counting from zero (`crates/editor-core/src/names/emit_sweep.rs:347`,
`cut`; the tail is `RoleSeg::SubArc`,
`crates/editor-core/src/names/role.rs:1265`). A segment of one piece keeps
its one-segment name, so a polygon loft's names and an arc of up to 95.6°
are unchanged.

A `NamePat` path constrains the role path to EXACTLY its length
(`crates/editor-core/src/names/select.rs:832`, `pats.len() !=
name.path.len()`), and `NamePat::seg` is a path of one segment
(`select.rs:814`). So `NamePat::of_kind(Face).seg(tag(LoftWall))`, which
read "every loft wall" before, now matches only the walls of segments of
one piece, and none of a cut segment's; `seg(tag(RimEdge).side(Start))`
misses the same rims. pncad-py's builder is the same door
(`crates/pncad-py/src/py/select.rs:578`). On a lofted circle of two half
arcs that selector read 2 walls; it reads none of the 4 now.

## Fix

A design question for the naming slate: either a one-segment pattern
matches a name whose extra segments are inert (`SubArc`, `SubArcJoint`
are neither name nor locator, `role.rs` `inert_seg`), or the selector
grows an explicit "and any inert tail" form, or the documentation says a
loft wall's selector spells `[LoftWall, any]`. Until then, say it in
`names/README.md` "Loft walls" beside the sub-arc names.

