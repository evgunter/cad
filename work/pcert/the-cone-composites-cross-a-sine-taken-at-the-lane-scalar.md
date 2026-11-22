---
id: the-cone-composites-cross-a-sine-taken-at-the-lane-scalar
kind: issue
title: The edge reader's cone composite and metres lever take sin α and cos α at the lane scalar and then cross them, so at f64 they are rounded values, not brackets
status: open
opened: 2026-11-21
priority: P2
cost: E
---


Filed by SECTCONE (`sectcone/cone-tube-arm`, the tube's cone arm),
from its sweep for cone half-angles crossed into certification
arithmetic. Designer B on the SECTCONE P0 raised it, unverified as a
defect.

## What

Two sites take the stored half-angle's sine and cosine at the lane
scalar `T` and then cross the results into `Interval`:

- `crates/geom-brep/src/pcurve_cache.rs` `twin_span_hulls`, the
  `Surface::Cone` arm of the `CanonicalSurface` it composes:
  `let (s, c) = half_angle.sin_cos(); let (s, c) = (cross(s), cross(c));`
- `crates/geom-brep/src/pcurve_cache/projected.rs` `Meters::over`, the
  `Surface::Cone` arm: `let (sin, cos) = half_angle.sin_cos();` and
  then `sin: cross(sin), cos: cross(cos)`.

At `T = f64`, `half_angle.sin_cos()` is the correctly rounded `f64`
sine, and `from_certified` turns it into a point interval. That point
is within an ulp of the stored cone's sine, but it is not a bracket of
it. So the composite's `cos²α`, `sin²α` and the lever's `sin α`,
`cos α` are not enclosures of the stored cone's coefficients, and
limb 2's bound is off by that rounding times the carrier's reach.

The order that brackets is to cross first and take the sine on the
interval lane. `crates/geom-brep/src/curve_face/composite.rs`
`framed_form` already does this
(`cross(half_angle).sin_cos()`), and so does the tube's cone arm
(`crates/geom-brep/src/implicit.rs` `cone_sin_cos_enclosure`, read by
`ssi::enclose`).

## Fix

In both arms, take `cross(half_angle).sin_cos()` (or call
`implicit::cone_sin_cos_enclosure`), the way `framed_form` does. The
`Meters::over` arm is generic over the crossed scalar `S`, so it needs
`S`'s own `sin_cos` on the crossed angle.

If `[ev]` PR 7326 has the cone store its slope, the question goes away:
`cos α = 1/√(1+k²)` needs no transcendental.

`sectcone/one-limb-two-reader` is editing `pcurve_cache` concurrently
and may take this up.
