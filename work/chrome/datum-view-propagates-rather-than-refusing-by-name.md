---
id: datum-view-propagates-rather-than-refusing-by-name
kind: issue
title: datum_view hands back a View that is not a view, where the camera refuses one by name
status: open
opened: 2026-09-15
---


## Finding

`crates/viewer/src/datums.rs`, `datum_view`. The function takes a
`Camera` and a `ViewportSize` and returns a `View` unconditionally.
A viewport whose height is zero or is not a number has no scale, and
what comes back is a `View` whose `metres_per_pixel_at_one_metre` is
`inf` or `NaN` and whose `viewport_px` is `NaN`.

That is now HONEST — the sweep that closed
`metres-per-pixel-swallows-a-nan-depth` removed the two floors
(`height_px.max(1.0)`, and `f64::max`'s NaN-preference in
`width_px.max(height)`) that used to launder those into a one-pixel
window — and every mark below refuses on it, so nothing invented
leaves the module. What is left is that **the refusal has no name**.

**The crate already knows how to say it.** `crates/viewer/src/camera.rs`
refuses the same input by name at its own door —
`finite("viewport height", viewport.height_px)?`, answering
`CameraError::NotFinite` with the value in the message. `datum_view`
is the sibling door on the same two inputs and answers with a struct
a caller cannot tell from a working one without inspecting its
fields.

## What the fix is, and why it did not ride the sweep

`datum_view -> Option<View>` (or a `Result` carrying the camera's own
error shape, which would make the two doors read alike). That changes
the signature, and the only in-tree caller is
`crates/viewer/src/pane/viewport.rs` — VIEW's ground, outside the
sweeping lane's fence.

**No live path reaches it today**, which is why this is a door
hardening rather than a defect: `ViewerBehavior::viewport_ui` returns
early when `ViewportSize::aspect` refuses a pane with no area, so the
app never calls `datum_view` with a zero or non-finite viewport.
`datum_view` is `pub` in a `pub` module, so the guarantee is the
caller's and not the door's.

## Fence

`crates/viewer/src/datums.rs` and `crates/viewer/src/pane/viewport.rs`
— CHROME's and VIEW's; the call-site edit needs VIEW's lane or its
consent.
