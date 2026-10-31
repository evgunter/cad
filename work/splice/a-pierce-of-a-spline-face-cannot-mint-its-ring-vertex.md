---
id: a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex
kind: issue
title: A pierce of a spline face refuses PcurveMint SplineChart at Delta 3: ring_vertex_in mints its chord strut into the complete spline face with mev, which writes no spline row
status: open
opened: 2026-10-31
priority: P1
cost: M
---


Filed by `a-vertex-piercing-a-spline-face-reads-no-outward-normal`'s
lane. This is lily wall 8's next stop once the pierced spline face
reads its outward normal.

## What

`demos/tour/src/lily.rs` wall 8 unions the lower stem (A) and the long
leaf (B). Stem vertex `5v1` pierces the leaf's NURBS wall `3v1`, and the
vertex-on-face pass now reads the wall's outward normal at the chart
point the sweep placed the pierce at
(`face_normal::spline_outward_normal_at`). The pierce crosses the face,
so the pass goes on to Delta 3 and mints the ring on the pierced face.
The union then refuses

`Euler(PcurveMint { face: 3v1, refusal: SplineChart })`

at ε 1e-9 and 1e-6 alike. At 1e-12 the stem's declared frontier stands
the lily walls down first.

The site, traced at both ε, is `boolean::vtxfac::ring_vertex_in`. It
mints the ring vertex by a chord strut, a line edge from the outer
loop's anchor to the pierce point, with `Body::mev` into the face, and
then `kemr`s the chord. On a spline face the face is complete when the
`mev` lands, so `pcurves::site_walks` refuses
`SiteRowRefusal::SplineChart`. The Euler operators are `Decide`-generic
and carry no fitted lane to derive the chord's row on the spline chart.
On an analytic curved face the same `mev` mints the chord's row and the
pass goes on.

## What it needs

A ring vertex inside a complete spline face that leaves the face
minted. One way is to mint the lone vertex's ring with no chord through
the chart. Another is to defer the face's rows until the ring surgery
is done (the refusal's own recourse, "mint once the surgery is done").
Then comes whatever the ring and its struts read next on a spline face.
The NURBS × torus section (SECTARMS
`a-nurbs-face-against-a-torus-has-no-section-arm`) still stands behind
it.
