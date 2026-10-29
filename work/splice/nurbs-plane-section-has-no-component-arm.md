---
id: nurbs-plane-section-has-no-component-arm
kind: issue
title: A NURBS face and a plane whose carrier cuts its control net refuse on reach at the section certificate, which blocks every crossing NURBS union once containment serves the kind
status: closed
priority: P0
cost: H
opened: 2026-09-29
closed: 2026-10-29
refs: [nurbs-face-meeting-a-plane-in-an-interior-loop-is-unguarded-on-the-crossings-path]
---

## What

The section certificate's NURBS × plane arm (`section_cert.rs`
`nurbs_plane`) has W0 only: a plane clear of every control point is
apart. A plane that cuts the net has no arm, so the pair refuses on
reach (`CurvedPairUnsupported { site: InteriorLoopGuard }`) whether or
not it has events, because nothing counts a spline section's
components and W4 needs the count to be one.

That costs nothing today. Every union with a NURBS operand on the
crossings path refuses before the guard is raised: the join's role
resolution probes the other operand's regions against the NURBS body
(`join.rs` `resolve_roles_geometric` → `point_in_solid`), and
`solid_contain.rs` `face_geo` has no NURBS arm (`KindUnsupported`). The
full `topo` and `sweep` suites showed no row moving when the pair
entered the scope.

The day containment serves NURBS, every NURBS part crossing planar
stock refuses here instead. Recovering it soundly needs a component
argument on the spline: a certified count of the section's components
(so W4 reads the pair's events), or a witness per component. The
bump × clamp row in `crates/sweep/tests/germ_interior_oval.rs`
(`a_nurbs_graze_behind_crossings_is_refused_on_every_op`) pins the
join refusal and goes red that day.

## A consumer (2026-10-28, SPLICE)

The sheath-on-blade sweep row (`crates/sweep/tests/a_sheath_grafted_on_its_blade.rs`
`a_sheath_on_its_blade_stops_where_its_walls_touch_on_the_rim`) reaches this arm
once its rim certifies: with limb 3 temporarily stubbed to pass, blade ∪
sheath refuses `CurvedPairUnsupported { site: InteriorLoopGuard,
operand: B, kind: Nurbs, other_kind: Plane }` at ε 1e-9, 1e-6 and
1e-12, every pair evented. Each body's base cap is a plane that the
other body's walls stand on along their rim, so the plane touches the
net and W0 cannot part them. The NURBS × NURBS pairs behind it refuse
the same way
(`splice/no-certificate-excludes-a-closed-loop-interior-to-two-spline-faces`).
So "costs nothing today" holds only until SPLICE's rim certifies.

## Moved to SPLICE

2026-10-29 (SPLICE). Moved from GERMTAIL and re-banded P0: lily's
caps reach this arm once her rim certifies, and so does every NURBS
part on planar stock once containment serves the kind. The design is
the fork of `docs/DESIGN-FORK-LOG.md` row 213, question 1 on `[ev]`
PR 5990.

## Closed

2026-10-29 (SPLICE). The section value builds plane × NURBS (and
plane × `Approx` as its fit) from C3's whole reading
(`geom_brep::pair_section_with`, `pair_section/plane_spline.rs`), and
the guard reads it after W0 (`section_cert.rs` `spline_plane`): a
closed branch is a loop read by W3, W4 or the no-event decision; an
open branch, a `Side` region and a `Corner` region reach the wall's
chart boundary in a channel the chart does not close and clear (W1′,
by C4); a refusal maps by name. A spline witness is the wall's point
at the middle of the branch's pcurve, with a point of the branch's own
arc bracketed within a metre bound of it inside its tube window, and
that bound is its pad, which the rule pads the band by twice over
(`Component::pad`). The witnesses are
`crates/topo/src/boolean/section_cert_spline_rows.rs`,
`crates/sweep/tests/nurbs_plane_guard.rs` (lily's four cap pairs, a
plane cutting the blade, and a witness by a box's edge), and the bump × clamp row, re-pinned to
the loop's undecided refusal.
