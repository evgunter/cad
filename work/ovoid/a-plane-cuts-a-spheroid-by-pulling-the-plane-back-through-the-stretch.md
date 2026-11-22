---
id: a-plane-cuts-a-spheroid-by-pulling-the-plane-back-through-the-stretch
kind: unit
title: A plane cuts a spheroid: the section is the sphere's circle pushed forward through the stretch, and the boolean against a plane-faced body builds
status: parked
opened: 2026-11-22
priority: P1
cost: H
parent: an-ovoid-cannot-be-authored
blocked_on: [the-revolve-mints-a-spheroid-from-an-ellipse-arc-centred-on-its-axis]
---



Unit 9 of `an-ovoid-cannot-be-authored` (Ev, PR 7289). The ruling:
"parity with the sphere at first (plane × spheroid exact by pulling the
plane back through the stretch; other curved pairs, blends, offset and
shell refuse typed)". The designers: "pull the plane back by the
inverse stretch, cut the sphere, push the circle forward to an
ellipse. Margins are scaled back into true space before they count."
The bar: "is cut by a plane".

## What is true when it is done

- `geom_brep::intersect` has a plane × spheroid section beside
  `plane_sphere_section` (`crates/geom-brep/src/intersect.rs`), built
  by the pull-back: its trilean is the sphere's (`ps_center_gap`'s
  shape), read on a margin in metres of true space, not of the
  un-stretched frame. The section is a `Curve3::Ellipse`, and a
  `Curve3::Circle` where the plane is normal to the axis, decided at
  the band (so it never mints an ellipse with equal semi-axes; ONECOPY's
  `an-ellipse-with-equal-semi-axes-is-a-second-circle` is the general
  defect). A tangent plane is a point, refused as a carrier, as the
  sphere's is. `route`'s `(Plane, Spheroid)` rows flip to implemented.
- Every reader a plane cut reaches learns the spheroid, each composed
  with the stretch: the line × spheroid crossing is the line pulled
  back against `line_sphere_roots` (`curve_face/line_quadric.rs`) with
  its root metered in true space; the section bounds
  (`intersect/section_bounds.rs`); the germ arm (`topo/src/boolean/join.rs`'s
  `(Plane, Sphere)` → `KindArm::PlaneWall` row, its spheroid twin);
  containment of a trimmed spheroid face (`topo/src/sphere_region.rs`'s
  door, composed); the point-in-solid ray's spheroid arm.
- Through the public boolean, ∪, ∖ and ∩ of a full spheroid (prolate
  and oblate) with three bricks build, at rest, and measure inside
  their certified bounds: a brick crossing the wall at the equator, a
  slab over a pole, a brick wholly inside. A body cut by a plane
  oblique to the axis has an ellipse rim.
- Every other curved pair against a spheroid refuses typed, naming the
  pair (unit 1's refusals unchanged).

## Check first

Which sphere readers the plane cut reaches. The designers named the
section and nothing below it; the sphere's path through the boolean
went through several doors before it built (`docs/KERNEL-VERBS.md`'s
curved-boolean row records the sequence: the operand gate, then F7, then
`CurvedPierceUnsupported`, then containment). Measure the refusal
sequence on a spheroid against a brick on a scratch branch first and
report it; if a door the sphere passes by a closed form with no affine
composition (a distance, a curvature) is on the path, the unit grows
by that door, and if it grows past H, **stop and report**.

## Rows and ground

- Changes no other row.
- Crosses GERM, SECT and TANG (`intersect.rs`), SECTARMS, SPLICE and
  the join programs (`join.rs`), INSIDE (`sphere_region.rs`), and the
  curve-face lanes. Announce on SECTARMS' and GERM's logs.
- Runs beside units 6, 7 and 8.
- Review: **dual**: a new section class through the boolean, where a
  margin read in the wrong frame is a silent wrong answer. Difficulty
  **H**.
