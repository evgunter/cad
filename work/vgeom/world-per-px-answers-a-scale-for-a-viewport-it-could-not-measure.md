---
id: world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure
kind: issue
title: world_per_px's height guard does not bound a NaN, so it answers Some(NaN) where None is its refusal
status: open
opened: 2026-09-17
priority: P1
cost: E
---

Found by the sweep `a-clamp-is-not-a-bound-against-nan` ran over
comparison guards a `NaN` passes in `crates/viewer/src`.

## Finding

`crates/viewer/src/input.rs`, `world_per_px`:

```
        if viewport.height_px <= 0.0 {
            return None;
        }
        let visible_height = 2.0 * camera.distance() * (camera.fov_y() * 0.5).tan();
        Some(visible_height / viewport.height_px)
```

`None` is this door's refusal — it means there is no viewport to scale
against. `height_px <= 0.0` is **false** for a `NaN`, so a height that
is not a number takes neither side of the guard, and the door answers
`Some(NaN)`: a scale in the field its callers read a scale from,
rather than the refusal it has.

The shape is the one
`a-clamp-is-not-a-bound-against-nan` states — an ORDERING used as a
domain test, on a value that has no order — and the repair is the
same: ask whether the height is a number before treating the bound as
one. `crates/viewer/src/camera.rs`'s `finite` and
`crates/viewer/src/datums.rs`'s four-way `is_finite` guard are the two
places in this crate that already do it.

## Reachability: no producer found

`viewport.height_px` is filled from the toolkit's pane rectangle. No
route was found by which egui hands back a rectangle whose height is
not a number, and that is a negative result rather than a proof: the
search was over this crate's own writers of `ViewportSize`, not over
egui's layout.

## Fence

`crates/viewer/src/input.rs` — VIEW's, the standing double claim with
CHROME.
