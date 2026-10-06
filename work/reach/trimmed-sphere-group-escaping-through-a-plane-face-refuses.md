---
id: trimmed-sphere-group-escaping-through-a-plane-face-refuses
kind: issue
title: A trimmed sphere face group that a plane face cuts with no edge crossing refuses: the escape re-chart serves only closed groups
status: closed
branch: reach/trimmed-sphere-escape
pr: 4044
closed: 2026-10-06
opened: 2026-10-02
priority: P1
cost: H
refs: [ball-inside-a-two-sphere-body-refuses-at-the-extent-scan, tilted-sphere-pair-section-refuses-at-the-polar-gate]
---


Found by the review of PR 3801 (the extent scan's face-scoped reading).

## Measured (branch `reach/extent-scan-faces`)

The lens `ball(1, y=0) ∩ ball(0.8, y=1.4)` (`snowman.rs` constructors)
against a 6 × 1 × 6 slab whose near face lies 0.985 from the origin
along the y axis tilted 20° about x (`snowman.rs` `tilted_slab`): the
plane cuts a cap of height 0.015 off the lens's top face, its circle
wholly inside that face and inside the plane face, at azimuth π/2 or
3π/2 of the lens's chart, so no edge of either body crosses a face.
Every op in both orders refuses `FallbackExtentUnsupported`, "a
TRIMMED sphere face group escapes through a plane face"
(`boolean::ops::sphere_extent_scan`'s plane arm). The oracle is
closed form: lens ∩ slab is the cap, `πh²(3 − h)/3` at `h = 0.015`,
and the rest follows from the lens's two caps and the slab's 36.
Pinned as the refusal by
`a_tilted_slab_against_the_lens_builds_or_refuses_the_trimmed_escape`.

Tilted about z instead (the circle across the lens's seam meridians),
the pose reaches the crossing layer and refuses
`Join(SectionNotPolar)`, the tilted-section door.

## The shape

The scan's escape conclusion feeds a re-chart that rotates a CLOSED
sphere group about its own centre so that its seams cross the escape
plane and the crossing layer sees the section circle. A trimmed group
(any boolean result carrying part of a sphere) cannot be rotated
alone, so its real escape has no repair and refuses. A fix supplies
an event for the circle on a trimmed face: re-charting the face's own
seam, or cutting the circle in directly.

## Fixed (branch `reach/trimmed-sphere-escape`)

Re-measured on `origin/main` `38a4e17083`: the 20° pose refused every
op in both orders as filed; the z-tilted pose already BUILT every op to
the cap closed form (PR 3985 retired `SectionNotPolar`).

Where the plane arm finds a trimmed group's circle inside the plane
face, it now reads the section certificate's verdict on the group's
faces: an R-loop names the face holding the circle, and that face is
cut along the meridian of its own sphere's chart through the circle,
from its boundary below the circle to its boundary above
(`ops::apply_cut_ins`, `SphereCutIn`); any other verdict refuses with
the certificate's own reason. The pipeline re-enters once, as after a
closed group's re-chart. The cut is a new seam of the face, so the
operand stays maximal, its faces stay chart rectangles where the face
was one, and the result carries no ring on a sphere face. The cut's
own predicates escalate as `SphereQuestion::CutIn`.

A ring edge across the circle was measured first and reached the
crossing layer, then the join's sphere ring lane
(`a-ring-on-a-sphere-face-has-no-island-winding`), props and point
classification (`sphere-face-with-a-hole-has-no-closed-form`); the
meridian cut needs none of them.

Rows (`snowman.rs`): `a_tilted_slab_against_the_lens_builds_off_the_seam`
(was `…_builds_or_refuses_the_trimmed_escape`), the z pose
`a_slab_tilted_across_the_lens_seams_builds`, a second carve
`a_slab_cutting_a_cap_off_a_banded_ball_builds`, the pole strut
`a_slab_cutting_a_cap_off_a_pole_strut_carve_builds` (a refusal until PR
4046 let the certificate read its face), the pole-to-pole wedge cut
`a_pole_to_pole_cut_of_a_ball_wedge_builds`, two cut-ins on one face in
either order `two_cut_ins_on_one_face_build_in_either_order` and
`a_circle_across_an_earlier_cut_takes_none_of_its_own`, and the
nearest of several boundary hits
`a_cut_ends_at_the_nearest_of_several_boundary_hits`; the R-loop gate is
`ops::cut_holder_rows`.
Filed from the sweep: `sphere-pair-meeting-inside-both-faces-refuses-spheres-meet`,
`closed-sphere-escape-is-re-charted-by-rotation-beside-the-meridian-cut`.
Filed from the dual review: `circle-plane-first-harmonic-has-three-hand-built-copies`,
`apply-cut-ins-walks-its-loops-twice-and-overloads-its-predicate-names`,
`cut-in-refusals-no-probe-reaches`.

## Closed (2026-10-06)

Merged by PR 4044. In the plane arm of `sphere_extent_scan`, a trimmed
group's in-face escape reads the section certificate. On R-loop (the
circle certified inside both faces) it records a `SphereCutIn` on the
holding face. `apply_cut_ins` then cuts that face along its own chart's
meridian through the circle and the pipeline re-enters. Any other
verdict refuses with the certificate's own reason. Later cut-ins on one
face run on the piece that holds both crossings. Rows:
`crates/sweep/tests/snowman.rs` (lens, banded ball, pole strut, wedge
pole to pole, two cut-ins in both orders, nearest of several hits) and
`cut_holder_rows`. Verified independently: 0 wrong bodies over random
poses at three ε (`analysis/reach-verify/4044`).
