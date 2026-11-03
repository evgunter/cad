---
id: frame-azimuth-waits-on-the-frame-module
kind: issue
title: Frame::azimuth, the third angle birth, waits on PR 6463's geom::frame module
status: open
opened: 2026-11-03
refs: [6463]
---


Found by `onecopy/angle-reads`. The decision in
`an-angle-read-clear-of-its-cut-is-spelled-five-ways-and-angle-of-by-hand`
names three births: `Conic::azimuth` and the spiric tube section
(`Curve3::azimuth`, landed there), and `Frame::azimuth(p)` once `Frame`
has its own module. PR 6463 (`onecopy/frame-readings`) moves `Frame`
into `geom::frame` and was open when the angle-reads unit landed.

## Fix

Once PR 6463 is on main: add `Frame::azimuth(p) -> geom::Azimuth<T>`,
`Azimuth::of(in_plane(p).1, in_plane(p).0)`, and make
`Conic::azimuth` for the circle read through it (scaled by the radius,
as it reads now). The sites still spelling a point's or a vector's
direction in a frame by hand, `Azimuth::of(w·v_ref, w·u_ref)`, which
`scripts/gates/atan2-sites.sh` cannot see (its header names this blind
spot), then read it.

Written by the angle-reads unit:
- `topo/src/ring_path.rs` `CircleArc::param` (6463 makes it the circle's
  `param_in`), `ConicArc::distance_floor` (`in_plane` then
  `Azimuth::of(wv, wu)`, a plain-frame read on purpose), and
  `plane_roots`' roots (the phase direction `(ca, cb)` turned by
  `±offset`, a harmonic's roots, kept as a rotation);
- `topo/src/boolean/torus_region.rs` `arc_span` (`(x, y)` from its caller);
- `topo/src/boolean/circle_torus.rs`'s `θ₀` (a vector's direction);
- `sweep/src/blend/surgery.rs` `CircleFrame::past` (6463 deletes
  `CircleFrame` for `ArcReads` on `geom::Circle`; `past` becomes
  `circle.azimuth(q).near(t + π) − t`);
- `mesh/src/walk.rs`'s `mid_az` (a chart frame's `u_ref`/`v_ref`);
- `step-import/src/adopt.rs`'s ring check;
- `geom-brep/src/pair_section/winds.rs` `Channel::azimuth`'s axis arm;
- `step-import/src/recognize_curve.rs`' step reads (`plane(..)` then
  `Azimuth::of(y, x)`).

Older, chart placements and reads:
- `geom-brep/src/curve_pair/pose.rs` (the `toward` read);
- `geom-brep/src/edge_nurbs.rs` (the seam read);
- `geom-brep/src/pcurve_cache.rs`, eleven `Azimuth::of(x.dot(cv),
  x.dot(u_ref))` placements (`place.phase(Along::Azimuth, ..)` and the
  focal and spline-chart reads).

Sweep `Azimuth::of(` with `.dot(` on both arguments, over multi-line calls too, after the move.
