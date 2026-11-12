---
id: a-curved-partners-arc-is-imaged-at-rest-no-finer-than-the-mints-finest-rung
kind: issue
title: A sphere or torus arc on a spline wall refuses at rest at 1e-12 and on a ten-metre wall: the lane's image of the arc is the pcurve mint's finest rung, which misses its foot path by about 5e-10 of the wall's size
status: open
opened: 2026-11-12
priority: P1
cost: M
---


Filed by the `sectarms/spline-curved-at-rest` lane (stage B of
`the-join-reads-a-spline-implicit-section`).

## What

The at-rest arm for a sphere, cylinder or torus beside a spline wall
(`edge_nurbs::curved_nurbs_limbs`) derives the wall's image of the
carrier as the pcurve mint does (`edge_nurbs::general_chart_image`):
feet at a fixed schedule, interpolated at degree 5 through up to 257 of
them. Limb 2 then bounds `|S(P(t)) − C(t)|` through that image, so the
image's own miss of the carrier's foot path is charged to the edge.

Measured on the carry rows' wall against the sphere of radius `0.4·s`
(`spline_implicit::rest_rows`), at the default ε = 1e-9:

| s | carrier (the trace's) | the door's limb-2 bound | carrier's foot distance | finest image's worst miss |
|---|---|---|---|---|
| 1 | cubic, 554 spans | 2.7e-11 m | 5.0e-13 m | 5.1e-10 m |
| 10 | cubic, 985 spans | 6.0e-11 m | 4.7e-13 m | 3.8e-9 m |

The worst miss is at the span's end (`t ≈ 0.999`), between two carrier
knots, not at one. So the carrier is on both surfaces to 1e-12 and the
door certified it with a pcurve fitted on the carrier's own
parameterization (`ssi::ends::fit_states`); rest's image is the coarser
object.

What refuses, by ε row (each a refusal on the image, never on the
carrier: `PlaneNurbsRefusal::Limb { limb: ImageMiss }`, or
`CertifyError::Escalated { check: PlaneNurbsHullValue }` where the miss
lands in the band):

- ε = 1e-6: nothing; the sphere at s = 0.01, 1 and 10, the cylinder and
  the torus all certify.
- ε = 1e-9: the sphere at s = 10 escalates (miss 1.31e-9 m at a break).
- ε = 1e-12: the sphere at every s and the torus refuse (1.3e-12 to
  1.7e-11 m). The cylinder's arc, straight in the chart, certifies.

The degree-1 image `plane_nurbs_limbs` reads misses the same arcs by a
fraction of a millimetre (4.6e-4 m for the ball of radius 0.25 across
the loft prism's flat wall), which is why the curved lane climbs at all.

## Witnesses

- `crates/geom-brep/src/pair_section/spline_implicit/rest_rows.rs`
  `a_carried_rim_arc_certifies_at_rest_and_refuses_once_a_surface_stands_off_it`
  holds the table above as a measured tripwire (`image_holds`).
- `crates/sweep/tests/a_partner_across_a_lofted_wall_is_joined_on_its_section_value.rs`
  `a_ball_across_a_lofted_wall_is_joined_and_its_chord_certifies_at_rest`: at
  ε = 1e-12 the union stops at the chord's certificate on this miss;
  at the other rows it passes the join.

## The choice this leaves

C2 says rest re-derives the door's certificate and that the image is
evidence limb 2 decides; no clause says which image rest derives for a
fitted arc that bends in the chart. Three candidates, none built:

1. An image in the carrier's own spline space: its feet at the
   carrier's Greville points, interpolated on the carrier's knots, as
   the trace fits its pcurve on the carrier's parameters.
2. The door states its pcurve (`SsiBranch::pcurve_b`, already on the
   carrier's parameter), the edge certificate reads it as it stands
   (`EdgeCurve::certify_stated`, the `OnPair` lane's shape), and the
   face stores it as the edge's row.
3. A finer rung on the mint's climb, which moves every stored
   `Pcurve::General` that reaches it.

The same miss stands behind the plane's lane, which reads the degree-1
image alone
(`a-planes-carried-arc-across-a-flared-spline-wall-misses-its-own-image-at-rest`),
and the stored row of the same chord on the wall's face is the mint's
climb too, so whichever is chosen should serve both.
