---
id: a-torus-meridian-rims-span-bound-is-wider-than-the-band
kind: issue
title: The jet schedule's span bound on a circle carrier off a torus's axis (the G1 tube chain's meridian rim) is ~1.2 m, so its declared Seam cannot verify
status: open
opened: 2026-10-10
priority: P1
cost: M
---

Found by CONTACTHOLD's `contacthold/tangent-locus` lane while measuring
its on-surface fix in `topo::boolean::contact_verify`
(`tangent_locus_relation`, `on_surface`).

## What

`geom_brep::tangent_span_bounds` (`crates/geom-brep/src/tangent.rs`,
`circle_span_bounds`) gives the G1 tube chain's shared rim
(`crates/sweep/tests/mate7a_torus_rest.rs`, rings 5 and 1.1, tube 0.06)
`residual_sag = 1.2141624731781409` m on its `[0, 2π]` schedule (and
`0.32` m on the semicircle rims), against ε = 1e-9. Every sample's
implicit residual is below 4e-16. The rim is a MERIDIAN circle of
both tori, whose axis is perpendicular to the torus's, so the
equivariance argument that zeroes the bound for a coaxial circle does
not apply. The docs on `tangent_certificate_lane` name this class
("in-lane but uncertifiable") and say it was latent. It is not latent
now: the chain is minted by `sweep::tube_along_arc`, and the boolean's
witness lane reads it.

`contact_verify` used to subtract that sag from the residual, so a
1.2 m pad read definitely negative and "verified" the seam. Once the
verify side carries the sag (as the edge certifier's
`tangent_hull_sup` does), the declared seam escalates at
`contact_tangent_on_1`: the sample is decided on the wall, the span is
not. The test is re-pinned as
`mate7a_torus_rest::the_g1_tube_chain_declared_a_seam_escalates_at_the_witness_lane`,
and `a-torus-seam-graze-needs-the-rim-root-deflated`'s fixture now stops
there, before the graze it is about.

## The shape of a fix

A meridian-aware bound. In the torus's meridian half-plane the
linearized residual is `((ρ − R)² + z² − r²)/(2r)` with `ρ = |w|`. For a
carrier whose plane contains the torus's axis and which stays on one
side of it, `ρ` and `z` are first harmonics of `t`, so `f∘C` is a
trigonometric polynomial of degree ≤ 2. The circle arm's own
`A₁ + 4·A₂` bound then applies with no `√g` term, and it is exactly
zero on a true meridian. Whether that is a gate or a continuous
blend with the present bound is the design question the lane docs
leave open.

