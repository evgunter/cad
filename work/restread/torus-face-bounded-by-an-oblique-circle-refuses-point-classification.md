---
id: torus-face-bounded-by-an-oblique-circle-refuses-point-classification
kind: issue
title: A torus face that is not a chart rectangle refuses point classification (PartialTorusFace): the torus arm reads only the chart-window box, where the sphere reads a region
status: closed
opened: 2026-10-04
priority: P1
cost: H
refs: [4046, cone-chart-trim-reads-a-tilted-section-as-its-vertex-window, point-in-solid-refuses-an-l-shaped-torus-face]
closed: 2026-10-18
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


## 2026-10-17 — weighed; the decision (RESTREAD orchestrator)

Two designers (docs/prompts/designer.md) weighed this row concurrently
and agreed on the final state; it is not a fork for Ev.

**Premise corrections (both, sure).** The only circles on a ring torus
are parallels, meridians and the Villarceau families, so "any other
oblique circle" is empty, and the Villarceau face is reachable today
only by a hand `mef`. The defect is wider and measured: the torus arm
reads only chart rectangles, so every L-shaped, spiric-bounded or
ringed torus face the plane × torus boolean builds refuses
`PartialTorusFace` at every sampled point, and the shell sort refuses
`Pieces(Probe { PartialTorusFace })` (`germ_plane_torus_spiric.rs`,
`germ_plane_torus_chord.rs`, `contact11_torus_chart_l.rs`;
`point-in-solid-refuses-an-l-shaped-torus-face`, which now rides this
row). A Villarceau circle has an exact chart image
(`Pcurve::FocalSection`); the refusal is the reader's.

**Decision.** One torus region reading, `boolean::torus_region`, beside
`sphere_region` on `crate::ray_walk`'s driver and closest-crossing
reading, becomes the only torus arm of both point doors
(`point_in_solid`'s face geometry and `contain::torus_face_containment`)
for a face outside a closed group. It replaces the window class
(`torus_face_windows`, `torus_chart_windows`, `chart_polygon_box` and the
window half of `point_on_torus_in_face`) rather than running after it
(`[one-way.readings]`: no method reached because another declined). The
closed-group class stays, as on the sphere.

- Boundary first: the point is read against each edge on its own
  carrier; in band of an edge refuses, never a ray.
- Paths: the point's parallel and meridian, each either way round; when
  neither meets the boundary, aimed two-leg paths (parallel then
  meridian, and the mirror) to targets at fixed shares of each edge, as
  the sphere's `TARGET_SHARES`. A corner, a vertex hit, a leg along an
  edge, a tie or a tangency is a graze: next path. No path settles →
  `RayExhausted`.
- Crossings are computed in 3-D: every edge a torus carries at rest
  (parallel, meridian, Villarceau circle, axis-parallel spiric) lies in
  a plane, so a leg (itself a plane-section circle) meets an edge at the
  roots of one first harmonic against the edge's plane, then a span test
  and the which-component sign. Chosen over the chart-image crossing
  (one closed form per image kind, with lift and winding bookkeeping)
  because it is one equation for every edge and shares the sphere's
  certified root door. Near the top and bottom circles (cos v ≈ 0),
  where a horizontal plane is tangent to the torus, the schedule's
  predicates pick meridian-first paths; never a coordinate comparison.
- `PartialTorusFace` narrows to a boundary edge whose carrier lies in
  no plane (a spline; a future torus × cylinder quartic).
- `curved_interior_placement`'s ring short-circuit exempts the torus as
  it exempts the sphere.

Owed with it: per meter (span, order, which-component, crossing
heading), which bound makes each verdict sound; a random-query
wrong-answer probe on the plane × torus bodies against analytic
membership; the stood-down `germ_plane_torus_*` and
`contact11_torus_chart_l` rows re-baselined to assert; the
`PartialTorusFace` pins on rectangle-adjacent cases re-baselined; a note
on `work/boxes/chart-window-walk-written-twice.md` when
`torus_chart_windows` retires, and a check whether
`contacthold/notched-half-donut-owes-its-notch-and-volume-when-torus-plane-lands`
fires. Review tier: dual, class H, concurrent.

## Closed (RESTREAD, `restread/torus-region`, 2026-10-18)

`boolean::torus_region` reads a torus face outside a closed group from
its boundary edges, and is the only torus arm of both point doors
(`solid_contain`'s `FaceGeo::TorusPatch`, `contain::torus_face_containment`).
The window class is gone (`torus_chart_trim`, `torus_face_windows`,
`torus_chart_windows`, `torus_chart_joint`, `torus_chart_meets`,
`point_on_torus_in_face`); the closed group stays as `FaceGeo::Torus`.
`chart_polygon_box` stays: the cone trim decides `bool_cone_chart_box`
through it. `PartialTorusFace` narrows to an edge in no plane, or an
uncertified edge between two points.

One correction to the decision above, measured: "a first harmonic
against the edge's plane" cannot cross a parallel on the tube's top or
bottom circle, whose plane is tangent to the torus along it, so every
leg meets it at a double root. The dumbbell's fillet waist
(`germ_torus_doors.rs`) is bounded by one and went `RayExhausted` on that
reading. Each pair is read against the plane transversal to it: a
circle on a meridian leg in its own parameter against the leg's plane
(normal to the torus everywhere); everything else in the leg's
parameter against the edge's plane. With that, the near-top predicate
the decision names is not needed: no leg is refused at the top circle.

Measured on main and re-baselined: `contact11_torus_chart_l` asserts
the notch `Out` and a 400-point lattice; the chord rows' `PARTIAL` ops
and every `s` of `germ_plane_torus_spiric` build and hold to the truth;
the section-cert L, U, small-notch and gap rows read the notch. The
wrong-answer probe (`torus_region_probe.rs`) found none. Its one
refusal, `VolumeUncertified`, is
`at-infinity-probe-measures-in-closed-form-only`'s, with evidence there.

