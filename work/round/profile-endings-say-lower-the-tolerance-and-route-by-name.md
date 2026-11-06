---
id: profile-endings-say-lower-the-tolerance-and-route-by-name
kind: issue
title: profile: the path door's turn refusal and FILLET_OFFSET_LEVER_RECOURSE say 'lower the tolerance', and PathError::Escalated routes its ending by predicate name
status: closed
opened: 2026-09-28
priority: P1
cost: M
rides_with: the-path-doors-escalation-carries-its-gate
closed: 2026-11-06
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

- **The ending table is keyed by name** (ENCL, PR 4780):
  `validate::DECISION_ENDINGS` gives each decided name an `EndingKind`,
  read by both `PathError::Escalated` and `ProfileError::Escalated`. The
  closed type below retires the name key with the routing.
- **`JunctionTangent` contradicts its decision's kind.** The
  `path_junction_turn` row is `Lever(JUNCTION_TANGENT_LEVER)`: an
  undecided turn offers the lever alone, while the definite
  `JunctionTangent` still ends "(or lower the tolerance)".

## Repair shape

Carry the decision (a closed enum of the path door's gates) on the
escalation. Derive each ending from (gate, arm), through
`geom_brep::recourse::SizedDecision` where the gate is sized. Re-pin
the path door's rendered-text rows.

## Re-homed at PATHS's close (2026-10-21)

Moved from `work/paths/` to PROFILEPROSE (`work/profileprose/`) when PATHS closed. PATHS's live rows went to four programs opened at this close and cut along the priority seam (PROFILE, PROFILEREST, PROFILETAIL, PROFILEPROSE; `docs/doc-ledger/paths-leaves-the-tracker.md`). The item's id, content and history are unchanged.

## Rides with `profile-refusals-say-what-they-mean` (2026-10-26)

PROFILEPROSE folded into PROFILETAIL; its refusal-text rows travel as one unit. This row is that unit's spec for its part and closes with its own evidence when the unit lands.

## Claimed by ROUND (2026-11-06)

Moved from `work/profiletail/` to ROUND by its orchestrator, now riding
`the-path-doors-escalation-carries-its-gate`, which carries ROUND's
fillet-recourse rows on the same arms. Id and body unchanged.

## Closed (2026-11-06)

PR 6546: one closed `profile::Gate` routes every in-band ending at the path door, validation and the guided door, derived from (gate, verdict) per D4 ¶1 (i)–(ii); see the carrier `the-path-doors-escalation-carries-its-gate` and the PR body for what this row's part became.
