---
id: a-cusp-on-nested-near-concentric-carriers-validates-along-a-long-eps-stretch
kind: issue
title: a tangent cusp on nested near-concentric carriers validates while its two arcs run within ε of each other for a quarter radian
status: open
opened: 2026-10-15
priority: P0
cost: M
---



Found by the review of #5175, whose fix pass closed the crossing half
of the same witness (`an-internal-tangency-reads-its-spans-at-the-radical-foot-off-both-arcs`).
The same on `main` (the PR's merge base `70ec028a81`), where `g = 0` also validates at 1e-9.

## The witness

One loop: an arc of the unit circle about the origin from angle −2
into the joint `v = (1, 0)`, an arc of the circle about `(d, 0)` of
radius `1 − d + g` out of `v` back through −1.6 rad, and a line home
(`rejections.rs` `joint_loop`, with `at = 0`, `sweep = −1.6`).

| d | g | ε | f64 and Interval |
|---|---|---|---|
| 17ε | −0.5ε | 1e-9, 1e-6, 1e-12 | validates |
| 17ε | 0 | 1e-6 | validates |
| 0.1 | −0.5ε | 1e-9, 1e-6, 1e-12 | validates (second arc back through −0.5 rad) |

The carriers are nested (`g < 0`) or tangent at `v` (`g = 0`; at 1e-9
and 1e-12 the `f64` radius rounds `g` to a few ulps, which reads
`carrier_circles_chord` in band at 1e-9 and a crossing 3.6e-3 rad out
at 1e-12, both refused). They stand `|g − d(1 − cos θ)|` apart at angle
`θ`, so the two arcs never meet but at the joint, and stay within ε of
each other out to `√(2(ε + g)/d)`: 0.24 rad at `g = −0.5ε`, 0.34 rad
at `g = 0`, at every ε. The joint pass records a tangent cusp; the pair
pass reads the tangency at the joint (`carrier_circles_chord` Zero: a
foot outside both circles reads zero depth), and the shared vertex
discounts it. At `d = 0.1` the arcs stay within ε for `√(ε/d)` rad
(1e-4 rad at ε = 1e-9), a tangent cusp's ordinary stretch.

## What is wrong, and what is a question

Nothing here crosses: every point the two arcs share in the reals is
the joint. What the profile does carry is two boundary arcs within ε
of each other, a quarter of a radian from their shared vertex, which
the pair pass refuses between any two segments that are not adjacent.
An ordinary tangent cusp has the same shape at a smaller scale: its
arcs stay within ε for about `√(2ε/Δκ)`, and `end_in_stretch`'s doc
accepts that as the joint's own. Where that stretch stops being the
joint's is the open part: the fix pass's `carrier_circles_chord` draws
the line for a crossing (a chord deeper than the band is two
crossings, refused), and nothing draws one for a sliver that never
crosses. Banded P0 as the review asked; a ruling that a tangent cusp's
ε-stretch may run to any length makes this a contract note, not a
defect.

## Remedy, sketched

A bound on the stretch, read where the joint pass or the pair pass
already has the carriers: refuse a tangent cusp whose carriers stay
within ε further than the band of a point tangency reaches (the
stretch `√(2(ε + |g|)/d)·r` against `√(2r·Kε)`, or the radical foot's
depth `|g|·r/d` against Kε, which the fix pass already computes). A
smooth tangent joint on the same carriers meets only at the joint and
must keep validating (`a_cusp_on_near_concentric_carriers_refuses_its_crossing`'s
smooth rows). Touching how the joint pass records or verifies a tangent
joint is D10's hold.
