---
id: a-junction-under-subtract-or-intersect-refuses-at-the-fallbacks-extent
kind: issue
title: A G1 tube chain under ∖ or ∩ passes the crossing layer and refuses at the fallback's extent: tangent carriers with no crossing event leave the section's component count uncertified
status: open
opened: 2026-11-22
priority: P2
cost: M
---


Found by the elbow lane's crossing-layer change (`heldrecords/elbow-ends`,
2026-11-22). Once an edge that ends on a glued junction's rim has that
end's root factored (`reduce::junction_window`,
`geom_brep::curve_face::end_root`), the G1 tube chain of
`crates/sweep/tests/mate7a_torus_rest.rs` (two `tube_along_arc`
segments sharing their terminal meridian G1) gets through the crossing
layer under every op. The union stops at the finish on its seam's
stand-in descriptions (the elbow lane's PR 3). Under A ∖ B, B ∖ A and
A ∩ B all three refuse

> `FallbackExtentUnsupported { operand: A, face: …, what: "a curved
> face and a face of the other solid have tangent or near-tangent
> carriers and no crossing layer saw an event; the section's component
> count is not certified there" }`

pinned in `subtract_and_intersect_on_the_torus_rest_fixtures`. The two
solids share no interior, so ∖ is each minuend whole and ∩ is empty.
The fallback's extent reads the junction's tangent carriers as a
section it cannot count, though the glue door decided the pair a
junction whose interiors do not meet.

Not traced: the raising site's reading, and whether it reads the
glue's junction record at all. Not measured: the elbow's ∖ and ∩.
