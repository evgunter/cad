---
id: CONTACT-10
kind: unit
title: the containment refusals carry their decision: contain.rs's endings follow D4 ¶1 (the grazed-parity ending, RayExhausted's Display, and ContainError::Escalated through Undecided)
status: closed
opened: 2026-09-29
priority: P3
cost: M
branch: contact/10-contain-endings
closed: 2026-10-08
---


Carries `contact-near-boundary-endings-say-lower-the-tolerance` and
`contain-escalation-carries-no-decision`. Both edit the same refusals in
`boolean/contain.rs`, which is why they are one unit.

Spec: `docs/CONTACT-10-SPEC.md`.

Review tier: **single style review.** The change is refusal text and a
closed decision type carried through; no verdict moves.

## Closed

A containment refusal that escalates now names the decision it could
not make, and ends in that decision's own recourse (D4 ¶1 (i)).
- `LoopDecision { Boundary, Ray, ArcSpan, Plane }` lives in `splitting`;
  `ContainDecision` carries it. Each carried decision is tagged
  Margin, Straddle or Decided, so a sound escalation never takes the
  kernel-bug note.
- `placement_subject`, `placement_lever` and `placement_sized` are the
  one source of subject, lever and sized ending. contfp, `classify_contain`,
  the census and the Boolean all read them.
- A tolerance is quoted only where tightening can decide the margin.
  Boundary and OneCircle are AnySign, and WindowPeriod is Positive.
  Carrier is lever-only, following TOPO's PR 3493: a residual at
  `wall_crossing`. The Boolean never quotes a value, because it asks
  at many points and no single margin binds it.
- The Boolean carries the walk's decision inside
  `BooleanDecision::Containment`, so there is one decision type, not
  two. `CONTAINMENT_RAISED` lists the 13 pairs that sites can raise.
- The census's `WitnessTooClose(Option<LoopDecision>)` ends on the
  lever alone, because `CensusUndecidable.what` is a `&'static str`.
- Four sites are driven with real geometry: Boundary, Ray, Carrier and
  Plane.

Review: the tier was a single style review.
- The first review (pre-pause) returned APPROVE-WITH-FIXES with 17
  findings.
- The pause and main's rework of this ground (the `ContainError`
  split, one ray-walk driver, `SizedPass` in `geom_core`, PR 3493)
  meant the unit was re-applied on top of main.
- A delta review returned APPROVE-WITH-FIXES with no MAJOR. The fix
  pass took 10 of its 12 items and filed the other two. The
  orchestrator read both passes.

Filed:
- `census-containment-escalation-drops-its-decision`: four doors that
  drop or ignore the decision;
- `work/restfront/check-9-drops-the-one-circle-escalation`;
- `point-in-solid-escalation-carries-no-decision`, narrowed;
- `full-turn-question-has-three-spellings`, extended.
