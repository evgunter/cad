---
id: sphere-pair-meeting-inside-both-faces-refuses-spheres-meet
kind: issue
title: Two sphere faces meeting in a circle inside both, with no edge crossing, refuse SpheresMeet: the meridian cut serves only the plane arm
status: closed
opened: 2026-10-05
priority: P1
cost: M
closed: 2026-10-08
refs: [4044, interior-loop-cut-in, 4344]
---


Found by the sweep of `reach/trimmed-sphere-escape` (the class: a
section circle inside both faces with no event, cut in so the crossing
layer sees it).

## Measured (branch `reach/trimmed-sphere-escape`)

The lens `ball(1, y=0) ∩ ball(0.8, y=1.4)` (`snowman.rs` constructors)
against `ball_poled_y(0.2)` turned so its seam plane is normal to the
centre line and moved to `0.93·(0, cos 20°, sin 20°)`: its sphere meets
the unit sphere in a circle of angular radius about 11° inside the
lens's top face and inside the small ball's own face, and no edge of
either body crosses a face. Every op in both orders refuses

```
SpheresMeet { operand: A, face: FaceKey(1v1),
  verdict: Negative { margin: MarginDiag(Value(-0.13…)) } }
```

raised by `boolean::ops::sphere_extent_scan`'s sphere arm, where the
section certificate reads the circle as inside both faces (R-loop).

## What a fix owes

The plane arm's cut-in (`ops::apply_cut_ins`, `SphereCutIn`) cuts one
of the two faces along its own chart's meridian through the circle, so
the meridian crosses the other sphere's face twice and the crossing
layer's circle × sphere roots and the radical-plane join take it. The
arm records the cut where it now refuses, for a trimmed group and for a
closed one (a closed ball's escape circle also lies inside one
half-band), and the rows are this pose under every op against the
two-sphere lens closed form.

## Closed (2026-10-08, JOIN `join/sphere-pair-whole-circle`)

This row and JOIN's
`a-sphere-crossing-a-sphere-face-off-every-edge-refuses-spheres-meet`
are one defect. The sphere arm now records the cut where it refused,
for a closed group as for a trimmed one, from each operand's side.
This pose is pinned under every op in both orders against its closed
form: `crates/sweep/tests/spheres_crossing_off_every_edge.rs`,
`a_ball_crossing_both_faces_of_a_lens_builds`. The small ball crosses
both of the lens's sphere faces in whole circles, not only the top
face, so its own face is cut twice. That is why a single re-chart could
not have served it.
