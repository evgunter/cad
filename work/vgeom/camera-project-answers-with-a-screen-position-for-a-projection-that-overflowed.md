---
id: camera-project-answers-with-a-screen-position-for-a-projection-that-overflowed
kind: issue
title: Camera::project returns a screen position for a projection that overflowed
status: open
opened: 2026-09-21
priority: P1
cost: E
---


## Finding

`crates/viewer/src/camera.rs`, `Camera::project`. The guard on the
homogeneous `w` before the perspective divide is

```rust
if out[3].is_nan() || out[3] <= 0.0 {
    return Ok(None);
}
Ok(Some([out[0] / out[3], out[1] / out[3], out[2] / out[3]]))
```

`w` is asked whether it is a NUMBER and whether it is positive. It is
never asked whether it is FINITE, and neither are the three numerators.
So an overflowed projection divides through and comes back as a screen
position, through a door whose own rustdoc says `None` means *"on or
behind the eye plane (`w ≤ 0`)"* — a caller reading that doc has been
told the only refusal, and this is not it.

**Driven, not read** — `Camera` as `common::framed(1.6)`, `aspect`
1.6:

| point | answer |
| --- | --- |
| `[-f64::MAX, f64::MAX, -f64::MAX]` | `Ok(Some([NaN, NaN, 0.0]))` |
| `[1e300, 1e300, -1e-300]` | `Ok(Some([6.502, 1.394, 3.44e-304]))` |

The second is the worse of the two: a point 1e300 metres away comes
back as a perfectly ordinary-looking pair of normalized device
coordinates just outside the frustum, with nothing about it to
distinguish it from a real answer. The first at least poisons.

`crates/viewer/src/pickindex.rs` is the consumer (its one
`.project(world, aspect)` call).

## Why its own row

Found by the sweep that gave `datums.rs` its finiteness doors, and
first written down as a near-miss line inside
`positive-finite-predicate-has-six-homes-outside-datums-rs` — where
the other six entries are a DUPLICATION finding and the disposition
is "give them a door". That is the wrong reader for this. This is a
member of the class `viewer-substituted-value-class-is-crate-wide`
names — *a value the function did not compute, returned in the shape
of one it did* — and it is a live one behind a `pub` door with a
consumer, not a tidy-up. It is pulled out of that row and priced
here.

## What the disposition might be

The project's standard is not "never substitute" — it is "weigh the
two failures where you do". Two shapes:

- **Refuse.** Ask `w` and the three numerators whether they are
  finite, and answer `None`, widening the rustdoc's sentence about
  what `None` means. `None` already means "no screen position for
  this point", which an overflowed projection also is.
- **Refuse by name**, since this door already returns a
  `Result<_, CameraError>` and `CameraError::NotFinite { what, value }`
  exists and is exactly this fact. That distinguishes "behind the
  eye", which is ordinary, from "the arithmetic left the number
  line", which is not — and the doc for `None` stays true as written.

The second looks right and the reading that settles it is
`pickindex.rs`'s: what that caller can DO about each answer.

## Fence

`crates/viewer/src/camera.rs`, and `crates/viewer/src/pickindex.rs`
for the caller read. Both are chrome's by the territories table, and
`camera.rs` is also fit's, vgeom's and view's.
