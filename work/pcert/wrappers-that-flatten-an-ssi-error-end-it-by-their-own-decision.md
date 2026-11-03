---
id: wrappers-that-flatten-an-ssi-error-end-it-by-their-own-decision
kind: issue
title: Two lane wrappers flatten an SsiError into Unsupported and end it 'not yet', losing the SSI arm's decision
status: open
opened: 2026-11-03
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
