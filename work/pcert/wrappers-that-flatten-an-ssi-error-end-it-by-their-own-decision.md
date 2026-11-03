---
id: wrappers-that-flatten-an-ssi-error-end-it-by-their-own-decision
kind: issue
title: Two lane wrappers flatten an SsiError into Unsupported and end it 'not yet', losing the SSI arm's decision
status: closed
opened: 2026-11-03
closed: 2026-11-03
pr: 6474
priority: P3
cost: E
---


(PCERT implementer, from the §5 sweep of
`plane-nurbs-refusal-ending-groups-the-producers-refusals-into-none`,
branch `pcert/plane-nurbs-ending`. The same shape as
`wrappers-that-flatten-a-pcurve-certify-error-end-it-by-their-own-decision`,
one layer down: there the flattened refusal is a `PcurveCertifyError`,
here an `SsiError`.)

## What

Every arm of `PlaneNurbsRefusal` and `AnalyticRung3Refusal` now ends by
its own decision. Two wrappers decide what that arm is by flattening an
`SsiError`, and both end a whole tail of SSI arms in one arm of their
own whose decision is a capability gap:

- `edge_nurbs::refusal` (`crates/geom-brep/src/edge_nurbs.rs`, the
  `SsiError -> PlaneNurbsRefusal` map): its `_ =>` arm folds every
  `SsiError` but the eight it names into
  `PlaneNurbsRefusal::Unsupported { what: "the rung-3 certificate
  refused for a reason outside this lane's vocabulary" }`, which ends
  "not yet". Among them are a step that cannot be taken
  (`StepCollapsed`, the operands' scale), a refinement budget spent
  (`RefinementExhausted`, a lever or the tolerance), a march tolerance
  mismatch (`MarchTolMismatch`, the door's defect) and a self-crossing
  locus — each of which `SsiError::ending` ends by its own decision at
  the SSI door.
- `AnalyticRung3Refusal::of_tube` (same file): its `_ =>` arm, and both
  `TubeProbeSilent` arms, do the same into
  `AnalyticRung3Refusal::Unsupported`.

So an SSI refusal read through a lane door ends "There is no way through
yet" where the same refusal read at the SSI door states a lever, a
tolerance or a defect.

## Repair shape

Decide per wrapper whether the SSI arm's decision is the one the lane
door should end by. Where it is, carry the refusal (or its
`SsiError::ending`) rather than its `what` string; where the lane's own
decision is the right one, say so at the arm and make the match
exhaustive by variant.

## Resolved (PR 6474)

Both wrappers now carry what reaches them. The SSI refusals the lane's
producers return are named arms ending by `SsiError::ending`: the
tube's structural refusals as `Tube(TubeRefusal)` on both lane
refusals, a wrong-lane call as `PlaneNurbsRefusal::WrongLane`. What is
left in the `_` of `edge_nurbs::refusal` and `of_tube` is the SSI arms
no producer of these lanes returns (`SsiOperand::nurbs`,
`certify_branch` and `incidence_limbs` reach none of them); they end as
`OutsideVocabulary`, the door's defect, rather than a false "not yet".
A silent tube that built no chain now ends as the kernel's defect, as
the SSI door ends it; one with an enclosure refused stays "not yet".
