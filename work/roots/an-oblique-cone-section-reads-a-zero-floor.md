---
id: an-oblique-cone-section-reads-a-zero-floor
kind: issue
title: An oblique ellipse section lying on a cone reads Uncertain: the cone floor's apex-distance bound goes negative
status: open
opened: 2026-10-08
priority: P2
cost: M
---


## What

An ellipse that lies exactly on a cone (an oblique plane section of
one nappe, clear of the apex) answers `CircleRoots::Uncertain` from
the conic × quadric door's cone arm
(`topo::boolean::conic_quadric::cone_roots`), not `OnSurface`. So the
reduction's `(Zero, Zero)` arm cannot record it through
`reduce::lying_on`, the way it records a circle on a sphere or a
meridian on a torus.

Measured (TANG's F ≡ 0 sweep, `tang/torus-meridian-lies-on`, a
throwaway probe): 1200 exact sections, cone half-angle α ∈ [0.1, 1.4],
the cutting plane's slope `m` along the axis's normal with
`k·m ∈ [0, 0.9)` (`k = tan α`), at scales 1e-3, 1 and 1e3 and
random poses. Every one is on the cone to 1e-10 by a sampled oracle.
553 read `Uncertain`, about equally at each scale, and every failure
has `k·m ≳ 0.5`. Coaxial circles on the cone, circles on a sphere in
any plane, and coaxial circles and oblique ellipses on a cylinder all
read `OnSurface` (1200 each).

## Why

`cone_roots` takes the first-harmonic arm (each harmonic of `Q` is
rounding) and its constant residual answers `OnSurface`. It then
reads the reach again through the floor (`bool_conic_cone_on_surface`,
`reach = (max(|lo|, |hi|) + noise) / h.floor`). The floor
(`geom_brep::conic_cone_harmonics`, in `implicit.rs`) is
`min(sin α, cos α)·near/per`, where `near` is a lower bound on the
carrier's least distance from the apex:
`|d|² + (aa + bb)/2 − |first harmonic| − |second harmonic| − rounding`.
That triangle bound drops below zero for an eccentric ellipse well
clear of the apex, so `near = 0`. Then `floor = 0` and `reach = ∞`,
which reads `Uncertain`. Every failing pose printed `floor 0.0`.

## What would fix it

A `near` that is the least value of `|q(θ)|²` itself, a degree-2
trigonometric polynomial whose minimum is decidable. For example,
read the minimum on the half-angle quartic, or take a tighter bound
than the sum of the two amplitudes. The ladder's floor would then be
the true one too, wherever it is read. `circle-torus-clear-margin-reads-the-floor`
(`work/hone/`) is a cousin: there the floor overstates a clear margin.
