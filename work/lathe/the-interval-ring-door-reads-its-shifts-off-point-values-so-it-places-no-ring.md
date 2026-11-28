---
id: the-interval-ring-door-reads-its-shifts-off-point-values-so-it-places-no-ring
kind: issue
title: At Interval, face_lift_as reads the periods and the ring's point off point values, so the door places no ring on a sphere face
status: open
opened: 2026-11-25
priority: P2
refs: [an-analytic-outer-loops-whole-span-boxes-never-tighten-under-the-ring-reader]
cost: E
---


Found in the re-review of PR 7581 (`lathe/outline-by-decision`) as NEW-1.
It is pre-existing, and it was demonstrated there.

## What

`crates/topo/src/pcurves_rings.rs` `face_lift_as` hands `place_ring` two
readings taken through `value_channel`:

- the periods, `lengths = [periods.u(), periods.v()].map(|p| p.map(|p| p.length().value_channel()))` (line 440);
- the ring's reference point, `[at.x.value_channel(), at.y.value_channel()]` (line 447).

`value_channel` is the scalar's point value, which is NaN for any interval
that is not a point. At `Interval` the period `2π` is an enclosure and the
reference point an interval, so both readings are NaN. `place_ring` then
reads the `u` channel as open (`periods[k].filter(|p| *p > 0.0)` drops a
NaN) and a NaN candidate shift. Every ringed sphere face of an `Interval`
body answers `Undecided { ring_place_arm, MarginDiag(Invalid) }`: the
Interval door places no ring.

## The class

It is the class PR 7581 fixed in `Outer::sheet_v`, the `v`-period copy
counted off `value_channel`. That fix counts the copy, and the period
length, off the brackets' middles (`structure_bracket` →
`geom_core::mid_param`), since which copy is read is gauge, not a decision.
The two readings above are the same kind: inputs to the candidate count
and the offsets, not decisions. Read them off the brackets the same way,
and pin a ringed sphere face of an `Interval` body placing its ring.
