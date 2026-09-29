---
id: nurbs-plane-section-has-no-component-arm
kind: issue
title: A NURBS face and a plane whose carrier cuts its control net refuse on reach at the section certificate, which blocks every crossing NURBS union once containment serves the kind
status: open
priority: P3
cost: H
opened: 2026-09-29
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
