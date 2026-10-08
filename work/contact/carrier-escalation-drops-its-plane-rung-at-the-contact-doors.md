---
id: carrier-escalation-drops-its-plane-rung-at-the-contact-doors
kind: issue
title: contact: the Rest verify and the flush detector drop CarrierEqError::Escalated's plane rung, so an orientation refusal reads as a coincidence
status: open
opened: 2026-09-30
---


(TOPO, the §5 second pass of PR 3506: every door that wraps the plane
ladder's escalation.)

## What

PR 3506 made `CarrierEqError::Escalated` carry the rung that could not
decide (`boolean::PlaneRung`: parallelism, or orientation) and the
orientation rung's decided margin on its zero verdict. The Boolean's
plane-identity sites and the merge route by the rung
(`BooleanError::plane_identity`, `MergeDecision::DeclaredPlanes`): an
orientation refusal ends in its own lever ("turn one of the two faces
so they clearly face the same way or clearly opposite ways", valued),
with no declaration. Two doors drop the rung:

- `boolean::contact_verify`'s Rest ladder
  (`crates/topo/src/boolean/contact_verify.rs`, the `carrier_eq` match
  in the Rest verify) maps `Escalated { diag, .. }` to
  `ContactRefusal::Escalated`, which the census renders as
  `ValidationError::CensusEscalated` ("whether two parts of the body
  touch is too close to call …"), a coincidence story, on a DECLARED
  contact whose orientation is what could not be read.
- `flush::pair_finding` (`crates/topo/src/flush.rs`, TOPO's ground) maps
  it to its bare `Indeterminate`, which `FlushRefusal::PairInBand`
  renders with "separate the geometry or widen the tolerance" (an
  unvalued loosening) and a `flush detection:` label; editor-core's
  `names::flush` forwards it as `SelectRefusal::PairInBand`.

## Repair shape

Carry the rung (or the routed decision) through `ContactRefusal` and
`FlushRefusal`, and end the orientation rung from
`boolean::plane_eq::PLANE_ORIENTATION` at the reading each door is
(`Reading::AtRest` at the census).
