---
id: a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever
kind: issue
title: In any walk with a face that is not a line-bounded plane, the role read fans every face, line-bounded planes included, over its edges' carrier ends, so a gap between consecutive ends reads at the face's length times its lever
status: open
opened: 2026-10-09
priority: P2
cost: M
refs: [the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell]
---

Found by ENCL on
`work/encl/the-role-reads-certified-volume-enclosure-straddles-zero-on-a-sliver-shell`
(PR 4386).

## What

The role read's interval re-derivation (`topo::props::rederive`) takes
a face's flux about the walk's least corner `c` as `(a − c)·A⃗`, with
`A⃗` summed over the loop fanned from its first point `a`. Each edge's
ends are its carrier evaluated at `t0` and `t1`, and consecutive edges'
ends meet only to that evaluation's rounding, or to the in-band
distance between two carriers that pass near one vertex. A loop that
does not close adds a sliver triangle from `a` to each gap, so the flux
moves by about `|a − c| · |a − gap| · gap`: the face's length times its
lever times the gap, whatever the shell's volume.

**This can certify a wrong role.** The enclosure holds the fan's value,
not the solid's, so where the enclosure is narrower than the fan's error
the role read certifies the fan's sign. ENCL measured it at ε = 1e-12 on
`vee300 nt e1 a4 d1e-10 cp I`
(`crates/sweep/examples/near_tangent_census_probe.rs`). The ∩ keeps a
wedge 2.3 m long whose vertex tetrahedron is +2.1e-19 m³ (exact, from
its `f64` vertex points). Fanned over its carrier ends, the exact value
of the same four faces is −3.4e-17 m³, from gaps of 2.2e-16 m. An
enclosure narrowed to the fan's own rounding read it `Void`, and the ∩
refused `ShellWinding`. On main the enclosure was wide enough to cover
the error, so that pose refused undecided instead. Nothing bounds the
error by the width in general: an in-band gap between a curved edge's
end and its neighbour's can be far larger than the rounding the width
carries.

## What PR 4386 changed, and what it left

PR 4386 reads a walk as the polyhedron of its vertex points only when
**every** face is a plane bounded by lines (`props::shell_polygons`).
Those loops close exactly. The route is all or nothing, so in any walk
with one other face, **every** face fans over its carrier ends, the
line-bounded planes included. That walk's re-derivation is unchanged
from main. A row checks it bit for bit against each face's own closed
form summed in walk order, on a fixture whose slanted edges end off
their vertex points
(`quad_lane::tests::polygon_tests::a_shell_with_a_curved_edge_keeps_the_fan_route_bit_for_bit`).
So, for a mixed walk, the class is exactly as it was on main. The fan
sites are:

- `quad_lane::planar_face_about`: every plane in a mixed walk, both
  the planes bounded by lines and the planes with an arc, ellipse or
  spline edge;
- `quad_lane::closed_form`'s `about_centre`, through `loops_vector_area`:
  a quadrature face whose lane refused about the centre;
- `quad_lane::cut_face_rounds`: the cylinder's position term
  `loop_vector_area(outer, origin)`;
- `geom_brep::props::curved` (cone, sphere and torus closed forms),
  reached through `quad_lane::translated_surface`, which sum their loops'
  vector areas over the carrier ends.

None of these has a witness yet. The cure has the polygon's shape: close
each loop on its vertex points, and take a curved edge's own
contribution between them. It has to cover every face of a walk at once,
because a line edge read as its vertices on one side and as its
carrier's ends on the other opens a gap of the same order.

The polygon route's own width on a long face in a general orientation
is a separate question, with no gap involved:
`work/tally/the-polygon-routes-width-on-a-long-face-in-a-general-orientation-is-l-cubed-ulps`.
