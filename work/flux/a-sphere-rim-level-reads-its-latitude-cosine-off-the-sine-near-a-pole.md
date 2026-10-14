---
id: a-sphere-rim-level-reads-its-latitude-cosine-off-the-sine-near-a-pole
kind: issue
title: The sphere's linear boundary completes a latitude level's cosine as sqrt(1 - s^2), which carries about sqrt(2u) near a pole, so a rim point there sits about r*sqrt(2u) off
status: open
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
