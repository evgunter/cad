---
id: certify-admits-an-ellipse-wound-past-its-period-on-the-slow-rate
kind: issue
title: certification's ellipse and spiric winding check admits Zero on the inf rate alone, so an edge wound past τ by up to about ε·a/b metres certifies
status: dispatched
opened: 2026-10-25
priority: P1
cost: M
rides_with: period-headroom-margin-has-no-shared-home
branch: pred/period-turn-rule
---

Found independently by both designers who weighed
`period-headroom-margin-has-no-shared-home` (2026-10-25). Read, not
executed.

`geom_brep::certify`'s check 2 decides the ellipse and spiric winding
(`interval_span_winding`, through its `span_at_floor` closure) as
`(τ − w)` levered at the carrier's LOWER speed (`min(|a|, b)` on the
ellipse). A `Zero` there is admitted. But `Zero` claims the extent is
metrically within the band of a whole turn, and only an UPPER rate
certifies smallness. On an eccentric ellipse the overlap can sit where
the speed is high.

Worked example, from the designer's report: a = 20, b = 1, ε = 1e-9
(escalation at 10ε). An edge stored 0.9e-9 rad past τ, with the
overlap where the speed is about 20:
- certification reads −0.9e-9 at b and admits the edge (`Zero`);
- `topo::splitting::containment`'s `ConicArc::of` reads the overlap's own
  speed, about −1.8e-8. That is definitely negative, so it escalates on
  an edge certification passed.

The two doors disagree on the same edge. Certification is the
unsound one: a doubled stretch roughly 20ε long carries a certificate
saying it is not.

Fix: the period-headroom unit's two-sided rule (definite `Over` at the
inf rate, `Whole` only when the sup rate is in band, escalate between)
applied at certification. It rides that unit. Owed with it: a red-first
row at these numbers, and a census of whether an import or construction
door can actually reach such an edge. If one can, re-band this P0.

## Fixed, with its census (PR 5686, unit 1 of period-headroom)

Certification decides the ellipse and spiric winding through
`geom_core::period::Period::turn`. The red-first row is
`crates/geom-brep/tests/certify_turn_rule.rs::an_ellipse_wound_past_its_period_on_the_fast_side_does_not_certify`.
It certified on the pre-fix tree and escalates on
`interval_span_winding_straddle` now.

**Census of the doors that write a conic span near a whole turn.** No
shipped door is known to reach the edge at a sane size, so the row
stays P1.

Every production writer found leaves the span past τ by at most a few
roundings of `t + τ`, call it δ ≲ 2e-15 rad:
- STEP import (`step-import/src/geometry.rs` `endpoint_params`):
  `t1` is wrapped into `(t0, t0 + τ]`, and a self-loop is exactly
  `t0 + τ`.
- `topo::chord_join`'s `oriented_arc`: `th1 + span`, with
  `span = (th2 − th1).reduce_periodic(τ)` (`∈ [0, τ)` up to rounding)
  or exactly τ for a whole turn.
- `Real::reduce_periodic`'s other callers (`splitting/classify.rs`,
  `shell.rs`, `boolean/boxes.rs`, `profile/src/sugar.rs`): they place a
  parameter inside a span; none writes an edge's span end.

To straddle, the overlap must read Zero at the floor and past the band
at the sup: `δ·b ≤ ε` and `δ·sup ≥ 10ε`. With δ ≈ 2e-15 that needs a
semi-major of at least `10ε/δ`:
- ε = 1e-6: a ≳ 5e9 m;
- ε = 1e-9: a ≳ 5e6 m;
- ε = 1e-12: a ≳ 5e3 m — closer to reach. At that size the model's
  extent over ε is past f64's resolution (≈ 4.5e15), so nothing built
  there is sound for other reasons.

The one door that reaches the edge at any size is the public
`EdgeCurve::certify` with a caller-built `EdgeCurveSpec`, which is
certification itself.

The sweep was a grep for production writers of a conic span end past τ:
`param_end: … TAU`, `(t1|end|hi) (+=|= x +) tau`,
`+ (T::tau()|tau|TAU)`, and `reduce_periodic(` callers. It cannot match
an end computed through a helper that spells neither. `split_specs`
pieces are sub-spans of a certified edge.
