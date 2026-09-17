---
id: datum-view-propagates-rather-than-refusing-by-name
kind: issue
title: datum_view hands back a View that is not a view, where the camera refuses one by name
status: closed
opened: 2026-09-15
closed: 2026-09-16
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

## Re-homed to VIEW, 2026-09-15

Moved out of `work/chrome/` by the CHROME orchestrator. The Territory
section above already said **VIEW acts on this, not CHROME** — the only
in-tree caller is `ViewerBehavior::viewport_ui` in
`crates/viewer/src/pane/viewport.rs`, which the 2026-09-15 carve-out
cedes to VIEW. A row nobody on the owning slate can act on is a row on
the wrong slate, and `work/README.md` is explicit that a finding goes
onto the slate of the program whose ground it lands on.

**CHROME had been holding this waiting to `park` it on VIEW's
viewport-adapter lane.** That was the wrong instinct twice over: it
needed a trigger id CHROME could not identify (searched `work/view/`
for an open row citing `pane/viewport.rs` near the statements VIEW
described narrowing — nineteen rows cite the file and none matched, and
none of the three rows in `review` cites those lines), and more simply,
**VIEW does not need CHROME's permission to sequence its own work.**
Re-homing beats parking: VIEW schedules it against its own viewport
lane and nothing waits on a cross-program handshake.

**The `datums.rs` half stays CHROME's and CHROME will take it on
request** — the door has to raise a named refusal before a caller can
render one. That half is a signature change inside CHROME's fence and
needs no negotiation; say when the call site is ready.

Signed: (CHROME orchestrator)

## Closed, 2026-09-16

`datum_view` answers `Result<View, CameraError>`. It refuses a width
or a height that is not finite with `CameraError::NotFinite`, naming
the side and carrying the value, and a viewport with no area with
`CameraError::UnusableBounds` — which is `Camera::ray_through`'s
answer, variant for variant, on the same two quantities, and
`datum_view_refuses_a_window_the_way_the_cameras_own_door_does`
compares the two doors' replies rather than asserting either
separately.

**The class instance this row records is retired, not survived.** With
both sides refused, `viewport_px` is `width.max(height)` over two
finite positive numbers, so the `if width.is_nan() || height.is_nan()
{ f64::NAN }` arm is gone and no `NaN` leaves the door. `View`'s two
field docs no longer cite `datum_view` as the producer of one; what
they say now is that the fields are the CALLER's — the struct is
public and the suite writes them directly — and that the doors below
owe their own check whatever is written, which
`a_hand_built_view_that_is_not_pixels_still_draws_no_invented_mark`
holds.

**The row's reachability premise is right about the conclusion and
wrong about the reason, and the difference is live.**
`ViewportSize::aspect` refuses zero and refuses `NaN`, so those two
never reach the door from the app. It does **not** refuse an INFINITE
extent: it asks whether both sides are above zero and `inf` is, so a
pane of infinite extent has an aspect, passes the early return and
reaches `datum_view`, where every mark then refuses in silence. The
refused arm is therefore this door's alone and not a subset of the
pane's guard.
`the_panes_aspect_guard_admits_an_extent_this_door_refuses` pins both
halves. No production route was found that hands egui an infinite pane
extent; what is established is that the guard does not exclude one.

At the call site a refusal is held in `projection_fault` and the pane
returns. That is not a new latch: every input this door refuses is one
`Camera::view_projection` refuses a hundred lines further down — an
infinite height gives an aspect of `0.0` and `UnusableBounds`, an
infinite width an aspect of `inf` and `NotFinite("aspect")`, both
infinite an aspect of `NaN` — so the badge was already going to be
written on that frame, and what changes is that it now names which
side was not a number of pixels instead of saying the framing request
names no view.

Landed with `a-datum-the-view-cannot-scale-vanishes-without-a-word`;
the signature change is that row's prerequisite.
