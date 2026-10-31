---
id: a-rays-root-rounding-reaches-its-order-but-not-its-trim-test-or-the-blocking-arms
kind: issue
title: A ray's root rounding reaches its order decision but not its trim test, the partial arms' reach, or the torus quartic's roots
status: open
opened: 2026-10-31
---


Found by FLUSH's `flush/aimed-ray-reach` (the aimed rays' reach and
re-centred ray × quadric roots). That branch reads each ray × wall,
sphere and cone root about the line's closest approach and gives it a
first-order rounding bound (`solid_contain.rs` `Centred::roots`); a
plane crossing gets one too (`PlaneCross::slack`). The bound reaches
the count's order (each hit's span, and `RayHit::pair`'s margin), and
nothing else. Three readers of the same roots still take them as exact:

- **The trim test at a root.** `root_held` (`solid_contain.rs:4103`)
  places `q + d·t` against the face's region on the band alone. A ray
  that crosses a wall a little past its disc rung's zero has a root
  slack of about `u·r²/(2|d⊥|·√(2rε))`: about `10⁻⁹` m on a unit
  wall at ε = 1e-12. The hit point moves that far along the ray, near
  tangent to the wall, and a trim edge within that of it is decided
  with no allowance for it.
- **The partial arms' reach.** `bool_wall_outline_reach`
  (`solid_contain.rs:4575`) and `bool_cone_partial_reach`
  (`solid_contain.rs:4525`) decide `|q + d·t − anchor| − reach` with
  the root taken as exact. They only decide whether to block a ray, so
  an error here costs a ray rather than a verdict.
- **The torus quartic's roots.** These are depressed about `(q − c)·d`,
  already the closest approach. Their slack on that branch is the
  shift back alone (`rounding_charge(|t|)`, `solid_contain.rs:3958`).
  The depressed quartic's own root error is its ladder's to state
  (`depressed_quartic_roots`), and nothing carries it to the count.

Owed: fold each root's slack into the readers that place its point, or
state for each why its error cannot move a verdict.
