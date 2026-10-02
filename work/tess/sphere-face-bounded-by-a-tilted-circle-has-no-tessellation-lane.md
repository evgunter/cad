---
id: sphere-face-bounded-by-a-tilted-circle-has-no-tessellation-lane
kind: issue
title: A sphere face bounded by a circle tilted against its chart has no tessellation lane, so a carve that builds cannot be drawn
status: open
opened: 2026-10-02
priority: P0
cost: H
---


## Measured

Found by the `reach/tilted-sphere-pair` lane. A sphere pair whose
centre line is off the charts' polar axis now builds (the join's
run-side arc rule, `chord_join::select_arc_by_run_side`), and the
resulting sphere faces carry arcs of the section circle — circles
tilted against the sphere's chart, neither rims nor meridians. They
pass all three tiers and measure (`props::curved::sphere_circle_loop`),
and `mesh::tessellate` refuses them:

```
UnsupportedCurvedShape { face: FaceKey(2v7),
  source: NotIsoRectangle { what: "props_rim_axis_parallel" } }
```

on lily wall 7's carve (`demos/tour/src/lily.rs`,
`wall7_the_cone_pair_is_box_looseness_the_ball_meets_the_zone`), and
`props_meridian_great` on two unit balls 1.4 apart along X
(`crates/sweep/tests/tilted_sphere_pair.rs`). The swept-rectangle lane
cites `require_iso_rectangle` and refuses (`mesh/src/curved.rs`,
`require_iso_rectangle_face`); the trimmed lane
(`mesh/src/trimmed.rs`) has no sphere arm ("conic trims on
cone/sphere/torus charts refuse typed naming that frontier"). The
closed-form chart door has no image for a tilted circle on a sphere
(`UncoveredClass::SphereGeneralCircle` in `geom_brep::pcurve_cache`);
the pcurve mint routes it through the fitted lane, and a fitted image
"still refuses typed on every chart" in the trimmed lane (its doc), for
want of a certified UV chord-step bound.

## What a fix owes

A sphere arm in the trimmed lane whose trim polygon comes from the
tilted circles' chart images (sampled at the shared chord parameters,
as the cylinder arm samples its ellipses), with a UV chord-step bound
for them: on the fitted rows' certificate, or on a closed form for
`(u(t), v(t))` of a circle on the sphere. The lily's three
tepal seams and every tilted sphere pair wait on it to be drawn.
