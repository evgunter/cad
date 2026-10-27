---
id: circle-pair-crossing-is-spelled-three-times
kind: issue
title: Whether two circles cross transversally is decided three times over: the coaxial-tube rows, the parallel-tori arm's reach and nest rows, and profile's circle_circle
status: open
opened: 2026-10-27
priority: P1
cost: E
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
