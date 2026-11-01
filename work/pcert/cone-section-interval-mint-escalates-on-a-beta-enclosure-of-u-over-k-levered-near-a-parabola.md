---
id: cone-section-interval-mint-escalates-on-a-beta-enclosure-of-u-over-k-levered-near-a-parabola
kind: issue
title: chart_pcurve minted at the Interval scalar escalates on a near-parabola cone section, because beta encloses like u/k and the envelope levers it by about 1/k near |beta| -> 1
status: closed
opened: 2026-10-13
priority: P2
cost: M
refs: [focal-section-vertex-azimuth-is-read-off-a-cancelling-sum, conic-section-just-past-its-band-fails-certification-downstream]
closed: 2026-10-13
---


Found by `pcert/focal-section-vertex-azimuth`, building the Interval
leg of `a_skew_section_near_the_parabola_certifies`
(`geom-brep/tests/pcurve_cone_section.rs`).

## What happens

`chart_pcurve`'s cone arm (`pcurve_cache.rs`, `derive_class`, the
`Curve3::Ellipse` case of the cone chart) run at `T = Interval` on a
lifted (exact, degenerate-interval) carrier and chart reads
`ecc = −r0·ra / ra·ra` and `β = ecc / (1 + √(1 − ecc²))` as intervals.
Near a parabola (`k² = 1 − ecc²` small) the root's enclosure widens, and
`β`'s width grows like `u/k`. `focal_section_envelope` then levers that
width by the azimuth's sensitivity near `|β| → 1`, which grows like
`1/k`, so the envelope's contribution goes like `u/k²` and the
certificate escalates. `v0`'s width also grows like `1/k²`, but that is
the ellipse's size (`A`) in the scale, not the mechanism. The same
image minted at `f64` and lifted afterwards (the certifying-scalar
convention `pcurve_villarceau.rs` uses) certifies. `u0`'s enclosure is
about 4e-15 throughout, so the vertex azimuth is not the cause.

## Measured

Frustum of half-angle `atan ½`, the probe's skew pose
(`sweep/examples/conic_past_band_probe.rs`, `ROT=1`), plane through the
axis' midpoint tilted `f·(π/2 − α)`, ε = 1e-9, span `[−1, 1]`.

`β`'s width at scale 0.1 (review of PR 4960): 5.25e-14 at `k = 0.23`,
6.05e-13 at 2.35e-2 and 5.74e-12 at 2.35e-3, i.e. like `u/k`.

The table is **after** the fix that closed
`focal-section-vertex-azimuth-is-read-off-a-cancelling-sum` (PR 4960).
Before it, `u0` was read off a sum cancelling near a parabola and its
own enclosure widened like `u/k²` (1.55e-13 at `f = 0.99`, 1.33e-11 at
0.9999, 1.26e-9 at 0.999999, scale 0.1). The first row escalated then
(`[0, 2.05e-9]`) and certifies since, so the first row shows post-fix
behaviour; that row is now `a_skew_section_minted_at_interval_reads_a_tight_vertex_azimuth`.

| scale | f | k | width β | width v0 | Interval mint | f64 mint, lifted |
|---|---|---|---|---|---|---|
| 0.1 | 0.9999 | 2.4e-2 | 6.0e-13 | 6.8e-13 | ok 2.1e-10 | ok 3.6e-12 |
| 1 | 0.9999 | 2.4e-2 | not read | not read | Escalated, enclosure [0, 2.7e-9] | ok 3.0e-11 |
| 1 | 0.99999 | 7.4e-3 | 2.0e-12 | 6.2e-11 | Escalated, [0, 6.8e-8] | ok 3.4e-10 |
| 0.1 | 0.99999 | 7.4e-3 | not read | not read | Escalated, [0, 6.5e-9] | ok 9.4e-11 |
| 0.1 | 0.999999 | 2.4e-3 | 5.7e-12 | 6.5e-11 | Escalated, [0, 2.0e-7] | ok 2.8e-10 |

No suite fixture mints a skew-pose cone section at `Interval` (the 26
Interval mints in the six crates are axis-aligned), so outside that
row this is reached only by a caller that derives the image at the
certifying scalar.

## What closes it

Read `β` at the interval scalar in a form whose enclosure does not
widen like `u/k` near a parabola. For example, derive `√(1 − ecc²)` as
`|rb|/|ra|`, the projected ellipse's axis ratio, rather than from
`1 − ecc²`. Or state that the Interval image is the f64 mint lifted,
and route every Interval caller through that.

## Closed 2026-10-13

Fixed on `pcert/cone-section-interval-beta`: the cone arm reads
`√(1 − ecc²)` as the projection's axis ratio `|rb|/|ra|`. `β`'s
enclosure is about 1.4e-14 at every `k` in the table, and each
escalating row certifies minted at `Interval` (envelopes 9.8e-11,
8.2e-10, 8.2e-11 and 7.8e-10 in table order). The rows are
`beta_encloses_to_rounding_however_near_the_parabola` and
`a_near_parabola_section_minted_at_interval_certifies`
(`geom-brep/tests/pcurve_cone_section.rs`).

The pose one step nearer the parabola at scale 1 (`f = 0.999999`,
`k = 2.4e-3`) is refused before any image, at chart incidence: the
carrier's own rounding (`u·A`, enclosure `[0, 1.9e-9]`) is past the
band in this pose (the parent's probe pose at the same `α`, `f` and
scale is admitted and certifies). That is the carrier half of
`conic-section-just-past-its-band-fails-certification-downstream`
(GERM), noted there; it is not this item's.

The same read changes `β` at `f64`, for the better: see the parent's
note.
