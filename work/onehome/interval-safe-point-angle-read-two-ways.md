---
id: interval-safe-point-angle-read-two-ways
kind: issue
title: The interval-safe angle of a point about a conic is spelled twice: on_carrier's angle_near and curve_pair::pose's angle
status: open
opened: 2026-11-02
---


Found by the conic-frame unit's `atan2`-on-in-plane-components sweep
(`onehome/conic-frame`, riding `a-conics-eccentric-anomaly-is-read-five-ways`).

Two private helpers answer one question, the angle of an in-plane point
read clear of `atan2`'s cut so an `Interval` enclosure of a direction near
the cut stays narrow:

- `geom::curves::on_carrier::angle_near` (`crates/geom/src/curves/on_carrier.rs`):
  relative to a seed `near`, and half a turn on where the point lies
  definitely behind it (`curve_foot_branch`); read by `Curve3::foot`'s
  circle and ellipse seeds.
- `geom_brep::curve_pair::pose::angle` (`crates/geom-brep/src/curve_pair/pose.rs`):
  `atan2(y, x)`, or `atan2(−y, −x) + π` where `x` reads negative
  (`row::BRANCH`); read by the pose's harmonic ranges and
  `pose::window`'s foot seed.

Both feed a conic's in-plane components, which the unit now reads as
`geom::Frame::in_plane`. They differ in the anchor (a seed against
the fixed axis), so one is not plainly the other with an argument, and the
unit kept both. `geom::Conic::angle_of` is the principal value, which
neither can use at `Interval`. The question for this row: is there one
interval-safe angle reading (in `geom`, beside `on_span_branch`) that both
call, and which anchor does it take?
