---
id: cylinder-sphere-germ-pair-has-no-section-frame
kind: issue
title: A cylinder wall x sphere germ pair has no section frame at the join (GermFrameUnsupported) once the crossing layer and sector side pass
status: closed
opened: 2026-10-02
priority: P1
cost: H
refs: [3805, 3627]
closed: 2026-10-04
pr: 4025
branch: join/cylinder-sphere-frame
---


Found by REACH's ellipse-rim lane (2026-10-02), measured on its
merge of main after PR 3627 (the sector-side charge read at its peak).

## Measured

The lower part of the tilted drum cut (`crates/sweep/tests/conic_edge_curved_face.rs`:
radius 0.5, height 1, split by the plane through `(0, 0, 0.5)` at
0.3 rad) against a ball straddling the cut's ellipse rim — radius 0.2
at `(0.5, 0, 0.35)`, and radius 0.1 at `(0.45, 0, 0.3)`, both full
revolves about `y`: the crossing layer certifies every crossing (the
rim's against the sphere, the ball's meridians against the wall), the
sector side passes, and ∪, ∩ and ∖ refuse

```
GermFrameUnsupported { a_face: FaceKey(4v1), a_kind: Cylinder,
                       b_face: FaceKey(2v1), b_kind: Sphere }
```

raised by `boolean::join`'s frame dispatch (`FrameError::NoArm`): the
wall × sphere germ pair's section is a quartic space curve in general
(two circles only when the sphere's centre is on the wall's axis), and
no frame names it. Pinned by `a_rim_crossing_reaches_the_join`.

## What a fix has to supply

A frame for the cylinder × sphere germ pair. The section is not a conic
off the coaxial pose, so the rotational-sense test does not apply as it
stands; `geom_brep::cylinder_sphere_section` mints the rung-3 fitted
chord this join would have to read (`join.rs`'s comment at the no-arm
dispatch names "cyl×sphere's rung-3 fitted chords").

## Built (branch `join/cylinder-sphere-frame`)

`boolean::join::cs_transverse_frame` names the frame for a cylinder ×
sphere germ pair off the cylinder's axis, after the declared-coaxial
arm routes it to the general rung. With the sphere's centre `d` off the
axis along `û`, cylinder radius `r` and sphere radius `R`, the section is
one loop when `R < r + d` and turns once about the axis `û` through the
sphere's centre (the projection's turn rate is
`−r·rd·((cos θ − c₀)² + 1 − c₀²)/h`, of one sign whenever the loop
exists); two loops when `R > r + d`, each a graph over the wall's
circle, turning about the cylinder's axis. Two decided predicates:
`bool_germ_frame_cs_offset` (only a definite offset names a frame; a
coaxial pose keeps `NoArm`, as the hold requires) and
`bool_germ_frame_cs_reach` (the walls' tangency, `R = r + d`, keeps
`NoArm`; in band escalates).

Every pose in the Measured section now passes the frame and the matcher
and stops one door on, at the join's lane for the pair:
`cylinder-sphere-germ-pair-has-no-join-lane`. The sweep for other germ
pairs at `NoArm` filed `skew-cylinder-germ-pair-has-no-section-frame`
and `torus-germ-pairs-have-no-section-frame`.
