---
id: sphere-rim-level-cosine-cancels-near-a-pole
kind: issue
title: A sphere rim level's cosine is recovered as √(1 − s²) and cancels near a pole
status: open
opened: 2026-10-06
priority: P3
cost: M
---


## What

`sphere_boundary` (`crates/geom-brep/src/props/curved.rs:3803`) lifts
each folded latitude SINE `s` to the direction pair
`RimLevel::Unit(s, √(1 − s²))`. `1 − s²` cancels as `s` nears `±1`: `s²`
rounds by `u` absolute, so the recovered cosine is off by about
`u/(2·cos v)`, and below `cos v ≈ 1e-8` it is lost outright. The pair
reaches `level_gap` (`:1200`), whose chord `√(Δs² + Δc²)` is levered by
the sphere radius (`:1190`) and decided against the band: a rim within
about `1e-8` rad of a pole on a sphere of radius `1e3` carries up to
`1e-5` m of rounding in its level, against a band of `1e-9` m.

This is the class `boxes::sphere_reach` and `boxes::perp_room` were
fixed for on PR 4123 (the share of a unit vector recovered by
`√(1 − x²)`), but here it is a decision meter, not an extent: a wrong
gap refuses or admits a rim-at-extreme match rather than tightening a
box. It is filed rather than fixed there because the fix is not a
re-spelling of one line: the sine has already lost the cosine's bits
by the time it is folded. The cure is the one
`solid_contain::sphere_chart_trim` uses, carrying each level as its
exact `(axial, radial)` pair from the geometry (a rim's own radius, a
vertex's own radial norm) so neither is recovered from the other.

## Done when

A row with a rim `1e-9` rad off a pole on a sphere of radius `1e3`
decides its rim-at-extreme match the same as the same rim posed with
the pole on a coordinate axis, at ε 1e-9 and 1e-12.

## Re-homed from FLUX to FLUXTAIL (2026-10-10)

(FLUX orchestrator) FLUX measured 125.5 budget points against 30 and was cut on its priority seam: FLUX kept the curved closed-form arms. FLUXTAIL collects the curved arms' numeric honesty rows: rim levels, chord lengths, span folds and bounds that cancel, drop a refusal or depend on the frame, under FLUX's closed forms. The id and the body above are unchanged; the move may have set `priority`, `cost` or `status` in the header.
