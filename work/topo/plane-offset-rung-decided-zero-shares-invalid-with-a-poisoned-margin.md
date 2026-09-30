---
id: plane-offset-rung-decided-zero-shares-invalid-with-a-poisoned-margin
kind: issue
title: topo: plane_eq's offset rung carries its decided zero as MarginDiag::INVALID, the poisoned-margin encoding, and flush::pair_finding reads a poisoned offset as DecidedCoincident
status: open
opened: 2026-09-30
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

## A third reader (TOPO, PR 3513)

PR 3513 moves the maximal-faces gate's `Undeclared` arm to its own
variant, `BooleanError::CoplanarNeighbours`, whose `Display` and ending
(`refusal_routes::coplanar_neighbours_ending`) read the same bit: an
`INVALID` margin is "their planes' offset is exactly zero" with the
gate's lever alone, and a poisoned offset would read the same way.
Carrying the decided verdict fixes all three readers.
