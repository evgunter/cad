---
id: focal-section-vertex-azimuth-is-read-off-a-cancelling-sum
kind: issue
title: chart_pcurve reads a cone ellipse's vertex azimuth off r0 + ra, a sum that cancels to (1 − e)·ra near a parabola, so the image's error grows like u·A/k² where u·A/k is reachable
status: closed
opened: 2026-10-12
priority: P2
cost: M
refs: [conic-section-just-past-its-band-fails-certification-downstream]
closed: 2026-10-13
---


Found by GERM's measurement for
`conic-section-just-past-its-band-fails-certification-downstream`
(probe: `crates/sweep/examples/conic_past_band_probe.rs`).

## What happens

`chart_pcurve`'s cone arm (`pcurve_cache.rs`, the `Curve3::Ellipse`
case of the cone chart) builds the `Pcurve::FocalSection` image's
vertex azimuth `u0` from `start = (r0 + ra)·ns`, `r0` the radial
projection of the ellipse's centre and `ra` that of its major
semi-axis. For a section of the cone, `r0 = −e·ra` exactly (the axis
is a focus of the radial projection, `e = tan α·tan φ`), so `start =
(1 − e)·ra`: a sum of two vectors of size about `A` that cancels to
`A·k²/2`, `k² = 1 − e²`. Any error the carrier's centre carries off
the plane of the axis and the normal, about `u·A` at a generic pose,
turns `u0` by about `u/k²`, and the image misses the carrier by about
`u·A/k²` on the far side of the ellipse. `start`'s direction is
`ra`'s (`1 − e > 0` on every ellipse), so reading `u0` off `ra·ns`
alone loses no information and has no cancellation.

## Measured

Frustum of half-angle `α`, the plane through the axis' midpoint
tilted `φ = f·(π/2 − α)` off it, in a generic pose (the probe's
`ROT=1`), scale 1, ε = 1e-9. Whole-turn map residual
`max |S(P(t)) − C(t)|` of `chart_pcurve`'s image against the same
image with `u0` read off `ra` (`k` as above):

| α | f | k | as minted | `u0` off `ra` |
|---|---|---|---|---|
| 0.2 | 0.9999 | 3.8e-2 | 2.8e-10 | 1.8e-12 |
| 0.2 | 0.999999 | 3.8e-3 | 3.2e-6 | 1.7e-9 |
| atan ½ | 0.99999 | 7.4e-3 | 1.6e-7 | 2.0e-10 |
| atan ½ | 0.999999 | 2.4e-3 | 1.7e-5 | 1.4e-8 |
| 1.0 | 0.9999 | 1.6e-2 | 9.5e-9 | 3.7e-11 |
| 1.0 | 0.999999 | 1.6e-3 | 1.5e-4 | 4.3e-8 |
| 1.3 | 0.9999 | 1.5e-2 | 2.2e-8 | 1.0e-9 |
| 1.55 | 0.9999 | 1.4e-2 | 3.6e-6 | 8.8e-7 |

The corrected column tracks `u·A/k`, the share `β`'s own conditioning
(`1/√(1 − ecc²)`) leaves. In axis-aligned fixtures (axis and normal
in a coordinate plane) the off-plane error is exactly zero and the
minted image is already at `u·A/k`, which is why the frustum rows of
the parent issue read better than a skew pose does.

## What closes it

Read `u0` off `ra·ns` (or off `u_major` directly) where the arm reads
it off `r0 + ra`; re-measure the parent's sweep and the
`FocalSection` rows (`pcurve_cone_section.rs`,
`envelope_lemma_fuzz.rs`) with it.

## Closed

`chart_pcurve`'s cone arm reads `u0` off `ra·ns`. Re-measured with the
probe at `ROT=1`, scale 1, ε = 1e-9: the minted image's whole-turn miss
is the table's "`u0` off `ra`" column to the digit, and three rows that
refused now certify (`α = 0.2` and `atan ½` at `f = 0.99999`, `α = 1` at
`f = 0.9999`). The rest still refuse; that residue is the parent's and
the two siblings'.

Rows in `geom-brep/tests/pcurve_cone_section.rs`:
`a_skew_section_near_the_parabola_certifies` (`α = atan ½`, scale 0.1,
`f = 0.999999`, `k = 2.35e-3`: read off the sum the image misses by
9.2e-7 and refuses; minted, 1.2e-10, and certifies at `f64` and lifted
to `Interval`), and
`a_near_circular_section_off_its_plane_misses_by_its_offset_not_twice`
(a centre `δ` off the major-axis plane: `δ` minted, `2δ` read off the
sum), and `a_skew_section_minted_at_interval_reads_a_tight_vertex_azimuth`
(minted AT `Interval`, `u0` encloses to about 4e-15 where the sum
widened it like `u/k²`; at `f = 0.9999` that mint now certifies). Minted
AT `Interval` the image's `β` still encloses like `u/k`, which the
envelope levers by about `1/k`; that is filed as
`cone-section-interval-mint-escalates-on-a-beta-enclosure-of-u-over-k-levered-near-a-parabola`.
