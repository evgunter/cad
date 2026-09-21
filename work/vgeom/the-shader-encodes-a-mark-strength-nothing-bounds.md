---
id: the-shader-encodes-a-mark-strength-nothing-bounds
kind: issue
title: The paint path a NaN actually reaches is the shader, and the Rust/WGSL parity row compares constants only
status: open
opened: 2026-09-17
priority: P1
cost: D
---

Found by the review of #2798, which asked the question that PR's own
reachability table pointed at and did not follow.

## Finding

#2798 made `theme::channel_to_srgb8` refuse a channel that is not a
number. That door is on the **measurement** path — `Mark::over` and
`from_linear` are reached only by `crates/viewer/tests/theme.rs`'s
colourblind check, because the mixing and the encode a pixel goes
through are both in WGSL. **The paint path is the shader, and it is
unguarded at both ends.**

`crates/viewer/src/gpu.rs`, `mark_lane`:

```
fn mark_lane(mark: Mark) -> [f32; 4] {
    let [r, g, b] = crate::theme::linear(mark.tint);
    [r, g, b, mark.strength]
}
```

The tint reaches linear light through `channel_to_linear`, which is
total over `u8` and finite for every input. `mark.strength` is a
`pub` field on a `pub` struct and goes into the uniform's `w` lane
raw, with no door between the field and the GPU.

In the shader, `tint` is `mix(base, mark.xyz, mark.w)` — a `NaN`
weight makes the whole mixed colour a `NaN` — and `to_display` is

```
    let c = clamp(linear, vec3<f32>(0.0), vec3<f32>(1.0));
    let toe = c * 12.92;
    let curve = 1.055 * pow(c, vec3<f32>(1.0 / 2.4)) - 0.055;
    return select(curve, toe, c <= vec3<f32>(0.0031308));
```

which is the same shape #2798 repaired on the Rust side, in a language
whose `clamp` is specified as `min(max(e1, e2), e3)` and gives no
guarantee about a `NaN` either. `c <= 0.0031308` is false for a `NaN`,
so `select` takes `curve`, and `pow(NaN, …)` is a `NaN`. What a
poisoned strength paints is whatever the surface does with a `NaN`
channel, which is not a decision this repo has made anywhere.

## The parity row is blind to this by construction, and wants correcting

`gpu.rs`'s `the_shaders_srgb_curve_states_the_same_constants_the_palette_does`
is the one row holding the two spellings of IEC 61966-2-1 together, and
it says why it is the constants it compares:

> The constants are what is compared, because they are what a
> divergence would be made of.

That was true when it was written and #2798 made it false: the two
spellings now differ in a **guard**, not in a constant, and the row
cannot see a guard. The sentence is the interesting half of this item —
a row that states the population it covers, and a change that adds a
member outside it. Whatever repair the shader gets, that sentence is
part of it.

## Fence

`crates/viewer/src/gpu.rs` — VIEW's, the standing double claim with
CHROME.
