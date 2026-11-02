---
id: circle-pair-crossing-is-spelled-three-times
kind: issue
title: Whether two circles cross transversally is decided three times over: the coaxial-tube rows, the parallel-tori arm's reach and nest rows, and profile's circle_circle
status: closed
opened: 2026-10-27
priority: P1
cost: E
closed: 2026-10-29
branch: sect/circle-crossing-once
---


Found in review of PR 5869 (`sect/lily-parallel-tori-certificate`),
2026-10-27.

## Measured

The two margins that say whether two circles in a plane cross in two
points (`r₁ + r₂ − d` and `d − |r₁ − r₂|`) are written out three times:

- `crates/topo/src/boolean/section_cert.rs` `torus_pair`, the coaxial
  torus rows `section_torus_coaxial_tube_reach` and
  `section_torus_coaxial_tube_nest` (the two tube circles in one
  meridian plane);
- the same file's `torus_parallel_torus`, per meridian-circle pair, as
  `section_torus_parallel_tori_reach` and `section_torus_parallel_tori_nest`,
  followed by the crossing points' closed form;
- `crates/profile/src/path/arc_fillet.rs` `circle_circle`, as the
  `path_carrier_meet` rows beside a concentric row, also followed by
  the crossing points.

## What the taker owes

One spelling of "two circles cross transversally, and where", in a home
both crates read (each site keeps its own predicate names and its own
across-the-slack reading), or a ruling that the three are not one
predicate, recorded at each site.

## Closed (2026-10-29, `sect/circle-crossing-once`)

The home was already there: `geom_core::Circles`
(`crates/geom-core/src/arc/pair.rs`), which both `geom-brep` and
`profile` depend on through `geom-core` and which profile's `seg.rs`
and geom-brep's `intersect.rs` sphere × sphere already read. It gains
`reach()` (`r₁ + r₂ − d`, its own difference as `nested()` is) and
`secant()` (the lesser of `reach()` and `internal()`). The sites now
read it, each keeping its row names and its across-the-slack reading:

- `pair_section/torus.rs` `TorusTorusPose::decide`, the coaxial tube
  rows (`reach()`, `internal()`), and `crossing_parallels`, the two
  parallels (`Circles2::crossings`);
- `pair_section/torus_arms.rs` `parallel_torus_arm`: the meridian
  circle pairs' reach and nest rows and their crossings; the sheet
  circles' gap and span rows (`internal()`, `reach()`) and the point
  (`radical()`, `half_chord()`), the same predicate under other names;
  and `square_folds`' `section_torus_square_wall_tube` row
  (`secant()`) and its two folds;
- `profile/src/path/arc_fillet.rs` `circle_circle`, the two
  `path_carrier_meet` clearances (`reach()`, `internal()`; the
  concentric row reads `d()`). The corner keeps the squared-radius form:
  `Circles`' placement reads the radii themselves, which LB4 rules out,
  and the site says so. That fork is
  `work/profilerest/arc-fillet-corner-circle-pair-cancels-at-unequal-radii.md`;
- `profile/src/sugar.rs` `offset_circles`, the
  `fillet_offset_circles_external` and `_internal` margins.

Every margin above is bit for bit the old spelling's, so no verdict
moved: the rows' decisions, logged on each site's existing tests
before and after, are identical, and 800,000 seeded pairs at `f64` and
at `Interval` give the same bits. The placements are `Circles`'
cancellation-free ones, off both circles by at most 3.2 ulps of scale
where the old form reached 9.4·10⁵. The sphere's rows against a tube
circle are the same predicate in another decomposition, and reading
`secant()` there moves a verdict at `Interval`:
`sphere-against-a-tube-circle-decides-its-secant-in-its-own-decomposition`.
