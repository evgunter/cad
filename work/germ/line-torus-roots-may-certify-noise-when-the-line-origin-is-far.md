---
id: line-torus-roots-may-certify-noise-when-the-line-origin-is-far
kind: issue
title: The line × torus and ray × torus quartic ladders lever by R+r and may certify f64 noise when the line's parameter origin sits far from the torus
status: open
opened: 2026-09-29
priority: P1
cost: M
---


## What

PR 3375's delta review measured the circle × torus half-angle quartic certifying f64 rounding noise at large circle radius, with the lever at R+r. The coefficients scale as ρ⁴, so the harmonics' rounding noise, about ρ⁴·u, outgrows the band: 96 of 1500 dip/bump poses were wrong at ρ=100 m for R=1, r=0.25. The fix was a noise meter in `half_angle_roots`.

The line × torus quartic (`line_torus_roots`, used by `wall_crossing` since PR 3265) and the ray × torus quartic (`point_in_solid`'s torus arm, PR 3255) share the Ferrari ladder (`depressed_quartic_roots`). If a line's parameter origin, or its direction scale, sits far from the torus, the same coefficient growth may apply. That is unmeasured.

## Measure

Take lines and rays whose parameter origin is 10–1000 m from a torus with R=1, r=0.25, passing within ±[1e-8, 1e-5] m of the tube, and compare against an exact-ish oracle. Include `point_in_solid`, whose ray origin is the query point. If any answer is certified wrong, raise this item to P0 and give both lanes the same noise meter, or recenter the parameter at the foot nearest the torus.

## Home

GERM, beside `circle-crosses-a-torus-face-with-no-root-lane` (PR 3375).
