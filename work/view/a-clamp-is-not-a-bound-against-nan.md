---
id: a-clamp-is-not-a-bound-against-nan
kind: issue
title: Three clamps in the viewer pass NaN through and hand back a value the arithmetic did not compute
status: closed
opened: 2026-09-16
closed: 2026-09-17
branch: view/clamp-nan
pr: 2798
---



## Finding

#2788's sweep for *a door that hands back a value it did not compute,
in a field shaped like one it did* stated a blind spot in writing: its
patterns matched named defaults (`unwrap_or`, `map_or`, `.max(`) and
could not match **a substitution made by arithmetic** — a saturating
float→int cast, or a `clamp` that does not bound what it looks like it
bounds. The review of that PR walked the blind spot and found three
members. Each line below was **executed**, not reasoned about.

`f64::clamp` and `f32::clamp` return `self` when `self` is `NaN`.
A float→int `as` cast saturates, and `NaN as uN` is **0**.

### 1. A colour channel that is not a number becomes pure black

`crates/viewer/src/theme.rs`, `channel_to_srgb8`:

```
    let c = channel.clamp(0.0, 1.0);
    ...
    (encoded * 255.0).round() as u8
```

`f32::NAN.clamp(0.0, 1.0)` is `NaN`; `encoded` is `NaN`;
`(NaN * 255.0).round() as u8` is **0**. A channel that is not a number
is rendered as a channel that is zero, so a poisoned colour arrives on
screen as a legitimate pure black.

**The doc comment above it claims the opposite in as many words**: *"a
mix of two in-gamut colours stays in gamut, but the clamp is what makes
that a property of the arithmetic rather than an assumption about
it."* That is true of every value except the one a clamp cannot bound,
which is the case the sentence is written to cover.

### 2. An arc drawn as one point

`crates/viewer/src/sketch.rs`, `arc_points`:

```
    let step = 2.0 * ratio.clamp(-1.0, 1.0).acos();
    if step <= 0.0 {
        return MAX_ARC_POINTS;
    }
    ((theta.abs() / step).ceil() as usize).clamp(1, MAX_ARC_POINTS)
```

A `NaN` `ratio` gives a `NaN` `step`, and `step <= 0.0` is **false**
for a `NaN` — so the guard above does not take it. `(NaN).ceil() as
usize` is `0`, and `.clamp(1, MAX)` lifts it to **1**. An arc whose
subdivision could not be computed is drawn with one point.

### 3. A pane share that is not a number

`crates/viewer/src/app.rs`, the features auto-size:

```
        let stack = above.height() + below.height();
        if stack <= 0.0 {
            return;
        }
        ...
        let fraction = ((wanted + FEATURES_SLACK) / stack).clamp(0.0, FEATURES_SHARE_CAP);
```

`stack <= 0.0` is **false** for a `NaN` stack, so the early return does
not take it and `fraction` reaches the share assignment as `NaN`.

## The class, stated once

**A `clamp` is not a bound against `NaN`, and a cast is not a
conversion.** Each site wants the same repair in the same shape: ask
whether the value IS a number before treating the bound as one, and
say what happens when it is not. Whether any of the three inputs can be
`NaN` today is a separate question per site and is not asserted here —
what is asserted is that the guard at each site does not answer it.

## Fence

`crates/viewer/src/{theme.rs, sketch.rs, app.rs}` — VIEW's, with
`app.rs` the standing double claim with CHROME.

## Closed — #2798, 2026-09-17

Each site was re-executed before it was changed; all three of the row's
claims held. Three different repairs, because the three doors are
different doors.

**`theme::channel_to_srgb8`.** Answers `Option<u8>`, `None` for a
channel that is not a number, and `from_linear` and `Mark::over` carry
that up as `Option<Rgba8>`. The test is `is_nan` and NOT `is_finite`,
which is the argument this site turns on: an infinity is ORDERED and
sits above the whole gamut, so the clamp bounds it correctly; a `NaN`
has no order, so there is no bound to put it under. The row's claim
that this is a paint path is **wrong** — see below.

