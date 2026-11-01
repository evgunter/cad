---
id: circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound
kind: issue
title: The circle x cylinder square arm charges its root slack with the whole harmonic term bound, not the near extreme's
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [3847]
---


## What

The conic × quadric door's first-harmonic arm on a circle against a
wall (`crates/topo/src/boolean/conic_quadric/mod.rs`,
`conic_quadric_roots`; the circle × cylinder door's square arm before
the two doors were one)
hands the shared first-harmonic door
(`circle_roots::first_harmonic_roots`) the extremes `c₀ ∓ A₁` with the
harmonics' whole `noise` on each and no phase charge. That is the
door's documented fallback ("a door whose only account is `noise`"),
and its slack is `ρ·(noise/√(−lo·hi) + 16u·τ)` — the residual error at
the root over the slope, plus the angle arithmetic's charge every
first-harmonic root takes — with `noise` `NOISE_ULPS` half-ulps of the
m² term bound `(|C₀ − o| + ρ)² + r²` over `2r`, plus the dropped
second harmonic. The extremes the arm decides on are `c₀ ∓ A₁`, computed
as differences.

The circle × sphere door no longer does this: its extremes are the
factored `(D∓ − r)(D∓ + r)/2r` with first-order running rounding
bounds, and at ε 1e-12 the near-tangent snowman went from refusing
at δ 1e-5 to building through δ 1e-6
(`circle-sphere-root-slack-refuses-near-tangent-pairs-at-1e-12`).

## Evidence

The dual review of PR 3752 (recorded in
`circle-sphere-root-slack-refuses-near-tangent-pairs-at-1e-12`):
at ε 1e-12 the square arm certifies none of 8,120 near-tangent poses,
every one refused `Uncertain`, none answered wrongly.

The arm now takes every circle whose second harmonic is in the zero
band, up to a tilt of `2√(r·zero)` rather than the zero band's own
width, so the class is wider: on the arm-switch probe of
`conic-quadric-first-harmonic-arm-refuses-crossings-the-ladder-places`,
the newly routed poses the ladder had certified and the arm refuses
with `A₂` under a hundredth of the band — this charge, not the
dropped harmonic — number 38 of 380 at ε 1e-9, 7 of 220 at 1e-6 and
85 of 500 at 1e-12 (shallow crossings of two near-equal circles, e.g.
a 1 m circle 1e-6 m off a 1 m wall's axis).

## What the fix is

The same as the sphere's: with the circle square to the wall, its
residual is the wall's cross-section circle against the carrier, so
`D∓ = |(|e⊥| ∓ ρ)|` in the cross-section plane, and
`geom_brep::circle_cylinder_harmonics` can return the factored
extremes with their running bounds beside the harmonics (the
`Rounded` helper in `crates/geom-brep/src/implicit.rs`). The dropped
second harmonic stays charged to both. Measure the 8,120-pose probe
before and after.

The tilted arm's ladder slack (`circle_roots::half_angle_roots`,
the `bool_conic_quadric_*` ladder rows) has the same shape — a
uniform `noise` over `|F′|` — but no closed-form extremes to factor;
GERM's `circle-torus-root-slack-crowds-the-zero-band-at-1e-12` asks
that question of the same ladder.
