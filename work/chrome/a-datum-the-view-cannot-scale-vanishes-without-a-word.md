---
id: a-datum-the-view-cannot-scale-vanishes-without-a-word
kind: issue
title: A datum the view cannot scale draws nothing and says so nowhere
status: open
opened: 2026-09-15
---


## Finding

`crates/viewer/src/datums.rs` refuses per mark: `screen_metres_at`,
`half_patch_at` and `grid_pitch` each answer `None` when this view
lends that point no length, and the mark is simply not appended. A
datum whose every mark refuses contributes a `DatumDraw` with an
EMPTY segment list, which `pane::viewport` pushes zero positions from.

So the viewport shows nothing, and nothing anywhere says a datum was
dropped rather than absent: no fault, no badge, no line in the tree.
A reader cannot tell "this document has no datums" from "this view
has no scale for the ones it has".

**The sweep on `datums.rs` widened this rather than creating it, and
by less than a first draft of this row claimed.** Refusing was already
the answer for a scale that overflowed. The sweep added one input
reachable through document data — **the eye exactly on a datum**,
reachable by flying the camera into a plane, which used to draw a mark
about `1e-307 m` across off a `f64::MIN_POSITIVE` floor and now draws
nothing — plus **a ruling whose extent is lost to the datum's own
magnitude**, which used to emit zero-length segments and now emits
none. The viewport cases (`viewport_px`, and `datum_view`'s two
sides) are NOT among them: `ViewerBehavior::viewport_ui` returns
before `datum_view` when `ViewportSize::aspect` refuses a pane with no
area, so no viewport that is not a positive number of pixels reaches
this module from the app. All of these refusals are right and none is
announced.

**The project is fail-loud** (`CLAUDE.md`, `docs/DESIGN.md`), and the
crate has the machinery: `pane::viewport` already carries a
`projection_fault` latch for exactly the neighbouring case — a camera
that will not project — with `work/view/projection-fault-has-no-sweeper.md`
tracking its sweep. A datum drawing that refused is the same shape of
fact.

## What it would take

A per-frame count or latch — "n datums this view has no scale for" —
raised out of `datums::draws` and shown the way the projection fault
is. That is a change to `draws`'s return shape in `datums.rs` and to
its one caller in `crates/viewer/src/pane/viewport.rs`, which is
VIEW's ground.

## Fence

`crates/viewer/src/datums.rs` and `crates/viewer/src/pane/viewport.rs`
— CHROME's and VIEW's.

## Territory

**VIEW acts on this, not CHROME.** The call site is
`ViewerBehavior::viewport_ui` in `crates/viewer/src/pane/viewport.rs`
— the `for drawn in datums::draws(...)` loop inside the `show_datums`
block, and the `projection_fault` latch a few lines above it, which is
the existing machinery for exactly this shape of fact and whose own
sweep is `work/view/projection-fault-has-no-sweeper.md`. The
`datums.rs` half — raising a count or a reason out of `draws` — is
CHROME's.

Filed on CHROME's slate because the refusals are `datums.rs`'s; it
cannot be discharged without VIEW.
