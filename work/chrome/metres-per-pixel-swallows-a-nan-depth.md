---
id: metres-per-pixel-swallows-a-nan-depth
kind: issue
title: View::metres_per_pixel_at's floor substitutes a scale for two different non-scales
status: open
opened: 2026-09-12
---

## Finding

`crates/viewer/src/datums.rs`, `View::metres_per_pixel_at` (~`:144`):

```rust
(depth * self.metres_per_pixel_at_one_metre).max(f64::MIN_POSITIVE)
```

The floor makes this function total, and it is the reason the module's
refusal — `grid_pitch` and `View::screen_metres_at`, both `Option`
since the grid-pitch row closed — can only ever see `+inf`. Two
distinct non-scales are converted to `f64::MIN_POSITIVE` upstream of
it and never reach it:

1. **A NaN depth.** `f64::max` returns the other operand when one is
   NaN, so a datum or an eye with a NaN coordinate leaves this
   function as a finite positive length.
2. **A zero depth** — the eye exactly on the datum. This is the arm
   the doc at the site names, and names as REACHABLE: *"Floored at a
   hair above zero so a datum lying exactly at the eye — reachable by
   flying the camera into a plane — produces a degenerate drawing
   rather than a division by zero."*

## What is and is not the defect

**The two arms produce the identical drawing**, and saying so is the
point of this row: a plane draws **126 positions** over a patch about
`3.1e-305 m` across, first position `[3.13e-305, -3.0e-305, 0.0]`,
under either input (measured with a throwaway row against the tree at
`door/grid-pitch-refusal`). So the drawing does not distinguish them
and no measurement can — **the distinction is argued versus unargued**,
and that is the whole finding:

- The **zero-depth** arm is argued at the site, and the argument is
  good as far as it goes: a division by zero is worse than a
  degenerate drawing. What the argument does not cover is that the
  degenerate drawing is now *also* the shape the module refuses
  everywhere else, so the site's answer and the module's answer to the
  same question disagree. That is a decision to revisit, not a bug.
- The **NaN** arm is not argued at all. It is not the case the doc
  names, it arrives through a different route, and `f64::max`'s
  NaN-preferring behaviour is the kind of thing a reader has to know
  to see it.

Both are the class the closed row
(`viewer-grid-pitch-nonfinite-fallback`) objected to — a plausible
reading substituted for a refusal — one call above the fix.

`screen_metres_at`, and through it every mark of every datum kind,
reads this function, so whatever is decided reaches the whole module.

## Why it was not fixed in the closing PR

The fix is `metres_per_pixel_at -> Option<f64>` with the check on
`depth` **before** the floor (it cannot be spelled on the output:
`f64::MIN_POSITIVE` is a legitimate answer for an eye a hair off the
plane), and the zero-depth arm needs the site's own argument revisited
rather than deleted. That is a design call inside CHROME's house, not
the mechanical propagation the DOOR row named, and DOOR does not
widen.

## Fence

`crates/viewer/src/datums.rs` — CHROME's and VIEW's by the territories
table; filed on CHROME's slate.
