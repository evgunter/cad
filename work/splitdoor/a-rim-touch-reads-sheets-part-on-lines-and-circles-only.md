---
id: a-rim-touch-reads-sheets-part-on-lines-and-circles-only
kind: issue
title: A covered rim tangent to an elliptical boundary edge is never split there: sheets_part reads a boundary carrier as a line or a circle only
status: open
opened: 2026-10-28
---


Found by the sweep on `splitdoor/two-rims-coplanar` (the class: conic-pair readings that stop at circles).

`crates/topo/src/boolean/reduce.rs` `sheets_part` decides whether a covered edge's parent and the face it meets part at the meeting, reading the boundary edge's tangent there. It reads that tangent only from a `Curve3::Line` (its `dir`) or a `Curve3::Circle` (`axis × (p − center)`). Every other carrier answers `Ok(false)`, which its docs list among "nothing to read". An elliptical boundary edge (a rod's wall cut by a slanted plane) therefore never splits there, and the interior question answers `Interior::Unseen` for a covered rim that meets it. Its tangent has a closed form like the circle's: the derivative of `c + a·cos t·û + b·sin t·v̂` at the meeting's parameter.

Not measured: no body row is known to reach a covered rim tangent to an elliptical boundary edge (`a_closed_conic_in_the_partners_face::a_boss_over_a_hole_by_pose` meets circles only). The first step is a row that does: two rods of different radii, tangent inside one another, both cut by one slanted plane, would put an elliptical rim tangent to the other's elliptical rim.
