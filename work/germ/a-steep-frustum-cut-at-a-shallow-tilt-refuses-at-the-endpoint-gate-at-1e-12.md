---
id: a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12
kind: issue
title: a steep frustum (α ≥ 1.3) cut by a plane half-way to its parabolic tilt refuses at the Euler gate's endpoint residual at ε = 1e-12, by 180–340 u·A; cause unmeasured
status: open
opened: 2026-10-12
priority: P3
cost: M
refs: [conic-section-just-past-its-band-fails-certification-downstream]
---


Found by the measurement for
`conic-section-just-past-its-band-fails-certification-downstream`
(probe: `crates/sweep/examples/conic_past_band_probe.rs`, `ROT=1`).

## Measured

Frustum of half-angle `α`, height `S`, top radius `S/2`, revolved
about `y`, in a generic rigid pose, split by the plane through
`(0, S/2, 0)` whose normal stands `φ = 0.5·(π/2 − α)` off the axis.
That is half the parabolic tilt, `k = 0.87`, nowhere near the
`pn_conic_type` band. At ε = 1e-12:

| α | S | φ | A | outcome | margin | `u·A` |
|---|---|---|---|---|---|---|
| 1.3 | 100 | 0.135 | 306 | `Join(Euler(Certification { Escalated { EndpointStart } }))` | 6.0e-12 | 3.4e-14 |
| 1.55 | 1 | 0.0104 | 33 | the same | 1.2e-12 | 3.6e-15 |

The arm's carrier stands 1.5e-13 off both surfaces at the second
pose (double-double, over the consumed arcs). That is about 40·u·A,
which is itself the flat cone's own factor
(`focal-section-image-on-a-flat-cone-misses-by-a-cos-squared-factor`
has the image side). The endpoint misses by eight times more again.
At these tilts the plane stands close to parallel to the frustum's
caps (`φ` is 0.0104 rad at α = 1.55), so the vertex the gate reads
the carrier's end against is cut where the plane crosses a cap's rim
at a shallow dihedral angle. Whether the miss is the vertex's or the
carrier's end parameter is not measured.

## What closes it

Measure the two sides of the gate's endpoint residual at these poses
(the vertex against the true plane ∩ rim in double-double, and the
carrier at its stored end parameter), then fix whichever carries it.
The cylinder's and the near-parabolic cone's endpoint refusals in the
parent row are `u·A`-scale and are not this.
