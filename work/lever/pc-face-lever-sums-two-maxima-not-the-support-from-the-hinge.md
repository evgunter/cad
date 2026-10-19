---
id: pc-face-lever-sums-two-maxima-not-the-support-from-the-hinge
kind: issue
title: the plane×cylinder face lever sums its axial and cross-wall maxima instead of reading the face's support from the hinge
status: open
opened: 2026-10-08
priority: P3
cost: M
---

Re-filed from `whole-turn-conic-reach-over-states-a-rim-faces-lever` (its N-1), which closed on the span-bounded conic reach.

## What

`intersect::plane_cylinder_ruled` levers `pc_axis_plane_parallel` at
`|c|·(Reach::hinge_lever + Reach::turn_lever)` (`geom_brep::extent`). On a
`Reach::Face` that is the sum of two maxima:

- the face's axial reach from the hinge's station;
- its Euclidean reach from `at` plus `at`'s own distance across the wall, standing in for the reach across the wall from the hinge.

They peak at different points of the face. Both are sound, but the sum serves a conic where the exact offset of the real plane from the rulings' plane over the face is in the band.

**Evidence.** `crates/geom-brep/tests/span_reach_differential.rs`, 10,000 cylinder patches per caller:
- head serves a conic against an in-band truth 16 times (chord_join) and 9 times (germ), every one shared with main;
- 16 still serve with the exact reach across the wall from the hinge (the two maxima);
- 9 serve on the Euclidean reach standing in for the cross-wall component.

## The shape of a fix

Read the face's support along `c·a − (1 − cos)·ŵ` from the hinge, one boundary span at a time (`Reach::range_along` takes any direction). On a cylinder face a linear function peaks on the boundary, since every ruling through an interior point meets it. That reads both terms at the same point. It needs `Reach::Face` to carry its boundary spans, because `ŵ`, `c` and the hinge are the classifier's, not the caller's.
