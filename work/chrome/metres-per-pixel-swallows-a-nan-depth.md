---
id: metres-per-pixel-swallows-a-nan-depth
kind: issue
title: View::metres_per_pixel_at swallows a NaN depth into a plausible scale
status: open
opened: 2026-09-12
---

## Finding

`crates/viewer/src/datums.rs`, `View::metres_per_pixel_at` (~`:144`):

```rust
(depth * self.metres_per_pixel_at_one_metre).max(f64::MIN_POSITIVE)
```

`f64::max` returns the OTHER operand when one is NaN, so a NaN depth —
a datum or an eye with a NaN coordinate — comes out of this function as
`f64::MIN_POSITIVE`, a finite positive length, with nothing anywhere
saying a scale was substituted. The doc at the site argues one case and
one only: *"Floored at a hair above zero so a datum lying exactly at
the eye — reachable by flying the camera into a plane — produces a
degenerate drawing rather than a division by zero."* The NaN arm is not
that case and is not argued.

**It is the same class as `viewer-grid-pitch-nonfinite-fallback`, one
call up**, and it is the reason that row's fix does not cover the NaN
input. `grid_pitch` now refuses a scale that is not one, but it never
sees the NaN: it is handed `f64::MIN_POSITIVE` and answers a rung,
because `f64::MIN_POSITIVE` is a legitimate scale.

**Measured, not reasoned** (throwaway row against the tree at
`door/grid-pitch-refusal`, an eye at `[NaN, 0.0, 0.1]` and a plane at
the origin): `grid_pitch(f64::NAN)` is `None`, and the plane
nevertheless draws **126 positions** over a patch about `3.1e-305 m`
across, the first at `[3.13e-305, -3.0e-305, 0.0]`. That is a complete,
well-formed, entirely invented drawing at a scale nobody asked for —
the exact shape the closed row objected to, surviving its fix.

`window_metres_at`, `axis_segments`, `point_segments` and
`frame_segments` all read this same function, so the substitution
reaches every datum kind, not just the two plane-like ones.

## Why it was not fixed in the closing PR

The honest fix is `metres_per_pixel_at -> Option<f64>` and the refusal
propagated through `window_metres_at` and all four `*_segments`
functions — a second, wider unit than the one the DOOR row named, and
DOOR's charter is one PR per row with no widening. Filed here rather
than carried.

Note that `f64::MIN_POSITIVE` is a legitimate value on the OTHER arm
(the eye exactly on the plane), so a refusal cannot be spelled by
testing the output; the check belongs on `depth` before the `.max`.

## Fence

`crates/viewer/src/datums.rs` — CHROME's and VIEW's by the territories
table; filed on CHROME's slate.
