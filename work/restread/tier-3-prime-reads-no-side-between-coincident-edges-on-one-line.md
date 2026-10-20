---
id: tier-3-prime-reads-no-side-between-coincident-edges-on-one-line
kind: issue
title: Tier 3′ licenses coincident edges on one line by their shared ends and never reads their wedges
status: open
opened: 2026-10-19
priority: P2
cost: E
---


Found by `census-passes-a-face-inside-an-edges-material-wedge`'s lane
(branch `restread/face-in-edge-wedge`), by reading.

## What

Pass 5's collinear lane (`census.rs` `ee_collinear_lane`,
`ee_bound_backed`) certifies two coincident line edges on one line by
their bounds alone (shared vertices or one point key), and reads no
side. D1 (`crates/topo/README.md` tier 3) has the material sectors on
one line disjoint, and tier 3′ "reads every touch for sides, the
coincident edges on one line included".

The Boolean result gate runs `census::line_wedges` over every line
`census::collinear_overlaps` finds (CLEAVE,
`boolean-doors-build-overlapping-coincident-edges-from-inputs-that-hold-none`),
so no Boolean result reaches rest with overlapping wedges. Tier 3′
does not: STEP adoption admits any body tier 3/3′ passes, and
`insert_voids(box, [s, s], cavity)` grafts the unequal-pinch touch as
two 270° edges whose wedges overlap by 180°, which tier 3′ refuses
only for want of records
(`work/topo/revert-re-pairs-the-sheets-on-a-line-where-coincident-edges-meet.md`).

## Fix

Run `census::line_wedges` in the census, on the lines where pass 5
certifies a collinear overlap, and refuse an overlap typed beside
`FaceThroughEdgeWedge`. Witnesses: a reverted slit body handed to
`validate_pseudomanifold` directly, and the `insert_voids` graft
above with its records.
