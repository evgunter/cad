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
