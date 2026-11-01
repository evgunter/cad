---
id: world-per-px-answers-a-scale-for-a-viewport-it-could-not-measure
kind: issue
title: world_per_px's height guard does not bound a NaN, so it answers Some(NaN) where None is its refusal
status: closed
opened: 2026-09-17
priority: P1
cost: E
closed: 2026-09-21
branch: vgeom/refusal-floor
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

## Closed — the height is asked whether it is a number, and so is the quotient

`world_per_px`'s guard is `!(height_px.is_finite() && height_px > 0.0)`,
which is the ordering it always had with the domain test the ordering
cannot make. `None` — this door's own refusal, *there is no viewport
to scale against* — is what a height that is not a number gets.

**The quotient is guarded too, and that is a deliberate addition.**
The scale is `2 · distance · tan(fov/2) / height`, so the smallest
subnormal height divides an ordinary visible height to infinity: a
number, perfectly ordered, and not a rate. This is `datums.rs`'s rule
from #2644 — *the check is on the PRODUCT and not on the
metres-per-pixel, because a scale that is a length does not make every
multiple of it one* — applied to the quotient. Executed: at
`f64::MIN_POSITIVE` the answer is about `1.8e307` and finite, so the
guard is not a magnitude cut-off; at `f64::from_bits(1)` it is
infinite.

**Row**: `crates/viewer/tests/input_mapping.rs`,
`a_viewport_height_that_is_not_a_measurement_binds_no_pan`, driven
through the public `InputMap::map`. Three refusing heights (`NaN`,
`inf`, the smallest subnormal) answer `None`, and an ordinary viewport
still binds a pan with both components finite and non-zero — the pair,
because either half alone is satisfied by a door that refuses nothing
or by one that refuses everything. The refusing half asserts `None`
rather than a difference from the ordinary answer, for the reason the
register gives: `assert_ne!` against a float is not a
distinguishability test, and the unfixed door's `Pan { right: NaN }`
would have passed one.

**Mutation**: restoring `height_px <= 0.0` reds that row and nothing
else in the 798-row app-feature suite; separately, dropping the
quotient's `is_finite` reds that row and nothing else.

## The reachability is unchanged and is still a negative result

The row's *"Reachability: no producer found"* stands as written. No
route was found by which egui hands back a rectangle whose height is
not a number, and **that is a negative result over this crate's own
writers of `ViewportSize`, not a proof about egui's layout.** Nothing
in this unit searched further, and nothing in it should be read as
upgrading the claim in either direction. The infinite-quotient arm is
in the same position: `2 · distance · tan(fov/2)` overflowing needs a
stand-off no scene here produces, and the subnormal height has no
traced producer either.

PR: `vgeom/refusal-floor`.
