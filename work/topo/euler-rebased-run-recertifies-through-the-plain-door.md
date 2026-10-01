---
id: euler-rebased-run-recertifies-through-the-plain-door
kind: issue
title: certify_rebased_run re-certifies a moved run through the lane-free door, so an M7-8 edge in the run refuses NurbsLaneNotSupplied at a scalar that holds the lane
status: open
opened: 2026-10-01
priority: P3
cost: M
refs: [graft-recertifies-through-the-narrow-lane, the-re-basing-gate-refuses-m7-8-where-nothing-moves]
---


Found by CLEAVE's plane x NURBS lane unit while sweeping for generic
operations that re-certify an edge carrier through the lane-free door.
**Executed on main:** `euler::tests::a_fan_mev_refuses_the_plane_x_nurbs_class_where_nothing_moves_and_mev_null_splits_it`
drives a public `mev` fan on an M7-8 pillow at `f64` and pins the
refusal (`RebasedCarrier { NurbsLaneNotSupplied }`).
The closed row `the-re-basing-gate-refuses-m7-8-where-nothing-moves`
recorded the over-refusal where nothing moves and closed on the
bit-equality question; this row is the other half of the remedy, which
that row could not take: the lane itself is now a per-scalar policy
answer.

`Body::certify_rebased_run` (`crates/topo/src/euler.rs`, the fan
site's re-basing gate) re-certifies every edge of a run about to start
at a moved vertex through `EdgeCurve::recertify`, which supplies no
plane x NURBS lane. An edge of the M7-8 class (an `Intersection` of a
plane and a described NURBS wall) in that run therefore refuses
`CertifyError::NurbsLaneNotSupplied` before any check of the edge runs,
at `f64` too, where `AtRestPolicy::nurbs_lane()` holds the lane and the
at-rest validator re-derives the same certificate. The gate then reads
that refusal as a non-endpoint certification failure.

It is the transform's split (closed by the CLEAVE unit that made
`transform_rigid` read `T::nurbs_lane()`) at an Euler door, and it
belongs with the mint-door collapse (`Body::set_edge_curve` reading the
policy), which raises the same bound: `certify_rebased_run` is
`T: Decide` under the Euler surface.

**The fix shape:** the gate reads `T::nurbs_lane()` once its bound
carries `AtRestPolicy`, which is the same bound raise as the mint-door
collapse; at a dual it keeps refusing, typed, naming the scalar.
