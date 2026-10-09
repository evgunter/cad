---
id: a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever
kind: issue
title: The role read fans a planar face with a curved edge, and measures a quadrature face, over its edges' carrier ends, so an ulp gap between consecutive ends reads at the face's length times its lever
status: open
opened: 2026-10-09
priority: P3
cost: M
refs: [the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell]
---


Found by ENCL on
`work/encl/the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell`
(branch `encl/shell-volume-local-origin`).

## What

The role read's interval re-derivation (`topo::props::rederive`) takes
a face's flux about the shell's least corner `c` as `(a − c)·A⃗`, with
`A⃗` summed over the loop fanned from its first point `a`. Each edge's
ends are its carrier evaluated at `t0` and `t1`, and consecutive edges'
ends meet only to the rounding of each carrier's evaluation. A loop that
does not close adds a sliver triangle from `a` to that gap, so the flux
moves by about `|a − c| · |a − gap| · gap`: the face's length times its
lever times an ulp, whatever the shell's volume.

ENCL's unit measured it on a planar shell before routing planar faces
bounded by lines to their vertex polygons
(`quad_lane::polygon_face_about`, through `props::vertex_rings`). At
ε = 1e-12, `vee300 nt e1 a4 d1e-10 cp I`
(`crates/sweep/examples/near_tangent_census_probe.rs`) keeps a wedge
2.3 m long whose vertex tetrahedron is +2.1e-19 m³ (exact, from its
`f64` vertex points). Fanned over its carrier ends, the exact value of
the same four faces is −3.4e-17 m³, gaps of 2.2e-16 m. Once the
enclosure was narrow, that read the shell `Void` and the ∩ refused
`ShellWinding`.

The polygon route covers a planar face whose every edge is a line. Two
cases still fan over carrier ends:

- a planar face with an arc, ellipse or spline edge
  (`quad_lane::planar_face_about`);
- a quadrature face (`quad_lane::cut_face_rounds`), and the line edges
  it shares with a polygon face, whose ends it reads off the carrier
  where the polygon reads the vertex.

Neither has a witness yet. The cure has the polygon's shape: close each
loop on its vertex points, and take a curved edge's own contribution
between them.

## Also measured: the width of a long face in a general orientation

With exact differences, a polygon face's width is the cross product's
rounding, about `|p − a|·|q − a|·2⁻⁵²` per fan triangle, times the
lever. On the probe's poses every long face lies in a coordinate plane
or close to it, and the 74 straddles all decide. On a 2 m face in a
general orientation the width is about `L³·2⁻⁵²`, about 2e-15 m³ at
`L = 2 m`. That is past the volume of a wedge thinner than about
1e-7 m. The `mapped_cube` fixture cannot build such a wedge at
ε = 1e-12 (`newell_plane_residual` escalates at w = 1e-7), so this is a
bound with no witness.
