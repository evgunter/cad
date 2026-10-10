---
id: lift-same-carrier-repair-is-reached-by-no-row
kind: issue
title: the lift's same-carrier repair and LiftRefusal::SameCarrierClose are reached by no row once the lift reads the derived tangent set
status: open
opened: 2026-10-09
priority: P3
cost: E
---

Found by stage 4 G (`tangent-joints-are-derived`). The lift
(`crates/profile/src/lift.rs`) now reads a source loop's tangent
junctions as validation derives them (`validate::table_tangent_joints`),
and every zero-turn joint is one, a same-carrier continuation included.
So `chain_form` spells every cocircular and collinear joint with a
continuation verb up front, and the census rows that used to reach the
repair now lift without it: `lift_census.rs`'s `unequal_split` (which
pinned `LiftRefusal::SameCarrierClose { joint: 2 }`) lifts value-equal,
and `collinear_run` (which pinned the driver's wall) lifts bit for bit.

What is left reaching `repair_same_carrier` and `SameCarrierClose` is a
junction the driver's turn band calls zero-turn (`JunctionTangent`,
`sin φ · arm`) while validation's carrier band reads it transversal —
two bands over different margins, so the window may be empty or may be
a sliver; no row exhibits either. Measure it: build the boundary case
(a line→arc or arc→arc junction swept across both bands) and either pin
the row that reaches the repair, or delete the repair, the variant and
`is_carrier_continuation`.
