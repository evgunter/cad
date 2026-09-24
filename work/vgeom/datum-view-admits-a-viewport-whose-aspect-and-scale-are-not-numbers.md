---
id: datum-view-admits-a-viewport-whose-aspect-and-scale-are-not-numbers
kind: issue
title: datum_view's field doc says the door refuses a window with no scale, and the door admits two
status: open
opened: 2026-09-22
priority: P3
cost: E
---



Found by the sweep of
`camera-new-admits-a-scene-radius-whose-distance-band-is-not-finite`
(`vgeom/camera-band`), whose class is **a door that guards its INPUT
and not the PRODUCT it derives from it**. This is the same shape one
crate-file over, and here it also makes a written claim false.

## The finding

`crates/viewer/src/datums.rs`, `datum_view`. The door checks that each
viewport side is finite, by name, and then that `viewport.aspect()` is
`Some` — and builds the `View` from there:

```
    for (what, value) in [("viewport width", width), ("viewport height", height)] {
        if !value.is_finite() { return Err(CameraError::NotFinite { what, value }); }
    }
    if viewport.aspect().is_none() { return Err(CameraError::UnusableBounds); }
    Ok(View { ..., metres_per_pixel_at_one_metre: 2.0 * (camera.fov_y() * 0.5).tan() / height, ... })
```

`ViewportSize::aspect` (`crates/viewer/src/input.rs`) answers
`Some(self.width_px / self.height_px)` for **any** two positive sides,
so it is a check on the inputs' sign and not on the ratio. Two
products therefore leave this door as non-numbers:

- **the aspect itself** — `width = 1.0`, `height = 1.0e-320` gives
  `Some(inf)`, and `aspect().is_none()` is false for it;
- **the scale** — the same viewport gives
  `metres_per_pixel_at_one_metre = inf`, since dividing an ordinary
  numerator by a subnormal overflows.

Both sides are finite and strictly positive, so every guard the door
actually makes takes them.

## Why it is a row rather than a shrug

The `View::metres_per_pixel_at_one_metre` field doc asserts the
opposite in as many words:

> *"[`datum_view`] refuses the window that would produce a non-length
> rather than carrying one, and [`View::metres_per_pixel_at`] is where
> a value that is not a scale is refused anyway, once, for every mark
> — the field's promise and the door's check are separate claims and
> the door owes its own."*

The second half holds: `metres_per_pixel_at` asks
`positive_length(depth * self.metres_per_pixel_at_one_metre)`, so no
mark is drawn from an infinite scale and nothing invented leaves the
module — which is why this is a claim defect rather than a wrong
picture. **The first half is false**, and it is the half the sentence
says the door owes.

`a-datum-the-view-cannot-scale-vanishes-without-a-word` (closed) and
`datum-view-propagates-rather-than-refusing-by-name` (closed) settled
the announcement and the by-name refusal for a viewport side that is
not a number. Neither reaches this: both are about the **inputs**, and
the sentence quoted above was written by the second of them.

## What the fix is

Ask the products at the door, in the shape
`crate::scene::DisplayTolerance::new` and (since
`vgeom/camera-band`) `Camera::new` use: refuse a viewport whose
aspect or whose metres-per-pixel is not a finite positive number, with
the arm the door already returns for a window that is not one
(`CameraError::UnusableBounds`, or a typed arm of its own if the
caller needs to tell the two apart). Whether `ViewportSize::aspect`
should answer `None` for a ratio that is not one is the same question
one level out and is the more useful place to settle it, since
`Camera::fitted` and `Camera::projection_matrix` each re-ask
`finite("aspect", …)` on their own — three callers asking one
question the producer could answer once.

## Fence

`crates/viewer/src/datums.rs` and `crates/viewer/src/input.rs` — both
VGEOM-claimed, both double-claimed (datums.rs with author, chrome,
view; input.rs with chrome, view). Held out of `vgeom/camera-band`,
whose fence was `crates/viewer/src/camera.rs` alone.
