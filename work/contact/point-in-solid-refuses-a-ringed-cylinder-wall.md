---
id: point-in-solid-refuses-a-ringed-cylinder-wall
kind: issue
title: point_in_solid's wall outline refuses any ringed cylinder wall, so a later union member's containment probe stops on a pierced boss
status: open
opened: 2026-10-02
---


## What

`boolean::solid_contain::wall_outline` answers
`WallOutline::Unsupported { reach: None }` for any cylinder wall face
that carries a ring (`if !f.rings.is_empty()`), so a containment probe
whose ray hits such a wall anywhere it could be inside refuses
`PointInSolidError::WallOutlineUnsupported`.

Ringed cylinder walls are now ordinary boolean output: a pierce ring
whose section closes inside one wall face leaves the wall with a hole
(`work/tang/pierce-ring-has-no-join-arm.md`, closed by
`tang/pierce-ring`). Measured on that branch: editor-core's
`reach_slab_cut_sector_side`, `a_slab_across_a_round_boss_answers_its_volume_in_every_order`
— the plate `[0,3]×[0,2]×[0,1]`, a three-face boss `r = 0.6` about
`(1.5, 1)`, `z ∈ [0.44, 2.24]`, and a slab `x ∈ [1.4, 1.6]`,
`y ∈ [−1, 3]`, `z ∈ [0.5, 2]` unioned in every member order. The
two orders that union boss and slab first (`[boss, slab, plate]` and
`[slab, boss, plate]`) leave the boss wall ringed, and the plate's
containment probe then refuses
`Boolean(Containment(WallOutlineUnsupported))`; the other four build
to the closed form `8.163522747862315`. The row pins exactly that
split, so it reds when the outline reads rings, and should then hold
every order to the volume.

## The shape of a fix

The outline's parity argument (`wall_outline`'s docs, "Why parity
along the ruling is exact") holds loop by loop: a ring is one more
simple closed curve of the same pieces inside the face's window, and
an upward ruling ray from a point crosses the face's boundary an odd
number of times iff the point is in the face, rings included. So the
pieces and junctions could be gathered over every loop rather than
the outer one.
