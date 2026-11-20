---
id: a-sphere-face-with-a-zero-angle-corner-refuses-its-measure
kind: issue
title: A sphere face whose loop has a zero-angle corner (a section circle tangent to a rim) refuses its Gauss–Bonnet measure at props_sphere_loop_cusp
status: open
opened: 2026-11-20
priority: P2
cost: M
---


Found by `charttail/pole-section` (2026-11-20), whose fix let a plane
through a sphere pole cut the hemisphere for the first time.

## What

The unit hemisphere `y ≥ 0`, less the half-space `n̂ · (p − (0, 1, 0)) ≥ 0`
at `n = (1, 1, 0)` or `(0, 1, 1)`. The section circle passes through the
pole and touches the base rim at one point (`(1, 0, 0)`, `(0, 0, 1)`).
The remainder's sphere face therefore has a zero-angle corner there:
the rim arrives and the circle leaves along the same tangent, reversed.
The boolean builds the body, and its finish refuses the measure:

```
ResultInvalid { findings: NotFinished { findings: [VolumeUncomputable { solid: …,
  source: Face { face: …, source: SphereLoop { what: "props_sphere_loop_cusp" } } }] } }
```

on the plain and the wrap-tipped hemisphere alike. The cusp gate is
`props::curved`'s `require_cusp_free`: a junction with antiparallel
tangents turns by ±π, and no signed angle decides which. The cap itself
(the intersect) builds at its closed form at both normals, and at
`(1, 2, 0)`, where the circle stays clear of the rim, both sides build.

The closed form is the hemisphere's `2π/3` less the cap `π h² (3 − h)/3`,
`h = 1 − n̂_y`. A zero-angle corner's exterior turn is `±π`, and which
sign it takes is the side the face lies on at the touch. That side is
known from the face's other corners and the loop's orientation.

Pinned in `crates/sweep/tests/a_plane_through_a_pole_cuts_a_cap.rs`,
`a_plane_through_the_pole_cuts_each_hemisphere_at_its_cap`.
