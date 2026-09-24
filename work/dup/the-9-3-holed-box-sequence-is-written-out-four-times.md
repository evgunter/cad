---
id: the-9-3-holed-box-sequence-is-written-out-four-times
kind: issue
title: The §9.3 holed-box sequence is written out four times, and the §9.4.2 census never looked for it
status: closed
opened: 2026-09-19
refs: [the-cube-sequence-is-written-five-times-and-twice-inside-src]
priority: P4
cost: E
closed: 2026-09-24
branch: dup/topo-fixture-batch
pr: 3152
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

## Re-census at the merge base (2026-09-24, `dup/topo-fixture-batch`, `6db5b87f2`)

**Instrument 1, the row's needle**: files holding both `kemr(` and
`kfmrh(`, `git grep -l` over every tracked file, no path argument.
**19 code files**, the same 19 the row lists (plus two tracker rows).
**Blind spot**: a member that plants the ring and stops before the
tube (no `kfmrh`), and a member reached through a helper in another
file.

**Instrument 2, aimed at that gap — the rim grown inside a planted
ring**: `MevSite::Lone { r#loop: <x>.ring }`, one-line and wrapped
forms, every tracked file. It found four **(f)–(i) ring-face
plants** no `kfmrh` needle can see:
`merge_faces.rs`'s `cube_with_membrane` and
`cube_with_arena_first_membrane`, `validate.rs`'s `lamina_with_ring`,
and `tests/review_f7_pole_r1_probes.rs`'s `inset_patch_prism`. Its
other hits are the production ring machinery
(`sweep/src/{extrude,loft}.rs`, `boolean/vtxfac.rs`), single-segment
rings on a line (`movefac.rs`'s tests, `review_m1_pr4.rs`'s
`body_with_cycle_ring`, `validate.rs`'s `detached_digon_body`) and `review_m1_pr5.rs`'s seven digon islands — a
two-corner rim closed by `mef(he_plus, he_minus)`, a different shape.

**A fifth hole body the row did not name**:
`review_m1_pr4.rs`'s `carve_hole` (the PR 3 recipe "compacted; no
ledger asserts"), whose one caller built `ops_holed_box` plus the
triangular hole — the body `fixtures::ops_genus2` builds, byte-identical
at the merge base (`{:?}` of both bodies compared).

**And a third plating pass**: the two `tests/` holed boxes each end in
the same "every face gets its outer loop's Newell plane" loop, and
`validate.rs`'s `plane_every_face` is that loop a third time.

## Dispositions

| site | disposition |
| --- | --- |
| `fixtures::ops_holed_box` | **folded** onto `test_support_fixtures::drill_hole` |
| `fixtures::ops_genus2` | **folded** onto `drill_hole` (its own copy of the loop-written recipe) |
| `review_m1_pr4::carve_hole` + `genus_two_double_hole_body_tears_down_to_nothing` | **folded**: the row now takes `fixtures::ops_genus2`, which asserts the genus-2 ledger the row used to re-assert; `carve_hole` deleted |
| `tests/m3_pr3_split.rs` `holed_box_geometric` | **folded** onto `test_support_fixtures::holed_block` (4, [1]) + the description step |
| `tests/review_m3_pr3_rings.rs` `holed_box` | **folded** onto `holed_block` (6, [1, 5]) at both scalars; the file's own builder deleted |
| `merge_faces.rs` `cube_with_membrane`, `cube_with_arena_first_membrane` | **folded** onto `test_support_fixtures::plant_ring_face` |
| `validate.rs` `lamina_with_ring` | ring half **folded** onto `plant_ring_face`; its n-gon lamina is a sheet, not this class |
| `validate.rs` `plane_every_face` + the two `tests/` plating loops | **folded** onto `test_support_fixtures::plane_every_face` |
| `review_m1_pr3::carve_hole` | **kept**: the row it serves (`independent_genus_one_and_two_builds_with_hand_ledger`) claims the hand ledger after EVERY operator, plus kill hygiene and arena-untouched-by-`kfmrh` checks interleaved between them. That is the surviving clause of `memories/review-and-dependency-policy.md` — own code where a row's claim needs its own derivation — not the withdrawn "never simplify" one |
| `tests/box_with_hole.rs` `build_holed_box` | **kept**, as `cube_by_hand.rs` is: the M1 acceptance test validates after every operator, and that per-operator validation is its subject |
| `tests/review_f7_pole_r1_probes.rs` `inset_patch_prism` | **kept**: an (f)–(i) relative, not a hole, in a file whose header records R1's fixture geometry as preserved verbatim; left as the file says |
| `euler_kill`, `euler_ring`, `movefac`, `seqgen`, `shell.rs`, `sweep/src/*`, `step-import/src/assemble.rs`, `tests/{m3_pr1_surgery,review_m1_pr5,review_m3_pr1,loop_reparenting_pcurve_rows}.rs` | not members: production ring machinery, or isolated `kemr`/`kfmrh` rows on pillows, segments and planted inner boxes — no strut → rim → membrane → tube sequence |

**Byte-identity, measured**: `{:?}` of every folded fixture's body
(and `ops_holed_box`'s whole key bundle) before and after, twelve
bodies — `ops_holed_box`, `ops_genus2`, `ops_ring_bridge`, both
membrane cubes, the cross lamina, `plane_every_face` on the declined
cube and on the holed box, `holed_box_geometric`, the rings box at
`f64` and at `Interval`, and the untouched f7 prism as a control. All
twelve identical.

## Closed (2026-09-24, PR #3152)

The §9.3 surgery has one home, `test_support_fixtures`, in two levels:
`plant_ring_face` (steps (f)–(i)) and `drill_hole` (f)–(l) on top of
it, with `plane_every_face` and `holed_block` beside them. The PR
body carries the plant table.
