---
id: plane-offset-rung-decided-zero-shares-invalid-with-a-poisoned-margin
kind: issue
title: topo: plane_eq's offset rung carries its decided zero as MarginDiag::INVALID, the poisoned-margin encoding, and flush::pair_finding reads a poisoned offset as DecidedCoincident
status: review
branch: topo/plane-offset-rung-carries-its-decided-margin
pr: 3974
opened: 2026-09-30
priority: P2
cost: M
---


(TOPO, the fix pass of PR 3506: the review's NOTE-5.)

## What

`boolean::plane_eq::oriented_plane_eq_verdict`
(`crates/topo/src/boolean/plane_eq.rs`, the `bool_plane_offset` match)
refuses a decided-zero offset as `PlaneEqError::Undeclared` carrying
`MarginDiag::INVALID`. An offset margin that is itself poisoned (a NaN
datum) escalates through the same match's `Err(diag)` arm as
`Undeclared` with an `INVALID` margin too, so the two share one
encoding.

- `flush::pair_finding` (`crates/topo/src/flush.rs`, the `Undeclared`
  arm) reads `diag.margin.is_invalid()` as "definite zero offset" and
  records `FlushRung::DecidedCoincident`. Its own comment concedes a
  NaN-poisoned margin shares the encoding and leaves C4's
  verify-at-use as the backstop.
- `BooleanError::UndeclaredCoincidence`'s `Display`
  (`crates/topo/src/boolean/mod.rs`) reads the same bit to say "the
  coincidence measure is exactly zero".

PR 3506 moved the orientation rung's decided zero off `INVALID`
(`decide_reported`, the decided margin riding the payload). The offset
rung is the one left.

## Repair shape

Decide the offset through `decide_reported` and carry its decided
margin on `Undeclared` (or a typed "decided" arm), so a poisoned margin
and a decided zero stop sharing one bit; then `pair_finding` and the
`UndeclaredCoincidence` text read the verdict, not `is_invalid()`.

## A third reader, typed (TOPO, PR 3513)

PR 3513 moves the maximal-faces gate's `Undeclared` arm to its own
variant, `BooleanError::CoplanarNeighbours`, and reads the ladder there
through `plane_eq::plane_eq_typed`, whose `LadderRefusal::Coplanar`
carries rung 4's decided zero with its decided margin (`decide_reported`).
So `CoplanarNeighbours { offset: NeighbourOffset }` ends a zero-band
offset through `NEIGHBOUR_OFFSET` with the tolerance it gives and quotes
the margin, with no runtime branch on `is_invalid()`. The public
`oriented_plane_eq` / `oriented_plane_eq_verdict` still map that arm to
`Undeclared { diag: INVALID }` (`LadderRefusal::untyped`), so
`flush::pair_finding` and `UndeclaredCoincidence`'s `Display` read
exactly what they read before: the two readers above remain, and the
repair is to hand them the typed arm too.

## Delivered

`CarrierEqError::Undeclared` carries a `CoincidenceMeasure`: `Zero` with
the decided margin, `Undecided` (in band, or past it where the declared
reading stands off), or `Unreadable` (a datum is not finite). The plane
ladder and the curved `data_rungs` decide through `decide_reported`;
`LadderRefusal` / `plane_eq_typed` retire. `pair_finding`, the pair
door and the Boolean's raise sites read the arm; an unreadable datum
refuses as `SelfCheck::CarrierData`, a defect, never as a coincidence
to declare. Rows: `flush::rows`, `plane_eq::tests::a_decided_zero_offset_is_not_a_poisoned_one`,
`boolean::tests::coincidence_pair_carries_the_shared_recourse_once`.
