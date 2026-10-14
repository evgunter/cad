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
cone/sphere/torus charts refuse typed naming that frontier"). A tilted
circle on a sphere stores its projected image (`Pcurve::Projected`,
`pcert/projected-image`), which the trimmed lane's chord pass reads
exactly. The face still refuses, at the trimmed lane's chart roster,
which has no sphere arm.

## What a fix owes

A sphere arm in the trimmed lane whose trim polygon comes from the
tilted circles' chart images (sampled at the shared chord parameters,
as the cylinder arm samples its ellipses), with a UV chord-step bound
for them: on the fitted rows' certificate, or on a closed form for
`(u(t), v(t))` of a circle on the sphere. The lily's three
tepal seams and every tilted sphere pair wait on it to be drawn.

## Evidence (2026-10-03, `reach/arc-from-pairing`)

A square bar through a ball now builds under every op (the bar's faces
cut the sphere in circles tilted against its chart), tier 3 clean, to
its slice-integral volume, and `mesh::tessellate` refuses the result
`UnsupportedCurvedShape { source: NotIsoRectangle { what:
"props_meridian_great" } }` (`crates/sweep/tests/snowman.rs`,
`a_bar_through_a_ball_builds_under_every_boolean`, which stops at the
volume and the tiers for that reason).

## Evidence (2026-10-05, `reach/trimmed-sphere-escape`)

The trimmed escape's rows build tier 3 clean to their closed forms and
refuse here on every op in both orders, `props_rim_axis_parallel`: the
lens against a slab tilted 20° about x and 20° about z, and a banded
ball (`ball(1) ∩ |y| ≤ 0.6`) against a slab toward latitude 10°
(`snowman.rs` `a_tilted_slab_against_the_lens_builds_off_the_seam`,
`a_slab_tilted_across_the_lens_seams_builds`,
`a_slab_cutting_a_cap_off_a_banded_ball_builds`, which stop at the
tiers and the volume through `assert_solid` for that reason).

## Evidence (2026-10-09, JOIN `join/sphere-pair-whole-circle`, PR 4344)

Two spheres crossing in a circle no edge reaches are re-cut by the
extent scan's sphere arm. A closed ball is re-charted with its pole on
the centre line, so the circle is a latitude of its chart and the
result meshes. A trimmed face, or a closed ball crossed along two
non-parallel axes, is cut along a meridian through the circle instead.
That circle lies tilted against the face's chart, and every result
holding such a face refuses here (`NotIsoRectangle`) on every op in
both orders, though it builds sound to its closed form:

- the lens union of two unit balls against `ball(0.3)` crossing one of
  its trimmed faces;
- two trimmed balls;
- a trimmed ball cut beside its pole;
- a small ball crossing both faces of a two-sphere lens;
- a unit ball crossed by two partners along non-parallel axes.

These are the `Tilted` and `Lumps` rows of
`crates/sweep/tests/spheres_crossing_off_every_edge.rs`. They go green
here when this lane lands.

A third witness (JOIN's tube-on-a-ball lane, 2026-10-09): a tube whose
rim lies on a ball, tilted so the rim is no latitude of the ball's
chart, builds every op sound and refuses the mesh with
`props_rim_axis_parallel` (`crates/sweep/tests/a_tube_ending_on_a_ball.rs`,
`a_tilted_tube_builds_at_its_closed_form`, reads it without a mesh).
