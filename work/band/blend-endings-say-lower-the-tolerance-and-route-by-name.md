---
id: blend-endings-say-lower-the-tolerance-and-route-by-name
kind: issue
title: blend: FILLET3_CONTACT_RECOURSE says 'lower the tolerance', and BlendError::Escalated routes its ending by predicate name
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of PR 3382.) The rule is D4 ¶1 (i)
in `docs/DESIGN.md`; `geom_brep::recourse::SizedDecision` is the shared
ending table for a sized decision.

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
