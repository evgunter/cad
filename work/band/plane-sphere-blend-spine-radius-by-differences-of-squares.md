---
id: plane-sphere-blend-spine-radius-by-differences-of-squares
kind: issue
title: plane_sphere_blend takes the spine radius and the rim radius as differences of squares, which cancel as the spine or rim circle shrinks
status: closed
opened: 2026-10-19
priority: P2
cost: E
closed: 2026-10-26
---


Found by the sibling sweep of `sheet-center-circle-pair-cancels-at-unequal-offsets`
(PR `band/arms-centre-placement`).

## The defect

`crates/sweep/src/blend/arms.rs` `plane_sphere_blend` forms the spine
radius as `s² = offset² − h²` and the rim radius as
`rim² = sphere_r² − depth²`. Each is a difference of squares whose true
value is small exactly where the circle is small relative to the sphere,
so `s` carries about `u·offset²/s` and `rim` about `u·sphere_r²/rim` of
error. Not measured.

## What it needs

`s² = (offset − h)(offset + h)` and `rim² = (sphere_r − depth)(sphere_r + depth)`:
each factor rounds at the inputs' scale. A row measuring the spine circle
against the offset sphere and plane at `s ≪ offset` (a ball centre near
the offset sphere's pole) and showing the factored form at a few ulps.
Whether a door reaches a small `s` short of predicate 3's refusal
(`0 < s ≤ r` refuses `SpineIrregular`) bounds how small `s` gets in
practice: `s > r`, so the loss is at most `u·offset²/r`.

## Closed (2026-10-26, easy-fixes lane)

`crates/sweep/src/blend/arms.rs` `plane_sphere_blend` now forms `s² = (offset − h)(offset + h)` and `rim² = (sphere_r − depth)(sphere_r + depth)`. Pinned by `crates/sweep/tests/m5_pr12_blends.rs` `plane_sphere_blend_radii_hold_near_the_pole`: a pip ball `2⁻³⁰` below the sphere's top, with exact dyadic inputs, so the reference radii are one rounding each. The spine radius and the plane setback now hold to 4 ulps of `s`. The old form missed the spine radius by about 10⁶ ulps. Verified: `cargo nextest run -p sweep --test all -E 'test(/m5_pr12_blends|plane_sphere|review_h4_r1_probes|verbs_arms2_arms/)'`, 22 passed.
