---
id: topo-escalations-offer-a-declaration-the-door-cannot-take
kind: issue
title: topo: split_edge's in-band interiority forwards the declare menu to the split and blend doors, which take no declaration
status: review
opened: 2026-09-29
priority: P2
cost: E
pr: 3493
branch: topo/route-refusal-subjects
---


(CHROME, from the triage in
`work/chrome/escalations-forwarded-whole-are-untriaged-for-a-declarations-object.md`.)

## What

`EulerOpError::SplitParamEscalated` (`crates/topo/src/euler.rs`, the
variant near :852, its `Display` arm near :1065) renders
`split_edge: interiority test on edge {edge:?} escalated ({diag})`:
the whole `Indeterminate`, so it ends in `COINCIDENCE_RECOURSE`
("declare the coincidence, …"), after a stage label and an arena key.

`Body::split_edge` raises it (`crates/topo/src/split.rs`, the
`split_edge_param_interior` decision near :246). Three doors wrap it
whole:

- the split: `SplitReduceError::CrossingInsertion`
  (`crates/topo/src/splitting/classify.rs` near :442, rendered by
  `crates/topo/src/splitting/mod.rs` near :430). A split takes no
  declarations (`geom_core::SPLIT_PLANE_RECOURSE`'s doc;
  `Node::Split`, `crates/editor-core/src/node.rs` near :2018, has no
  `declare`).
- the blend: `BlendError::Op` through `surgery::op`
  (`crates/sweep/src/blend/surgery.rs` near :269, :2605, :3487). A
  blend takes no declarations (`Node::Fillet`/`Node::Chamfer`,
  `node.rs` near :1910/:1953).
- the Boolean: `BooleanError::CrossingInsertion`
  (`crates/topo/src/boolean/reduce.rs` near :2345). This door takes
  declarations, but a declared face pair has nothing to say about where
  on an edge a crossing lands.

## Repair shape

Render `diag.payload()` with a subject in plain words ("whether the
crossing lands strictly inside the edge") and a routed recourse. The
lever that reaches the split's case is the split plane or the geometry.
See `sweep::blend::BlendError::Escalated`'s `Display` (one match
returning subject and recourse) and `profile::validate::decision_subject`.
The guard is `test_utils::refusal::subjectless_escalations`.

## The same case inside `BooleanError::Escalated`

`BooleanError::Escalated` (`crates/topo/src/boolean/mod.rs` near :843,
`Display` near :1702) renders the payload, then
`Recourse: {COINCIDENCE_RECOURSE}`, for every decision that raises it.
For its coincidence raisers that is right: the `carrier_eq`/`plane_eq`
rungs, the tangent locus and the section escalations. A face-pair
declaration names nothing for these raisers:

- the sector rung, `SectorFault::Rung` (`crates/topo/src/boolean/sectors.rs` :235);
- the pierce point's face normal, `NormalAtError::Escalated`
  (`crates/topo/src/boolean/vtxfac.rs` :145);
- point containment, `ContainError::Escalated`
  (`crates/topo/src/boolean/reduce.rs` :1967 and :2274).

A subject routed by `diag.predicate`, as blend does, would let the
recourse follow the decision. `boolean/mod.rs` has no owner by
`work.py territory`, and the sector, normal and containment files are
TOPO's, so the row is here.

## How PR 3493 routes it

Not by `diag.predicate`, as the repair shape above prescribes: D4 ¶1
(i), ratified in PR 3352, makes the decision a closed type at its site,
so its recourse is an exhaustive match, never a lookup by predicate
name. `BooleanError::Escalated` carries a `boolean::BooleanDecision`,
set where each escalation is wrapped (`SectorFault::Rung` carries its
`SectorRung`, `NormalAtError::Escalated` and
`splitting::ConicRootFault` carry which rung escalated), and its
ending follows from the decision and the verdict through
`geom_brep::recourse` (`boolean/refusal_routes.rs`).
`SplitParamEscalated` ends through the same table's
`SPLIT_PARAM_INTERIOR`.
