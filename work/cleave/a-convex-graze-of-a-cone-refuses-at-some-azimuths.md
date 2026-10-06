---
id: a-convex-graze-of-a-cone-refuses-at-some-azimuths
kind: issue
title: a convex graze of a cone or filleted corner refuses at some azimuths before rule (b) reads it
status: open
opened: 2026-10-02
priority: P3
cost: M
---


## What

A plane grazing a convex curved wall from outside lands the body
whole on its material's side (`splitting/rules.rs`, `wall_graze`).
Some poses never reach that rule: they refuse earlier, with the same
payload on main and on PR 3892's head.

Each pose below was measured with both normals:

- Narrowing frustum (radii 1 → 1/2 over height 1, revolved about y),
  azimuth θ = 0.3: `Reduce(SliverSector)` on `split_conic_departure`.
- Widening frustum (1/2 → 1), θ = 2.9: `Reduce(SliverSector)` on
  `split_conic_departure`.
- Widening frustum, θ = 1.1: `Join(UnpairedLooseEnds { count: 2 })`.
- 6 × 4 slab with r = 0.5 fillet-door corners, NE corner wall grazed
  at φ = 1.2: `Reduce(SliverSector)` on `split_conic_departure`.
- Full cone (base r = 1, apex at y = 1), grazed along a ruling through
  the apex at θ ∈ {0, 0.7, 2}: `Reduce(SliverSector)` on
  `sector_straight` at the apex vertex.

At `CAD_TOLERANCE_EPS=1e-6`, narrowing θ = 0.3 and the slab at
φ = 1.2 ANSWER with the true volumes. The refusal is a band artifact,
not geometry.

The frustum and slab poses are allowed to refuse, and nothing else
is, in `crates/sweep/tests/split_tangent_edge_curved.rs`:
`CONE_GRAZES_REFUSED` for the frusta, and the φ = 1.2 arm of
`a_convex_graze_of_a_filleted_corner_lands_the_slab_whole` for the
slab. The allowance is not an exact pin, because it varies with ε.
Fixing this row means removing the allowance. The full cone is not
pinned.

The concave twin of the slab pose stops at the same door. The
rounded outline as a hole in a 10 × 8 plate, grazed from inside along
its NE corner wall at φ = 1.2, both normals
(`a_concave_graze_of_a_filleted_hole_refuses`, measured 2026-10-06 by
`cleave/concave-graze`):

- default ε: `Reduce(SliverSector)`, margin ≈ 3.9e-9, at the graze
  vertex before rule (b) reads the wall;
- `CAD_TOLERANCE_EPS=1e-6`: `Reduce(KnifeEdge)`, the graze read and
  refused as it should be;
- `CAD_TOLERANCE_EPS=1e-12`: the plane is read as cutting the wall
  rather than grazing it, and the join refuses
  `Euler(Certification { ResidualExceeded { check: EndpointStart } })`.

At 1e-12 the 3.9e-9 departure is outside the band, so the plane passes
through two roots ~4e-9 apart and the chord between them does not
certify. That is a near-graze cut refused at certification, not a band
refusal. The pin there only asks that the pose refuses. The conical
socket at θ = 0.3, which refused `SliverSector` at default ε on main, now
refuses the knife edge (`Reduce(KnifeEdge)`) instead, because the
wall is read at an earlier vertex. CLEAVE DR-4098's L-bracket with a
declared concave cove (r = 0.5) stops at the same door at φ = 4.2:
`Reduce(SliverSector)`, margin ≈ 5.3e-9, both normals.

## Where to look

`split_conic_departure` is the first-order departure trilean of a rim
arc at the inserted graze vertex (`splitting/neighborhood.rs`,
`classify_neighborhood`). At those azimuths the departure margin lands
in the band rather than at exactly zero, which would hand it to the
second-order descent. That is likely rounding in the inserted root's
position and the arc tangent there; this is a hypothesis, not traced.
`sector_straight` at a cone's apex vertex is a separate door. It refuses secants as well as
grazes: `cleave/seam-ruling-split`'s sweep put the full cone (base r = 1, apex at y = 1) through
the plane holding its ruling at θ ∈ {0, 0.3, 2}, turned t ∈ {1e-3, 0.05, 0.4, 1, π/2, 2, 3, −0.4,
−1.2} off tangency, both normals. Every pose refused `Reduce(SliverSector)` at the apex vertex, at
ε 1e-6, 1e-9 and 1e-12, on main at 78bee3ac68. Each such plane holds the apex, so each side is a
cone over a base segment, with closed-form volumes.

Measured again on branch `cleave/frustum-apex`, which pairs a two-ruling section's
crossings along each ruling: the full cone through a ruling at a = 0.3, t = 0.05 still refuses
`Reduce(SliverSector { predicate: "sector_straight" })` at the apex vertex, margin exactly 0.0, in
the reduction. That is before any pairing, so it is not the frustum's top↔top pairing defect
(`a-frustum-split-through-a-ruling-off-its-seam-refuses-a-degenerate-section`). Once the
reduction passes, a null-edge half at the apex lies on both rulings, so `ruling_pairs`
(`splitting/join.rs`) will refuse it undecided on `split_join_ruling_side` wherever the wall
face holds more than two halves. That is unmeasured, because nothing reaches it yet.

## Found by

CLEAVE DR-51's review of PR 3892, `review-tests/dr51` (74e151b6).
