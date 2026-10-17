---
id: census-rows-red-at-main-boss-union-interval-and-cut-cylinder-overlaps
kind: issue
title: boss_union's census escalates at Interval and cut_cylinder gains two edge-edge overlap findings: two census rows are red on main
status: open
opened: 2026-10-17
---


Two editor-core census rows fail on main as of merge `4760a784b2`, and
INTENT stage 3 B's branch fails them identically. The census reads the
same body on both sides. On the branch, boss_union's placed copy has the
union's value-channel digest, and the ids-masked corpus fence is
unmoved.

- `crates/editor-core/tests/m10_di_dual_corpus.rs:228`
  (`assemble_census_verdicts_match_f64_at_interval`):
  - boss_union's `assemble` is `Ok` at `f64`.
  - At `Interval` it refuses six `CensusEscalated` findings ("whether
    two parts of the body touch is undecided").
  - Each finding's cause is
    `Indeterminate { margin: MarginDiag(Invalid), predicate:
    "curve_pair_pose_one_curve" }`.
  - An `Invalid` margin points at a NaN-shaped interval somewhere in
    the curve-pair pose, not at a near-touch.
- `crates/editor-core/tests/perf12_census_goldens.rs:187`
  (`the_at_rest_census_verdicts_are_goldened_bit_exact`):
  - cut_cylinder's census has 14 findings where
    `tests/golden/perf12_census_1e-9.txt` pins 12. It fails at every ε.
  - The two extra findings are `UnrecordedContact { contact:
    EdgeEdgeOverlap { a: EdgeKey(5v1), b: EdgeKey(11v1) }, witness:
    "(0.0, -0.5, 0.5)" }` and its twin at `(0.0, 0.5, 0.5)`.
  - These are the split's tip copies, where the two halves meet at one
    point on the cylinder's seam line. `crates/topo/src/census.rs:3450`
    (`ee_collinear_lane`) says this point owes no record when both
    bounds are `Bound::Shared` (`:3510`, `geo.same_point`). The two
    halves' tip vertices no longer read as one point key there.

Both rows are in the per-PR slow set (`.config/nextest.toml`), so any PR
seeding editor-core shows them red until this is fixed.

A lead, not checked: `crates/editor-core/tests/m10_p_fence.rs`'s
module doc records a recent main change that moved recorded vertex
pairs onto one point key (`ops::record_classes`, through `kiss_carry`).
Point-key sharing is exactly what `Bound::Shared` reads.
