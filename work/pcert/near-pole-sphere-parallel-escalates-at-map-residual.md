---
id: near-pole-sphere-parallel-escalates-at-map-residual
kind: issue
title: A sphere parallel a band-sized radius from the pole mints a row that certify escalates at MapResidual (rho 1.5e-8 and 5e-9)
status: open
opened: 2026-10-18
priority: P3
cost: M
---

Found by the delta review of PCERT B2a (PR 5244). It predates B2a: it reads the same, bit for bit, at B1's head `82ad876826` and at B2a's `498b864fca`.

## What is there

A parallel of the unit sphere `ρ` from the pole, the circle centred `(0, 0, √(1 − ρ²))` with axis `z` and radius `ρ`, mints a harmonic row (`chart_pcurve`), and `PcurveCache::certify` over `[0, π]` escalates it at check 3's map residual (`pcurve_map_residual`), at the witness band (zero `1e-9`, escalate `1e-8`):

| ρ | certify |
|---|---|
| `1.5e-8` | `Escalated { check: MapResidual, sample: 1 }`, margin `5.83e-9` |
| `5e-9` | `Escalated { check: MapResidual, sample: 0 }`, margin `5.0e-9` |

The mint takes the row and the certificate cannot decide it: the residual sits in the band at a radius the band can no longer separate from the pole. It is loud (an escalation), never a wrong answer. It is the sphere's case of the band-sized-lever question in `chart-winding-escalates-on-the-first-candidate-at-a-band-sized-lever` (a cone rim at a band-sized radius escalates at `pcurve_chart_orientation`).

## Repro

```rust
let band = Band::linear(Tol::witness()).unwrap();
let sphere = Surface::Sphere { center: Point3::origin(), radius: 1.0, axis: Vec3::unit_z(), u_ref: Vec3::unit_x() };
let rho = 1.5e-8; // or 5e-9
let rim = Curve3::Circle { center: Point3::new(0.0, 0.0, (1.0 - rho * rho).sqrt()), axis: Vec3::unit_z(), radius: rho, u_ref: Vec3::unit_x() };
let p = chart_pcurve(&rim, &sphere, band).unwrap();
PcurveCache::certify(p, 0.0, PI, &rim, &sphere, band); // Err(Escalated { check: MapResidual, .. })
```

The reviewer's probe is `review_b2a_r2::p_band_sized_levers` on branch `review/B2a-delta` (`cd37f0f65c`).

## Owed

Decide what a row at a band-sized radius from a pole owes: the mint and the certificate should agree. Either the mint refuses the class where its residual cannot be decided, or the residual is metered at the lever that row actually moves at. Answer it together with the chart-winding item's question, which is the same question on the cone.

