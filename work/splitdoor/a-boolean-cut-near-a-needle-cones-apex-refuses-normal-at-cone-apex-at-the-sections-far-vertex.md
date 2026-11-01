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
