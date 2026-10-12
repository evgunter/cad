---
id: the-polygon-routes-width-on-a-long-face-in-a-general-orientation-is-l-cubed-ulps
kind: issue
title: The role read's polygon route is about L³·2⁻⁵² wide on a face L long in a general orientation, past the volume of a wedge thinner than about 1e-7 m at L = 2 m
status: open
opened: 2026-10-09
priority: P3
cost: M
---


Found by ENCL on
`work/encl/the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell`
(PR 4386). It is kept apart from
`a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever`
because that row is a value error on the fan route, and this one is the
width of a sound enclosure on the polygon route.

## What

`quad_lane::polygon_face_about` fans a face's vertex polygons from the
first point `a` and reads `(a − c)·Σ ½(p − a)×(q − a)` in interval
arithmetic. A vertex difference is one rounding wide, but the cross
product of two vectors `L` long rounds at about `L²·2⁻⁵²` per
component, and the lever `|a − c|` (up to `L`) scales that. So a face
`L` long in a general orientation leaves the walk's volume enclosure
about `L³·2⁻⁵²` wide: about 2e-15 m³ at `L = 2 m`. That is past the
volume of a 2 m wedge thinner than about 1e-7 m, so such a wedge reads
undecided while tens of bands thick.

On the near-tangent probe's poses every long face lies in a coordinate
plane or close to it. There the cancelling cross-product components are
exact, and all 74 runs that straddled decide.

## Witness

None yet. The `mapped_cube` fixture cannot build a wedge of 2 m × 1e-7 m
in a general orientation at ε = 1e-12: `newell_plane_residual`
escalates at w = 1e-7. A witness needs a construction whose faces stay
planar at that thinness, such as a boolean result.

## The shape to give

Evaluate each fan triangle's `det(a − c, p − a, q − a)` exactly, since
its inputs are `f64` vertex points (an expansion, or a filtered exact
determinant as the robust predicates use), and enclose the sum of the
exact terms.
