---
id: focal-section-envelope-bounds-the-whole-ellipse-not-the-span
kind: issue
title: the focal-section envelope bounds the image's miss over the whole ellipse, so a short consumed arc of a long ellipse refuses on its far side's error
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

`focal_section_envelope` (`pcurve_cache.rs`) bounds
`sup |S(P(t)) − C(t)|` by comparing the harmonic coefficients of the
image's lift with the carrier's (its doc's **Coefficients** step), so
the bound is the miss over the whole ellipse whatever `[t0, t1]` the
certificate is asked for; only the `vl·reach` terms read the span. A
plane × cone ellipse just short of the parabola is long (semi-major
`A` against a face extent of a few units) and an edge consumes a
short arc of it near its vertex, where the image's miss is a small
fraction of its miss on the far side.

## Measured

Frustum `1 → ½`, `α = atan ½`, plane through the axis' midpoint
tilted `φ = f·(π/2 − α)` off it, axis-aligned, ε = 1e-9. Map residual
over the arcs the face consumes against the whole turn and the
certificate:

| f | A | consumed arcs | whole turn | `PcurveCache::certify` over the arcs |
|---|---|---|---|---|
| 0.9999 | 3.0e3 | 7.3e-13 | 1.0e-11 | ok, envelope 1.0e-11 |
| 0.99999 | 3.0e4 | 5.6e-12 | 8.0e-11 | ok, envelope 7.4e-11 |
| 0.999999 | 3.0e5 | 8.4e-11 | 1.1e-8 | `ResidualExceeded { Envelope }` |
| 0.9999999 | 3.0e6 | 9.7e-10 | 4.2e-7 | `ResidualExceeded { Envelope }` |

At `f = 0.999999` the consumed arcs miss by `8.4e-11` and the split
refuses `Pcurves(Certify { ResidualExceeded { Envelope } })`.

## What closes it

A design question for PCERT: an envelope read over the span (the
coefficient comparison with the deviation's harmonic bounded over
`[t0, t1]` rather than its amplitude), or a statement that the
whole-ellipse bound is the contract and a consumer that needs a short
arc of a long ellipse refuses upstream.

**Not touched by the vertex-azimuth fix** (`focal-section-vertex-azimuth-is-read-off-a-cancelling-sum`, closed 2026-10-13): the table above is axis-aligned, where the off-plane error that fix removes is exactly zero, so its rows do not move; the question about the envelope statement stands.
