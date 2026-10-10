---
id: pn-apex-line-reach-is-read-at-the-apex-not-over-the-consumed-span
kind: issue
title: plane×cone bounds its apex line outcomes at the apex, so a consumer whose span stays clear of it is refused a line that is within band of its section there
status: open
opened: 2026-10-10
priority: P3
cost: M
refs: [pn-apex-line-outcomes-stand-in-for-a-section-off-the-apex]
---


## What

`plane_cone_section` (`crates/geom-brep/src/intersect.rs`) serves
`ApexLinePair` only where `pn_apex_pair_reach` (`|δ|/D`, where the
hyperbola's far branch starts) reads Zero, and `ApexTangentLine` only
where `pn_apex_tangent_reach` (`|δ|/M`) does. Both are read at the apex,
because the arm takes an `extent` and no consumed span. A consumer whose
span stays clear of the apex is therefore refused a line that is within
the band of the true section over that span.

Measured on a quarter wedge of a frustum of the cone `tan α = 1/64`
(apex at the origin, axis `+y`, `y ∈ [1/2048, 3/8]`), with a cap moved
by `±0.9·ε` through `topo::offset_edge_plans_for_tests`:

- Main plans the cap's wall edge, `Ok(None)`.
- Since the bound landed, it refuses: `NeighborPoseUnroutable`, naming
  the hyperbola.
- Over the edge's span, the true hyperbola `{x = δ, y² + δ² = z²·tan²α}`
  stands within `δ` of the served generator. Its lateral offset there is
  `δ²/(2·z·tan α) ≈ 5e-14` m at the frustum's small end, at ε = 1e-9.
- The same wedge closed at its apex is the case the bound exists for:
  `a_cap_moved_off_a_needle_apex_refuses_the_hyperbola` in
  `crates/sweep/tests/a_plane_moved_to_a_cone_apex.rs`.

The refusal is typed and fails loud. Only gaps inside the zero band are
affected, because a gap past it reaches the off-apex lane's hyperbola
refusal on main too.

## The shape of a fix

Read both reaches over the consumed span, as `plane_cylinder_section`
reads its tilt over a `Reach`. On a span whose nearest point to the apex
is `ρ` away, the served line stands within about
`max(start − ρ, 0) + |δ|/sin θ` of the section, with `θ` the planes'
dihedral angle along the generator. That needs a `Reach` in place of the
arm's `extent` at its three callers (`offset_derive`, `chord_join`,
`boolean::join`'s frame).
