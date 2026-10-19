---
id: census-collinear-overlap-reads-along-the-first-edges-line
kind: issue
title: The census's collinear edge-edge test reads the second edge's offset along the first edge's line, so a short tilted first edge levers a within-zero pair into the band
status: closed
opened: 2026-10-11
priority: P3
cost: E
refs: [an-apex-edge-in-the-partners-face-plane-fails-tier-3-prime-on-one-edge-face-overlap]
closed: 2026-10-17
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
within the zero. `on_edge_interior` had the same shape
(`pm_census_ve_line_gap` read along the extended line before the span):
a point 1.2 past a 0.03 piece's end read 4.1e-9 there. FUSE's stage c3
(PR 5227) retired that row: the lane now reads `Span::contact`, the span
first and the line's gap (`curve_carrier_gap`) only inside it.

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

## 2026-10-12 — the lineage routes around it; the census reads as before (FLUSH, PR 4737)

CONTACTHOLD's PR 4740 (merged) gave the boolean's carried edge-edge
lineage a span-first read (`census::past_segment`,
`census::segments_apart`) and lets it take either piece's line, so the
op no longer meets this lever. PR 4737 took that design and left the
census's own reading order alone. Still open here:
- the confirm pass (`interiors_meet` → `collinear_overlap`) and pass 5's
  collinear lane, which read a pair in cell or sweep order;
- `on_edge_interior`'s line gap, read before the span.

A record of a short tilted piece and a long one can still escalate at
rest where the op that made it did not.

## 2026-10-13: names retired by PR 4820 (FUSE)

PR 4820 retired `ee_lineage`. An operand's carried edge-edge record
is now placed on the pieces of a split edge by `boolean::ops::carried`
(`operand_fate`, then `records::landings`), through the census's own
record confirm.

## Closed

2026-10-17, RESTREAD's crossing lane. `census::collinear_overlap` reads
the projected overlap first, then the gap as the least distance from
`ea`'s points over that run to `eb`'s line (`census::overlap_gap`):
where the two run side by side, so neither one's tilt is levered along
the other's extended line. Either reading's definite no settles the
pair. The confirm pass and pass 5 read it alike in either cell order.
Pinned by `census::edge_pairs_as_segments`
`a_short_piece_tilted_within_the_zero_overlaps_a_long_one_in_either_order`:
a 0.03 piece tilted 3.5e-9 against a long one starting 0.36 back read
1.26e-9 on main in one order and Zero in the other; both orders now
read the overlap with nothing pushed. `on_edge_interior`'s line gap went
span-first with FUSE's PR 5227.
