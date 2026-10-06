---
id: torus-face-bounded-by-an-oblique-circle-refuses-point-classification
kind: issue
title: A torus face bounded by a circle off both chart families refuses point classification (PartialTorusFace), where a sphere face so bounded is now read
status: open
opened: 2026-10-04
priority: P1
cost: H
design: true
refs: [carved-sphere-body-cannot-be-classified-or-reused-as-an-operand, cone-chart-trim-reads-a-tilted-section-as-its-vertex-window]
---

Found by the class sweep of `reach/carved-sphere-classify`, by reading.

## The defect

`solid_contain::torus_chart_trim` and `torus_face_windows`
(`crates/topo/src/boolean/solid_contain.rs`) read a torus face through its
two chart windows, and a face whose boundary edge has no closed-form image
on the torus chart — a Villarceau circle, or any other oblique circle — is
`PointInSolidError::PartialTorusFace` (the variant's own doc names this
remainder). The face door (`contain::torus_face_containment`) returns
`None` on the same faces, so the pierce arm keeps
`CurvedPierceUnsupported` there. This is the torus half of the shape the
sphere half closed: a face cut by a section tilted against its chart
cannot be classified against or reused as an operand.

## Why the sphere's reading does not carry over

The sphere's reading (`boolean::sphere_region`) casts geodesic rays and
crosses each boundary arc with a great circle's PLANE, a first harmonic.
A torus has no family of curves through every point whose crossings
with a circle arc are that cheap: a meridian or a parallel through the
point meets an oblique circle at the roots of a degree-2 trigonometric
polynomial (the circle × torus quartic's machinery, `circle_torus`), and
the ray must still close on the torus's two chart periods. Which curve
family to walk, and how the walk handles the two periods, is the open
question (`design: true`).

## Reachability

Not measured: no fixture here builds a torus face bounded by an oblique
circle at rest. A plane tilted against a torus's axis meets it in a spiric,
not a circle, except at the Villarceau angle, so the one route is a
Villarceau cut (a coaxial sphere or cone meets it in parallels, which the
chart reads).

