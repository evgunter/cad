---
id: the-census-pierce-refuses-a-grazing-edge-within-its-placement-span-of-the-boundary
kind: issue
title: "The census's edge–face pierce refuses a grazing edge whose crossing lies within its placement's span of the face's boundary: residuals read off a far origin place it no closer than their rounding over the lever"
status: open
opened: 2026-10-20
priority: P3
cost: M
---

Found by review 1 of `restread/ef-pierce-charge` (m3), measured by its
fix pass.

## What

`crates/topo/src/census.rs` `pierce_span` places the crossing of a
face's plane with an edge's line at `t = r₀/(r₀ − r₁)`, each residual
`rᵢ = (pᵢ − o)·n` read off the plane's stored origin `o` and charged
`geom_brep::rounding_charge(|pᵢ − o|)` (16u). The enclosure's
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

At ε = 1e-9 the same draw refuses 66 of 1 200, and at 1e-6 none.

## Direction

Read the residuals to `u·|rᵢ|`, not `u·|pᵢ − o|`: compensated `p − o`
and dot (`geom_core::exact::two_sum` / `two_product`, as
`geom-brep/src/ssi/compensated.rs` does for NURBS points), with an
a-priori bound to charge in place of `rounding_charge`. The span then
shrinks by about `|p − o|/|ℓ|`, and only crossings within a few ulps
of the boundary refuse. This has to hold at `Interval` too, and it
moves `pm_census_ef_pierce_lever`'s k-lint distribution again.
