---
id: a-boolean-cut-near-a-needle-cones-apex-refuses-normal-at-cone-apex-at-the-sections-far-vertex
kind: issue
title: a block cut near a 3° needle cone's apex refuses NormalAtConeApex where the section's far vertex sits in band of the axis
status: open
priority: P2
opened: 2026-10-29
---



## What

Found as a NOTE in the review of PR 6002, measured by its
`review_6002_needle_join::review_6002_boolean_germ_hunt` (ignored).
The 3° needle cone (`revolve_common::needle`, apex (0, 1, 0)), ∩ and ∖ a
block whose top face is the plane through the point `1000ε` down the
seam ruling, its normal tilted 1.5 off the axis toward azimuth 0. At
ε 1e-9 both ops refuse:

    NormalAtConeApex { operand: A, face: FaceKey(2v1), band: Band { zero: 1e-9, escalate: 1e-8 } }

## Is it in band

Very likely, by the pose's geometry; the refusing vertex was not read.
In the plane holding the axis and the seam, the cut runs 1.06° off the
seam ruling and meets the opposite ruling `1000ε·sin 1.06° / sin 7.06°
≈ 150ε` from the apex, where the cone's radius is `150ε·sin 3° ≈ 7.9ε`.
That is inside the band `face_normal::cone_point` reads the distance
off the axis against (`cone_off_axis`, (ε, 10ε)), so the section's far
vertex is near the axis and `cone_axis_clearance` refuses with
`AtConeApex`. At 1° the same far vertex sits `≈ 605ε` out at radius
`≈ 10.6ε`, just clear, and the 1° pose does not refuse this way. At
spin 2 the 3° pose refuses differently (`dihedral_arm` and a volume
escalation, both in band).

## What is owed

Read the vertex the refusal comes from to confirm it is the far vertex.
If it is, the refusal is honest but its text names no lever: in the
definite-band half `NormalAtConeApex` "refuses the same way, with no
tolerance named". A near-axis vertex at `7.9ε` has the levers a band
reading has (move the cut away from the apex, or tighten the
tolerance below `7.9ε`).

## The refusal's own name (2026-11-01, SPLITDOOR, `splitdoor/cone-apex-boolean`)

A vertex in the band of a cone's axis now refuses
`VertexNearConeAxis { operand, face, band }`, not the apex's name. That
covers both a vertex definitely off the apex and one undecided between
the two. Its text names this row's levers: move the parts clear of the
tip, or tighten the tolerance.

`review_6002_boolean_germ_hunt`, re-run at the unit's head at ε 1e-9:

- every 3° and 1° pose at `k = 40`, tilt 0.4 or 1.2, refuses
  `VertexNearConeAxis`, as does 3°, `k = 1000`, tilt 1.5, spin 0;
- the poses at `k = 2` and `k = 10` that refused under the apex's name
  now build. The apex lies in the band of the block's top, and the
  pierce reads it as its rulings.
- Every build was checked against the closed form for the plane through
  the apex: 1e-15 relative. A `point_in_solid` lattice was also checked
  against the analytic region.

Pinned in `crates/sweep/tests/a_cone_apex_on_a_plane_through_it.rs`:
`a_needle_vertex_near_its_axis_refuses_by_name` (3°, `k = 40`,
tilt 1.2) and `a_needle_apex_in_the_band_of_a_plane_builds` (3°,
`k = 2`, tilt 1.2). Which vertex refuses is still not read, so this row
stays open.
