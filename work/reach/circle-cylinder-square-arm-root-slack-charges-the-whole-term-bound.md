---
id: circle-cylinder-square-arm-root-slack-charges-the-whole-term-bound
kind: issue
title: The circle x cylinder square arm charges its root slack with the whole harmonic term bound, not the near extreme's
status: open
opened: 2026-10-02
priority: P2
cost: M
refs: [circle-sphere-root-slack-refuses-near-tangent-pairs-at-1e-12]
---


## What

The circle × cylinder door's square arm
(`crates/topo/src/boolean/circle_cylinder.rs`, `circle_cylinder_roots`)
hands the shared first-harmonic door
(`circle_roots::first_harmonic_roots`) the extremes `c₀ ∓ A₁` with the
harmonics' whole `noise` on each and no phase charge. That is the
door's documented fallback ("a door whose only account is `noise`"),
and its slack is `ρ·noise/√(−lo·hi)`, `noise` being `NOISE_ULPS`
half-ulps of the m² term bound `(|C₀ − o| + ρ)² + r²` over `2r`.

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
`bool_circle_cylinder_ladder_root_slack`) has the same shape — a
uniform `noise` over `|F′|` — but no closed-form extremes to factor;
GERM's `circle-torus-root-slack-crowds-the-zero-band-at-1e-12` asks
that question of the same ladder.
