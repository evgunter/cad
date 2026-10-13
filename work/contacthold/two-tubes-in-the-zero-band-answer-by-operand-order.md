---
id: two-tubes-in-the-zero-band-answer-by-operand-order
kind: issue
title: Two tubes side by side in the zero band answer Ok in one operand order and CurvedPierceUnsupported in the other
status: open
opened: 2026-10-13
priority: P2
---



## What

Two tubes about parallel axes whose outer walls are tangent within the
zero band answer every op by operand order: one order `Ok`, the other
`CurvedPierceUnsupported`. The same under the glue door before PR 4918
and after it, so it is not that PR's.

## Poses (measured on PR 4918's branch, 2026-10-13)

- The PR 4918 review's `probe_two_tubes_outside` (branch
  `contacthold/glue-door-escalates-review`,
  `crates/sweep/tests/review_glue_door_escalates_probes.rs`): a tube
  `r ∈ [0.3, 1]`, `y ∈ [0, 1]` about the `y` axis, and a second tube
  `r ∈ [0.3, 1]`, `y ∈ [0.2, 0.8]` about the parallel axis at
  `x = 2 + 0.5·ε` (`Tol::witness`). ∪, ∖ and ∩: `[Ok] / [CurvedPierceUnsupported]`.
- A quarter tube (`Revolution::Partial(π/2)`, spun `3π/4` about its
  axis so its wall faces the first tube) at `x = 2 + 0.5·ε`, both
  bodies turned 0.5 rad about `z`: the same split.
- A quarter rod `r = 0.2` in that tube's bore at `x = 0.1 − 0.5·ε`,
  spun `−π/4` so its wall faces the bore's ruling: the same split.

The glue door reads the pair tangent in the zero band in either order
(`boolean::glue::tangency`), so the asymmetry lies past the door:
the pierce stage that refuses one order is where to start.
