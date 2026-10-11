---
id: census-collinear-overlap-reads-along-the-first-edges-line
kind: issue
title: The census's collinear edge-edge test reads the second edge's offset along the first edge's line, so a short tilted first edge levers a within-zero pair into the band
status: open
opened: 2026-10-11
priority: P3
cost: E
refs: [an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap]
---


Measured by FLUSH's `an-apex-edge-…` (branch
`flush/apex-edge-in-face-plane`) in the boolean's carried-record
lineage, which asks this question through
`census::segment_interiors_meet`. Read, not measured, at rest.

## What

`census::collinear_overlap` (`crates/topo/src/census.rs`) decides
`pm_census_ee_line_gap` as the distance of `eb.p0` from `ea`'s line,
read along `ea.dir`. Where `ea` is short and tilted within the zero
against a long `eb`, `eb.p0` sits far along `ea`'s extended line, and
the tilt levers into the band. `topo` `three_solids_on_one_line`
`carrying_each_steps_records_serves_the_same_body` ("three, one moved
0.1ε"): a 0.03-long piece tilted by 1e-10 read against a piece
starting 0.36 along the line gives 1.1e-9 and 1.19e-9 in a band of
`[1e-9, 1e-8]`, while the same pair read along the long piece is
within the zero. `on_edge_interior` has the same shape
(`pm_census_ve_line_gap` read along the extended line before the span):
a point 1.2 past a 0.03 piece's end read 4.1e-9 there.

FLUSH's lineage now reads along the longer piece and skips pieces
decided apart along the line (`boolean::ops::ee_lineage`). The census's
confirm pass reads an edge-edge record in its canonical cell order, so
a record of a short tilted piece and a long one can escalate at rest
where the op that made it did not.

## Owed

Read the collinear test along the longer edge, or as the symmetric
distance of each edge's ends from the other's segment. Measure a pair
first: a long edge and a short piece of a within-zero tilted edge
recorded together, confirmed at rest in both cell orders.

## 2026-10-11 — the confirm pass reads along the longer edge (FLUSH, PR 4737)

PR 4737's fix pass puts the reading order in the census, where the op
and the confirm pass share it: `census::interiors_meet` (an edge-edge
record's confirm, and `segment_interiors_meet` for the boolean's
lineage and span records) hands `collinear_overlap` the longer edge
first (`longer_first`, a plain comparison of the lengths' value
channels, ties and pointless values keeping the given order). So a
record's confirm no longer depends on its canonical cell order.

Still open here:
- pass 5's own collinear lane (`ee_collinear_lane`), which reads a
  found pair in sweep order;
- `on_edge_interior`'s line gap, read before the span, so a point far
  past a short edge's end is read off its extended line (the boolean's
  lineage routes around it by `census::segment_past`; the census's
  sweeps do not).
