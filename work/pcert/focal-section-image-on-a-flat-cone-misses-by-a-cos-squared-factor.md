---
id: focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor
kind: issue
title: a cone ellipse's chart image on a flat cone (α near π/2) misses its carrier by about u·A/cos²α, so a frustum cut far from any degenerate pose refuses at 1e-12
status: open
opened: 2026-10-12
priority: P3
cost: M
refs: [conic-section-just-past-its-band-fails-certification-downstream]
---


Found by GERM's measurement for
`conic-section-just-past-its-band-fails-certification-downstream`
(probe: `crates/sweep/examples/conic_past_band_probe.rs`).

## What happens

On a flat cone the `Pcurve::FocalSection` image `chart_pcurve` mints
misses its carrier by about `u·A/cos²α`, independent of how close the
section is to a parabola. At `α = 1.55` (`1/cos²α ≈ 2300`) that puts
a well-conditioned ellipse (`k = 0.87`, half the parabolic tilt) past
the band at ε = 1e-12.

## Measured

Frustum of half-angle 1.55, height `S`, top radius `S/2`, revolved
about `y`, split by the plane through `(0, S/2, 0)` tilted
`φ = f·(π/2 − α)` off the axis, axis-aligned:

| S | f | A | carrier residual | whole-turn map residual | split at 1e-12 |
|---|---|---|---|---|---|
| 0.01 | 0.5 | 0.33 | 2.2e-15 | 7.2e-14 | splits |
| 0.01 | 0.9 | 1.3 | 3.5e-15 | 2.4e-13 | `Pcurves(Certify)` |
| 1 | 0.5 | 33 | 2.2e-13 | 7.2e-12 | `Join(Euler(Certification { Escalated }))` |
| 1 | 0.8 | 68 | 2.0e-13 | 7.8e-12 | `Join(Euler(Certification { Escalated }))` |

At ε = 1e-9 the `S = 100` frustum refuses the same way at `f = 0.5`
(map residual 7.2e-10). Reading the vertex azimuth off `ra`
(`focal-section-vertex-azimuth-is-read-off-a-cancelling-sum`) moves
it by at most 3× at this half-angle (generic pose: 2.4e-11 → 2.3e-11
at `f = 0.9`, 7.1e-8 → 3.6e-8 at `f = 0.999`), so this is a second
mechanism; unmeasured beyond the
scaling, which points at the slant `v = h/cos α` the image's second
channel is stored in.

## What closes it

Measure which of the image's numbers carries the `1/cos²α`, and
re-derive it from the carrier where it does not have to divide by
`cos α`.
