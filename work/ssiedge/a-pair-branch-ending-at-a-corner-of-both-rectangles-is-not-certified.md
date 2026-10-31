---
id: a-pair-branch-ending-at-a-corner-of-both-rectangles-is-not-certified
kind: issue
title: A NURBS x NURBS branch that ends at a corner of both knot rectangles is not certified: limb 3's end walk refuses an end on four domain faces at once
status: open
opened: 2026-10-31
priority: P2
cost: M
---


## What

The pair lane's handed-in corner (`ssi/pair.rs`, `PairLane::boundary_pass`,
`PairCorner`) reads the section's direction over a corner cell. Where every
component points into the domain, or every one points out, a branch starts at
the corner, and the pass adds the corner state as a crossing
(`PairLane::corner_crossing`). The trace runs from there, but limb 3's one-arc
proof (`four::one_arc_pair`, its end reading `reaches_end`) refuses that end:

> at 20 rungs the uniqueness tube was a graph but not proved one arc; at the
> narrowest, the walk at an end of the carrier resolved nothing, and that end
> lies on 4 domain sides at once, where no face's walk isolates its root

Witness: `topo` `section_cert_spline_pair_rows::a_shared_corner_is_a_region_or_a_crossing`.
That is the flat sheet `z = 0.2xy` against the plane `z = x − y/2` over
`y ∈ [0, 1.5]`, sharing the corner `(0, 0)` of both rectangles. The row pins
the refusal (R-reach at the interior-loop guard), which is sound but blocks a
boolean whose two spline faces cross out of a shared vertex at a corner of
both.

## What is open

An end reading for a carrier end at a corner of both rectangles. The corner
cell's direction reading (all four components one-signed over the cell)
already proves the section there is one monotone arc. It may be enough to
take the corner cell as the end's box in place of the face walk.
