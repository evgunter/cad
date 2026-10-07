---
id: declared-cylinder-pair-offsets-read-off-the-reach
kind: issue
title: the declared cylinder pair's offset rows read at a stored origin or one operand's foot: chart_region_cyl_offset, carrier_cyl_reach's pivot, carrier_cyl_axis_offset
status: parked
opened: 2026-10-07
priority: P2
cost: M
blocked_on: [d10-one-way-to-say-intent-is-unbuilt]
refs: [cylinder-offsets-read-at-a-stored-origin-off-the-reach]
---


Split off `cylinder-offsets-read-at-a-stored-origin-off-the-reach`
when that item closed: these sites run only on DECLARED pairs (Door 1's
ladder and Door 2's cylinder enclosure), and the D10 hold (JOIN's log,
Ev, 2026-10-03: no new unit that meaningfully uses declared pairs or
declared contact) keeps them out of that unit. Parked until the hold
lifts.

## What

- `cylinder_pair_overlap` (`crates/topo/src/chart_region.rs`), the
  `chart_region_cyl_offset` row: `perp_a`, `perp_b` are each stored
  origin's distance from the OTHER description's axis, after
  `chart_region_cyl_tilt` admits a tilt levered at
  `hyp = √(reach² + r_max²)`, `reach` the trims' largest axial
  coordinate about either STORED origin. It is sound (the lever grows
  with the stored origin's distance), but both rows read further from
  the trims than the trims are: a pair whose origins are stored 1000 m
  along refuses `CarrierTilt` for a tilt that is in the band at the
  trims. Read the offset at the feet of the trims' centre on each axis
  and lever `hyp` about those feet. The transfer parameter
  `c = (o_b − o_a)·â_a` beside it is NOT a defect: B's chart `v` is
  measured from `o_b`, so `c` has to be `o_b`'s A-coordinate, and to
  first order the transfer error's axis term is the offset of each trim
  point's own foot on B's axis from A's axis, whatever `o_b` is.
- `reading`'s cylinder arm (`crates/topo/src/boolean/carrier_eq.rs`,
  `carrier_cyl_reach`): pivots on operand 2's foot
  (`reach.foot_on(o2, a2)`) and measures `apart` from it to operand 1's
  axis. It decides the sum, but the pivot depends on operand order;
  `Reach::lever_between` is the order-free lever.
- `cylinder_data` (`carrier_eq.rs`, the `carrier_cyl_axis_offset`
  datum, reached from the declared `relation` arm): `perpendicular(p2 −
  p1, a1)` at the raw stored origins. It names the contradiction rather
  than deciding it, but the datum it names is read off the reach.

Found by `tang/cylinder-offsets-at-the-reach`'s sweep.
