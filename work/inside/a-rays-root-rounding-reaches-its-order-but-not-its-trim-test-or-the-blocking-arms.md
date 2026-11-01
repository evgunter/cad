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

- **A met edge's far root** (PR 6168). `reach_margin`
  (`solid_contain.rs:4171`, read by `reaches_root_in_band`) reads a curved
  face's root's offset from the passage (`off`) and its rise off the
  carrier (`rise`) with the root taken as exact. It does not read
  `RayHit::slack`.
- **A premise the `Interval` lane can break.** `curved_edges.rs:182`
  decides `hi − lo` (the extremes of the passages along the ray) through
  `decide_magnitude`, whose premise is "nonnegative by construction".
  At `f64` it is: `hi` and `lo` are a max and a min of one set. At
  `Interval` its enclosure's lower bound is `hi.lo − lo.hi`, which goes
  below zero by the passages' widths, and past `−escalate` it panics.
  It was not reached.

Owed: fold each root's slack into the readers that place its point, or
state for each why its error cannot move a verdict. Decide `hi − lo`
signed, or clamp it at zero.
