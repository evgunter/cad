---
id: an-interval-revolve-of-a-bulge-an-ulp-under-a-semicircle-is-contradicted-at-its-placed-carrier-end
kind: issue
title: An Interval revolve of a profile arc whose bulge is an ulp under a semicircle's panics Contradicted at its placed carrier end
status: open
opened: 2026-11-22
priority: P2
cost: E
---

Found by EMBED's one-plane lane (`embed/one-plane`), measuring turned balls at
`Interval`; not traced past the panic.

## What

`sweep::revolve` at `Interval`, debug build, of the unit ball's profile
`[((0, -1), 0.9999999999999999), ((0, 1), 0)]` (the bulge the test helper
`crate::common::bulge((0, -1), (0, 1), (0, 0))` computes at `f64`, one ulp
under a semicircle's 1), lifted with `T::from_f64` and placed on
`SketchPlane::xy()`, panics in `sweep::swept::register_placed_carrier_end`
(`placed_segment_spec_about`, `revolve::full::build_wire`):
`SymRegistration::Contradicted` on "a placed arc's carrier end from its
placed sketch carrier end" (`geom_core::real`'s `handle`, the debug
assertion). The same profile with the bulge written `1.0` builds, and the
`f64` build of either builds.

`Contradicted` is the exact witness's proof that the registrant's theorem is
false, or that an upstream enclosure does not contain its real. Which of the
two this is was not traced.

## Reproducer

In `crates/sweep/tests/self_intersection_proves_injectivity.rs` terms:

```rust
let loops = [profile::test_support::bulge_loop(vec![
    (Point2::new(0.0, -1.0), 0.999_999_999_999_999_9),
    (Point2::new(0.0, 1.0), 0.0),
])];
let place = SketchPlane::from_frame(SketchPlane::xy().frame().map_scalar(Interval::from_f64));
let profile = Profile::new(place, lifted::<Interval>(&loops)).validate(tol()).unwrap();
revolve(&profile, axis_y(), Revolution::Full, tol()); // panics in debug
```

Measured on `embed/one-plane-measure` (main `2da45c59f` with `embed/pole`, which touches no sweep file), 2026-11-21.
