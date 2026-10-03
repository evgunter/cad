---
id: cylinder-sphere-germ-pair-has-no-section-frame
kind: issue
title: A cylinder wall x sphere germ pair has no section frame at the join (GermFrameUnsupported) once the crossing layer and sector side pass
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [non-circle-conic-edge-refuses-against-every-curved-face, slab-cut-cylinder-refuses-sector-side]
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
