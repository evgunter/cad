---
id: a-thin-torus-tube-reads-no-crossing-angle-on-lily-wall-8-at-1e-6
kind: issue
title: At eps 1e-6 lily wall 8's second torus x spline pair refuses at limb 3: over its 106-box chain the uniqueness tube's crossing-angle bound reads 0 m
status: open
opened: 2026-11-25
priority: P2
cost: M
refs: [the-chart-accounting-clears-a-thin-torus-only-in-sub-millimetre-cells, the-lily-lanterns-weld-still-refuses-at-the-interior-loop-guard-at-1e-6]
---

Found by `ssiarith/thin-torus-exclusion`, re-measuring lily wall 8
once the thin torus's form cleared its cells at the cell's own
distance.

## What

At ε 1e-6, the union of lily wall 8 (`demos/tour/src/lily.rs`, the lower
stem's torus tube, R = 5 m, r = 0.06 m, with the long leaf) refuses
after about 20 s (`ci-tour` profile) at a torus × NURBS pair:

> `GermSectionUnbuilt { a_face: FaceKey(4v1), a_kind: Torus, b_face: FaceKey(4v1), b_kind: Nurbs, why: Refused(Unaccounted { cause: "ssi: over its 106-box chain the uniqueness tube's lower bound on the crossing angle is not clear of the zero band (bound 0e0 m), so a second branch may lie inside the tube" }) }`

At 1e-9 the same union gets past every torus × spline section and stops
at the pierce pass
(`work/splice/lily-wall-8s-leaf-wall-lifts-no-closed-trim-at-the-pierce-pass.md`).
Before the factored torus form both ε rows stopped at the 200 000-cell
budget.

## Not measured

Which rung of the tube's ladder the chain is, its boxes' widths against
the pipe's 0.06 m, and which reading gives the bound exactly zero (a
clamped lower bound, or a gradient enclosure straddling zero). One
suspect, unconfirmed: the box chain's metre widths, about 4e-2 by 3e-3
on the accounting's tubes at 1e-9, approach the pipe radius, where the
torus's gradient turns by about `w/r` over a box.

## Why P2

Nothing reaches it: at 1e-6 lily wall 2 stops the wall probes first
(`work/sectcone/the-lily-lanterns-weld-still-refuses-at-the-interior-loop-guard-at-1e-6.md`),
and the tour pins wall 8 only at the stop it reaches. Re-band when wall
2 clears at 1e-6. — (SSIARITH orchestrator)

