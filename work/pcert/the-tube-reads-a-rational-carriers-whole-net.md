---
id: the-tube-reads-a-rational-carriers-whole-net
kind: issue
title: The analytic rung-3 tube reads a rational carrier's whole net: no piece of it has f64 weights at an enclosure scalar
status: open
opened: 2026-10-08
priority: P3
cost: M
---


Found by `pcert/projected-image` (PR 4304, the confirming review's
delta).

## What happens

`edge_nurbs::analytic_rung3` states C2's limbs over the edge's interval.
Limb 2 reads the stored net through a window
(`pcurve_cache::projected::net_offset_sup`), so it needs no cut. The
tube (`ssi::certify::certify_branch`, `Limbs::Tube`) takes a
`NurbsCurve3<T>`, so it reads a piece of the carrier cut to the interval
(`edge_nurbs`'s `edge_piece`). `NurbsCurve3` stores `f64` weights. For a
carrier whose weights are all one value the piece's weights are that
value exactly and its controls are formed with ratios in the scalar,
which is certified. For a rational carrier with unequal weights the
piece's true weights are not `f64`: an `f64` insertion plan would re-round
them into a neighbouring curve's, which the review ruled out. So the tube
reads the whole carrier.

That is sound, because the chain around the piece is a stretch of the
whole carrier's chain. It is conservative, because geometry past the
edge's ends can refuse the edge. The row is
`analytic_rung3_certificate::a_rational_carriers_tube_reads_its_whole_net`:
an edge on a Steinmetz stretch clear of the crossing refuses on it.

## Open

A tube that takes its chain as homogeneous enclosures (`w`, `w·P` per
control, both in the scalar), or a window on `certify_branch`'s carrier,
would let a rational piece be read as certified.
