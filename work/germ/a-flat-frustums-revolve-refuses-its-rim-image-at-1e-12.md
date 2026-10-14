---
id: a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12
kind: issue
title: a flat frustum (α ≥ 1.45) a few hundred units across refuses to revolve at ε = 1e-12, certifying a pcurve's envelope or a carrier's endpoint; cause unmeasured
status: open
opened: 2026-10-14
priority: P3
cost: M
refs: [a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12, conic-section-just-past-its-band-fails-certification-downstream]
---

Found while closing
`a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12`.
No split is involved: the operand itself does not build.

## Measured

`sweep::test_support::revolved_about_y` of the profile
`(0, 0), (r0, 0), (S/2, S), (0, S)`, `r0 = S/2 + S·tan α`, full turn,
axis-aligned (no rigid pose), at ε = 1e-12. Probe:
`crates/sweep/examples/conic_past_band_probe.rs`
(`FAMILY=cone ALPHAS=… SCALES=… FRACS=0.5`; a `NOBODY` row is this
refusal, its panic text on stderr). Main at 2026-10-14, after #5043.

| α | S | r0 | outcome |
|---|---|---|---|
| 1.4 | 10, 30, 100 | 63, 175, 630 | builds |
| 1.45 | 10, 30 | 88, 262 | builds |
| 1.45 | 100 | 932 | `Pcurve(Certify { HalfEdgeKey(14v1), Escalated { Envelope } })`, margin 1.69e-12 |
| 1.5 | 10 | 146 | builds |
| 1.5 | 30 | 438 | the same, margin 1.20e-12 |
| 1.5 | 100 | 1460 | the same, `EnvelopeTerm(Line)`, margin 1.86e-12 |
| 1.55 | 1 | 49 | builds |
| 1.55 | 10 | 486 | the same, `EnvelopeTerm(Radius)`, margin 5.42e-12 |
| 1.55 | 30 | 1458 | `Pcurve(Certify { HalfEdgeKey(14v1), ResidualExceeded { EnvelopeTerm(Radius) } })` |
| 1.55 | 100 | 4858 | `Op { Certification { Escalated { EndpointEnd } } }`, margin 1.19e-12 |

α ∈ {1.2, 1.3, 1.45, 1.55} × S ∈ {1, 100} all build at ε = 1e-9 and
1e-6. With #5043's TiltedEllipse arm reverted locally, the α ∈
{1.45, 1.55}, S = 100 rows refuse at 1e-12 just the same.

The refusal is not a fixed number of `u·r0`: α = 1.4 at r0 = 630
builds where α = 1.5 at r0 = 438 does not, and α = 1.55 at r0 = 486
misses by 5.4e-12, about 100·u·r0. Which edge `HalfEdgeKey(14v1)` is
(a cap rim on the cone face is the likely one) and which of its
image's numbers carries the miss is not measured.

## What closes it

Name the edge and its image, measure the envelope's terms against the
true miss in double-double, and fix whichever is levered by the flat
aperture (`1/cos α` is 48 at α = 1.55). Or show the refusal is the
band's honest floor at this extent, and say so on the revolve door.
