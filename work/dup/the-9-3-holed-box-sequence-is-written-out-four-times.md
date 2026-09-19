---
id: the-9-3-holed-box-sequence-is-written-out-four-times
kind: issue
title: The §9.3 holed-box sequence is written out four times, and the §9.4.2 census never looked for it
status: open
opened: 2026-09-19
refs: [the-cube-sequence-is-written-five-times-and-twice-inside-src]
---


## Finding

- **Where**: `crates/topo/src/fixtures.rs` (`ops_holed_box`),
  `crates/topo/src/review_m1_pr3.rs` (`drill` / the two hole recipes
  its three `build_box` callers plant),
  `crates/topo/tests/box_with_hole.rs` (`build_holed_box`),
  `crates/topo/tests/m3_pr3_split.rs` (`holed_box_geometric`), and
  `crates/topo/tests/review_m3_pr3_rings.rs` (`holed_box`), against
  each other.
- **Importance**: medium
- **Confidence**: sure that the sequence recurs and that no census ran
  for it; NOT measured how much of each copy is the same sequence, and
  not dumped
- **Raised by**: the cube-sequence fold lane
  (`dup/fold-the-cube-sequence`), 2026-09-19, out of the X4 re-census
  on `the-cube-sequence-is-written-five-times-and-twice-inside-src`

## The class

The §9.4.2 row's subject is the CLOSED box: 1 `mvfs` + 7 `mev` +
5 `mef`. Mäntylä §9.3's genus-1 box is a strict superset of it — the
closed box, then `mev` + `kemr` to plant a ring, grow and close the
rim, drop the verticals, cut the tube walls, `kfmrh` the membrane —
and it is written out at least four separate times in `topo` alone (the title's count, and it is a floor).
Now that the §9.4.2 half folds onto
`test_support_fixtures::prism_ops`, the hole half is what is left
standing beside it, and in two of the four the box half was folded
away while the hole half was not.

The hole recipes are not byte-identical: the extents differ (unit,
2x2x2, 4x2x2), one is triangular through a SIDE face rather than
square through the top, and one plates its faces afterwards. That is
the same shape the cube row had before it was measured — candidates
until dumped.

## Why the §9.4.2 censuses could not see it

All three instruments that row ran are keyed on the CLOSED sequence:
the arity census thresholds on `mev`/`mef` call-site counts, the name
census on cube/box/prism builder names, the geometry census on
`mvfs(` + `newell_plane`. A §9.3 body matches the first two trivially
(it contains a §9.4.2 body) and so is REPORTED as a cube-sequence hit
and dispositioned as one — which is exactly what happened to
`crates/topo/tests/m3_pr3_split.rs`. Nothing asked the separate
question. The needle that finds this class in one line is
`kemr(` together with `kfmrh(`, which returns nineteen files over
every tracked path (run 2026-09-19, no path argument): the five above,
plus `sweep/src/{extrude,loft,revolve/partial}.rs` and
`step-import/src/assemble.rs` (production ring machinery, not
fixtures), `topo/src/{euler_kill,euler_ring,movefac,seqgen,shell}.rs`
and `topo/src/review_m1_pr4.rs`, `topo/tests/m3_pr1_surgery.rs`,
`topo/tests/review_m1_pr5.rs`, `topo/tests/review_m3_pr1.rs`
(`plant_detached_box` grows a box FROM a planted ring rather than from
an `mvfs`, so it is a relative, not a member) and
`topo/tests/loop_reparenting_pcurve_rows.rs`. Which of those hold a
§9.3 BODY rather than an isolated ring op is the first thing the unit
measures; this row records the needle and the hit list, not a
count.

## What is unmeasured

- Whether the four bodies agree operator-for-operator once the extent
  and the hole placement are parameters, which is what the cube row
  had to dump to establish.
- Whether `fixtures::ops_holed_box`'s consumers depend on its declined
  face geometry the way `ops_cube`'s do (measured there: 10 rows red
  when handed real planes).
- Whether `box_with_hole.rs` is exempt the way `cube_by_hand.rs` is —
  both are M1 acceptance tests that validate after EVERY operator, and
  that per-operator validation is plausibly their subject.
