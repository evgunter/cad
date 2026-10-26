---
id: a-union-touching-along-a-coincident-rim-records-only-its-vertex-pair
kind: issue
title: A boss over a concentric hole of its own radius unites as two solids touching along the rim, and the union's records name only the vertex pair
status: open
opened: 2026-10-25
priority: P2
cost: M
---


Found measuring
`a-boss-over-a-hole-tangent-inside-its-rim-at-its-vertex-unites-as-two-shells`.
Pinned by `crates/sweep/tests/a_closed_conic_in_the_partners_face.rs`
`a_boss_over_a_hole_by_pose` (its `Reads::Unrecorded` cells).

The boss `r = 0.3`, `z ∈ [0, 1]`, over a hole or a pocket `r = 0.3`
about the same axis: `union` in either order returns two shells, the
boss and the holed plate, at the closed-form volume, tier 2 and the
certificate passing. That body is right: the two solids touch along the
rim circle and nowhere else, and pieces that only touch are distinct
solids (`crates/topo/README.md`). Its contact records name only the
vertex pair at the rims' vertex, so tier 3′ fails `UnrecordedContact {
contact: EdgeEdgeOverlap }` on the two rim edges, beside
`CensusUndecidable` on the curved pairs. Measured with the vertices at
one azimuth and with the hole's at 1 (the boss's rim is split there),
one-segment and two-arc forms, through and pocket, at the default ε,
1e-6 and 1e-12, alike.

At the vertex pair the floor's sector and the top face's sector share
the rim's tangent with equal bends (`sectors.rs` `crescent` reads them
as touching), so the pair is a touch, and the edges that run along it
from the vertex are never recorded as coincident. `boolean/ops.rs`
`BooleanBody` does not run the census, so the body ships without the
record.
