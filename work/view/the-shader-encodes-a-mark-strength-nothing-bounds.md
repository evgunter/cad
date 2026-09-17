---
id: the-shader-encodes-a-mark-strength-nothing-bounds
kind: issue
title: The paint path a NaN actually reaches is the shader, and the Rust/WGSL parity row compares constants only
status: closed
opened: 2026-09-17
closed: 2026-09-17
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

## Closed

**The guard went on the type, not at `mark_lane` and not in the
WGSL.** `Mark::strength` and `Theme::ambient` are
`theme::MixFraction`s: `[0, 1]` is a property of every value of that
type rather than a claim in a doc comment, so `mark_lane` and
`ViewportCallback::block` read `.get()` and need no door of their own,
and the shader receives a weight it can mix with because no other kind
exists. `MixFraction::new` refuses a caller at run time;
`MixFraction::literal` asserts in a `const` context, so a registry
palette outside the range fails the BUILD.

`ambient` was not in this item and is the same defect: a `pub f32` on
the same `pub` struct, written into `base_color`'s `w` lane raw, and
consumed by `ambient + (1 - ambient) * lambert`, which has no more
standing against a weight that is not a number than the mix does. One
type closed both.

**What this item was wrong about**, in one place: it says
`Mark::strength` goes to the uniform "with no door between the field
and the GPU", which was true, and implies that is where a NaN would
come from. **No producer of one exists.** Every `Theme` that reaches
`ViewportCallback` comes from `Theme::ALL`; `mark.strength` is read
everywhere and computed nowhere; and no public door admits a foreign
`Theme` — `gpu` is a private module and `ViewerApp::theme` is private,
so the crate's API lets a consumer BUILD a poisoned `Mark` and gives
it nowhere to put one. The defect was latent, and the base-tree red
that proves it is a probe through `mark_lane` rather than a route a
user has.

**The parity row's sentence is corrected, and the row is widened.** It
no longer says the constants are what a divergence would be made of:
it names the divergence that is made of a guard (#2798's NaN refusal,
which `to_display` has no counterpart for) and says why the shader
needs none. The widening is separate and was a real hole — the row
read only `SHADER` and never the Rust half, so on the base tree
rounding `channel_to_srgb8`'s exponent to 2.2 left it **green**.

The rest of the boundary sweep is
`the-viewport-and-position-lanes-narrow-to-f32-with-no-door`.
