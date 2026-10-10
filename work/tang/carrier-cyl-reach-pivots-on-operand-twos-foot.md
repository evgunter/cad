---
id: carrier-cyl-reach-pivots-on-operand-twos-foot
kind: issue
title: carrier_cyl_reach pivots on operand 2's foot, so its lever depends on which carrier is named first
status: open
opened: 2026-10-07
priority: P3
cost: E
refs: [cylinder-offsets-read-at-a-stored-origin-off-the-reach]
---


## What

`reading`'s cylinder arm (`crates/topo/src/boolean/carrier_eq.rs`, the
`carrier_cyl_reach` margin) pivots on operand 2's foot,
`pivot = reach.foot_on(o2, a2)`, measures `apart` from it to operand 1's
axis, and levers the tilt at `reach.lever_from(pivot) + apart`. It decides
the SUM, as `geom_brep::extent`'s module docs require, and it is sound:
either operand's foot gives an upper bound on the axes' displacement
across the reach. But the pivot depends on which operand is named first,
so one pair can read different margins in its two orders.
`Reach::lever_between` is the order-free lever; the lesser of the two
orders' bounds is sound as well.

It runs on UNDECLARED pairs too, so the D10 hold does not cover it:
`pair_door_verdict` → `coincident_as_declared` → `reading`, reached from
`boolean::reduce` and `flush` (PR 4231's review, probe
`review_4231_carrier_cyl_reach_reads_undeclared_pairs` on the
`tang/cylinder-offsets-review-probes` branch: two coaxial-within-band
cylinders escalate on `carrier_cyl_reach` at the face-pair door in both
orders).

`cylinder_data`'s `carrier_cyl_axis_offset` datum beside it is not a
defect: both callers hand it operand 2's foot (the undeclared ladder
through `at_consumed_extent`, and `reading`'s `pivot`), and the
rejection is perpendicular to `a1`, so it is already read at the reach.

Found by `tang/cylinder-offsets-at-the-reach`'s sweep; re-homed out of
the D10-parked item by PR 4231's review (MINOR-1).
