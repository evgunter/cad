---
id: chamfer-ends-in-a-curved-end-face
kind: issue
title: blend: a chamfer's straight band refuses a curved end face, where its section by a cylinder is an exact ellipse and by a sphere an exact circle
status: open
opened: 2026-10-06
priority: P3
cost: M
---


From Ev's question on `[ev]` PR 4085 ("i take it we still can't handle a
curved edge face properly, then?"). The ruling there retires the H7
sentence to "a curved end face refuses typed". For the CHAMFER that
refusal is narrower than the geometry: a plane band ends in a cylinder end
face along a plane–cylinder ellipse and in a sphere end face along a
circle, both stored exact kinds (`geom_brep::intersect::plane_cylinder_section`).
The FILLET's cylinder band against a cylinder end face meets it in a space
quartic with no stored carrier (special positions — coaxial, or equal radii
with meeting axes — give a circle or planar ellipses); that half waits on a
consumer and on D2's intensional `Intersection` or the fitted lane, and is
not this row.

Lands after the run-out row's step 2 (the local planar carve), whose
cut-off this widens.
