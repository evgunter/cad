---
id: ray-cone-discriminant-under-reads-by-its-lead-coefficient
kind: issue
title: The ray x cone discriminant (bool_ray_cone_disc) is metered over the reach without dividing by the lead coefficient, so a ray definitely clear of the cone or definitely crossing it grazes
status: open
opened: 2026-10-25
priority: P2
cost: M
---



Found by PRED's D46 dimension verdicts (the audit's F27,
`docs/predicate-dimension-audit.md`).

## What

`crates/topo/src/boolean/solid_contain.rs` reads a ray against a cone
through `line_cone_quadratic` (`a₂t² + 2b₂t + c₂`, `d` unit) in two
places: `carrier_roots`' cone arm and `ray_crossings`' `PartialCone`
arm. Both decide `bool_ray_cone_disc` on
`Margin::over_lever(b₂² − a₂c₂, reach)`: m² over m, a length by
dimension. Near a tangency it is about `|a₂|·sin 2α·(ρ/reach)·δ` for a
ray `δ` clear of the cone at radius `ρ` from the apex, so it under-reads
by `|a₂|`, and the lead row (`bool_ray_cone_lead`, `a₂` levered by the
reach) only holds `|a₂|` above `Kε/reach`.

Measured by direct evaluation of the quadratic: at `α = π/4`,
`ρ = reach = 1`, a ray 0.01 rad off the generator and 1e-6 m clear of
the cone reads a margin of 5e-11, so it reads Zero and grazes. At
`α = 1.5` a ray square to the generator and 1e-6 m clear reads 7e-10.

The cylinder's discriminant had the same missing division and was
corrected (the audit's F2: `bool_ray_cylinder_disc` is
`disc/|d⊥|²` over `2r`); the cone's was not.

## Cost

Decidedness, not correctness. A definite sign is the discriminant's
own, so nothing definite is read wrong; a ray definitely clear of the
cone or definitely crossing it can read Zero, graze, and be set aside
for another. Where the tangency lies past the reach the margin
over-reads by `ρ/reach`; every root there lies outside the face, so
that reads nothing wrong either.

## What would close it

Meter the discriminant as a point deviation: divide by `|a₂|` and by
the cone's implicit gradient at the tangency (`ρ·sin 2α`, with `ρ`
bounded by the reach), or decide the roots' separation itself. Either
moves k-lint margins and wants re-baselining.
