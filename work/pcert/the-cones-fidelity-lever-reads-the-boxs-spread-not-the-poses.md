---
id: the-cones-fidelity-lever-reads-the-boxs-spread-not-the-poses
kind: issue
title: The cone's fidelity lever reads the box's spread, not the pose's
status: open
opened: 2026-11-04
priority: P2
cost: E
---


Filed by PCERT unit E4 (`pcert/circle-general-lane`, PR 6485), from
the delta verifier's fix list X (X3).

## What

Check 4's fidelity term on a cone carries the stored carrier's
distance `d` from the image through `N`'s Lipschitz bound over the
part's chart box
(`crates/geom-brep/src/pcurve_cache/projected.rs:2716`):

```rust
T::one() + (boxed.v_reach() + d) * half_angle.sin().abs() / reach(rho_floor)?
```

That is the box's largest slant over its least radius, taken over the
whole box. Near the apex a part's box spreads from a radius close to
the apex out to its far end, so the ratio is the box's spread, not the
pose's: about 2700 on the needle-cone ruling of
`pcurve_cache::circle_lane_rows::a_lane_choices_envelope_refusal_ends_not_yet`
(`circle_lane_rows.rs:1184`), read from `9e-4` of the apex. Pointwise
along the ruling the factor is about 2.

This is the same looseness E4 removed from the incidence term's metre
conversion, which halves a part where the cone's lever spreads
(`line_off_cone_sup`, `projected.rs:2406`, `LEVER_SPREAD`). Because the
fidelity term is not halved the same way, that row's fixture certifies
at `f64` but ends "not yet" at `Interval` and ε `1e-12`: there the
enclosures' width, read through the box's spread, reaches the band.

## Fix direction

Read the fidelity lever per part, halved where the lever spreads, as
the incidence conversion now is. Or bound it pointwise along the
image (slant over the radius at the same parameter) rather than as a
quotient of the box's extremes.
