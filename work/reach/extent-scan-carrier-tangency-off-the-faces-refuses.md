---
id: extent-scan-carrier-tangency-off-the-faces-refuses
kind: issue
title: A carrier tangency on the no-crossings path refuses though the touch point lies off every face
status: review
opened: 2026-10-02
priority: P1
cost: M
refs: [ball-inside-a-two-sphere-body-refuses-at-the-extent-scan]
branch: reach/extent-scan-off-face-tangency
---


Found by the class sweep of the face-scoped extent scan
(`work/reach/ball-inside-a-two-sphere-body-refuses-at-the-extent-scan.md`).

## Measured (branch `reach/extent-scan-faces`)

Both against the snowman `u1 = ball(1, y=0) ∪ ball(0.8, y=1.4)` (the
`snowman.rs` constructors), each pair disjoint or nested with no edge
touching a face, so the no-crossings path runs:

| operand | outcome |
|---|---|
| a ball of radius 0.4 centred at `0.6·(0, 0.7, 0.3)/‖(0, 0.7, 0.3)‖`, internally tangent to the radius-1 sphere at a point the snowman's trim cuts away | `SpheresMeet { verdict: Zero(..) }` |
| `brick((1.5, 3), (1, 2), (−1, 1))`, whose bottom plane is tangent to the radius-1 sphere at `(0, 1, 0)`, outside the plane face | `Escalated { Sphere(AgainstPlane) }`, `bool_sphere_extent_gap` margin 0 |

The union is the snowman plus the brick, and the snowman alone,
respectively.

## The shape

`boolean::ops::sphere_extent_scan` decides the CARRIER margin first
(`bool_sphere_extent_gap`, `bool_sphere_sphere_nested`) and refuses on
a decided zero before any face is read. A tangency of carriers touches
at one point (sphere × plane, internal sphere × sphere), and that point
lies on the two faces or it does not: placing it against each face
(`place_witness`, the chart trim on a sphere face, `contfp` on a plane)
and finding it `Out` of one certifies the faces apart at the touch.
The crossing case of the same arms now asks the faces
(`sphere_faces_apart`); the tangent case still refuses on the carrier.

The section certificate's R-tan arms (`section_cert`: every
`section_*` margin `signs` turns into `Section::Tangent`) have the same
shape at the pass: a tangency of carriers refuses whether or not the
touch locus meets the faces. Unmeasured there except for the coincident
cylinder pair, which is
`rounded-stack-subtract-and-intersect-refuse-fallback-extent` (on PR
3657's branch) and a different question (a 2-D overlap of trims, not a
touch locus).

## Sweep gap from CLEAVE far-plane (PR 3866)

The far-plane fix made two changes in `boolean/solid_contain.rs`:

- `point_in_solid`'s plane pre-pass asks for the face of the point's
  foot before an in-band carrier elevation refuses;
- a ray that runs along a carrier within the band skips that face only
  where `q` is definitely off the carrier.

Its sweep covered `solid_contain` and the loop walks. It did **not**
search `ops`' extent passes (`sphere_extent_scan`,
`section_extent_pass`) for the same shape: a carrier decision taken
before, or instead of, its face. This row is the one measured instance
of that shape there. Any sibling found should be added to this row
rather than filed separately.

## Fixed (branch `reach/extent-scan-off-face-tangency`)

Re-measured on `origin/main` `b14b1ceb0`. Both rows above refuse as
filed, and three siblings do too, each at a carrier decision: a ball
outside the lens touching its trimmed unit sphere
(`Escalated { Sphere(Apart) }`, `bool_sphere_sphere_gap` margin 0), a
rod whose wall carrier touches a ball before the rod begins, and two
skew rods whose walls touch past one rod's end
(`FallbackExtentUnsupported`, the section certificate's R-tan text).
`review_m6_5_pr2_sweep_probes::x4` pinned a fifth, a filleted die's
corner spheres touching a far box's plane carriers, as the plane arm's
tangency refusal.

The section certificate now gives a **touch** where a reach margin
decides `Zero` on an arm whose tangent pose meets in one point
(sphere × plane, sphere × sphere outside or inside, sphere × cylinder,
skew cylinders outside one another). Each arm bounds the section
there: every point the carriers share lies in a ball about the touch,
in every pose the decided margin admits. The pair clears when the touch
places `Out` of one face and no edge box of that face reaches the ball.
One point would not do: a face holed within the ball could hold the
section's small loop while missing its centre. The extent scan asks
the faces (`sphere_faces_apart`) at a decided zero or in-band margin
of `bool_sphere_extent_gap`, `bool_sphere_sphere_gap` and
`bool_sphere_sphere_nested`, and refuses as before only where they are
not certified apart. Pinches stay R-tan: the sphere × cylinder girdle,
the inner skew cylinder tangency, coincident walls, undecided margins
and every torus arm.

Pinned by `crates/sweep/tests/extent_scan_off_face_tangency.rs` (five
poses build in both operand orders under every op against closed
forms, at ε 1e-9, 1e-6 and 1e-12; the same tangencies on both faces
keep their refusals), `section_cert_rows`
`a_touch_holds_the_section_of_every_pose_its_margin_admits`,
`a_touch_clears_only_out_of_a_face_its_boundary_does_not_reach` and
`pinches_and_undecided_tangencies_are_not_touches`, and the filleted
die's `x4` row, which now builds.

Residue:
- `torus-touch-off-the-faces-refuses-at-the-section-pass`;
- `edge-tangent-to-a-curved-carrier-off-the-face-refuses-at-the-pierce`
  (the crossing layer, not the extent passes).
