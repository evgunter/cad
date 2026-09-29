---
id: ray-wall-and-cone-near-root-cancels-over-a-small-lead
kind: issue
title: the cylinder and cone ray casts take the near root through a cancelling -b + sqrt(disc) divided by a lead only decided nonzero, 5.8e-11 off at eps 1e-12 against a 1e-11 band
status: open
opened: 2026-09-25
priority: P2
cost: D
---


Found by GERM's sweep for siblings of the ray-torus resolvent
cancellation (`work/germ/ray-torus-root-search-finds-a-counterexample-at-eps-1e-12.md`).

## The finding

`line_wall_roots` (`crates/topo/src/boolean/solid_contain.rs`, the
`WallRoots::Two` return) and the cone arm of `cast_ray` (the loop over
`(T::zero() - b2 - root) / a2, (T::zero() - b2 + root) / a2` after
`bool_ray_cone_disc`) take both roots from the textbook quadratic
formula. The root whose numerator is `−b2 + sgn(b2)·√disc` cancels
whenever `a2·c2` is small beside `b2²`, and the cancelled numerator is
then divided by `a2`, which is only DECIDED nonzero
(`bool_point_in_solid_denom` on the wall, `bool_ray_cone_lead` on the
cone). So the absolute error of the near root is `≈ ulp·|b2|/a2`, not
ulp-scale.

On the wall: `b2² ≤ |w0p|²·a2`, and `a2/(2r)` is decided above
`K·ε`, so the error is at most `ulp·|w0p|/√(2r·K·ε)`. Measured, in a
throwaway harness (exact-decimal roots of the same f64 quadratic),
over queries 1.5e-11 to 1e-10 outside a unit cylinder with rays at
`|dp|` 4.6e-6 to 4.1e-5: **worst absolute error 5.8e-11 at ε = 1e-12**,
where the escalate band is 1e-11. At ε = 1e-9 the same bound is
~1.6e-12, far inside its band, so this is an ε = 1e-12 exposure. A
crossing order (`bool_point_in_solid_order`) between two hits 1e-11 to
6e-11 apart could be decided the wrong way. No live instance observed.

## Why it was not fixed with the torus

The stable form (`t₁ = (−b2 − copysign(√disc, b2))/a2`, `t₂ = c2/(a2·t₁)`)
needs the sign of `b2`, and at the `Interval` scalar `b2` straddles zero
for a query on the axis (`w0p = 0`), a common pose. There `copysign`
returns the two-sided hull and `t₂`'s denominator straddles zero, so the
enclosure would explode where the naive formula is exact. The repair
wants a named frame decision on `b2` (the shape
`split_conic_phase_frame` takes in `crates/topo/src/splitting/classify.rs`,
with the naive formula on its Zero arm), and that is a new k-stats
predicate. It needs its own unit.

The torus PR first shipped exactly this hazard: its Cardano radicand
took `copysign(√(Q²/4 + P³/27), Q)`, and its review found generic rays
on the `Q = 0` surface escalating `Invalid` at `Interval` (MINOR-1 on
PR 3255). Its fix does NOT transfer here, and the reason is the point.
There the sign picks between two representations of the SAME root, so
it can be moved onto `A − B`, a factor that vanishes at `Q = 0`, and
the two-sided hull is then as narrow as the root itself. Here the sign
of `b2` picks WHICH of two roots the stable formula names, and at
`b2 = 0` the two are `±√disc/a2`, which do not coincide. So no factor
vanishes, and the frame decision above is still the repair.
