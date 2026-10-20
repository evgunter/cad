---
id: a-flat-frustums-revolve-refuses-its-rim-image-at-1e-12
kind: issue
title: a flat frustum (α ≥ 1.45) a few hundred units across refuses to revolve at ε = 1e-12, certifying a pcurve's envelope or a carrier's endpoint; cause unmeasured
status: closed
branch: germ/flat-frustum-revolve
pr: 5382
opened: 2026-10-14
closed: 2026-10-20
priority: P3
cost: M
refs: [a-steep-frustum-cut-at-a-shallow-tilt-refuses-at-the-endpoint-gate-at-1e-12, conic-section-just-past-its-band-fails-certification-downstream, a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns, focal-section-image-reads-its-slant-off-height-on-a-flat-cone, cone-point-reads-take-the-slant-as-height-over-cos-alpha]
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

## Measured (GERM, `germ/flat-frustum-revolve`)

Probe: `crates/sweep/examples/flat_frustum_revolve_probe.rs`.

**The edge is not a rim.** `HalfEdgeKey(14v1)` is the cone wall's seam
ruling, a `Line` from the top rim `(S/2, S)` to the bottom `(r0, 0)`,
about `S/cos α` long, anchored at its top end, `S/(2·tan α)` below the
apex in height. Its image's envelope terms (`Radius`, `Line`; every
other term zero), against the image's true map residual and the line's
true distance from the cone (mpmath, 50 digits, on the stored data):

| α, S | `Radius` | `Line` | true map residual | distance from the cone |
|---|---|---|---|---|
| 1.45, 100 | 7.2e-13 | 9.7e-13 | 9.8e-14 | 1.2e-14 |
| 1.5, 30 | 5.8e-13 | 6.2e-13 | 4.9e-14 | 3.5e-15 |
| 1.5, 100 | 8.0e-13 | 1.1e-12 | 1.5e-13 | 1.0e-14 |
| 1.55, 10 | 2.7e-12 | 2.8e-12 | 2.8e-14 | 5.9e-16 |
| 1.55, 30 | 8.1e-12 | 8.2e-12 | 8.5e-14 | 1.8e-15 |

Two defects stack, both levered by the flat aperture:

- **The image** read its slant as height over `cos α`
  (`crates/geom-brep/src/pcurve_cache.rs` `derive_harmonic`, the
  `Curve3::Line` cone arm), so it missed the line by its distance from
  the cone times `1/cos α` exactly (8.3, 14.1, 48.1 in the ratio
  column above, to three digits).
- **The envelope** (`incidence`, `Derivation::ConeRuling`) bounded that
  image's miss by splitting the anchor's radius gap from the
  direction's misalignment, each divided by the anchor's height
  `|h_ref| = |w|·cos α`. The two parts cancel in the true residual and
  not in the bound: 10–100× over the true map residual.

**Fix.** Both cone harmonic images (ruling and rim) take the slant as
the projection on the generator through the carrier (`ruling_generator`,
`rim_slant`), so an image misses its carrier by the carrier's distance
from the cone; the ruling's lemma is restated as `‖w − (w·ĝ)ĝ‖ + ‖l −
(l·ĝ)ĝ‖·T`, and the rim's gains the axial term `|v·cos α − h|`. The
table's first five rows now revolve at ε = 1e-12. In a hunt of 300 fresh
flat frusta at ε = 1e-12, main refuses 15 (13 on this envelope) and the
fix refuses 3, none on it.

**The sixth row is another door.** `(1.55, 100)` refuses `EndpointEnd`
on the far cap rim's return half: filed as
`a-full-revolves-return-half-rim-misses-its-start-by-two-rounded-half-turns`
with the hunt's other three refusals (the coordinates' own resolution
near 2000).

**Siblings.** The cone's `FocalSection` image reads its slant the same
way (`focal-section-image-reads-its-slant-off-height-on-a-flat-cone`,
on PCERT), and five crates' point reads do
(`cone-point-reads-take-the-slant-as-height-over-cos-alpha`).

