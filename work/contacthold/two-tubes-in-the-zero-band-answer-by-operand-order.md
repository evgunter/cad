---
id: two-tubes-in-the-zero-band-answer-by-operand-order
kind: issue
title: Two tubes side by side in the zero band answer Ok in one operand order and CurvedPierceUnsupported in the other
status: closed
branch: contacthold/two-tubes-zero-band
pr: 5389
opened: 2026-10-13
priority: P2
closed: 2026-10-19
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

## Where the orders part (2026-10-19)

In every pose the first body's seam ruling lies on the tangent line, so
the second body's rim circles touch its wall mid-arc. Sweeping the
first body's edges first, its seam pierces the second's cap planes on
their rim circles and splits them there; the second direction then
reads each circle fragment with its end on the touch, and records it.
Sweeping the second body's edges first, `reduce::curved_face_arm`'s
covered circle branch meets each whole circle, both ends definitely
off the carrier and its interior unseen, and refused before the other
direction could split it. A covered line in that state was already
deferred to `settle_deferred`; the circle now is too, so both orders
read the same fragments. The `Ok` was right: ∪ is the two solids with
the operands' summed volume and the same two vertex–edge records in
both orders, ∖ the first operand, ∩ empty. Rows:
`crates/sweep/tests/two_tubes_in_the_zero_band.rs`.
