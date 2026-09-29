---
id: contain-escalation-carries-no-decision
kind: issue
title: topo: ContainError::Escalated and the point-in-solid refusals collapse their decisions, so their endings name the lever alone
status: open
opened: 2026-09-29
---


(ENCL implementer, `work/encl/validate-own-close-levers-follow-the-d4-recourse-ruling.md`.)
The rule is D4 ¶1 (i) in `docs/DESIGN.md`: the recourse follows from the
decision and its verdict, and the decision is a closed type at its site.

## What

- `ContainError::Escalated(Indeterminate)` (`crates/topo/src/boolean/contain.rs`)
  is the point-in-loop walk's escalation from any of its decisions
  (`PointInLoopError::Escalated`), so the decision is gone by the time
  `topo::validate::classify_contain` renders it. It ends in
  `OFF_BOUNDARY` alone ("Recourse: move the geometry clear of the
  boundary"), with no tolerance, since no one decision's margin gives
  one.
- `census::Undecided::of_point_in_solid` (`crates/topo/src/census.rs`)
  folds `PointInSolidError::Escalated`, `RayExhausted` and `Loop(_)` into
  `WitnessTooClose`, whose sentence now ends in the lever alone
  ("move the parts until their bounding boxes no longer overlap").

`ContainError::RayExhausted`'s own `Display`
(`crates/topo/src/boolean/contain.rs`, `impl Display for ContainError`)
still ends "move the point off the boundary or lower the tolerance",
where D4 ¶1 (i) offers no unvalued lowering; validate renders its own
ending over it, but a caller printing the refusal reads the old one.

A point's side of a boundary passes on either nonzero sign, so once the
decision is carried its ending wants a two-sided `SizedPass` (the one
PR 3390 adds as `SizedPass::NonZero`) to quote the tolerance below
`|m|/K` on either side.

## Repair shape

Carry the decision as a closed type on the escalation (and through
`Undecided`), end it through `geom_brep::recourse::SizedDecision` at
`Reading::AtRest` in `classify_contain`, and pin the valued ending.
