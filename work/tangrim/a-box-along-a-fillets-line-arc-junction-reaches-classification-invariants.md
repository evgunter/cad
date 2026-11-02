---
id: a-box-along-a-fillets-line-arc-junction-reaches-classification-invariants
kind: issue
title: A box along a fillet's line–arc junction reaches ClassificationInvariant (odd surviving crossing records; a germ sector with both edge bounds on the partner face)
status: closed
opened: 2026-10-27
priority: P1
cost: M
branch: tangrim/fillet-line-arc-junction
closed: 2026-11-01
pr: 6219
---

Found by PR 5808's re-review (probe `rev5808r`, scene S9), on `main`
and the PR alike.

## The pose

`A` is the square `[−1, 1]²` with its `(+, +)` corner rounded by a
quarter circle `r = 0.2` about `(0.8, 0.8)`, extruded `z ∈ [0, 1]`. `B`
is the box `x ∈ [0.5, 1.5]`, `y ∈ [0.5, 1]`, whose face `y = 1` lies
along `A`'s straight side `y = 1` up to the line–arc junction
`(0.8, 1)` and past it.

- `B` over `z ∈ [0, 1]` or `[0, 0.5]`: `A ∪ B`, `B ∩ A` and `B ∖ A`
  refuse `ClassificationInvariant { "odd number of surviving crossing
  records at a vertex pair" }` (`boolean/insert.rs` `plan_null_pairs`).
- `B` over `z ∈ [−0.5, 1]`: the same three refuse `ClassificationInvariant
  { "a germ sector with both edge bounds on the partner face" }`
  (`boolean/sectors.rs` `along`).

Every one at rest and turned, at the default ε. The other operand
orders build at the closed form.

## Related

`work/contacthold/a-flush-quarter-round-refuses-an-odd-crossing-count-at-its-rim-vertex.md`
(closed by PR 4704) fixed the same invariant for a quarter round
resting flush with a slab's edge. This pose is different: a box face
along the round's straight side, through the junction where the side
meets the arc tangentially. That fix does not reach it.

## Slate triage

2026-11-01: moved from `work/cleave/` by CLEAVE's split on its semantic seams (Ev, in chat: CLEAVE measured 114 budget points against 30). TANGRIM holds the tangent curved rims and crescents; id and body unchanged. (CLEAVE orchestrator)

## Closed

2026-11-01, PR 6219. The ruling edge at the line–arc junction flanks a face coplanar with `B`'s and a wall glued `Tangent` to it; three sites could not represent membership changing there. `recl::resolve_edge_sector` re-keys a tangent-glued flanker by the side it curves to where both flankers read `On`; `vtxfac::classify_vertex_on_face` lets a one-carrier lump win a bound it shares with a tangent lump and reads a tangent sector's germ bounds at second order (`TangentRead`); `join::bool_connect` gives a germ along a line edge of the wall's solid a straight chord on the plane. Pinned by `crates/sweep/tests/a_box_along_a_fillets_line_arc_junction.rs` (168 cells SOUND at three ε, plus an inflection ruling and a tangent wall beside a falling side). Full review: every claim held, no built → refused move at three ε. Residue: `a-box-face-along-an-inflection-ruling-refuses-not-same-face-past-both-ends` (filed here), the falling side's `B ∖ A` on PARTING's `the-boolean-parting-refuses-its-rarer-configurations`; the two new second-order readers fold into `the-sector-passes-read-a-bounds-tie-first-order-beside-the-one-reading`.
