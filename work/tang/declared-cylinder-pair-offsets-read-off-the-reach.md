---
id: declared-cylinder-pair-offsets-read-off-the-reach
kind: issue
title: chart_region_cyl_offset reads each declared cylinder's stored origin against the other's axis, levered from the stored origins
status: parked
opened: 2026-10-07
priority: P2
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
refs: [cylinder-offsets-read-at-a-stored-origin-off-the-reach]
---

Split off `cylinder-offsets-read-at-a-stored-origin-off-the-reach`
when that item closed: this site runs only on DECLARED pairs (Door 2's
cylinder enclosure, `declared_pair_overlap`, which consumes Door 1's
`ContactVerdict`), and the D10 hold (JOIN's log, Ev, 2026-10-03: no new
unit that meaningfully uses declared pairs or declared contact) keeps it
out of that unit. Parked until the hold lifts.

## What

`cylinder_pair_overlap` (`crates/topo/src/chart_region.rs`), the
`chart_region_cyl_offset` row: `perp_a`, `perp_b` are each stored
origin's distance from the OTHER description's axis, after
`chart_region_cyl_tilt` admits a tilt levered at
`hyp = √(reach² + r_max²)`, `reach` the trims' largest axial coordinate
about either STORED origin. It is sound (the lever grows with the stored
origin's distance), but both rows read further from the trims than the
trims are: a pair whose origins are stored 1000 m along refuses
`CarrierTilt` for a tilt that is in the band at the trims. Read the
offset at the feet of the trims' centre on each axis and lever `hyp`
about those feet.

The transfer parameter `c = (o_b − o_a)·â_a` beside it is NOT a defect:
B's chart `v` is measured from `o_b`, so `c` has to be `o_b`'s
A-coordinate, and to first order the transfer error's axis term is
`−[(q − o_a)⊥â_a]`, `q` each trim point's own foot on B's axis, whatever
`o_b` is.

Found by `tang/cylinder-offsets-at-the-reach`'s sweep.
