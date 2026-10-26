---
id: full-turn-question-has-three-spellings
kind: issue
title: topo/geom-brep: the arc's gap to a full turn is asked under four spellings (LoopDecision::ArcSpan, WindowPeriod, certify ParamWinding, BooleanDecision::ArcSpan)
status: open
opened: 2026-09-29
priority: P4
cost: E
---


(CONTACT-10 fix pass, `contact/10-contain-endings`; the rule is D4 ¶1
(i) in `docs/DESIGN.md`, one recourse per decision.)

## What

Several decisions ask how far an arc or window falls short of a full turn:

- `geom_brep::certify::CertCheck::ParamWinding`
  (`crates/geom-brep/src/certify.rs`, `CertCheck::ending`): a
  `SizedDecision` with `NonNegative` passes, size "arc", and lever "move
  the geometry so this arc stays clearly short of a full turn". The
  pcurve cache's `AzimuthPeriod` routes to it.
- `topo::splitting::LoopDecision::ArcSpan`
  (`crates/topo/src/splitting/containment.rs`, `ConicArc::of`): the
  same question for an elliptic loop edge, with the same pass set and
  the same lever text, written out a second time in
  `LoopDecision::lever`.
- `topo::boolean::ContainDecision::WindowPeriod`
  (`crates/topo/src/boolean/contain.rs`, the cylinder arm's period
  guard): a face's azimuth window rather than an edge's arc. It passes
  only on a positive gap, because a zero or negative one is the door's
  remainder, not an answer. Its lever is "move the geometry so the wall
  sweeps clearly less than a full turn".

The first two are one decision under two spellings. `ArcSpan` cannot
read `ParamWinding`'s lever: `CertCheck::ending` is private to certify,
so the text is copied, not shared. The third is a different pass set on
a different object.

## Repair shape

Give certify's table a public lever per `CertCheck`, or move the
full-turn decision into `geom_brep::recourse` as a named constant.
`LoopDecision::ArcSpan` then reads it. Keep `WindowPeriod` separate
unless the door's remainder is ruled a pass.

## A fourth spelling (CONTACT-10 fix pass)

`boolean::BooleanDecision::ArcSpan` (`crates/topo/src/boolean/refusal_routes.rs`,
`BooleanDecision::ending`) ends as `Unsized::Defect`. Its reasoning is
that the edge's certification decided this same margin at this band,
so no arm of it is the user's. `splitting::LoopDecision::ArcSpan` asks
the same question of a loop edge and ends in a user lever ("move the
geometry so this arc stays clearly short of a full turn", sized "arc",
`NonNegative`).

Both readings can stand, because they are different doors:
- the Boolean's is a re-read of a certified edge;
- the walk's is the first reading of a loop at rest, where an elliptic edge's span rule is the walk's own.

Whether they are one decision, with a defect ending at the Boolean and
a lever at rest, belongs in the same repair.

