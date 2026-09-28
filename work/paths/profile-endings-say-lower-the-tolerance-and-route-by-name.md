---
id: profile-endings-say-lower-the-tolerance-and-route-by-name
kind: issue
title: profile: the path door's turn refusal and FILLET_OFFSET_LEVER_RECOURSE say 'lower the tolerance', and PathError::Escalated routes its ending by predicate name
status: open
opened: 2026-09-28
---

(ENCL implementer, from the §5 sweep of PR 3382.) The rule is D4 ¶1 (i)
in `docs/DESIGN.md`; `geom_brep::recourse::SizedDecision` is the shared
ending table for a sized decision.

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
