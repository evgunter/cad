---
id: harmonic-crest-in-span-has-spellings-beside-arc-window
kind: issue
title: the crest of a harmonic over a parameter span is decided beside geom's ArcWindow in mesh, sweep and topo, each in its own spelling
status: open
opened: 2026-10-25
priority: P3
cost: M
---


Found by the sweep of `D292`, which gave the fact a home:
`geom::curves::boxes::ArcWindow` — the extent of `c + p·cos t + q·sin t`
over `[t0, t1]` (`extent`) and whether its crest lies in the span
(`crest_outside`, comparison-free). `geom`'s box doors, the boolean's
and the census's conic edge boxes and axial spans, and
`geom_brep::extent`'s `Reach::range_along` and `circle_arc_reach` read
it now. The same question is answered beside it, each in its own
spelling and margin:

- `crates/mesh/src/cert.rs`, `abs_radial_max` and `abs_sin_max` — the
  extremum of `R + r·cos φ` and of `|sin φ|` over a chart interval, by
  `geom::periodic_window_may_hold` on the crest angles (f64 only; the
  "may hold" reading is conservative-inclusive). CHORD's / TESS's file.
- `crates/sweep/src/blend/surgery.rs`, `past_misses` (with `past`) — a
  circle arc's nearest and farthest distance from a point, by
  `periodic_window_may_hold` on the realising angle; the same shape as
  `geom_brep::extent::circle_arc_reach`, which reads
  `ArcWindow::crest_outside`. BAND's / CARVE's file.
- `crates/sweep/src/revolve/axis.rs`, `radial_extent` — a profile
  `Arc2`'s radial extremum, its own `copysign` membership margin. 2-D,
  so not the same type; the same fact. CARVE's file.
- `crates/geom-brep/src/implicit.rs`, `conic_arc_residual_range` — a
  conic arc's implicit residual over `[t0, t1]`, by
  `ARC_RESIDUAL_SAMPLES` + 1 samples widened by the chord-dip charge
  `max|F″|·h²/8`. Against a plane the residual is a first harmonic of
  the parameter, and a circle's against a sphere is one too
  (`conic_residual_extremes`'s own docs, `ConicHarmonics`): in those
  cases it is the extent `ArcWindow::extent` reads in closed form, with
  no charge (the degree-two cases are a different fact). Read in
  production by `topo::boolean::reduce`'s `conic_clearance` as its
  `arc_margin`, so the sampled margin is what that clearance decides
  on. ROOTS's file. The D292 sweep's patterns missed it: it loops on
  `eval` of a residual, not on `sin_cos` or `.dot(`.
- `crates/topo/src/boolean/boxes.rs`, `conic_extent` — the full-turn
  case (`h = π`, both crests in), in the `Span` lane's arithmetic.
  Consistent by construction; listed so the count is honest.

Whether each should read `ArcWindow` or keep its own spelling with an
agreement row is the unit's call (PRED's charter asks for the row
either way). `periodic_window_may_hold` stays a legitimate door for a
caller selecting between two bounds that each hold either way; what is
duplicated is the crest's membership, not that door.
