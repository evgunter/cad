---
id: camera-new-admits-a-scene-radius-whose-distance-band-is-not-finite
kind: issue
title: Camera::new admits a finite scene radius whose distance band is not finite, so max_distance is inf before any render sees it
status: open
opened: 2026-09-21
priority: P1
cost: E
refs: [renders-that-multiply-a-finite-guarded-length-spell-the-product-inf, finite-bounds-yield-an-infinite-scene-radius]
---


Found by `vgeom/render-spelling` while answering
`renders-that-multiply-a-finite-guarded-length-spell-the-product-inf`
for the camera readout. **The render half is closed on that branch and
this half is not reachable from a render at all** — the product is
formed above it, which is exactly the blind spot that row names.

## The finding

`crates/viewer/src/camera.rs`:

```
pub fn min_distance(&self) -> f64 { self.scene_radius * MIN_DISTANCE_FACTOR }
pub fn max_distance(&self) -> f64 { self.scene_radius * MAX_DISTANCE_FACTOR }
```

`MAX_DISTANCE_FACTOR` is `100.0`. `Camera::new` runs
`finite("scene_radius", scene_radius)?` and then
`scene_radius < MIN_SCENE_RADIUS` (`f64::MIN_POSITIVE`), and **nothing
bounds it above**. So every `scene_radius` above `f64::MAX / 100.0` —
`1.7976931348623156e306`, executed — is accepted, and `max_distance`
answers `inf`: a dolly limit that is not a distance, from a door whose
own guard is `is_finite`.

`clamp_distance` then clamps into
`scene_radius * MIN_DISTANCE_FACTOR ..= inf`, so the camera's
`distance` is bounded above by nothing. The arithmetic is the same
shape `finite-bounds-yield-an-infinite-scene-radius` closed one
function up, in the one direction that row did not reach: it made
`sphere` ask the PRODUCT (the radius) rather than the endpoints, and
`Camera::new` still asks its own input rather than the products it
derives from it.

## Reachability: through the public door, not through `framing`

`Camera::framing` cannot produce it. Since
`finite-bounds-yield-an-infinite-scene-radius` closed, `sphere`
refuses a non-finite radius, and a finite radius through the
three-axis sum means each half-extent squares inside the type — about
`7.7e153` at the top, whose `max_distance` of `7.7e155` is an ordinary
number. **`Camera::new` is public API** and takes a `scene_radius`
directly; a consumer of the library that has its own scene bound
reaches this with one call.

## Where the bound goes, and why not at the render

`crates/viewer/src/pane/view.rs`'s `camera_mm` is the render that
shows the band, and no bound inside it reaches this: the value is
`inf` before the function is called, and a render can report that or
refuse to name it (which it now does — `no mm reading` rather than
`inf`), but it cannot restore a number nobody computed.

The bound belongs at `Camera::new`, beside the `finite` call it
already makes on this very argument, in the shape
`crate::scene::DisplayTolerance::new` already uses for the same class:
refuse an input whose derived product is not a number, with its own
typed arm. The candidates for the arm are
`CameraError::NotFinite { what: "scene radius band", .. }` or a new
one; that choice, and whether `MIN_DISTANCE_FACTOR`'s direction needs
the same question asked (it multiplies DOWN, so it cannot overflow and
can only flush a subnormal radius to zero), are what the unit decides.

## Fence

`crates/viewer/src/camera.rs` — VGEOM's, with the standing double
claim with CHROME and VIEW. Held out of `vgeom/render-spelling`
because `camera.rs` was another lane's in the same wave.
