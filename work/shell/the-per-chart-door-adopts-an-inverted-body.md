---
id: the-per-chart-door-adopts-an-inverted-body
kind: issue
title: the per-chart offset door adopts an inverted body: tier 2 does not read orientation, so a move through a neighbour returns Ok with negative volume
status: closed
opened: 2026-10-08
priority: P2
cost: M
pr: 4403
closed: 2026-10-09
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

## Closed (PR 4403, 2026-10-09): by contract, under unit 10's ruling

The door is a construction step (tier 2 in, tier 2 out), so `Ok` on an
inverted body is its contract; the result finishes only through
`AtRestBody::validate`, which refuses both cases on `RingOutsideOuter`.
Every caller that hands an offset door's result out as finished does so
through `shell_open`'s closing `validate_geometric` (audit in the PR
body). Pins: `crates/sweep/tests/a_move_through_a_neighbour_inverts_the_body.rs`
— both moves through the door, signed volume in closed form and the
tier-3 refusal, and `shell` past half the wall refusing `NotValid`
typed at its closing gate. The module doc and the README row name the
class and its refusal kind.
