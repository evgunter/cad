---
id: cone-section-interval-mint-encloses-beta-like-one-over-k-squared
kind: issue
title: chart_pcurve minted at the Interval scalar encloses a near-parabola cone section's beta and slant wide enough that its own envelope escalates where the f64 mint, lifted, certifies
status: open
opened: 2026-10-13
priority: P2
cost: M
refs: [focal-section-vertex-azimuth-is-read-off-a-cancelling-sum, conic-section-just-past-its-band-fails-certification-downstream]
---


Found by `pcert/focal-section-vertex-azimuth`, building the Interval
leg of `a_skew_section_near_the_parabola_certifies`
(`geom-brep/tests/pcurve_cone_section.rs`).

## What happens

`chart_pcurve`'s cone arm (`pcurve_cache.rs`, `derive_class`, the
`Curve3::Ellipse` case of the cone chart) run at `T = Interval` on a
lifted (exact, degenerate-interval) carrier and chart reads
`ecc = −r0·ra / ra·ra` and `β = ecc / (1 + √(1 − ecc²))` as intervals.
Near a parabola (`k² = 1 − ecc²` small) the root's enclosure widens,
and so do `v0` and `va`. `focal_section_envelope` then levers `β`'s width
by the azimuth's sensitivity near `|β| → 1`, and the certificate
escalates. The same image minted at `f64` and lifted afterwards (the
certifying-scalar convention `pcurve_villarceau.rs` uses) certifies.
`u0`'s enclosure is about 4e-15 throughout, so the vertex azimuth is not
the cause.

## Measured

Frustum of half-angle `atan ½`, the probe's skew pose
(`sweep/examples/conic_past_band_probe.rs`, `ROT=1`), plane through the
axis' midpoint tilted `f·(π/2 − α)`, ε = 1e-9, span `[−1, 1]`:

| scale | f | k | width β | width v0 | Interval mint | f64 mint, lifted |
|---|---|---|---|---|---|---|
| 0.1 | 0.9999 | 2.4e-2 | 6.0e-13 | 6.8e-13 | ok 2.1e-10 | ok 3.6e-12 |
| 1 | 0.9999 | 2.4e-2 | not read | not read | Escalated, enclosure [0, 2.7e-9] | ok 3.0e-11 |
| 1 | 0.99999 | 7.4e-3 | 2.0e-12 | 6.2e-11 | Escalated, [0, 6.8e-8] | ok 3.4e-10 |
| 0.1 | 0.99999 | 7.4e-3 | not read | not read | Escalated, [0, 6.5e-9] | ok 9.4e-11 |
| 0.1 | 0.999999 | 2.4e-3 | 5.7e-12 | 6.5e-11 | Escalated, [0, 2.0e-7] | ok 2.8e-10 |

No suite fixture mints a skew-pose cone section at `Interval` (all 26
Interval mints in the six crates are axis-aligned), so this is reached
only by a caller that derives the image at the certifying scalar.

## What closes it

Read `β` (and the slant terms) at the interval scalar in a form whose
enclosure does not widen like `u/k²` near a parabola. For example,
derive `√(1 − ecc²)` as `|rb|/|ra|`, the projected ellipse's axis ratio,
rather than from `1 − ecc²`. Or state that the Interval
image is the f64 mint lifted, and route every Interval caller through
that.
