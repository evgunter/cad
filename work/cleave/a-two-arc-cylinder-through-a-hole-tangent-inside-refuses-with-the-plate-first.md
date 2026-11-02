---
id: a-two-arc-cylinder-through-a-hole-tangent-inside-refuses-with-the-plate-first
kind: issue
title: A two-arc cylinder through a plate, a hole tangent inside its wall with only the hole's vertex on the ruling, refuses CurvedPierceUnsupported with the plate first and builds with the cylinder first
status: open
opened: 2026-10-31
priority: P2
cost: M
---


Found sweeping `a-one-segment-cylinder-through-a-plate-tangent-to-a-hole-refuses-single-site`
for its class.

## The pose

The cylinder `r = 0.3`, `z ∈ [−1, 1]`, as two arcs with vertices at
azimuths 1 and 1 + π, through the plate `[−1, 1]² × [−0.5, 0]` holed
through by `r = 0.1` about `(0.2, 0)`, tangent inside the wall along the
ruling through `(0.3, 0)`, the hole's rim two arcs with a vertex on that
ruling (vertices at 0 and π, or π and 0).

## What it reads

At rest, default ε, on `main` and on
`cleave/one-segment-cylinder-through-a-holed-plate` alike (the PR 5808
review's probe `rev5808r`, S4, extended):

| op | cylinder first (A) | plate first |
|---|---|---|
| ∪ | `SOUND` | `CurvedPierceUnsupported { operand: A, face: FaceKey(3v1), edge: EdgeKey(5v3) }` |
| ∩ | `EdgeInFace { gap: CurvedFace }` | `CurvedPierceUnsupported` (the same) |
| A ∖ B | `SOUND` | — |
| B ∖ A | — | `CurvedPierceUnsupported` (the same) |

The union builds in one operand order and refuses in the other, so a
reading at the ruling's tangency depends on which operand the sweep
takes first; the operand the refusal names is the plate in both orders
(`operand: A` with the plate first). Outside the wall (hole about
`(0.4, 0)`) both orders build every op but the plate less the cylinder,
which refuses `EdgeInFace` with the `CurvedFace` gap. The raising site
among `boolean/reduce.rs`'s `CurvedPierceUnsupported` raises is
unmeasured.
