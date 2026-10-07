---
id: whole-turn-conic-reach-over-states-a-rim-faces-lever
kind: issue
title: the whole-turn conic reach in face_reach_from and face_extent over-states a rim face's lever and serves definite conics against in-band truths
status: open
opened: 2026-10-07
priority: P2
cost: M
---


Found by PR 4255's second full review (M-1, with N-1 folded in).

## What

`splitting::rules::face_reach_from` (and `face_extent`, which calls it
from a vertex) levers every conic edge at its whole turn:
`geom_brep::Reach::lever_from` of a circle or ellipse span is
`|centre − at| + r`, whatever span of the conic the edge holds. On a
face bounded by a rim arc that is cylinder-sized, and it feeds two
levers:

- main's chord_join lever (`face_extent`), which PR 4255 keeps as the
  cone lane's lever and as the cylinder lane's reach across the wall;
- PR 4255's `Reach::turn_lever` term (`across` is `face_extent` in
  chord_join and `face_reach_from(at)` in the germ frame).

**Witness (the review's).** A 1 cm × 10 µm patch bounded by circle arcs
on a wall of r = 1 km, read at a corner and cut by the plane through
the corner and the axis.

- Head serves a conic at a margin of 900–9,025·ε, against a Zero truth.
- Main serves the same conic.

**Differential (the review's, 20,000 configurations per caller).**

| Truth | Head serves a conic against it (chord_join + germ) |
|---|---|
| Zero | 66 + 66 |
| in band | 301 + 364 |

Every one of these is shared with main, so nothing regresses.

**N-1 (the same class).** A margin that is sound but not tight serves a
definite verdict where the owed answer is escalation. Both terms of the
plane×cylinder lever are upper bounds that peak at different points of
a face. PR 4255's own differential found 9 + 11 configurations on thin
sheets where head serves a conic against an in-band truth, all shared
with main.

## The shape of a fix

Read a conic edge's Euclidean reach over the span it holds, not the
whole turn. The farthest distance from a point over an arc is the
larger of the ends' and of any interior stationary point of
`|c + u·a·cos t + v·b·sin t − p|²`. For a circle that point lies on the
line through the projected point and the centre, so it is closed
form; for an ellipse it is a quartic, or a sound bound by the span's
chord and sagitta.

For the plane×cylinder row, the tight measure is the support of the
face's boundary along `c·a − (1 − cos)·w` from the hinge, one span at
a time (`Reach::range_along` takes any direction). That reads both
terms at the same point instead of summing two maxima.
