---
id: a-cylinder-rim-on-a-turned-frame-stores-a-rounding-in-its-axial-channel-at-f64
kind: issue
title: A circle square to a cylinder's axis is stored with a rounding residue in its axial channel where the frame is turned, at f64, so the injectivity check reads a fillet strip's end arc as a curve and its corner by the ruling run on past the vertex
status: open
opened: 2026-11-22
priority: P0
cost: E
refs: [a-cylinder-rims-axial-channel-holds-zero-only-to-an-enclosure-at-interval, a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation]
---

## Finding

From EMBED's lane `embed/tangent-corner` (2026-11-22), main `0c1eb97ad`
with the at-rest gate (`embed/gate-measure`) and the lane's change,
`f64`, a probe at `topo::injective::present_line_corner` printing the
corner's other edge's use.

`band_closed_mixed_chain_shapes::rounded_polygons_fillet_whole` (a
triangle of corner radius 0.3, every edge filleted at 0.1): the
cylinder strip `26v1` ends at the arc `26v3`, its section circle,
square to the strip's axis. The strip's chart image of that arc is
stored as

```
Harmonic { p0: [3.926990816987241, -0.3464101615137755],
           pa: [0, -6.938893903907228e-18], pb: [0, 0], pl: [-1, 0] }
```

a parallel `v = −0.34641…` but for a cosine term of `−6.9e−18` in the
axial channel: the residue of `u_ref · axis` on a frame turned off the
axes. `topo::clearance::iso_of` reads a coordinate line by exact
structure (the channel's coefficients lifting to the point zero), so
this use carries no `iso`. On an axis-aligned body
(`band_mixed_corner::an_inset_corner_with_all_three_edges_requested_builds_at_its_closed_form`)
the same arc is stored as a coordinate line.

`a-cylinder-rims-axial-channel-holds-zero-only-to-an-enclosure-at-interval`
(closed) found the same channel at `Interval`, an enclosure of zero,
and named its source: `geom_brep::pcurve_cache::derive_class`'s
cylinder arm writes the axial channel as raw dot products, where the
cone arm beside it decides the question (`pcurve_cone_chart_axial`)
and writes a constant. That row was closed at the reader, for the
enclosure. At `f64` the residue is a point that is not zero, which no
reader's exact test takes for one.

## What it costs

The injectivity check reads the strip's corner at the arc's end (the
ruling that is the strip's foot on its support plane, meeting the
section arc) as a coordinate line meeting a curve
(`present_line_corner`): its rectangle is not clipped at the arc, and
its zero side runs on along the ruling past the vertex, `6.8e−4` in
the traced cell. The support plane's cell at the same vertex is one
run of the straight foot and the next patch's arc
(`a-line-tangent-to-an-arc-at-their-vertex-is-read-by-no-corner-presentation`),
and the graph separator along that run cannot vanish on the ruling's
continuation as well. The pair refuses
`Uncertified { Departure }`, plane against cylinder across the straight
foot, at depth 40.

**Count**, of the 24 census tests of the tangent-joint row, with the
gate asking and that row's change in: five refuse here.
- `band_closed_mixed_chain_shapes::rounded_polygons_fillet_whole`
  (`1v1 × 26v1`, edge `48v1`; traced)
- `band_closed_mixed_chain_shapes::turned_rounded_outlines_fillet_whole`
  (`1v1 × 30v1`, edge `60v1`)
- `band_closed_mixed_chain_shapes::adversarial_outlines_build_where_their_bands_fit_and_refuse_where_they_overlap`
  (`1v1 × 82v1`, edge `176v1`)
- `band_mixed_rim_shared_supports::a_hole_beside_a_corner_reaching_a_links_strip_refuses_at_its_depth`
  (`1v1 × 33v1`, edge `69v1`)
- `blend_recourse_followability::the_foot_inside_a_face_recourse_names_a_request_that_builds`
  (`1v1 × 23v1`, edge `47v1`)

One is traced to the stored use; the other four are the same pair
shape (plane against cylinder across a line, after the arc pair
holds) on outlines that run off the axes, and are not traced.

## What is needed

The cylinder arm's axial channel decided as the cone arm's is, so a
section circle is a coordinate line by variant at every scalar and
pose; or a reading of a cylinder's corner that clips at a curve a
rounding off a parallel, which would be a concession and Ev's.

