---
id: chord-join-face-reach-misses-a-curved-edges-bulge
kind: issue
title: chord_join's section reach levers a wall at its boundary vertices' distance from the base vertex, not the consumed points' from the table's pivot
status: open
opened: 2026-10-06
priority: P3
cost: E
---


Found by PR 4118's first full review (MINOR-2), rescoped by its
second.

## What

chord_join's section reach (`section_reach`,
`crates/topo/src/chord_join.rs`) hands the section table the base
vertex and `face_extent` (`crates/topo/src/splitting/rules.rs:539`), the
farthest boundary VERTEX of the face from it, as the tilt's lever. That
is the lever this lane used before the table read its gap at an axis
foot. It is wrong in both directions:

- **Under:** a curved boundary edge can bulge past every vertex. For
  example, an obliquely trimmed cylinder wall whose ellipse rim carries
  one seam vertex reaches about `2r` along the axis beyond it. An
  under-stated lever reads a tilt smaller than it is.
- **Over:** the lever is measured from the base vertex, but the table
  pivots at the vertex's FOOT on the axis, `r` away. A tilt of the axis
  off the plane moves the section by the tilt times a consumed point's
  AXIAL distance from that foot, so the exact `pc_axis_plane_parallel`
  lever is `max |(p − at)·a|` over the consumed points `p`, the axis
  `a`. That is at most `face_extent`, so the face extent over-states it
  wherever a face reaches sideways round the wall farther than along it.
  It was worse at a ball about the vertex (PR 4118's second head), which
  read `r + face_extent` and decided an in-band tilt as an ellipse. On a
  two-sided row whose definite side is served, an over-stated lever is
  as wrong as an under-stated one.

PR 4118's third fix pass floored every measured lever at the pivot's
distance from where it was measured. On this lane that raised a face
shorter than the radius to the radius, past the face extent main
levered by: the fourth review served a tilted ellipse where main
escalated (`h = 0.5`, `k ≥ 6`; `h = 0.2`, `k ≥ 3`). The fourth fix pass
keeps the floor only where the cylinder pair reads its foot-to-foot gap
(`Reach::lever_between`). This lane levers at the face extent again, and
`chord_join::tests::a_short_faces_pose_is_levered_at_its_face_extent_not_the_radius`
pins that.

## The shape of a fix

Measure the lever exactly: the consumed points' farthest axial distance
from the base vertex, `max |(p − at)·a|`, with each boundary edge's
per-carrier bound (`geom_brep::Reach::Span`'s rule) rather than vertex
distances alone. That needs the pivot before the lever,
so `Reach` would carry the consumed points (or the face's edges) rather
than a measured length. `face_extent` has other callers (its lever arms
in the split lane), so whether they move with it is part of the item.
