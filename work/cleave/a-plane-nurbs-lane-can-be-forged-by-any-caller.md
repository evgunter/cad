---
id: a-plane-nurbs-lane-can-be-forged-by-any-caller
kind: issue
title: NurbsLane is any closure and PlaneNurbsLimbs has public fields, so a caller at any scalar can hand certify_via limbs it never derived and mint a certified plane x NURBS carrier
status: dispatched
opened: 2026-10-01
priority: P3
cost: M
refs: [graft-recertifies-through-the-narrow-lane]
parent: graft-recertifies-through-the-narrow-lane
branch: cleave/nurbs-lane
---


Found while weighing `graft-recertifies-through-the-narrow-lane`.
Unexecuted; this is a reading of the code.

`geom_brep::NurbsLane<T>` (`certify.rs`) is a `&dyn Fn(..) ->
Result<PlaneNurbsLimbs<T>, _>`, and `PlaneNurbsLimbs` (`edge_nurbs.rs`)
has public fields. `EdgeCurve::certify_via`'s plane × NURBS arm
(`run_checks`) checks only the limbs it is handed against the band. So
a caller at any scalar, `Dual` included, can pass a closure that
returns made-up limbs (e.g. `hull_sup: 0`). That reaches through
`certify_via` and through `topo::transform_rigid_via`'s public
`Option<NurbsLane>`, and it mints a "certified" carrier whose
certificate was never derived. That breaks D4 ¶2 ("an uncertified
value is unrepresentable").

The fix is the shape `FittedLane` already has: a value type with
private fields and a single constructor bounded `Decide +
CertifiedBounds`. Confirm the forgery first, with a unit row that
passes the zeroed-limbs closure at `f64` and at `Dual64` and expects
it to be refused once the type is sealed.
