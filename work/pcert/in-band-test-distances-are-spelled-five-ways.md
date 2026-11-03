---
id: in-band-test-distances-are-spelled-five-ways
kind: issue
title: A distance inside the band is spelled five ways across the test helpers
status: open
opened: 2026-11-03
priority: P3
cost: E
---

Filed by PCERT unit E5 (PR 6484, review S1). E5 moved its own copy of
the semicircle prism's helpers into `crates/sweep/tests/common/semicircle_wall.rs`,
and found that the in-band distance those helpers carry is one of five
spellings of the same idea. All are "a distance the band can neither
confirm nor rule out". `fn in_band() -> f64` appears in 13 places
(as of `a2b8e463` plus E5's own files):

- the geometric mean `√(zero·escalate)`:
  - `crates/topo/src/boolean/ops.rs` (test module, near :9764);
  - `crates/sweep/src/swept.rs` (near :1979);
  - `crates/geom-brep/src/pcurve_cache/cylinder_line_rows.rs` (near :31);
  - `crates/geom-brep/src/dihedral.rs` (near :3551);
  - `crates/geom-brep/src/dihedral_side.rs` (near :487);
- the arithmetic mean `½(zero + escalate)`:
  - `crates/geom-brep/tests/spline_pair_arc_rim.rs` (near :227);
  - `crates/topo/tests/a_circle_not_shown_on_a_spline_row_refuses_alike_in_every_form.rs` (near :44);
  - `crates/topo/src/census.rs` (near :4192, which also asserts its own placement);
- `ε·√K`: `crates/sweep/tests/common/semicircle_wall.rs` (near :27);
- `ε·(1 + K)/2`: `crates/profile/tests/rejections.rs` (near :744);
- `5·ε`, which lies inside the band only while `K > 5`:
  - `crates/profile/tests/arc_fillet.rs` (near :1041);
  - `crates/sweep/tests/m5_pr12_refusals.rs` (near :34);
  - `crates/sweep/tests/review_blend1_r1_probes.rs` (near :27).

Besides these, `crates/topo/src/pcurves.rs`'s rows and the lamina rows in
`crates/topo/tests/an_off_iso_straight_edge_takes_its_general_row.rs`
spell `ε·√K` inline.

## Fix

One home for an in-band distance in `geom_core::test_support` (a `Band`
method or a free function over the run's band), and every copy reads
it. Keep the geometric mean (the midpoint in log scale, so it stays
inside the band at every `K > 1`) unless a row needs a specific point,
in which case it says which. The `5·ε` copies are the ones that can
leave the band, so check each row's pose survives the move.
