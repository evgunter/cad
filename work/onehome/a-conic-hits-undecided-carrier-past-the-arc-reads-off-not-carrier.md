---
id: a-conic-hits-undecided-carrier-past-the-arc-reads-off-not-carrier
kind: issue
title: A conic arc's hit reads an undecided whole carrier past the arc as Off, where the point may be on the carrier
status: open
opened: 2026-11-18
priority: P3
cost: M
---


Filed by ONEHOME's fork 275 step 3b (PR 7112), from its full review.

**The reading.** `ConicArc::hit` (`crates/topo/src/splitting/containment.rs:1013`)
reads a point the span reads past the arc with the carrier undecided at
the foot (`OffBy::Span { carrier: AtFoot::Undecided }`) by asking the
whole conic (`Curve3::on_carrier`). Where that refuses too, it answers
`ConicHit::Off`, though the point may lie on the carrier past the arc,
which is `ConicHit::Carrier`. The merge base did the same
(`Reading::Off` for a past point whose carrier refused).

**What the ray count makes of it: tolerated, never a wrong count.**
`Carrier` reaches the count only as `conic_crossings`' `on_carrier`
flag (`containment.rs:1869`). The flag skips a root at `q` whose advance
along the ray decides zero (`:2020`). Read `Off` instead, that root
either is not among the arc's roots (it lies past the arc, outside the
span the roots are read over), or it is met with its advance zero and
the flag false, which sets the ray aside (`Sign::Zero => return
Ok(None)`, "a crossing at `q` the pre-pass did not see"). Either way the
cost is a ray set aside, never a crossing miscounted.

**What would close it.** A typed refusal of its own for an undecided
whole carrier past the arc (the containment's escalation, as a straddle
at the foot already is), or a row pinning that such a point only ever
costs a ray, on a conic whose whole-carrier reading refuses past the arc.
