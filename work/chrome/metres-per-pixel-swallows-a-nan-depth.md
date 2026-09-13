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
3. **A NaN reaching `look_at` rather than the eye** — and this is the
   severe one. The floor keeps the SCALE legitimate, so every door
   answers `Some`, while the patch's centre `(cu, cv)` is computed
   from `look_at` and is NaN. The scale survives and the geometry does
   not.

## What is and is not the defect

**Arms 1 and 2 produce the identical drawing**, and saying so is half
the point of this row: a plane draws **126 positions** over a patch
about `3.1e-305 m` across, first position `[3.13e-305, -3.0e-305,
0.0]`, under either input (measured with a throwaway row against the
tree at `door/grid-pitch-refusal`). So the drawing does not
distinguish them and no measurement can — **the distinction there is
argued versus unargued**:

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

**Arm 3 is the other half, and it is not a degenerate drawing but an
invalid one.** Measured the same way, datums at the origin, eye
`[0, -0.15, 0.1]`, `look_at [NaN, 0, 0]`, against this branch with
every refusal installed:

| kind | positions | non-finite | first |
| --- | --- | --- | --- |
| plane | 6 | **4** | `[NaN, NaN, NaN]` |
| frame | 18 | **4** | `[NaN, NaN, NaN]` |
| axis | 6 | **6** | `[NaN, NaN, NaN]` |
| point | 6 | 0 | `[-1.3e-3, 0.0, 0.0]` |

The point is clean because its mark is scaled at its own position and
never reads `look_at`. Everything else is NaN geometry produced with
every door answering yes — **that is the severity the closed row was
written around, arriving through this floor**, and it is the reason
this row is not merely a tidiness item. (The residual ruling in the
plane and frame rows above is a second mechanism, filed separately as
`inclusive-rule-range-draws-a-line-on-a-nan-count`; fixing either one
alone leaves the other.)

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
