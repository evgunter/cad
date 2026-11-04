---
id: the-spiric-two-oval-regime-is-decided-in-three-places
kind: issue
title: The spiric two-oval regime is decided in three places under three site names, with no shared predicate
status: open
opened: 2026-11-04
priority: P3
cost: C
---


Filed by SPLICE's `splice/spiric-on-cap-plane` lane (PR 6523), from its
review.

## Finding

Whether a plane parallel to a ring torus's axis cuts it in two ovals
(`0 < |d| < R − r`, `d` the plane's offset) is decided in three places.
Each has its own site names and its own spelling of the margin:

- `crates/geom/src/curves.rs`, `Curve3::spiric` (the mint):
  `spiric_off_axis` on `offset`, then `spiric_two_ovals` on
  `(major_radius − minor_radius) − |offset|`.
- `crates/geom-brep/src/intersect.rs`, the plane × torus section (near
  the `PlaneTorusSection::MeridianCircles` arm): the off-axis reading,
  then `decide_across(["pt_spiric_two_ovals", "pt_spiric_two_ovals_floor"],
  (R − r) − |d|, tilt, band)`.
- `crates/topo/src/boolean/slit.rs`, `spiric_on_quadric`:
  `bool_rider_spiric_regime` on `offset`, then on `R − r − |offset|`.

No predicate holds "the two-oval regime" once. A change to the regime's
boundary (a lever arm on the margin, a floor) would have to be made
three times, and the three can drift apart. That would let a carrier
the mint admits read out of regime at the rider, or the reverse. One
home, a predicate beside `Curve3::spiric` that the other two call,
would hold it once. The section's `decide_across` floor would then be
that predicate's to state.
