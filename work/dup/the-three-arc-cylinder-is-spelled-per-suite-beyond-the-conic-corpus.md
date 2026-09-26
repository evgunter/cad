---
id: the-three-arc-cylinder-is-spelled-per-suite-beyond-the-conic-corpus
kind: issue
title: The three-arc cylinder is spelled per suite in about twenty places beyond the conic corpus's door
status: open
opened: 2026-09-26
priority: P4
cost: D
---


## Finding

- **Where**: the radius-0.5-ish three-arc circle (three 120° arcs,
  bulge `tan(π/6)`) extruded into a cylinder, written out per suite —
  the hit list below.
- **Confidence**: sure about the hits; not every hit is the same body
  (radius, centre, start vertex and pose differ), which is the taker's
  measurement.
- **Raised by**: the `dup/sweep-topo-drain` lane, 2026-09-26, closing
  `conic-corpus-cylinder-has-two-parameterisations`.

That PR gave the conic corpus's cylinder one door,
`crates/sweep/tests/common/operands.rs`'s `three_arc_cylinder(cx, z0,
height, first)`, with the two poses the two suites use as two knobs
(the profile slid in `x`, the sketch plane lifted in `z`), and
measured both suites' bodies bit-identical through it. The same
construction is written elsewhere, and
`crates/sweep/tests/mate2_common/mod.rs` already has a `three_arc(radius,
deg0)` loop door the MATE-2 suites use: two homes in one crate's tests.

## The hits, at `4e671da75`

Instrument: `git grep` over every tracked file, no path argument, for
`PI / 6.0).tan()` and `FRAC_PI_6.tan()`, and separately for
`(theta / 4.0).tan()`, the other spelling of the same bulge. **Blind
spots**: a bulge held in a named constant, or written as a literal;
the second pattern also matches arcs that are not 120° (it is `tan(θ/4)`
for any `θ`), so its hits are candidates, not members.

Definite (`tan(π/6)`): `crates/sweep/tests/curved_mergedoor.rs` (~:44),
`m5_pr9_boss_union.rs` (~:31, ~:127, ~:202), `m9_2b_r2_probes.rs`
(~:44), `m9_3_wall_door.rs` (~:32, ~:214), `m9_3_zip.rs` (~:26),
`mate2_common/mod.rs` (~:29, the `three_arc` door), `r1_probes_m9_3.rs`
(~:25), `review_blend1_r1_probes.rs` (~:513), `review_s12_adv.rs`
(~:255), `s49_census_jurisdiction.rs` (~:49), `verbs_pierce_r2_probes.rs`
(~:300); outside `sweep`, `crates/stl/tests/common/mod.rs` (~:120) and
`crates/step-export/tests/common/mod.rs` (~:280, ~:351), which sit with
`work/tint/tests-common-body-fixtures-triplicated.md`'s trees.

Candidates (`tan(θ/4)`, to classify): `m5_s12_curved_ops.rs` (~:97) and
`m5_s12_curved_ops_interval.rs` (~:73, the recut `boss` — three 120°
arcs at `(1.2, 1.7)`), `review_m5_pr9_boss_probe.rs` (~:44, ~:323),
`review_contact_edge_must_carry_r1_probes.rs` (~:262),
`review_blend4_r4_probes.rs` (~:46); in `sweep::test_support` (~:477,
a bore arc, not a member); in `mesh/tests` and `profile/tests`, arcs of
other sweeps.

## What a taker owes

Decide the home first: `common::operands` (body authoring, the conic
corpus's) and `mate2_common::three_arc` (a loop, not a body) are both
already there, and a third would be the defect. Then, per member,
measure bit-identity through the door at that member's own radius,
centre and start vertex before folding it, and keep any member whose
pose differs as a pose the door takes rather than a new spelling.
