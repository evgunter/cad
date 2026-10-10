---
id: blend-reach-refuses-a-bore-clear-of-a-ruled-cut-offs-sliver
kind: issue
title: blend: the reach meter refuses FaceClearance for D-rod holes that stay inside the ball's section, 1.5e-3 to 3.1e-3 clear of the sliver the cut-off removes
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
is `c = (0.2, √0.12 ≈ 0.3464)`. The bore is `≈ 0.0031` clear of the
sliver the band removes: `0.1 − ‖(0.21, 0.36) − c‖ − 0.08 =
0.1 − 0.01687 − 0.08`, its nearest approach along the direction
`53.6°` from `c`, inside the arc's wedge and inside the ball's section.
The cap meter (`RingClearance`) passes this bore; predicate 2's reach
meter (`crates/sweep/src/blend/reach.rs`) refuses it, `bounded`
meaning the margin is an enclosure's, not the wall's own. The same
bore at `(0.20, 0.36)` carves.

Found by a grid probe over 1371 bores clear of the sliver by
≥ 0.003; this was the only refusal.

**Not only bores** (review of PR 4271, R1's
`holes_near_the_d_rods_corner_refuse_iff_they_enter_the_sliver` on
`review/4271-band-dual-4271-r1`). Five straight-edged triangular
holes, each `1.5e-3` clear of the sliver, refuse the same way at
margins `−0.0018` to `−0.0031`. Each has a chord edge at radius
`0.1 − 2e-3` from `c`, at angle `θ = kπ/18` (`k = 1..5`), half as long
as the arc's chord there, and its third vertex `0.02` from `c` along
`θ`.

## Close

Find which enclosure of the band's volume reaches the bore's wall
(likely the band's bounding box or its cylinder to the sliver's
reach, rather than the sliver's own prism). Tighten it in closed form
and add the bore and the five holes as carve rows at `ΔV = −2·A·L`.
