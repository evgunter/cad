---
id: blend-reach-reads-two-extents-off-a-root-of-one-minus-a-square
kind: issue
title: blend reach reads a circle's meridian lean and a torus band's box extent off sqrt(1 - x^2), which floors at sqrt(2u) near x -> 1 where a cancellation-free form exists
status: closed
closed: 2026-10-26
opened: 2026-10-13
priority: P3
cost: E
---

Found by the review of PR 5004 (`pcert/cone-section-interval-beta`),
which fixed a `√(1 − x²)` read near `x → 1` in `chart_pcurve`'s cone
arm. Not measured here: this is the arithmetic, and whether a verdict
moves is the owner's to say.

## What happens

Two reads in `sweep/src/blend/reach.rs` take `√(1 − x²)` of a unit
quantity that can approach 1. Near there the root carries about
`√(2u)` (2e-8) from `x`'s own rounding:

- **`sheet_clip`, a circle's meridian lean** (`reach.rs:1763`):
  `lean = √(2 − 2·√(1 − dot²))`, `dot = ax·a`. Near a meridian circle
  (`dot → 0`) the inner root is near 1 and the outer subtraction
  cancels, so `lean·radius`, and with it `d_mer`, carries about
  `radius·√(2u)`. `2 − 2√(1 − dot²) = 2·dot²/(1 + √(1 − dot²))`
  reads it without cancelling, as does `|ax × a|` for the inner root.
- **`circular_reach`, a torus band's box extent** (`reach.rs:1218`):
  `ext(k) = R·√(1 − k²) + reach` per axis component `k` of the unit
  axis `a`. Where `a` is near a coordinate axis, the extent along it is
  read to about `R·√(2u)`, and the box can fall short of the torus by
  that. With `a` unit, `√(1 − a_x²) = √(a_y² + a_z²)`, which has no
  cancellation.

## What closes it

Read each through its cancellation-free form, or show that every use
of `d_mer` and of the box is conservative in the direction the
rounding errs, with a row near a meridian and one with the axis along
a coordinate axis.

## Closed (2026-10-26, tracker sweep)

Fixed on main by `f84f8ef95d` ("sweep: sheet_clip clips a cylinder and
a plane at every pose; ... respell the chord-from-dot spellings",
2026-10-14), which rewrote both reads in their cancellation-free forms:

- `sheet_clip`'s meridian lean, `crates/sweep/src/blend/reach.rs:1837-1842`:
  `lean = |dot|·√2 / √(1 + √(1 − dot²))` ("spelt without the
  cancellation near ⊥");
- `circular_reach`'s box extent, `reach.rs:1283-1284`:
  `ext(p, q) = R·√(p² + q²) + reach` over the axis's other two
  components.
