---
id: finite-bounds-yield-an-infinite-scene-radius
kind: issue
title: Bounds of a few hundred orders of magnitude pass both guards and return radius = inf as a scene radius
status: closed
opened: 2026-09-17
priority: P1
cost: D
closed: 2026-09-21
branch: vgeom/refusal-floor
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

## Closed — the radius is asked, not only the endpoints

`sphere` now runs `finite("scene radius", radius)?` on the PRODUCT,
before the `MIN_SCENE_RADIUS` comparison that an infinity passes.
`camera::finite` is the guard this file already had; what changed is
where it is applied. The refusal is
`CameraError::NotFinite { what: "scene radius", value: inf }`, which
`Display` renders as *the camera's scene radius is inf, which is not a
finite number*.

**The row's table re-derived rather than quoted.** Executed under
`rustc -O` against a verbatim replica of `sphere`'s arithmetic:

| bounds | result |
|---|---|
| `lo = -1e200`, `hi = 1e200` on every axis | `radius = inf`, **accepted** |
| `lo = -f64::MAX`, `hi = f64::MAX` | `half = inf`, `radius = inf`, **accepted** |
| `lo = hi = f64::MAX` | refused, `DegenerateScene { radius: 0 }` |
| an ordinary metre box | `radius = 0.866…` |

All four reproduce. The threshold is sharper than *a few hundred
orders of magnitude*: `half = 1e154` squares to `1e308` and the
one-axis radius is still finite, `half = 1e155` squares to infinity.
Across three axes the sum overflows from about `7.7e153`.

**The centre needed no guard of its own, and that is measured rather
than assumed.** `0.5 * (lo + hi)` overflows — `lo = 1e308`,
`hi = 1.7e308` gives `centre = [inf, 0, 0]` — so it is a second arm of
the same defect at the same site. It cannot survive the radius guard:
an endpoint past half of `f64::MAX` sits where the representable
numbers are about `2e292` apart, so that axis's `hi - lo` is either
exactly zero (a zero radius, already refused as `DegenerateScene`) or
at least that spacing, whose square is infinite. Scanned over every
pair of magnitudes from `1e150` to `1e308` in both signs and the first
64 floats above each: **126 non-finite centres, none with a finite
radius.** A guard on the centre would therefore be an assertion no
value can break, which this repo's discipline says to delete rather
than write; the argument is in `sphere`'s doc comment instead.

**Row**: `crates/viewer/tests/camera_ops.rs`,
`bounds_whose_half_extents_square_to_infinity_are_not_a_scene`, driven
through the public `Camera::framing`. It asserts the refusal is about
the radius by name, and pairs it with `±1e153` — whose three squares
sum to `3e306` and are still a number — which has to come back as a
camera, so a door refusing on magnitude rather than on the product
fails the second half.

**Mutation**: deleting the `finite("scene radius", …)` line reds that
row and nothing else in the 798-row app-feature suite.

## Reachability: unchanged, still unsure

The row's own section stands. Bounds come from the scene's AABB, which
comes from tessellated geometry; nothing was traced here that produces
a `1e200` extent and nothing was found that forbids one. What this
unit adds is the downstream cost if one arrives, which the row already
stated and which is now refused at the source: `clamp_distance`'s band
is `scene_radius * MIN_DISTANCE_FACTOR ..= scene_radius *
MAX_DISTANCE_FACTOR`, both infinite.

PR: `vgeom/refusal-floor`.
