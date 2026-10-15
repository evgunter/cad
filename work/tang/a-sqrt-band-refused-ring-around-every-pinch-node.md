---
id: a-sqrt-band-refused-ring-around-every-pinch-node
kind: issue
title: A √band refused ring around every pinch node: the crossing sweep reads a family edge through the quadric
status: open
opened: 2026-10-15
---


For two equal-radius cylinders whose axes meet (the U1 pinch family of
`pinch-carrying-machinery-valence-4`), an operand edge that lies on one
wall and passes within about √band of a node refuses, although U1 places
the node definitely inside the face. On main, at the witness band (1e-9),
with two extruded rods crossing at 90°:

| Pose | Refused | Builds |
|---|---|---|
| Seam turned φ off the node | φ = 0 … 1e-6 (`CurvedPierceUnsupported`); escalates at 1e-4 | from φ = 3e-4 |
| Branch cap dz from the axes' meeting point | dz = 0 … 1e-4 | from dz = 3e-4 |

Cause: the crossing sweep reads such an edge against the partner wall
through the full quadric. Near a node the edge is nearly tangent to the
partner wall, so its two real crossings, δ apart, show a discriminant
margin of order δ², and `line_wall_roots` reads Zero as
`WallRoots::Tangent` (`crates/topo/src/boolean/solid_contain.rs:4491`,
reached from `crates/topo/src/boolean/reduce.rs:3598` and `:3641`). Node
placement decides at δ and the sweep at δ², so between band and √band
the two disagree. U1's rows avoid the ring by posing their seams at
φ = 0.4.

The fix the U2 design fork converged on is to read the partner wall
through the two bisector planes: on wall A, f_B = (w·(â₁−â₂))(w·(â₁+â₂))
with w = x − p, so each crossing is a first-order line×plane or
conic×plane root, δ/sin(θ/2) or δ/cos(θ/2) from the node. Closure is
that design's U2a acceptance rows (seam and cap offsets of 1e-8, 1e-6
and 1e-4 build at the closed forms; band offsets escalate). Found by the
fork's designer probes (a temporary test in
`crates/sweep/tests/bisector_nodes.rs`, reverted).
