---
id: blend-reach-refuses-a-bore-clear-of-a-ruled-cut-offs-sliver
kind: issue
title: blend: the reach meter refuses FaceClearance for a D-rod bore that stays inside the ball's section, about 0.009 clear of the sliver the cut-off removes
status: open
opened: 2026-10-07
priority: P3
cost: M
---


## Finding

The D-rod (`crates/sweep/tests/band_ruled_cap_ring.rs`'s
`bored_d_rod`) with a bore at `(0.21, 0.36)`, radius `0.08`, filleted on
both creases at `ROD_FILLET = 0.1`, refuses
`FaceClearance { at: <the bore's wall>, bounded: true }`, margin
`−0.00289` (`fillet3_face_clearance`). The upper crease's ball centre
is `c = (0.2, √0.12 ≈ 0.3464)`. The bore's nearest approach to the
sliver the band removes is at its right, `(0.29, 0.36)`, inside the
ball's section; the arc at that height is at `x ≈ 0.2991`, so the
wall is about `0.009` clear of the removed material. The cap meter
(`RingClearance`) passes this bore; predicate 2's reach meter
(`crates/sweep/src/blend/reach.rs`) refuses it, `bounded` meaning the
margin is an enclosure's, not the wall's own. The same bore at
`(0.20, 0.36)` carves.

Found by a grid probe over 1371 bores clear of the sliver by
≥ 0.003. This was the only refusal, and it was this one.

## Close

Find which enclosure of the band's volume reaches the bore's wall
(likely the band's bounding box or its cylinder to the sliver's
reach, rather than the sliver's own prism). Tighten it in closed form
and add the bore as a carve row at `ΔV = −2·A·L`.
