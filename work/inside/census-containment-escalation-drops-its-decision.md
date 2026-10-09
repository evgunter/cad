---
id: census-containment-escalation-drops-its-decision
kind: issue
title: topo: four doors drop or ignore the containment decision a refusal carries (census, ring re-homing, rim wedge, sphere region)
status: open
opened: 2026-10-08
priority: P2
cost: M
---


(CONTACT-10 fix pass, `contact/10-contain-endings`; the rule is D4 ¶1
(i) in `docs/DESIGN.md`. This is the class of the closed row
`boolean-door-drops-the-containment-decision`, at the doors that row did
not cover.)

## What

`ContainError::Escalated` and `PointInLoopError::Escalated` carry the
walk's decision (`boolean::ContainDecision`, `splitting::LoopDecision`)
and how the reading stood (`splitting::Escalation`). Four readers drop
or ignore them.

- **`census::read_containment`** (`crates/topo/src/census.rs`, ~:1510)
  maps `ContainError::Escalated` to `ValidationError::CensusEscalated
  { cause }`. That keeps the `Indeterminate` and drops the decision, so
  the at-rest finding ends in `too_close`'s coincidence menu, whatever
  contfp was deciding. Carrying it needs one of two things:
  - a field on `CensusEscalated`, which is RESTFRONT's type, and the census's generic escalation for every census rung;
  - or routing the escalation to `CensusUnsupported`'s `Containment` cause. That moves the `Escalated`/`Unsupported` split `editor_core::attribute` classifies.

  Neither was contained in CONTACT-10.
- **`chord_join::SplitJoinError::RingHoming`**
  (`crates/topo/src/chord_join.rs`, `render`, ~:418). It carries the
  walk's `PointInLoopError::Escalated` whole, since CONTACT-10 made
  `ring_side` keep it. It still renders "which piece a hole loop falls in
  is too close to call (…). Recourse: {JOIN_RECOURSE}", the split's or
  the Boolean's generic lever, not the decision's.
- **`boolean::rim_wedge`'s `lift`** (`crates/topo/src/boolean/rim_wedge.rs`,
  ~:503) folds `ContainError::Escalated { diag, .. }` to its bare `diag`.
- **`boolean::sphere_region`** (`crates/topo/src/boolean/sphere_region.rs`,
  ~:315 and ~:322) folds the conic reading's `ReadEscalation` to its
  `diag` (`RegionRefusal::Escalated`). Its straddles therefore read as
  poisoned margins, and `contain::curved_face_placement` carries the
  region's refusal as `decision: None`.

## Repair shape

Each reader routes the carried decision to its own renderer:
`placement_ending` at rest, `ContainDecision::lever_ending` at a build
that offers no tolerance, and the census through a carried field. The
sphere region gets its own boundary decision, or the walk's
`LoopDecision::Boundary`, with the `Escalation` tag kept.
