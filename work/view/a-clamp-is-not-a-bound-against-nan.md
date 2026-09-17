---
id: a-clamp-is-not-a-bound-against-nan
kind: issue
title: Three clamps in the viewer pass NaN through and hand back a value the arithmetic did not compute
status: open
opened: 2026-09-16
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
