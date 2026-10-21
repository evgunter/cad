---
id: seg-reach-fuzz-skips-the-large-scale-draws
kind: issue
title: seg_reach_fuzz skips every draw whose scale puts its f64 oracle past a quarter of eps, about a quarter of the rivals draws at 1e-12, which are the large-scale draws where cancellation lives
status: open
opened: 2026-10-13
priority: P3
cost: M
---

Found by a review of #5032 (`arc-arc-secant-arm-misses-crossings-at-unequal-radii`).

## The gap

`crates/profile/src/seg_reach_fuzz.rs` `sweep` skips every draw whose
`oracle_slack`, `4u·scale` with the scale the largest coordinate or
centre-plus-radius it reads, exceeds a quarter of ε. The oracle works
in `f64`, so past that it cannot tell a pair within ε from one past it.
At ε = 1e-12 that is every draw with a scale past about 280. The
`rivals` family draws radii up to 10⁴ and centres up to 300, so it
skips the large-scale draws, which are where the circle-pair
cancellation of #5032 lived. A review counted 91,419 of the 400,000
`f64` draws per seed skipped at effort 10 and 1e-12; #5032's fix pass
re-measured 91,835 of 400,000 at `f64` and 22,979 of 100,000 at
`Interval` (seed `0x0e2443c99b7f6cce`), about 23% each, and none at
1e-9. The run prints the count (`past the oracle's resolution: …`) and
asserts nothing on those draws.

#5032 added a deterministic accuracy row for the placement itself
(`seg::circles_accuracy_tests::circle_pair_points_sit_within_a_few_ulps_of_scale`,
double-double residuals over near-concentric, near-tangent,
unequal-radii and far-out pairs), so the cancellation has a guard; this
row is the fuzz family's reach, not that guard.

## Where to look

The skip can go only if the oracle is an upper bound on the true
distance to within much less than ε at any scale. Endpoint and
point-to-carrier distances can be taken in double-double
(`crates/profile/src/dd.rs` `off_circle` already gives the
point-to-circle residual to about `1e-30` of scale). The crossing and
common-normal readings (`best = 0` where a crossing lies on both arcs,
`on_arc` near an end) decide membership by an angle rounded at
`u·scale/r`, and a membership read wrongly in the `0` direction
under-states the distance; such a candidate has to be dropped (an
upper bound stays sound) rather than read, unless it lies inside both
spans by more than that rounding.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEREST (`work/profilerest/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.
