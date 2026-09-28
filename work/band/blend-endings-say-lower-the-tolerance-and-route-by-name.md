---
id: blend-endings-say-lower-the-tolerance-and-route-by-name
kind: issue
title: blend: FILLET3_CONTACT_RECOURSE says 'lower the tolerance', and BlendError::Escalated routes its ending by predicate name
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of the offset-meters D4 ¶1 reshape,
`work/encl/offset-meters-follow-the-d4-recourse-ruling.md`.)

The D4 ¶1 ruling (Ev, `[ev]` PR 3352) derives a refusal's ending from
its decision and verdict. *Tighten the tolerance* is offered only on a
band-decided arm (in band, or Zero where Zero does not pass) of a
decision that passes on a nonzero sign, phrased conditionally and
valued: "if this size is intended, tighten the tolerance below m/K".
It is never offered on a sign-certain arm, nor on a residual. The
decision is a closed type at its site, so its recourse is an
exhaustive match, never a lookup by predicate name.
`geom_brep::recourse::SizedDecision` is that table for a sized decision
(lever, size noun, pass set, stored-definite reading); certification
(`geom_brep::certify::recourse`) and the offset meters
(`geom_brep::offset_meters::Meter`) both end through it.

## What

- `crates/sweep/src/blend/mod.rs`, `FILLET3_CONTACT_RECOURSE` (~:592)
  ends "...or blend a larger feature, or lower the tolerance".
  Unconditional and unvalued.
- **Routing by name.** `BlendError::Escalated`'s `Display` (~:1380–1398)
  matches `Some("fillet3_corner_independence" | ...)` on the predicate
  name and falls through to `geom_core::MissingRecourse`. The ruling
  asks for the decision's closed type, as
  `geom_brep::offset_meters::MeterError::Escalated` now carries a
  `Meter`.

`work/band/every-escalation-carries-the-coincidence-recourse-first.md`
is the neighbouring finding (the coincidence tail before the routed
sentence). This one is about the routed sentence itself.

## Repair shape

Carry the blend decision as a closed type on `Escalated`. Derive the
ending from (decision, arm) through
`geom_brep::recourse::SizedDecision` where the decision is sized, with
the conditional, valued tighten only on the band-decided arms.
