---
id: finite-bounds-yield-an-infinite-scene-radius
kind: issue
title: Bounds of a few hundred orders of magnitude pass both guards and return radius = inf as a scene radius
status: open
opened: 2026-09-17
priority: P1
cost: D
---

Found by the review of #2798, in a file that PR's sweep disposed of as
**clean**. The disposition was right about what it looked at — every
route into `clamp_pitch`/`clamp_distance` is preceded by a typed
`is_finite` refusal — and the sibling one function up was not looked
at.

## Finding

`crates/viewer/src/camera.rs`, the bounds fit:

```
    if lo.iter().chain(hi.iter()).any(|v| !v.is_finite()) {
        return Err(CameraError::UnusableBounds);
    }
    ...
    let radius: f64 = (half[0] * half[0] + half[1] * half[1] + half[2] * half[2]).sqrt();
    if radius < MIN_SCENE_RADIUS {
        return Err(CameraError::DegenerateScene { radius });
    }
    Ok((centre, radius))
```

The guard bounds `lo` and `hi`. It does not bound `half[i]`, and it
does not bound `half[i]²`, which is where the overflow is: squaring
costs the exponent twice, so a box only a few hundred orders of
magnitude across takes the sum to infinity long before either endpoint
is near `f64::MAX`.

Executed:

| bounds | result |
|---|---|
| `lo = -1e200`, `hi = 1e200` on every axis | `radius = inf`, **accepted** |
| `lo = -f64::MAX`, `hi = f64::MAX` | `half = inf`, `radius = inf`, **accepted** |
| `lo = hi = f64::MAX` | refused, `DegenerateScene { radius: 0 }` |
| an ordinary metre box | `radius = 0.866…` |

`radius < MIN_SCENE_RADIUS` is false for an infinity, so the door
returns it as a scene radius. It then reaches `clamp_distance`, whose
bounds are `scene_radius * MIN_DISTANCE_FACTOR` and
`scene_radius * MAX_DISTANCE_FACTOR` — both infinite — so the camera's
distance clamps to infinity.

**The sibling class, stated because it is not the same one.** #2798's
row is about `NaN`, the value a comparison cannot order. This is an
infinity, which a comparison orders perfectly well and which is still
not a length. The guard that catches both is the same guard — ask
whether the value IS a number — and `camera::finite` is already in
this file; it is applied to the inputs and not to the product.

## Reachability: unsure

Bounds come from the scene's AABB, which comes from tessellated
geometry. Nothing was traced that produces a `1e200` extent, and
nothing was found that forbids one either.

## Fence

`crates/viewer/src/camera.rs` — VIEW's, the standing double claim with
CHROME.
