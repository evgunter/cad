---
id: limb-two-has-two-decision-layers-over-one-reader
kind: issue
title: Limb 2 decides the one reader's reading in two layers: the edge door's and the search's
status: open
opened: 2026-11-22
priority: P3
cost: M
---

Filed by `sectcone/one-limb-two-reader` (PR 7350) at its review.

Since that PR, the search and the edge certificate read limb 2
against an analytic operand through one reader,
`pcurve_cache::limb2_offset_sup`, which returns a
`projected::OffsetRead` (the decided bound, a miss's floor, the net's
size, the uncleared parts). Two layers decide it, though:

- the edge door's `edge_nurbs::limb2`: at an enclosure scalar, one
  margin over `[floor, upper]` (`limb2_enclosure_as`) with the
  free-parameter residual as its identity at a family scalar; at a
  point scalar, a measured miss first, then the bound, which clears
  or is the kernel's approximation limit (`KernelLimit`), and a line
  against a plane decided on its measured ends (`measured_ends`);
- the search's `ssi::certify::analytic_limbs`: the miss's floor at
  `ssi_hull_value`, then `Margin::of(sup)` at `ssi_hull_sup`, with no
  kernel-limit story, no `[floor, upper]` enclosure margin and no
  measured ends.

C2 says the proof is the same at every door. The reading now is; the
verdicts on it are not. One decision function over `OffsetRead`, with
the doors differing only in what they name a refusal, would make them
one.
