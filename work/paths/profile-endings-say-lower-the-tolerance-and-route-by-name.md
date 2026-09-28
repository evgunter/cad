---
id: profile-endings-say-lower-the-tolerance-and-route-by-name
kind: issue
title: profile: the path door's turn refusal and FILLET_OFFSET_LEVER_RECOURSE say 'lower the tolerance', and PathError::Escalated routes its ending by predicate name
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

- `crates/profile/src/path.rs`, the turn refusal's `Display` (~:1612,
  and the variant doc ~:827): "...otherwise move the geometry (or lower
  the tolerance)". Unconditional and unvalued, and the margin
  (`turn margin {margin} m`) is right there to value it.
- `crates/profile/src/validate.rs`,
  `FILLET_OFFSET_LEVER_RECOURSE` (~:427): "...or lower the tolerance".
  `work/round/fillet-inband-recourse-drops-the-tolerance-lever.md`
  covers which fillet gates have a tolerance lever at all (only the
  linear-band ones). Under the ruling, where it is a lever, it is the
  conditional, valued tighten, not "lower".
- **Routing by name.** `PathError::Escalated`'s `Display` (~:2008–2016)
  routes by `source.predicate` through `validate::shared_clause_only`
  and falls through to `geom_core::MissingRecourse`. The ruling asks
  for the decision's closed type, as
  `geom_brep::offset_meters::MeterError::Escalated` now carries a
  `Meter`.

## Repair shape

Carry the decision (a closed enum of the path door's gates) on the
escalation. Derive each ending from (gate, arm), through
`geom_brep::recourse::SizedDecision` where the gate is sized. Re-pin
the path door's rendered-text rows.
