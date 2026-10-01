---
id: a-placed-step-instance-with-a-plane-nurbs-edge-refuses-at-transform
kind: issue
title: step-import places an instance through the plain transform_rigid, which refuses the plane x NURBS edges the importer itself certified (by reading; unexecuted)
status: open
opened: 2026-10-01
priority: P1
cost: E
refs: [graft-recertifies-through-the-narrow-lane, plain-transform-rigid-still-refuses-the-m7-8-class]
---


Filed by the CLEAVE orchestrator from a designer's reading while
weighing `work/cleave/graft-recertifies-through-the-narrow-lane`.
Unexecuted.

`step-import` adopts plane × NURBS rim edges through
`Body::set_edge_curve_nurbs_lane` (`adopt.rs`, `nurbs_plane_pair`).
It then places each instance with the plain `topo::transform_rigid`
(`lib.rs`, the `Placed { map: Some(map) }` arm), and that door refuses
the class with `CertifyError::Unimplemented`. So a STEP file holding a
placed solid whose NURBS wall meets a plane should refuse
`StepImportError::Placement { Certify(Unimplemented) }`. No test
exercises such an instance.

CLEAVE is about to make `transform_rigid` read its NURBS lane from
`AtRestPolicy`, which should fix this with no change in step-import.
What this row asks for is a test: import a placed NURBS-walled
instance, and check that it refuses on main today and succeeds after
that change.