**`sketch::arc_points`.** Answers `Option<usize>`, `None` when the
radius, the swept angle or the chord tolerance is not a finite number;
`flatten` turns that into `PreviewError::Unflattenable { loop_, vertex }`
and `preview` reports it. `is_finite` here rather than `is_nan`,
because an infinite radius emits `NaN` points just as surely.

**`ViewerApp::fit_features_share`.** The share arithmetic is
`features_fraction(wanted, stack) -> Option<f32>`, refusing both
measurements unless they are numbers. The caller's `stack <= 0.0` arm
stays what it was and means what it said — the very first frame,
before either tile has a rectangle — and the new arm is written apart
from it because they are different facts.

## What the row was wrong about

**Site 1 is not a paint path.** `Mark::over` and `from_linear` have
exactly one caller between them and it is `crates/viewer/tests/theme.rs`
— the colourblind check. The shader does its own mixing in WGSL. So
the consequence is not "a poisoned colour arrives on screen as black";
it is that the SAFETY MEASUREMENT would have taken pure black — the far
end of every distance it computes — as the composited colour, and
certified the palette on it. The door could therefore refuse outright,
where the row's framing implies it could not.

## What the row did not name

**An ordinary authored bulge reaches site 2's defect without any
`NaN` input at all.** A bulge of `1e-320` gives `theta = 4e-320`,
`sin(theta/2)` of the same order and a radius of `inf`; every point
along such an arc is `±inf` or a `NaN`. The bulge is a literal a
`Path` step authors through `widgets::named_scalar`, and
`Expr::literal` accepts it because it is finite.
`an_arc_whose_radius_is_not_a_number_refuses_at_the_preview` drives it
through the public `preview` door.

**It takes a third vertex.** The two-vertex shape this was first
written down in — an arc straight across a chord and a closing leg
back — never reaches the flattener: the seam reverses onto itself and
the driver refuses it as an undeclared cusp two steps earlier. The
claim was right and the shape it was stated in was not, which is what
driving it through the door rather than through the arithmetic
established.

**The `centre` and `start` checks are earned by a DIFFERENT case, and
conflating the two was this row's own error.** In the producer above
`radius` is `inf`, so `arc_points` alone refuses and the `centre`
check is never what fires. What the `centre` check exists for is an
arc whose radius is an ordinary finite number and whose frame is not:
two vertices near the top of the exponent range, where the chord's own
MIDPOINT overflows on its way to a value that would have been
representable. At `from.x = 1.6e308`, `to.x = 1.5e308`, bulge 1, the
radius is `4.999e306` — finite — `arc_points` answers `Some(256)`, and
`centre` is `[inf, −3.06e290]`. `radius` carries the apothem
(`apothem = ±radius·cos(θ/2)`) and does not carry the midpoint the
apothem is measured from.
`an_arc_whose_centre_overflows_refuses_at_the_preview` is that row, and
deleting the `centre` check reds it and only it.

## Residue

Six rows, all on this slate. Two from the sweep this unit owed:
`a-count-slot-launders-a-typed-nan-into-zero` and
`world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure`.
Four from the review of #2798, which walked the blind spots this
unit's sweep stated and one it did not:
`the-shader-encodes-a-mark-strength-nothing-bounds` — **the paint
path, which is the shader and not the door this unit fixed** —
`a-nan-edge-distance-wins-its-boundary-rather-than-losing`,
`finite-bounds-yield-an-infinite-scene-radius`, and
`flatten-emits-every-vertex-before-it-judges-any-of-them`.

## What held the guards, in the end

Three of this unit's landed guards were held by nothing when the PR
was first pushed, and the review's mutations are what said so. The
rows that hold them now are
`theme::a_channel_that_is_not_a_number_is_not_a_channel`,
`path_authoring::an_arc_whose_centre_overflows_refuses_at_the_preview`,
`path_authoring::an_arc_whose_radius_is_not_a_number_refuses_at_the_preview`
and `path_authoring::an_undrawable_arc_is_refused_and_not_skipped`.
