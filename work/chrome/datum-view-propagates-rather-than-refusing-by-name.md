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

## This is the closing unit minting an instance of its own class

Recorded here so it is not re-derived later as a surprise. `NaN` in
`View::viewport_px` is a value the function did not compute, sitting
in a field shaped like one it did — the shape
`docs/REVIEW-STYLE-DISPATCH.md` §2 names as *"the fix reproducing the
defect it closes"*, minted at the last site the sweep touched.

It is the honest option given the fence: the field's type is `f64`,
the alternative inside the fence is a floor (the defect), and the
alternative outside it is this row. What makes it survivable rather
than a second defect is that `View`'s two field docs now say the
fields are not promised to be a scale or a pixel count, and every door
below refuses both — so nothing reads the value as a number. What it
still is not is a refusal a caller can see.

## Territory

**VIEW acts on this, not CHROME.** The call site is
`ViewerBehavior::viewport_ui` in `crates/viewer/src/pane/viewport.rs`,
at its one `datum_view(self.camera, viewport)` call inside the
`show_datums` block — the only in-tree caller. The edit is to take
`datum_view`'s `Option`/`Result` and skip the datum loop (or badge)
when it refuses; the `datums.rs` half is CHROME's and is a signature
change plus the door's own check.

Filed on CHROME's slate because the substituted value lives in
`datums.rs`, which is CHROME's; it cannot be discharged without VIEW.
