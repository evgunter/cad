---
id: a-rectangle-the-walk-refuses-has-no-witness-the-chart-lane-can-read
kind: issue
title: A rectangle the curved walk refuses has no witness carrying chart images, so whether the chart lane meshes it is unmeasured
status: open
opened: 2026-11-16
---



## What

`tessellate::lane_of` routes a ringless analytic curved face by the
curved lane's two doors (`curved::iso_rectangle_shape`): a face both
pass goes to the curved lane's walk. Three walk refusals stand behind
the doors, for faces they pass that the walk still cannot take:
`UnsupportedCurvedDomain` (the zero-width slit,
`curved::require_swept_rectangle`), `MeridianFreeCurvedFace`
(`walk::require_a_meridian`) and `SingleColumnCurvedFace`
(`walk::require_two_columns`). Each is a valid or invalid face the
chart lane (`crate::trimmed`) might mesh, since it reads the domain
from the stored images and needs no rectangle.

Whether it does is unmeasured, because every witness of those rows is
assembled through the Euler doors (`mesh/tests/common/witness_bodies.rs`'s
`slit`, `one_circle_cut`; the single-column rows in `mesh/tests/errors.rs`
and `walk`'s units), which store no chart image and no joint element.
Measured on the slit: 0 stored images over its faces, and
`topo::Body::face_lift` answers `Gap(LiftGap { .. })`, so the chart lane
would refuse `DomainUnread` for want of data, not for its shape. No
public door is known to build a slit with its images (the boolean
splits a cut into rectangles; a sheet does not finish).

## What would settle it

A witness of each walk refusal that stores its chart images (a door
that builds one, or a test-support mint of the images on an Euler-built
sheet), meshed by the chart lane at the reach probes' δ, with
`reach_probes::geometry` read on the result. If the chart lane meshes
them, the route could send them there; that is a routing change with
its own measurement, not this row's.
