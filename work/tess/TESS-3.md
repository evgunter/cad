---
id: TESS-3
kind: unit
title: the exact zero of a degree-1 direction reaches split_steps as 0.0, measured under the RING-2 ring
status: closed
opened: 2026-09-22
priority: P0
cost: D
branch: tess/3-exact-zero
refs: [exact-zero-second-partial-leaves-cell-component-as-subnormal-dust]
pr: 3098
closed: 2026-09-22
---


## Closed (2026-09-22, PR 3098 merged at `cd0019ec2`)

Measured: SCALAR's RING-2 closed the row's mechanism — the backend's
exactness witnesses keep `0·0` and `0+0` at `[0, 0]`, so `sq_norm`'s
fold from the ring zero stays exact and `cell_component`'s `hi == 0.0`
arm fires; a degree-1 integral direction's `muu`/`mvv` are exactly
`0.0` (`3.85e-162` occurs nowhere). Pinned: `== 0.0` in place of
`< 1e-100` on both integral bilinear rows; the affine arm's only
observable (`hu == hv == inf`, `!cap`); a one-sided degenerate-arm row
with a windowless variant (the windowed one cannot separate the arms —
bitwise agreement); the rational degree-1 direction's dust bracketed
`[1e-14, 1e-11]` so the degenerate arm is never taken there. Docs state
the exact zero as an INTEGRAL-arm fact. Filed on TESS:
`chords-m-bound-zero-arm-is-dead-because-the-curve-collapse-has-no-
exact-zero-case` (CHORD's ground; `next_up(0.0)` is `5e-324`). Bottom
tier: docs and four test rows, merged on green CI and the
orchestrator's read of the assertions.
