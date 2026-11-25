---
id: a-rim-on-a-spun-balls-seam-ships-one-edge-tier-3-prime-cannot-read
kind: issue
title: A keel's rim on a revolved ball whose vertex rests on the seam meridian is asked of no face by the parting and ships one edge, which tier 3′ reads as a boundary crossing itself, undecided
status: open
opened: 2026-11-25
priority: P1
cost: M
---


Found by `heldrecords/parting-division` (2026-11-25), on
`heldrecords/split-on` at `80f11deb86`, default ε. The parting never
reaches this edge, so it is the same on the base.

## Witness

The ball of radius `√2` revolved about `y` from the half-disc
`(0, −√2) → (0, √2)` (bulge 1), less the full revolve about `y` of the
triangle `(√2·sin .4, √2·cos .4), (.45, 1.15), (.7, 1.05)` turned 1 rad
about `z`. The triangle's apex sweeps the circle of angular radius 0.4
about `(−sin 1, cos 1, 0)` on the sphere, and both its cones go into the
ball. `B ∖ A` builds one solid at the closed form `11.761815046932…`,
tier 3 passes, and tier 3′ refuses
`BoundaryCrossesItselfUndecided { witness: (−0.7985, 1.1672, 0), why: Curved(Unreadable) }`.
The records are a vertex-on-edge record of the rim's vertex on the
ball's seam edge and an edge-edge record of the seam and the rim.

## What

`boolean/slit.rs` `riders` asks the rim (one closed edge, its vertex
on the seam meridian) of both of the ball's faces and returns no face
for it. So the rim keeps one edge where both its faces leave its line
into the sphere's material, the form the parting exists to replace
(`crates/topo/README.md`, tier 3). The same keel turned 1.2 or 0.3
about `x` is asked and parted, and builds sound
(`crates/sweep/tests/a_closed_rider_lying_in_a_face.rs`
`a_rim_on_a_sphere_divides_its_face`).

## Done when

`holding` says why it holds no piece of the rim (the crossing at the
rim's own vertex is the likely suspect), and the witness parts or
refuses typed.
