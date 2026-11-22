---
id: conic-constructions-from-computed-sizes-unaudited
kind: issue
title: About 240 kernel sites build a circle or ellipse from a computed size; each should decide its sign before construction or be provably positive, and most are unaudited
status: open
opened: 2026-11-20
priority: P1
cost: H
---


Filed by GERM's `germ/ellipse-speed-magnitudes` lane (PR 7183). That PR
makes a negative or non-finite conic size unrepresentable:
`geom::Circle::new` and `geom::Ellipse::new` panic on one
(`crates/geom/src/curves/conic.rs`, `stored_size`). A kernel site that
computes a size and builds a conic from it now panics where it used to
store a signed or NaN size. The hosted suite surfaced six such sites,
fixed in that PR: `sweep::blend::battery::frustum_arm` (a chamfer's
plane trim past the axis), `geom_brep::pcurve_cache::limb2_memo::rendered`
(a memo key built conics at bracket ends), and four test fixtures.

## The sweep that remains

The lane listed every `Curve3::circle`, `Circle::new` and `Ellipse::new`
call outside the obvious test files whose size argument is not a
literal. There were about 248, the count including in-file test
modules that the listing could not tell apart. The lane read these:

- **decided before construction:** `geom_brep::intersect` plane × sphere
  (`ps_center_gap`, `intersect.rs` near 1457), cone × torus
  (`kot_apex_station`, `kot_reach`, near 3276), `Curve3::ellipse`, and
  STEP `CIRCLE`/`ELLIPSE` (`interpret.rs`, `positive_length`);
- **provably non-negative:** `sweep::blend::arms::Meridian::blend`
  (`ring_radius` turns a signed radius into its magnitude and the seam),
  STEP `build.rs` sphere stations (`radius·dir.x`, with `dir` the unit
  direction of a radial distance), `topo::offset_axial` concentric
  sphere seam (`radius + distance`, a great circle of the offset sphere);
- **stored values re-read:** `scalar_lift`, `end_face` (`major_frame`),
  `ring_path`, and `signed_zero`.

The rest are unread. Each needs one of those dispositions, or a pose
decision before the constructor on its own lever and band.
Reproduce the listing by grepping `(Curve3::circle|Circle::new|Ellipse::new)\(`
under `crates/*/src` and dropping the literal sizes.
