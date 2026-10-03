---
id: two-tangent-edges-parting-with-like-far-end-touches-stay-on-edge-both
kind: issue
title: Two tangent edges that leave a vertex-pair site together and part, with their far ends recorded alike, both read OnEdge and the join cannot pair either
status: open
opened: 2026-10-03
priority: P2
cost: H
design: true
refs: [a-germ-tangent-to-an-edge-reads-on-edge-where-its-arc-leaves-the-edge]
---


Found by JOIN-2 (PR 3880), fixing
`a-germ-tangent-to-an-edge-reads-on-edge-where-its-arc-leaves-the-edge`.

## What

`crates/topo/src/boolean/sectors.rs` `germ_loci`: where both sectors of
a germ at a vertex-pair site hold a bound read On, and the two edges
part (their far ends are not a recorded vertex pair), the segment runs
along at most one of them. `germ_loci` picks the one whose far end the
reduction recorded deeper in the partner (`Touch`: no contact, a vertex
pair, a vertex on a face). Two edges with the same `Touch` are left as
they read, both `OnEdge`. Then `join::partners` cannot pair either end,
and the op refuses `Join(UnpairedLooseEnds)`.

Two `Boundary` ends is the open shape: a line and an arc that leave a
site tangent, each ending on the other solid's boundary. The records
cannot tell which one lies inside the other's face. Which runs inside
is a trim fact, or second-order: the arc's curvature against the side
of the line its partner face lies on. Two `Apart` ends read `InFace` on
both sides, which is the truth: neither edge carries the segment.

No row in the suites or JOIN-1's batteries reaches the open shape
(measured on PR 3880's head). A fixture would be a rounded plate on a
rounded plate whose fillets are tangent at a shared vertex, with each
straight edge ending on the other's wall.

## Why it matters

It is the remaining case where a first-order reading names a germ's
cell. The fork is whether to decide it from the trim (a point of each
edge against the partner face) or from curvature. Neither is a record
the reduction already holds.
