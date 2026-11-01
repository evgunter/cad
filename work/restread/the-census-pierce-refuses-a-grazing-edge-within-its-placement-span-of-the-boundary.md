---
id: the-census-pierce-refuses-a-grazing-edge-within-its-placement-span-of-the-boundary
kind: issue
title: "The census's edge–face pierce refuses a grazing edge whose crossing lies within its placement's span of the face's boundary, at f64 and at Interval alike: residuals read off a far origin place it no closer than their rounding over the lever"
status: open
opened: 2026-10-20
priority: P2
cost: M
---

Found by review 1 of `restread/ef-pierce-charge` (m3), measured by its
fix pass.

## What

`crates/topo/src/census.rs` `pierce_span` places the crossing of a
face's plane with an edge's line at `t = r₀/(r₀ − r₁)`, each residual
`rᵢ = (pᵢ − o)·n` read off the plane's stored origin `o` and charged
`geom_core::rounding_charge(|pᵢ − o|)` (16u). The enclosure's
half-width is about `c/|ℓ|` of the edge, so an origin tens of metres
off and a lever of a few `Kε` give a span of millimetres.
`pierce_inside` refuses wherever the face's boundary meets that span.
That happens whenever the crossing stands nearer the boundary than the
span is wide.

Measured with `census::pierce_fuzz`'s grazing draw at ε = 1e-12
(origin up to 42 m off, ends 1.5e-11 to 0.1 off the plane, crossing
3e-10 to 1e-2 from the top edge), effort 20, at f64:

- 438 of 1 200 poses refuse; main refused none, and read 46 of a
  similar 1 198 wrong.
- 436 of the 438 have a span half-width at least the crossing's depth.
  The median half-width is 55× the depth, and the 10th percentile 2.5×.
  The other 2 refuse elsewhere.
- In 98 of them the f64 closed form's own placement is off by more
  than the depth, so any reading of those residuals must refuse there.
- The charge is not the driver. At 5u, near the first-order bound of
  `(p − o)·n` against a unit normal (u for `p − o`, γ₃ for the dot),
  refusals fall only from 397 to 347 on the same seed. At 4u, 366 of
  the 438 would still have the span reaching the depth.

At seed `0x4e6588c9abba038c` (effort 20, f64) the draw refuses 397 of
1 200 at 1e-12, 35 at 1e-9 and none at 1e-6, and reads none wrong.

**At `Interval` it refuses about as `f64` does, a little more.** A
refusal of a valid body is a capability loss, hence P2. The clearance
reads the span on the edge's own line (`Stretch`), so the span's
rounded ends no longer turn it. Seed `0x4e6588c9abba038c`, refused of
200 per cell (`census::pierce_fuzz::probe::probe_grid`, origin 3, 30,
300 and 1 000 m off):

| ε | main, f64 | main, Interval | branch, f64 | branch, Interval |
| --- | --- | --- | --- | --- |
| 1e-9 | 0 | 0, 5, 17, 21 | 2, 9, 22, 22 | 2, 10, 25, 28 |
| 1e-12 | 0 (3–18 wrong) | 49, 57, 70, 94 | 60, 67, 77, 98 | 63, 69, 85, 100 |

On the tilted non-convex caps (`census::pierce_fuzz::tilted`, effort
20, same seed), of 316, 582 and 600 poses at 1e-6, 1e-9 and 1e-12:

| | main | branch |
| --- | --- | --- |
| f64 | 0, 0, 0 (1 wrong) | 0, 5, 112 |
| Interval | 0, 19, 292 | 0, 29, 316 |

What refuses, logged over the whole 1e-12 draw (poses within `2Kε` of
the profile included):

- At `Interval`, the containment door at the span's ends: 305 times
  at its near end and 8 at its far end. The clearance's edge–edge
  reading refuses 32 more.
- At `f64`, the same door 152 times; the span's ends reading
  differently 73 times; a boundary edge meeting the span (`span_clear`)
  46 times, and the clearance's edge–edge reading refusing 3.

The span's end stands up to the span's half-width past the crossing,
so it reads the face nearer the feature the crossing is near, and at
`Interval` it is a box as wide as the fraction's own enclosure along
the chord. Shrinking the span shrinks both.

## Direction

Read the residuals to `u·|rᵢ|`, not `u·|pᵢ − o|`: compensated `p − o`
and dot (`geom_core::exact::two_sum` / `two_product`, as
`geom-brep/src/ssi/compensated.rs` does for NURBS points), with an
a-priori bound to charge in place of `rounding_charge`. The span then
shrinks by about `|p − o|/|ℓ|`, and only crossings within a few ulps
of the boundary refuse. This has to hold at `Interval` too, where the
enclosed residuals shrink the span's end boxes with it, and it moves
`pm_census_ef_pierce_lever`'s k-lint distribution again.
