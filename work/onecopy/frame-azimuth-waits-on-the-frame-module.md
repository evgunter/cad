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
as it reads now). The sites still spelling `Azimuth::of(w.dot(v_ref),
w.dot(u_ref))` by hand then read it:
- `topo/src/ring_path.rs` `CircleArc::param` (6463 makes it the circle's
  `param_in`, which needs nothing more) and `ConicArc::distance_floor`;
- `topo/src/boolean/torus_region.rs` `arc_span`;
- `topo/src/boolean/circle_torus.rs`'s `θ₀` (a vector's direction:
  `Azimuth::of(w·v_ref, w·u_ref)`);
- `sweep/src/blend/surgery.rs` `CircleFrame::past` (6463 deletes
  `CircleFrame` for `ArcReads` on `geom::Circle`; `past` becomes
  `circle.azimuth(q).near(t + π) − t`);
- `mesh/src/walk.rs`'s `mid_az` (a chart frame's `u_ref`/`v_ref`);
- `step-import/src/adopt.rs`'s ring check.

Sweep `Azimuth::of(` with `.dot(` on both arguments after the move.
