---
id: analytic-carriers-read-through-point-value-and-the-enclosure
kind: issue
title: Analytic carriers read through point_value and the enclosure, not map_scalar(closure)
status: open
opened: 2026-11-18
priority: P2
cost: M
refs: [7131, carrier-frames-the-types-and-the-lift]
---


Filed from the frame-lift fork (PR 7131, `work/onehome/log.md`, 2026-11-18).
Once a carrier's frame is a witness, a carrier crosses scalars by two
named reads, not by `Surface::map_scalar` / `Curve3::map_scalar` with an
arbitrary closure (a closure over the twelve numbers can unmake the
witness):

- `point_value`, any scalar to `f64`;
- the enclosure, any bracketed scalar to `Interval` (from `f64`, the
  frame's polar factor; `RigidMotion::map_scalar`'s `Interval` arm today).

The about 16 production sites that lift an analytic carrier today spell
the crossing five ways:
- `Interval::from_certified`;
- `lane.lift`;
- `Lift::of` (`topo/src/clearance.rs`'s `certified_lift`);
- `Interval::from_bounds(lo, hi)`;
- `Interval::from_bounds(x, x)`.

Each moves onto the enclosure read, and the closure form retires for
analytic carriers.

Two off-question findings ride on this row:
- `ConstructedLoop::<f64>::lift_computed`
  (`crates/profile/src/path/program.rs:3573`) has no caller and is an
  `f64` → any-scalar door;
- `geom-brep`'s `pcurve_cache` `rendered` (`crates/geom-brep/src/pcurve_cache.rs`,
  about line 4265) builds lo/hi carriers, which will not survive the
  witness.
