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

**The sweep on `datums.rs` widened this rather than creating it.**
Refusing was already the answer for a scale that overflowed; that
sweep added two more inputs that now refuse where they used to draw
something — the eye exactly on a datum (which drew a mark about
`1e-307 m` across off the old `f64::MIN_POSITIVE` floor) and a
viewport that is not a positive number of pixels. Both refusals are
right and neither is announced.

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
