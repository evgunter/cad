---
id: bounds-probe-guards-its-seed-and-not-the-ladder-it-doubles
kind: issue
title: BoundsProbe::new guards its seed and not the 2^11 ladder it derives, so furthest_reach is inf for a large seed
status: open
opened: 2026-09-22
priority: P3
cost: E
---



Found by the sweep of
`camera-new-admits-a-scene-radius-whose-distance-band-is-not-finite`
(`vgeom/camera-band`), whose class is **a door that guards its INPUT
and not the PRODUCT it derives from it**. `Camera::new` guarded
`is_finite` on `scene_radius` and nothing on the `×100` band it
derives; this is the same shape with a `×2^11` ladder in place of the
factor.

## The finding

`crates/viewer/src/bounds.rs`. `BoundsProbe::new` normalises its seed
and asks it exactly one question:

```
    let seed = if seed.is_finite() && seed != 0.0 { seed.abs() } else { 1.0 };
```

and `reach_offset` derives the ladder from it:

```
    fn reach_offset(seed: f64, doubling: u32) -> f64 { seed * f64::from(1u32 << doubling) }
```

with `MAX_REACHES = 12`, so the last rung is `seed · 2^11`. **Every
seed above `f64::MAX / 2048` — about `8.8e304` — is accepted and its
ladder is not a number.** `BoundsProbe::furthest_reach`, which is
`pub` and documented as *a contract a test has to reason against*,
answers `inf` for such a seed; `BoundsProbe::next` then hands the
caller `origin + sign * inf`, an infinite candidate to evaluate, and
`observe` records an infinite offset into the bracket. A refine over
`[finite, inf]` takes midpoints of an infinity, so the reported
`Bounds` carries `inf` or `NaN` as a field's valid range.

## Reachability

**Not through the session.** `session::probe::probe_seed` is
`props::from_written(1.0, unit)` — one of the field's unit in
canonical terms, which is `1.0` or a small unit factor, nowhere near
`8.8e304`.

**Through the door, in one call.** `BoundsProbe::new` and
`furthest_reach` are both `pub`, and the suite already calls them
directly (`crates/viewer/tests/valid_range.rs`,
`crates/viewer/tests/docm9_range_vs_probe.rs`). That is the same
standing the closed camera row rested on: a public door whose guard is
finiteness, one caller away from a product that is not.

## What the fix is

Ask the ladder rather than the seed, at the door that normalises it —
the shape `crate::scene::DisplayTolerance::new` uses and (since
`vgeom/camera-band`) `Camera::new` uses. `BoundsProbe::new` has no
`Result`, so the two candidates are a **substitution** in the existing
normalising shape (a seed whose furthest reach is not finite falls
back the way a non-finite one already does, which keeps the door
total) or **a typed refusal**, which changes the signature and every
caller. The substitution reads as the smaller change and matches what
the door already does with a non-finite seed; a lane taking this row
should say which and why, because a silent substitution is the thing
`work/vgeom/viewer-substituted-value-class-is-crate-wide` is about.

## Fence

`crates/viewer/src/bounds.rs` — VGEOM-claimed, double-claimed with
author, chrome and view. Held out of `vgeom/camera-band`, whose fence
was `crates/viewer/src/camera.rs` alone.
