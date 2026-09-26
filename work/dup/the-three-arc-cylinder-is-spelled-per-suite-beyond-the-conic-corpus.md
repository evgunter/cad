---
id: the-three-arc-cylinder-is-spelled-per-suite-beyond-the-conic-corpus
kind: issue
title: The three-arc cylinder is still spelled outside sweep, and two tan(θ/4) arcs are unclassified, after the fold onto common::three_arc
status: open
opened: 2026-09-26
priority: P4
cost: D
---


## Finding

- **Where**: the hit list under *Residue* below.
- **Confidence**: sure about the hits; each residue member is a
  DIFFERENT construction from the door, which is why it stayed.
- **Raised by**: the `dup/sweep-topo-drain` lane, 2026-09-26, closing
  `conic-corpus-cylinder-has-two-parameterisations`; re-scoped by the
  same PR's fix pass.

## What PR #3284 folded

One loop and one body door for the `sweep` suites:
`crates/sweep/tests/common/mod.rs`'s `three_arc(centre, radius, first)`
(section authoring, where that module's routing rule puts a loop) and
`common::operands`' `three_arc_cylinder(centre, radius, z0, height,
first)`, which extrudes it through `sweep::test_support::extruded` on
`sketch_at(z0)`. `mate2_common`'s `three_arc` and `extruded` are gone
(the MATE-2 collar and peg build through the two doors), so there is
one home, not two. `common::operands::plate6_cyl` names the peg the
M9-3 suites stand on `plate6`. Folded, each measured `Debug`-equal to
its old builder at the arguments its callers pass (and `sketch_at(z)`
measured equal to `SketchPlane::new(Affine3::translation(z))` at every
`z` used): `n3r1_prune`, `s16_box_soundness` (both poses),
`mate2_common` (collar, peg — starts 0° and 60°, so the old loop's
missing `% 360` never wrapped), `mate2_r2_probes` (two inline
extrusions), `mate2_cyl_rest` (its flange), `curved_mergedoor` and
`r1_probes_m9_3` (`cyl_at`, one a declared copy of the other),
`m9_3_zip` (`cyl`), `m9_3_wall_door` (`cyl`, and `lying_cyl`'s loop),
`m9_2b_r2_probes`, `m5_pr9_boss_union` (×3), `s49_census_jurisdiction`
(turns 0–60°, no wrap), `review_blend1_r1_probes` (its loop, on its
own tilted plane) and `verbs_pierce_r2_probes`.

## Also folded: the M5 `n`-arc boss

`m5_s12_curved_ops`' `boss(n, z0, len)`, its `Interval` twin in
`m5_s12_curved_ops_interval`, `review_m5_pr9_boss_probe`'s copy of it
and its inline overhang boss, and `review_s12_adv`'s inline boss were
one construction — radius 0.35, joints indexed in RADIANS (`2π·i/n`),
so different bits from `three_arc`'s degree-indexed joints. They are
`common::operands::n_arc_boss(centre, n, z0, len)` (generic in the
scalar) and its named placement `m5_boss(n, z0, len)` at `(1.2, 1.7)`,
each measured `Debug`-equal to the old builder at every argument used,
at both scalars (`review_s12_adv`'s `2π·i/3` indexing included).

## Residue, at the fix pass's head

Instrument as before: `git grep` over every tracked file for
`PI / 6.0).tan()`, `FRAC_PI_6.tan()` and `(theta / 4.0).tan()`.

- `review_contact_edge_must_carry_r1_probes.rs` (~:262) and
  `review_blend4_r4_probes.rs` (~:46): `tan(θ/4)` for arcs that are not
  all 120°, candidates only.
- Outside `sweep`: `crates/stl/tests/common/mod.rs` (~:120) and
  `crates/step-export/tests/common/mod.rs` (~:280, ~:351), which cannot
  reach `sweep`'s `tests/common` and sit with
  `work/tint/tests-common-body-fixtures-triplicated.md`'s trees.

## What a taker owes

The homes exist; do not make another. Classify the two `tan(θ/4)`
candidates first; for the two out-of-crate commons, decide whether
`common::three_arc` moves to `sweep::test_support` (reachable from
`stl` and `step-export` through their dev-dependency) or those copies
stay with the triplicated-commons row. Measure bit-identity per member
before folding.
