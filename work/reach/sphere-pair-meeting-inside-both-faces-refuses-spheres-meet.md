---
id: sphere-pair-meeting-inside-both-faces-refuses-spheres-meet
kind: issue
title: Two sphere faces meeting in a circle inside both, with no edge crossing, refuse SpheresMeet: the meridian cut serves only the plane arm
status: open
opened: 2026-10-05
priority: P1
cost: M
refs: [trimmed-sphere-group-escaping-through-a-plane-face-refuses, interior-loop-cut-in]
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
