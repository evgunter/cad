---
id: a-planar-face-bounded-by-a-rational-spline-rim-has-no-vector-area
kind: issue
title: a planar face bounded by a rational spline rim has no vector area: loop_vector_area's NURBS arm refuses Unimplemented on any weight other than 1, which is where the vase's cap moved alone stops at tier 3
status: open
opened: 2026-10-23
priority: P2
cost: M
---


Filed by ISO's `iso/vase-cap-moves-alone` lane.

## Measured

`crates/sweep/tests/encl_curved_loft_shell.rs`'s
`shelling_the_vase_refuses_at_a_walls_crease_and_its_cap_moves_alone_onto_a_wall_row`:
the vase (circle sections of radius 1, 1.3, 1 at z = 0, 1, 2, lofted
at degree 2) has its bottom cap moved 0.05 into the body by
`topo::replace_face_offset`. The cap moves: each rim is minted as the
moved plane and wall's `Intersection` on the wall's row `v = 0.025`,
the circle of radius 1.02925 at z = 0.05 to 1e-12, and its carrier is
that row, a rational quadratic (weights 1, √2/2, 1, √2/2, 1). Tier 3
(`topo::validate_geometric`) passes the edges and refuses its volume
check with exactly one error, at every ε row (default, 1e-6, 1e-12):

`VolumeUncomputable { source: Face { face: <the moved cap>, source: Unimplemented } }`

It comes from `geom-brep/src/props/loop_area.rs`'s `nurbs_vector_area`,
which `loop_vector_area` calls for a `Curve3::Nurbs` boundary carrier
and which refuses any weight other than 1: its per-span Gauss rule is
exact only on a polynomial integrand. So a plane face whose loop holds
a rational spline edge has no area or volume share, whatever the
spline traces.

## What it blocks

A cap moved alone on any loft with arc sections, and any planar face
whose rim is the row of a rational wall. The unmoved vase measures
(9.098052365638289) because its cap rims rest as chart images, not as
rational carriers.

## Shape of a fix

`(P − ref) × P′` on a rational carrier is a rational function per span,
so a fixed Gauss rule is not exact. Either a certified quadrature with
an error bound on the span, or recognising a rational quadratic span
that is a circular arc and taking `arc_vector_area`'s closed form.
