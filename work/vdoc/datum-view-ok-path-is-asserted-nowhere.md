---
id: datum-view-ok-path-is-asserted-nowhere
kind: issue
title: datum_view's Ok path is asserted nowhere — the larger-side rule survives being inverted
status: open
opened: 2026-09-16
---



## Finding

Found by the style review of #2788, which added `datum_view`'s
refusals and rows for every one of them, and no row for what it
answers when it does not refuse.

`crates/viewer/src/datums.rs`, `datum_view`:

```
        viewport_px: width.max(height),
```

`.max(` → `.min(` is **green on all 582 `--test all` rows**. The
field's whole reason to exist is the larger side — its own doc says a
patch covering the height of a wide window *"would still be pannable
off sideways"* — and nothing holds it.

`metres_per_pixel_at_one_metre` is in the same position: it is
`2 * tan(fov_y / 2) / height`, and no row reads what the door computes
for a window it accepts.

## Why the suite does not catch it

`crates/viewer/tests/datum_draw.rs`'s `view_at` **hand-builds a
`View`** with the same two formulas rather than calling the door:

```
        metres_per_pixel_at_one_metre: 2.0 * (core::f64::consts::FRAC_PI_8).tan() / height,
        viewport_px: 1280.0,
```

So every ruling, patch and pitch row in that file drives numbers that
AGREE with `datum_view` by construction and never through it. The
suite is thorough about what the module does with a view and silent
about where a view comes from — which is the shape where a second
spelling of a formula drifts from the first and nothing says so.

## What the fix is

A row through the door: build a `Camera`, hand it a wide window and a
tall one, and assert `viewport_px` is the larger side in both — plus
that the scale is the one the field's doc states. Worth considering at
the same time whether `view_at` should call `datum_view` instead of
restating its arithmetic, which would make the hand-built copy a
non-issue rather than a second thing to keep in step.

## Fence

`crates/viewer/src/datums.rs` and `crates/viewer/tests/datum_draw.rs`
— VIEW's, and the test file is the standing double claim with CHROME,
S-TCOST and S-TINT.
