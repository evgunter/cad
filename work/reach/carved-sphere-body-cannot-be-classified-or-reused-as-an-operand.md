---
id: carved-sphere-body-cannot-be-classified-or-reused-as-an-operand
kind: issue
title: A body carrying a sphere face bounded by a tilted circle refuses point classification (PartialSphereFace) and a ball nested in it (CurvedPierceUnsupported)
status: closed
opened: 2026-10-02
priority: P1
cost: H
refs: [tilted-sphere-pair-section-refuses-at-the-polar-gate, torus-face-bounded-by-an-oblique-circle-refuses-point-classification]
pr: 4046
branch: reach/carved-sphere-classify
closed: 2026-10-05
---


## Measured

Found by the `reach/tilted-sphere-pair` lane. The union of two unit
balls 1.4 apart along X (both `y`-poled, centres `(2, 2, 0.5)` and
`(3.4, 2, 0.5)`) now builds, tier 3 clean, to the lens closed form. Its
sphere faces are bounded by arcs of the radical-plane circle, tilted
against both charts. Reading that body again:

- `topo::point_in_solid` at `(2.7, 2, 0.5)` (in the lens),
  `(2.7, 2, 1.4)`, `(4.3, 2, 0.5)` and `(2.7, 2.6, 0.5)` — every query —
  refuses `PartialSphereFace { face: FaceKey(1v3) }`:
  `solid_contain::face_geo`'s sphere arm serves a trimmed sphere face
  only through `sphere_chart_trim`'s chart rectangle, and a boundary
  edge that is neither a rim nor a meridian great circle is outside it
  (the variant's own doc names this remainder).
- a ball nested in it (r 0.2 at `(2.7, 2, 0.5)`) or disjoint from it
  (r 0.3 at `(7, 2, 0.5)`), under ∪, ∩ and ∖, refuses
  `Containment(PartialSphereFace)` — the containment step classifies a
  point through the same arm;
- a ball of r 0.5 at the lens centre (inside the union, crossing
  nothing) refuses `CurvedPierceUnsupported { operand: B, face:
  FaceKey(2v1), edge: EdgeKey(1v1) }`.

So a carve that builds cannot be classified against, and cannot be a
later boolean's operand except where crossings carry it. Lily wall 7's
three tepal seams are three sequential carves of one lantern, so they
pass this way whether or not one seam's ball reaches another's face.

## What a fix owes

A sphere arm for point classification that reads a trimmed face
without a chart rectangle — the ray's hit point against the face's
boundary circles on the sphere (each circle bounds a cap; the face is a
signed combination of caps, decidable per circle by the hit's side of
its plane), or a spherical point-in-loop on the loop's arcs — and the
pierce arm's reading of the same faces. The rows: the four queries and
the three nested/disjoint balls above, under every op, against their
closed forms.

## Evidence (2026-10-03, `reach/arc-from-pairing`)

The pole-strut pose (`ball_poled_y(0.5)` against the box
`[−1, 0.25] × [−1, 1] × [−1, 0]`) now builds under every op in either
order, tier 3 clean, to its closed form; a result carrying the sphere
face the tilted circle `x = 0.25` bounds refuses as the next boolean's
operand with `Containment(PartialSphereFace)`
(`crates/sweep/tests/join1_r1_rows.rs`,
`a_pole_struts_halves_face_their_own_meridians`, which accepts that
refusal).

## Closed (2026-10-05)

Merged by PR 4046. `topo::boolean::sphere_region` is the one reading of
a trimmed sphere face: a geodesic closest-crossing rule over the face's
boundary arcs, cast both ways, read by `point_in_solid`'s sphere arm and
by the pierce arm's face door. The item's four queries, the nested,
disjoint and lens-centre balls, the pole-strut results and a tilted
plane × sphere cut all classify and build to their closed forms under
every op. `PartialSphereFace` now means only a sphere face with a
non-circle boundary edge. An independent verifier
(`analysis/reach-verify/4046`) found 0 wrong answers in 52,500 random
queries on five carved bodies at three ε, plus both reviewers' probes.
