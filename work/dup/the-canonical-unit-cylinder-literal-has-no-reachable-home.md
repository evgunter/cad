---
id: the-canonical-unit-cylinder-literal-has-no-reachable-home
kind: issue
title: The canonical unit cylinder is written out seventeen times and CylFrame::canonical is above most of them
status: open
opened: 2026-09-20
---


## Finding

- **Where**: 17 literal `Surface::Cylinder { origin: <world origin>,
  axis: Vec3::unit_z(), radius: …, u_ref: Vec3::unit_x() }`
  constructions in 10 files: `geom-brep/src/certify.rs` (4),
  `geom-brep/src/pcurve_cache.rs`, `geom-brep/tests/intersect_table.rs`
  (2), `geom-brep/tests/pcurve_p1a_meter.rs`,
  `geom-brep/tests/r2_probes.rs`,
  `geom-brep/tests/review_m2_pr3_certify.rs` (3),
  `geom/src/surfaces.rs`, `topo/src/boolean/boxes.rs`,
  `topo/src/chart_region_r2_probes.rs`, `topo/src/chord_join.rs` (2).
- **Importance**: low-medium — every copy is four fields of one
  convention, and the convention is already named once
- **Confidence**: sure about the 17; see the blind spots below
- **Raised by**: the `dup/src-cyl-sheet` lane, 2026-09-20. Folding the
  cylinder-sheet builders removed two of them (`topo::census`'s and
  `topo::chart_region`'s `cyl_surface`, both now one line over
  `CylFrame::canonical(r).surface()`), which is what turned the rest
  into a visible class.

`topo::test_support::CylFrame::canonical(radius)` already IS this
convention, with `surface()` rendering it — but it lives in `topo`, and
**8 of the 17 are in `geom-brep` and `geom`, below `topo`**, so they
cannot reach it. That is the whole of the question a unit here owes:
either the frame type belongs lower (`geom` owns `Surface`, and a
`CylFrame` there would be reachable by every copy), or the two halves
are separate classes and the lower one needs its own spelling.

## The instrument, and what it cannot see

Every tracked file, no path argument: `git grep -l 'Surface::Cylinder'`,
then each `Surface::Cylinder {` brace-matched and kept only when the
body names the world origin, `Vec3::unit_z()` and `Vec3::unit_x()`.

- It misses a canonical cylinder whose fields are bound to locals or
  reached through a helper — the denominator over ALL literal
  `Surface::Cylinder` constructions is **204 in 94 files**, and the
  other 187 were not classified. Many are production code, where a
  fixture vocabulary is not the answer.
- It misses a canonical cylinder built by a constructor this repo
  already has (`geom`'s own, if one exists) and any spelled through a
  type alias.
- It says nothing about the RADIUS: a home taking `radius` serves all
  17, but a home fixing it at 1 serves only some.
