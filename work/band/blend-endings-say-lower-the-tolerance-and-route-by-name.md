---
id: blend-endings-say-lower-the-tolerance-and-route-by-name
kind: issue
title: blend: FILLET3_CONTACT_RECOURSE says 'lower the tolerance', and BlendError::Escalated routes its ending by predicate name
status: closed
opened: 2026-09-28
closed: 2026-10-01
pr: 3690
branch: band/recourse-tables-decide-per-tag
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

## Closed (band/recourse-tables-decide-per-tag)

- `BlendError::Escalated` carries `sweep::blend::BlendDecision`, a closed
  type; its subject and ending are exhaustive matches over it, and the
  blend's one funnel (`classify`) takes it, so the k_stats name is the
  decision's (`BlendDecision::predicate`) and nothing routes by name.
- Every ending — in band, and the definite arms that carry a
  `ClassifiedMargin` — derives through `geom_brep::recourse::SizedDecision`
  (read at the build, `StoredDefinite::Lever`) where the decision is
  sized; the three that pass only at zero (`fillet3_chain_g1`,
  `fillet3_support_coaxiality`, `fillet3_cap_transverse`) end in the
  lever alone. `FILLET3_CONTACT_RECOURSE` no longer says "lower the
  tolerance", and offers no tolerance either: the must-carry relay
  cannot say whether its in-band verdict is the second-order reading
  or a wedge one, for which the offer would be false (ENCL's
  `must-carry-in-band-verdict-does-not-say-which-decision-escalated`).
- `ClassifiedMargin` stays a type: it carries the predicate's name and
  refusals on a positive sign (`ChainNotG1`), which `recourse::Refused`
  cannot. It reaches the table through `ClassifiedMargin::arm`.

Left open elsewhere: `dependent-normals-refusal-carries-no-margin-for-its-ending`
(one definite arm with no margin to end through the table) and ENCL's
`must-carry-in-band-verdict-does-not-say-which-decision-escalated`.
