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
vertex and `face_extent` (`crates/topo/src/splitting/rules.rs:481`), the
farthest boundary VERTEX of the face from it, as the tilt's lever. That
is the lever this lane used before the table read its gap at an axis
foot. It is wrong in both directions:

- **Under:** a curved boundary edge can bulge past every vertex. For
  example, an obliquely trimmed cylinder wall whose ellipse rim carries
  one seam vertex reaches about `2r` along the axis beyond it. An
  under-stated lever reads a tilt smaller than it is.
- **Over, or misplaced:** the lever is measured from the base vertex,
  but the table pivots at the vertex's FOOT on the axis, `r` away. The
  consumed points stand up to `r + face_extent` from that foot.
  Levering at a ball about the vertex (PR 4118's first head) read that
  sum and decided an in-band tilt as an ellipse, so it is not the fix
  either: on a two-sided row whose definite side is served, an
  over-stated lever is as wrong as an under-stated one.

## The shape of a fix

Measure the lever as an EXACT distance from the table's pivot to the
consumed points: each boundary edge's per-carrier farthest distance
(`geom_brep::Reach::Span`'s rule) from the foot, rather than vertex
distances from the base vertex. That needs the pivot before the lever,
so `Reach` would carry the consumed points (or the face's edges) rather
than a measured length. `face_extent` has other callers (its lever arms
in the split lane), so whether they move with it is part of the item.
