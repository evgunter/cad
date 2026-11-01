---
id: a-box-along-a-fillets-line-arc-junction-reaches-classification-invariants
kind: issue
title: A box along a fillet's line–arc junction reaches ClassificationInvariant (odd surviving crossing records; a germ sector with both edge bounds on the partner face)
status: dispatched
opened: 2026-10-27
priority: P1
cost: M
branch: tangrim/fillet-line-arc-junction
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
