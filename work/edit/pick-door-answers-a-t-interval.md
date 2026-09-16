---
id: pick-door-answers-a-t-interval
kind: unit
title: EDIT-PICK3: the pick door answers a t interval, orders by it, breaks a certified tie by width
status: review
branch: edit/pick-t-interval
opened: 2026-09-16
---


Builds the ruling `what-t-the-pick-door-answers-and-with-what-width`
(Ev, `[ev]` PR #2764, 2026-09-16). Kernel unit, v6 dual, block
EDIT-B1 slot 2.

## Spec (2026-09-16, EDIT orchestrator)

`docs/EDIT-PICK3-SPEC.md` (deleted at merge, recorded in the ledger).
Branch `edit/pick-t-interval`. Closes the ruling row at merge, which
unparks `pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`,
`pick-closed-acceptance-loses-a-graze-to-rounding` and
`pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle`
(each then closes on the unit's measurement or stays with a reason);
`pick-a-corner-graze-verdict-depends-on-the-corner-labelling` is
measured, not assumed.

## Built (2026-09-16)

`ray_triangle` answers [`TSpan`] — the rounded `t` with the interval
`[t_lo, t_hi]` the arithmetic certifies around it, derived at `t_span`
from `crossing`'s barycentric bounds through the projection plus the
projection's own rounding (`PROJECTION_ERROR_UNITS`, counted from the
operation list). The hit point is clamped into the closed triangle.
`pick_face` drops every candidate another candidate PRECEDES and breaks
the resulting certified tie by the narrower interval, then by `(target
position, flat triangle position)`; the traversal's early-out compares
the smallest upper end seen against the next candidate's box entry.
`PickHit` carries `t_lo` and `t_hi`; `pncad` re-exports it unchanged and
`pncad-py` spells both (LIB's files, mechanically).

Three premises of the spec were measured before being built on, and two
moved:

- **The order is a rule about a SET, not a pairwise comparison.**
  "A precedes B, else the narrower wins" applied pairwise has
  three-cycles, so a fold over it answers whichever candidate the
  traversal met first. The candidates no other candidate precedes are
  pairwise overlapping, so they are ONE certified tie; width-then-
  position orders that set totally. Pinned by
  `the_certified_order_does_not_depend_on_the_arrival_order`.
- **The gallery ring's wide candidate is not a tie and does not lose.**
  The certified width is relative to the TRIANGLE: the ring's triangles
  are `0.016` on a side, so the noise-floor candidate's interval is
  `0.030` across and lies wholly before the vertex `0.031` further on.
  It PRECEDES; the tie-break never runs; the fixture still answers
  `1.4488`. The class the parked row names is pinned where the
  intervals do overlap — `tube_arc`'s ray, where `main` answered
  `0.0041` short and the tie-break now takes the vertex.
- **Closed ∧ INFORM stays.** Measured both ways under the clamp
  (`crates/viewer/tests/pick3_acceptance.rs`): MEET now costs nothing in
  the early-out column (`0 Pruned ≠ Every`, the clamp removed
  EDIT-PICK2's mechanism) but loses 513 aimed vertices of the wide aim's
  441 126 rays against `main`'s 141 106, gaining none. The closed
  comparison loses none and gains three.

What did not land: nothing the spec asked for. The corner-labelling
asymmetry survives the clamp and stays its own row.
