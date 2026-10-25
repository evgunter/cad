---
id: edge-extent-reads-a-steep-nurbs-arcs-chord-four-to-ten-times-short
kind: issue
title: edge_extent levers a NURBS edge by its chord, a lower bound 4-10x short of the point-set diameter on a steep hyperbolic arc
status: open
opened: 2026-10-25
---


`crates/geom-brep/src/certify.rs` `edge_extent` reads a `Curve3::Nurbs`
edge's lever arm as its chord, documented as a certified lower bound on
the point-set diameter "for the open fitted branches the zip mints".
The plane×cone hyperbola is now minted as a NURBS arc
(`ConicBranch::arc`), and on a steep arc the farthest pair of its points
is the vertex and an end, not the two ends. Measured
(`crates/geom-brep/tests/plane_cone_arc.rs`,
`the_chord_on_a_steep_arc`, the plane `x = 0.3` on cones about `z`, the
window symmetric about the vertex): `α = π/6`, `y = ±2`, ratio 1.00;
`α = 0.1`, `y = ±2`, chord 4 against a diameter of 17.28, ratio 0.23;
`α = 0.05`, `y = ±6`, chord 12 against 114.2, ratio 0.105. A lower
bound is the safe side (a smaller arm only escalates more), so this is
conservatism, not a wrong verdict, and no such arc reaches an at-rest
body yet (the axial door refuses it at its gate). A distance between
any two points of the edge is a lower bound too, so the chord could be
maxed with the distances from the carrier's mid-parameter point to
each end. Found by the OFFSET conic-arc unit (branch
`offset/conic-arc-and-axial-section`).
