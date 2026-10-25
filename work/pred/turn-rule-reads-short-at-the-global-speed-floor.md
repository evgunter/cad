---
id: turn-rule-reads-short-at-the-global-speed-floor
kind: issue
title: The turn rule's Short and Over read an ellipse's global speed floor, so a near-whole arc on its fast side escalates where a local inf over the defect would decide it
status: open
opened: 2026-10-25
priority: P3
cost: E
---


Found building unit 1 of `period-headroom-margin-has-no-shared-home`
(branch `pred/period-turn-rule`).

`geom_core::period`'s turn rule reads `Short` and `Over` at a
`Bounded` rate's inf. Both callers pass the carrier's GLOBAL speed
floor there: certification's check 2 `min(|a|, b)`
(`geom_brep::certify`, `run_checks`, the `bounded` closure) and
`topo::splitting::containment`'s `ConicArc::of` `a.min(b)`. The sup is
already local, `geom::carrier_speed::turn_defect_speed_hi`: the speed at
the defect's middle plus `max(|a|, |b|)·|Δ|/2`.

On an eccentric ellipse, a span short of a turn by a gap on the fast
side reads `Zero` at the floor and definitely positive at the sup. That
straddles, so certification escalates an arc whose gap is definitely
there. The rows are in `crates/geom-brep/tests/certify_turn_rule.rs`
(`certification_agrees_with_the_old_winding_check`, the
`("ellipse fast", "inf", 0.9, false)` delta). A local inf, `speed_at(m)
− max(|a|, |b|)·|Δ|/2` floored at `min(|a|, b)`, is sound for the same
reason the sup is, and would decide those cases `Short`. The forward
decision keeps the global floor, since it reads the whole span.

Whether to take it is a call on admission. The cases are rare: a gap
within ε/b of a whole turn, which is a sliver at the vertex. The
escalation they get is sound, so nothing is wrong today. P3.

## The sup side, from the dual review (PR 5686)

The spiric arm's sup is also global and not tight:
`geom::carrier_speed::speed_range`'s `spiric_period_rate_bounds`,
over the whole period. Near tangency it loosens badly:
- at `d = R − r − 1e-6` it is about ×354 the sampled speed;
- a whole oval at `d = R − r − 1e-12` with ε = 1e-12 now escalates
  where it used to certify, because `h·sup` leaves the zero band on a
  rounding-sized `h`.

At the tangent limit `d = R − r` the sup is infinite. A whole oval there
escalates with an `Invalid` margin (pinned in
`crates/geom/tests/r2_turn_probes.rs`).

A local sup over the defect, as the conic has, would fix both. It is the
same refinement as the local inf above, on the other bound.
