---
id: iso-parse-refuses-a-circle-within-band-of-the-cone-as-off-surface
kind: issue
title: The cone and cylinder iso parses refuse as OffSurface a circle within band of the surface that is not the iso rim
status: closed
pr: 4932
opened: 2026-10-11
priority: P3
cost: E
closed: 2026-10-13
---


Found by the FLUX cone-tail unit (`flux/cone-tail`), which levers the
cone vector-area lane's circle incidence at the true distance from the
cone (`props_cone_circle_tilt`, `props_cone_circle_off_axis`).

The iso parses (`cone_boundary`, `cylinder_boundary`, through
`require_rim_incidence` in `crates/geom-brep/src/props/curved.rs`)
decide `props_rim_axis_parallel` at `r·sin θ` and
`props_rim_center_on_axis` at the offset `o`, under
`Premise::OnSurface`, so a miss refuses `OffSurface`. Those margins are
the circle's deviation from the iso rim, which is the question the
parse asks, since it reads a level off the circle's centre. They are
not its distance from the surface: on a cone that is
`r·(sin θ·sin α + (1 − cos θ)·cos α)` and `o·cos α`, and on a cylinder
`r·(1 − cos θ)` and `o`. So a circle that lies within the zero band of
the surface but is not the iso rim (tilted by `θ ≈ 10⁻⁵` on a unit
cylinder, or by `5·10⁻⁵` at slant 2 on an `α = 10⁻³` cone) refuses as
off the surface, where it is on it and is not iso.

The refusal fails closed; its kind is what is wrong. The iso question's
miss is the inventory's (`NotIsoRectangle`), as the sphere and the
torus already dispose of the same pair (`Premise::Inventory`), with
incidence decided at the true distance first.

Fix shape: decide incidence at the true distance as `OffSurface` (the
cone can share `require_on_cone`'s circle arm), then the iso pair as
`Premise::Inventory`. Rows near both ends of the half-angle range, and
a cylinder row at a second-order tilt.
