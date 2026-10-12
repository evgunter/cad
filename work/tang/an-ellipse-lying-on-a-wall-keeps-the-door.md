---
id: an-ellipse-lying-on-a-wall-keeps-the-door
kind: issue
title: An ellipse edge lying on a partner's wall keeps the door: the lying-on lane reads arcs and rulings only
status: closed
opened: 2026-10-06
priority: P2
cost: M
refs: [a-line-edge-lying-on-a-wall-keeps-the-door]
closed: 2026-10-12
---


## What

`reduce::lying_on` reads a circle (certificates (a), (b) and the
interior question) or a line (the interior question), and returns
`None` for any other carrier. `carrier_cross::boundary_crossing`
answers `Unread` when the swept edge is an ellipse (its
`metres_per_param` match). So an ellipse lying on a wall, its parents
decided distinct from the wall, keeps the crossing layer's door: the
root door answers it `LiesOn`, and the `(Zero, Zero)` arm of
`reduce::curved_face_arm` hands it to `lying_on`, which declines.

No fixture is known. The usual source of an ellipse on a wall is a
section of that same wall, whose parent shares the carrier and is
refused before this lane (the cosurface question the D10 hold covers).
A partner wall that the section's own wall is not, but that holds the
same ellipse, is the shape to look for: two cylinders of one radius on
crossing axes cut by the plane of their common ellipse.

## Direction

The interior question generalises: an ellipse meets a boundary line or
circle where that curve meets the ellipse's plane, the closed forms
`carrier_cross::meetings` already holds for an ellipse boundary edge,
with the swept span metred by the ellipse's speed
(`geom_brep::Conic::speed_at`) rather than a constant.

## Built (branch cleave/one-carrier-reads-ellipses)

CLEAVE's `a-tilted-plug-in-its-hole-refuses-at-the-ellipse-rims` was
the same code gap in the one-carrier lane, and this branch closes
both. `reduce::lying_on` now reads an ellipse the way it reads a
ruling, by the interior question alone. `carrier_cross::boundary_crossing`
reads a swept ellipse, metred by `Conic::speed_at` at the candidate,
against line, circle and ellipse boundary edges (`carrier_cross::meetings`'s
swept-ellipse arms). Unit rows:
`carrier_cross::crossing_rows::a_swept_ellipse_meets_the_ruling_and_the_rim_it_crosses`
and the `meetings_rows` swept-ellipse rows.

The fixture this row suggests (two radius-1 rods on crossing axes, one
cut by their bisector plane) now passes this lane. It refuses next in
the germ join, `JoinDesync` at the parallel-cylinder radical plane,
filed as
`cleave/crossing-equal-cylinders-take-the-parallel-radical-plane-and-desync`.
On main it refused here, `CurvedPierceUnsupported`.
