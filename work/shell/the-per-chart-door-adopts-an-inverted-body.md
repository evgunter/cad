---
id: the-per-chart-door-adopts-an-inverted-body
kind: issue
title: the per-chart offset door adopts an inverted body: tier 2 does not read orientation, so a move through a neighbour returns Ok with negative volume
status: dispatched
opened: 2026-10-08
priority: P2
cost: M
branch: shell/inverted-body-at-rest
---


Filed by SHELL's `shell/oblique-corner-derives` lane (unit 9, PR 4351)
from its dual review.

## Measured

`topo::replace_face_offset` validates its clone at tier 2
(`validate_closed`: closure and every edge certified on its charts),
which does not read orientation. A move that carries a face through its
neighbours inverts the body, and the door returns `Ok`:

- the frustum revolved from `(0.2, 0)-(0.4, 0)-(0.6, 0.6)-(0.2, 0.6)`
  (`crates/sweep/tests/offd_r1_probes.rs`'s `frustum_opening`), its
  cone moved `−0.3`: volume `−0.0055`, tier 3 `RingOutsideOuter`;
- the class predates unit 9: a tube's outer cylinder moved `−0.5`
  does the same.

Unit 9 made the frustum case reachable: its rims are now derived, so
the door builds where it used to refuse at the re-anchor.

## Shape of a fix

The door states a closed-body postcondition. Either it decides the
moved face's material side against its neighbours before adopting (a
signed-volume or orientation predicate on the clone), or its contract
says the result is construction state that the at-rest validator
finishes — which is what `crate::AtRestBody::validate` already does
for the shell (`replace_face.rs`'s module docs, "Discipline").
