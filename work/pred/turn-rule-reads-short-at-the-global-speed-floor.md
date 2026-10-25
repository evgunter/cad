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
