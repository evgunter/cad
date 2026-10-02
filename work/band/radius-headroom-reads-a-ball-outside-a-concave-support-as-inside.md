---
id: radius-headroom-reads-a-ball-outside-a-concave-support-as-inside
kind: issue
title: blend: fillet3_radius_headroom refuses r >= R on a hole's wall, where the ball rolls outside the cylinder and its curvature sets no limit
status: open
opened: 2026-10-02
priority: P2
cost: M
---


## Finding

`battery::radius_headroom` (`crates/sweep/src/blend/battery.rs`) meters
`(1 − r·κ_max)·r` for each support, whichever side of the support the
rolling ball sits on. That is the right limit when the ball rolls on
the CONVEX side of the support's material — inside a rod, centre at
`R − r` — and no limit at all when it rolls on the concave side: on a
hole's cylindrical wall the ball's centre is at `R + r` from the axis,
the wall's nearest point to it is the foot, and the wall's curvature
cannot interfere with the ball at any `r`.

Witness, the tour's `rocker` (`demos/tour/src/rocker.rs`,
`crease_narration`, wall 1): a keyhole through a plate — a disc of
`R = 1/2` and a slot of half-width `1/5` — has two CONVEX creases where
the disc meets the slot walls. `fillet_edges` on them refuses
`RadiusHeadroom` on the disc's wall face at `r = 0.5` (margin `0`,
decided Zero) and `r = 0.55` (margin `−0.055 = (1 − r/R)·r`). The ball
fits geometrically: its foot on the slot wall is at
`x = √((R + r)² − (w + r)²) = 0.714` from the disc's centre, inside the
slot's `0.8`, and the band's own closed form (`crease_cut` in the
scene; `review_band_ruled_ring_probes::keyhole_cut`) is defined there.
The scene pins the refusal live as `walls::wall("rocker", 1, ..)`.

## What the taker owes

The headroom read sided by the ball: only a support whose material is
convex toward the ball (the ball inside its curvature) limits `r`.
Then the rocker's wall 1 either flips (and the scene rounds its
creases at the outline's blend radius) or meets the next honest
refusal — at `r = 0.5` that is the cap meter's enclosure
(`ruled-cut-off-builds-a-bore-wholly-inside-the-removed-sliver`,
point 2), which refuses this keyhole from `r ≈ 0.32`.
