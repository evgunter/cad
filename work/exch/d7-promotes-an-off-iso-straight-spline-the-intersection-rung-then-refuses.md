---
id: d7-promotes-an-off-iso-straight-spline-the-intersection-rung-then-refuses
kind: issue
title: D7 recognition can promote an off-iso straight spline carrier on a stay-NURBS face to a Line, which the plane x NURBS edge certificate refuses
status: closed
opened: 2026-10-12
priority: P2
cost: M
closed: 2026-10-14
branch: pcert/spline-pair-any-carrier
---


Found by the spline-carrier designers (PR 4261), and probed on
`pcert/fitted-row-reads-carrier-and-chart`.

## What happens

D7 recognition (`crates/step-import/src/recognize_curve.rs`, the Line
arm) promotes any spline carrier certifiably on its chord to a
`Curve3::Line`, whatever face it bounds. On a stay-NURBS face, an
off-iso straight edge between a plane and a B-spline wall then reaches
the adoption ladder (`adopt.rs`, `adopt_edges`) as a line:

- the `Intersection` rung refuses at the edge certificate:
  `run_checks`'s plane × NURBS arm (`crates/geom-brep/src/certify.rs`,
  `Resolved::PlaneNurbs`) answers "the declared carrier of a plane ×
  NURBS Intersection must be a spline", as does its NURBS × NURBS arm;
- left a spline, the same edge would have certified there, its chart
  image a degree-1 interpolant of a straight foot path.

So the promotion removes the one rung that certifies this edge.

## Measured, and what moved

The pcurve mint's `Intersection` arm no longer refuses a line on a
spline chart: since `pcert/fitted-row-reads-carrier-and-chart` it
gives an off-iso line its `General` image. The refusal at the edge
certificate is unchanged and comes first. Not reproduced through a
STEP file: the measurement is the code path above plus the topo-level
probe in
`work/pcert/an-off-iso-straight-edge-on-a-spline-face-refuses-its-rows.md`.

## Fix, one of

- withhold the Line promotion for a carrier whose adjacent faces
  include a stay-NURBS one where the carrier is not an iso of it;
- or let the plane × NURBS and NURBS × NURBS edge arms take a line
  carrier as its exact degree-1 net (the fitted row already does,
  `carrier_net`).

## Closed (2026-10-14)

The second fix landed on `pcert/spline-pair-any-carrier`: the plane ×
NURBS and NURBS × NURBS edge certificates take a carrier of any kind,
reading a line as its exact degree-1 net. `geom-brep`'s
`spline_pair_any_carrier::a_promoted_line_certifies_where_its_spline_did_on_the_same_image`
is the item's own probe: an off-iso straight edge between a plane and a
flat bilinear wall, stated as a degree-1 spline and as the `Line` D7
promotes it to, certifies through the door both ways, on bounds within
roundoff of each other (2.8e-15 and 3.0e-15 m), and the chart image the
certificate derives is one net for both (feet within 1e-15). The
promotion no longer removes the rung that certifies the edge; D7 is
unchanged.
