---
id: pick-hit-point-from-an-out-of-range-barycentric-leaves-the-triangle
kind: issue
title: a hit point placed from a barycentric outside [0, 1] leaves the closed triangle, so its t can precede the candidate's box entry
status: closed
opened: 2026-09-16
closed: 2026-09-17
---


## The finding

`ray_triangle` (`crates/editor-core/src/resolve/pick.rs`) answers the
parameter of the point `a + u·e1 + v·e2` projected onto the ray. That
point is in the CLOSED triangle exactly while `u` and `v` are in
`[0, 1]` with `u + v ≤ 1`, which today's acceptance guarantees by
comparing the rounded values against those bounds. **Any acceptance
that admits a barycentric outside the range loses that guarantee**,
and with it the property the pick's early-out rests on: a candidate's
answer can fall BEFORE the parameter at which the ray enters the
candidate's own bounding box, so the walk that stops at
`best.t < cand.t_enter` stops before a candidate that would have won.

Measured on this branch's reverted implementation of EDIT-PICK2's
ruled conjunction (recoverable at `git show f096d84c5:crates/editor-core/src/resolve/pick.rs`),
over `index_memo`'s tie-break aim at every landing of the corpus and
the gallery ring — 19 296 rays:

| acceptance | `Pruned` ≠ `Every` |
| --- | --- |
| the closed comparison (`main`) | 0 |
| MEET ∧ INFORM on the derived interval | 2 |

One of the two, with the row's own message: `hollow_tube_elbow` after
the first edit, ray 136 (`+y` through
`(0.15915870375233154, ·, −2.2443637198591224)`) answers flat triangle
3072 at `t = 9.567` with the early-out and flat triangle 69 at
`t = 9.5208` over every candidate. Triangle 69's box is not entered
until after `9.567`; its `u` is admitted while outside `[0, 1]`, so
the point `a + u·e1 + v·e2` sits `0.046` up the ray from anywhere on
the triangle.

This is not a property of the interval ruling in particular — it
binds every rounding-aware acceptance on this door. Two shapes, both
untried:

- **place the hit at the nearest admissible barycentrics** (clamp
  `u`, `v` and their sum into the closed simplex before projecting):
  the door then answers a point of the closed triangle, as its own
  doc says it does, and a genuine graze whose `u` rounds to `−1e-14`
  is answered AT the vertex rather than `1e-14·|e1|` up the ray;
- **leave `t` where it is and accept that the early-out is no longer
  order-independent**, which `index_memo`'s `Pruned == Every` row
  refuses and EDIT-PICK2's spec named as not for trading.

The first is a change to what `ray_triangle` answers, not to what it
admits, so it is a ruling of its own and not resolved by implementing
it. It is the same ruling
`pick-a-wide-but-informative-barycentric-wins-over-the-transversal-neighbour`
waits on — what the door answers for `t`, and with what width — and
`pick-closed-acceptance-loses-a-graze-to-rounding` waits on both,
since MEET is what would close it.

EDIT-PICK2 landed the half that does not need this row (the closed
comparison ∧ INFORM); this row is why the other half did not land.

## Measured (EDIT-PICK3, 2026-09-16): the clamp landed

The first of the two shapes landed: `ray_triangle` places the hit at
the nearest admissible barycentrics (`u` into `[0, 1]`, then `v` into
`[0, 1 − u]`, the per-coordinate projection onto the simplex), so the
answered point is a point OF the closed triangle whatever the
acceptance admitted, and the traversal's early-out rests on a theorem
rather than on the acceptance.

Measured: under MEET ∧ INFORM the tie-break aim's `Pruned ≠ Every`
column is **0** where it was 2 before the clamp
(`crates/viewer/tests/pick3_acceptance.rs`, 19 296 rays over every
landing). The mechanism this row named is removed. MEET is still not
taken, for a different reason the same probe found — see
`pick-closed-acceptance-loses-a-graze-to-rounding`.

Under the closed acceptance the clamp is very nearly a no-op (every
admitted `u` is already in `[0, 1]` and every admitted `fl(u + v)` is at
most `1`), which is why no row reds when it is dropped; that is stated
at `every_admitted_hit_is_placed_on_the_closed_triangle`
(`crates/editor-core/src/resolve/pick.rs`) rather than claimed to be
covered. This row closes with EDIT-PICK3's merge.

## Unparked (2026-09-17, EDIT orchestrator)

The trigger fired: `what-t-the-pick-door-answers-and-with-what-width`
was ruled by Ev on `[ev]` PR #2764 and built by EDIT-PICK3, which
built the clamp this row asked for and closes it.

## Closed (2026-09-17, EDIT orchestrator)

Closed at EDIT-PICK3's merge (PR #2786): the hit point is
`retract_to_simplex`, the per-coordinate retraction into the closed
triangle with an exact `v` bound, so an admitted candidate's answer is
a point OF the triangle to the bit — the premise the early-out's proof
rests on. The `Pruned ≠ Every` column is 0 under both acceptances; the
mechanism this row named is gone, and MEET is not taken for the reason
`pick-closed-acceptance-loses-a-graze-to-rounding` records.
