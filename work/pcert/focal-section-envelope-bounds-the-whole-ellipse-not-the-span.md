---
id: focal-section-envelope-bounds-the-whole-ellipse-not-the-span
kind: issue
title: the focal-section envelope bounds the image's miss over the whole ellipse, so a short consumed arc of a long ellipse refuses on its far side's error
status: closed
opened: 2026-10-12
priority: P3
cost: M
refs: [conic-section-just-past-its-band-fails-certification-downstream]
closed: 2026-10-14
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

## Closed 2026-10-14: overtaken

PRs 4960 (the vertex azimuth read off `ra`) and 5004 (`β` read off
`|rb|/|ra|`) removed the error that grew away from the vertex, and with
it the gap between the consumed arcs and the whole turn. The
whole-turn read stays the contract. Measured on main at `a1b214d8`.

**The table above, re-measured** (`conic_past_band_probe`, `FAMILY=cone
SCALES=1`, axis-aligned, ε = 1e-9). Each cell is consumed arcs /
whole turn / certificate:

| f | before | now |
|---|---|---|
| 0.9999 | 7.3e-13 / 1.0e-11 / ok 1.0e-11 | 7.3e-13 / 2.4e-12 / ok 4.7e-13 |
| 0.99999 | 5.6e-12 / 8.0e-11 / ok 7.4e-11 | 5.5e-12 / 2.0e-11 / ok 3.3e-12 |
| 0.999999 | 8.4e-11 / 1.1e-8 / `Envelope` refuses | 1.0e-10 / 2.4e-10 / ok 1.2e-10 |
| 0.9999999 | 9.7e-10 / 4.2e-7 / `Envelope` refuses | 9.8e-10 / 2.0e-9 / ok 8.9e-10 |

Across `α ∈ {atan ½, 1, 1.3}` and `f` to 0.9999999, axis-aligned and
skew, no pose refuses at the envelope. The refusals left are the Euler
gate's `EndpointStart` on the carrier (GERM's half) and a `MapResidual`
sample of the consumed arc itself. No span bound reaches either.

**Census of every f64 `FocalSection` certificate** in geom-brep, topo,
sweep, step-import, mesh and editor-core (release, ci profile, 10,503
tests). The envelope and the whole-turn miss are each set against a
2049-point sampled miss over the certified span:

| chart | certificates | envelope / span miss: median, p90, p99, max | whole turn / span miss: median, p99, max |
|---|---|---|---|
| cone | 2,860 | 0.50, 1.15, 2.01, 2.15 | 2.0, 6.5, 9.0 |
| torus | 260 | 0.84, 1.00, 2.2, 3.6 | 1.0, 2.7, 2.7 |

A bound read over the span could gain at most about 2.2× on any
certificate. On many certificates the envelope sits below the sampled
miss, by at most 2.4e-14 (cone) and 5.9e-15 (torus) on positions of
order 1 to 100. That is f64 evaluation rounding, since the envelope
bounds the exact image's miss.

The census is a one-off measurement that nothing computes with, so it
is unguarded by design.

**No new row.** The two reads that made the far side worse are each
already guarded by rows that go red when the fix is reverted:
- the vertex azimuth by
  `pcurve_cone_section::a_skew_section_near_the_parabola_certifies` and
  `a_near_circular_section_off_its_plane_misses_by_its_offset_not_twice`;
- `β` by `beta_encloses_to_rounding_however_near_the_parabola`,
  `a_near_parabola_section_minted_at_interval_certifies` and
  `an_off_section_carrier_is_imaged_to_its_own_distance_from_the_cone`.
