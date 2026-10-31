---
id: a-pierce-of-a-spline-face-cannot-mint-its-ring-vertex
kind: issue
title: A pierce of a spline face refuses PcurveMint SplineChart at Delta 3: ring_vertex_in mints its chord strut into the complete spline face with mev, which writes no spline row
status: closed
opened: 2026-10-31
closed: 2026-10-31
priority: P1
cost: M
pr: 6166
branch: splice/spline-ring-vertex-mint
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

## Closed

- 2026-10-31 — Review tier: single FULL. On a spline chart,
  `boolean::vtxfac::ring_vertex_in` mints the ring through
  `Body::ring_vertex`, which fuses the chord strut's `mev` with its
  `kemr`. The pair's net delta adds no half-edge to any loop, so no row
  is owed: the face's rows stand as found, and the joint into the anchor
  keeps its element. The same path serves the fate pass's touch ring
  (`boolean::fate`). Every other chart keeps the chord strut, whose
  clear of a curved analytic face the join's keys-only doors rely on
  (`a-ring-vertex-in-a-curved-analytic-face-clears-its-rows`). Witnessed
  by `vtxfac_spline_rows`'
  `a_ring_vertex_in_a_nurbs_wall_leaves_it_minted_in_either_sense` and
  `a_ring_vertex_in_a_cylinder_wall_goes_through_the_struts_site_mint`.
  The kernel row for the pipeline is `sweep`'s
  `a_vertex_piercing_a_lens_wall`, which refused this row's
  `PcurveMint { SplineChart }` before and now stops at the plane × NURBS
  germ frame (`work/sectarms/a-nurbs-wall-against-a-plane-has-no-germ-frame`).
  Lily wall 8 moves on, at 1e-9 and 1e-6 alike, to the plane × torus
  spiric germ section of the stem's torus wall against the leaf's
  lower cap (`work/sectsite/c5-plane-torus-cone-cylinder-arms`).
