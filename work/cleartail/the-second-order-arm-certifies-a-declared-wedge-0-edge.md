---
id: the-second-order-arm-certifies-a-declared-wedge-0-edge
kind: issue
title: self-intersection's second-order arm is not built: a declared wedge-0/2π edge refuses Uncertified{SecondOrder} past CUSP_DEPTH
status: open
opened: 2026-10-28
priority: P2
cost: H
---


## Finding

The *Self-intersection scope* clause (`docs/ERROR-DESIGN.md`, E7)
lists three local certificates for a touching pair. SELFX built two:
departure and monotone (`crates/topo/src/injective.rs`). The third,
**second order at a declared wedge-0/2π edge**, is not built.
`docs/SELFX-SPEC.md` item 2 allows that order: land the first two,
have the cusp seam refuse typed, and file the arm as its own row. This
is that row.

## Where it stands

- `cusp` (`crates/topo/src/injective.rs:1863`) reads a wedge-0/2π
  shared edge: the two faces' outward normals are opposed at the
  middle of the side each presents on it, decided `Zero` at
  `self_intersection_cusp`. `departure` then returns `Outcome::Cusp`
  (`:1688`).
- The sweep (`crates/topo/src/clearance.rs:1092`) splits such a pair
  like any uncertified one until `CUSP_DEPTH` (`:151`, 10). Past that
  depth it refuses `EngineRefusal::Uncertified { arm: SecondOrder }`,
  naming the faces and the shared edge. The split depth keeps the
  cells away from the edge decided, and a violation there reported
  first.
- Pinned by
  `crates/sweep/tests/self_intersection_proves_injectivity.rs:363`
  (`a_declared_wedge_0_kiss_refuses_naming_the_second_order_arm`): the
  kiss refuses with `SecondOrder` on the kiss edge. It costs about 340
  cell pairs, and about 38 s unoptimized.

## What the arm owes

At a wedge-0 edge the two faces leave the edge on the same side, so no
first-order separator exists. The arm decides the sign of the faces'
separation at second order: the difference of their normal curvatures
across the edge, along the edge, under one presentation of both. When
the arm lands, the kiss row flips from a refusal to `Holds`.
