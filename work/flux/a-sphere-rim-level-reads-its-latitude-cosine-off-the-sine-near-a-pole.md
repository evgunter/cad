---
id: a-sphere-rim-level-reads-its-latitude-cosine-off-the-sine-near-a-pole
kind: issue
title: The sphere's linear boundary completes a latitude level's cosine as sqrt(1 - s^2), which carries about sqrt(2u) near a pole, so a rim point there sits about r*sqrt(2u) off
status: closed
pr: 5375
closed: 2026-10-19
opened: 2026-10-13
priority: P3
cost: E
---

Found by the class sweep of `pcert/cone-section-interval-beta` (PR
5004), which fixed a `√(1 − x²)` read near `x → 1` in `chart_pcurve`'s
cone arm. Not measured here; this is the arithmetic, and whether it
reaches a verdict is the owner's to say.

## What happens

`sphere_boundary` (`geom-brep/src/props/curved.rs`) returns a
`LinearBoundary` whose `as_level` completes a latitude level `s` (a
sine) with its cosine as `√(1 − s²)` (`.max(0)` keeps the root in
domain; `props/curved.rs:3865`). Near a pole, `s → ±1`, and `s`'s own
rounding `u` moves `1 − s²` by about `2u`, so the cosine by about
`√(2u)` (2e-8), and a rim point at that level by about `r·√(2u)`
horizontally: 2e-8 at `r = 1`, twenty times ε = 1e-9. The cone arm's
version of this gained `1/k`; this one is the root's own `√u` floor,
because the latitude is carried by its sine.

## What closes it

Either carry the level as the `(sin, cos)` pair from where the extreme
is found (the fold that produces `s` has the geometry to give the
cosine directly), or show that every consumer of `RimLevel::Unit`
near a pole is insensitive to a `√u` error in the cosine, with a row
at a level within `√u` of a pole.

## Resolution (FLUX, `flux/e-bundle-lane-honesty`)

**It reached a verdict**: the iso shape door's (`require_iso_rectangle`),
not the flux lane's. On the unit sphere, the window `[0, 1] × [v₀, v₁]`
between a rim of radius 0.5 and a rim `kε` from the axis has its upper
rim at the face's extreme level. `require_rims_at_extremes` compares the
rim's `(sin v, r_c/R)` with the extreme lifted as `(s, √(1 − s²))`,
whose cosine was off by:

| ε | k = 10.5 | 11 | 15 | 30 | 100 | 1000 |
|---|---|---|---|---|---|---|
| 1e-9 | 4.4e-9 | 3.9e-9 | 9.9e-11 | 2.0e-10 | 4.0e-11 | 4.4e-11 |
| 1e-12 | 1.05e-11 (all of r_c) | 1.1e-11 | 1.5e-11 | 3.0e-11 | 1.0e-10 | 1.0e-9 |

So `props_rim_level` refused (`NotIsoRectangle`) or escalated legal
windows: at 1e-9, k ≤ 11 at f64 and k ≤ 100 at the interval scalar; at
1e-12, every k through 1e4 at both. At 1e-6 the error stays under the
band. The flux lane measured every one of them all along.

**Fix:** `LinearBoundary` carries each sphere level's cosine beside it,
read where the level is found (a rim's `r_c/R`, a meridian end's
distance from the axis over `R`, a pole's zero), and
`LinearBoundary::ends` lifts the extremes with the cosine of the level
at each, selected by sine. The interval scalar hulls the cosines of
levels whose sines it cannot tell apart, which near a pole are one
latitude's. The levels' sines, and so every other reading, are
unchanged.

Row: `s58_iso_rectangle::a_sphere_window_with_a_rim_near_a_pole_is_an_iso_rectangle`
(k ∈ {10.5, 11, 15, 30, 100}, f64 and interval), red on main at 1e-9
and 1e-12. The mutant that completes the cosine from the sine is that
red. The digest does not move.

