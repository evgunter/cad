---
id: carved-sphere-body-cannot-be-classified-or-reused-as-an-operand
kind: issue
title: A body carrying a sphere face bounded by a tilted circle refuses point classification (PartialSphereFace) and a ball nested in it (CurvedPierceUnsupported)
status: open
opened: 2026-10-02
priority: P1
cost: H
refs: [tilted-sphere-pair-section-refuses-at-the-polar-gate]
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

## Evidence (2026-10-05, `reach/trimmed-sphere-escape`)

A slab cutting a cap of height 0.0075 off the pole-strut carve
(`ball_poled_y(0.5) ∖ [−1, 0.25] × [−1, 1] × [−1, 0]`), toward latitude
10° at azimuth π/2 and toward `(0.866, 0.1, −0.5)`, refuses every op in
both orders before any cut: the face holding the circle is bounded by
the tilted circle `x = 0.25`, so the section certificate places no
witness on it (R-undec, "no witness could place"). Pinned by
`snowman.rs`
`a_slab_cutting_a_cap_off_a_pole_strut_carve_refuses_the_unplaced_witness`.
With `reach/carved-sphere-classify` at `fb2e1803c9` merged in, both
poses build under every op in either order to the closed forms
(`v_strut + 36 − cap`, the cap, `v_strut − cap`, `36 − cap`), the
second through a meridian cut whose two ends split the one `x = 0.25`
arc: the row flips to `assert_cap_cut` when this lands.
