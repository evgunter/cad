---
id: chord-join-names-the-booleans-section-branch
kind: issue
title: "[topo.homes] chord_join, a root construction, names the boolean's Branch and borrows its test fixtures"
status: closed
opened: 2026-11-12
priority: P2
cost: M
closed: 2026-11-14
pr: 6929
branch: onedoor/branch-move
---


Found by `topo-shared-cores-hosted-in-one-half` (branch
`onedoor/shared-cores`), which emptied `JoinError`'s `Display` of engine
readings and left these.

## What

`[topo.homes]` (`crates/topo/README.md`) makes `chord_join` a crate-root
construction, which names no type declared under an operation's
directory. `crates/topo/src/chord_join.rs`'s `names-boolean` pin in
`scripts/gates/engine-leaves.counts` is 15, and it is three things
(`scripts/gates/engine-leaves.sh --lines chord_join.rs` lists the
lines):

- `crate::boolean::Branch` (declared in `boolean/mod.rs`, `pub enum
  Branch`), on eight lines and nine times: the field
  `SectionCtx::branch`, `JoinLane::BoolPlanar`'s `branch`,
  `section_loop`'s parameter and its match on `Branch::{Positive,
  Negative, Only, Component, Line}` (the last two on one line), and
  `bool_planar_chord_spec`'s parameter.
- its test module `run_edge_carrier_rows`, on three lines and four
  times: the import of
  `crate::boolean::reduce::planar_lane_carrier_rows::{arc_sheet, spiric_cap}`
  and one use of each.
- a miscount, twice on one line: `lone_ring`'s local
  closure `between`, which the gate reads as `boolean::region::between`
  because it counts an engine item by name. Nothing there names the
  engine. The count falls when `region::between` leaves the engine or
  is listed `ambiguous`; the closure is not renamed for the gate.

`Branch` is which loop, component or line of a pair's section a germ
rides. It takes no `Body` and no arena key, so `[topo.homes]` rule 1
puts it in `geom-brep`, beside `CarriedSection`, whose components it
indexes. `Branch` is also on the gate's `ambiguous` list (`census.rs`
declares a namesake), so the move wants a name that is not shared.

## Why it was left

The unit's list was `JoinError`'s `Display` and `chord_join`'s imports
from `splitting/`. `Branch` is named about 74 times under `boolean/`
(`join.rs` alone 43), so it is a move of its own.

## What closing it takes

Move `Branch` to `geom-brep` under a name `census.rs` does not share,
re-point `boolean/`, `chord_join`, `chart_region` and `offset_derive`,
and give `arc_sheet` and `spiric_cap` a home a root test may read
(`test_support_fixtures`). Then `chord_join.rs`'s `names-boolean` line
holds the miscount alone.

## Closed (PR 6929, 2026-11-14)

The boolean's `Branch` takes no `Body` and no arena key, so `[topo.homes]` rule 1 puts it in geom-brep. It moved there as `geom_brep::SectionPick` (`pair_section.rs`): a pick out of a section, as opposed to `SsiBranch` and `ConicBranch`, which are a branch's geometry.
- Its doc defines Positive/Negative/Line in geom-brep's own terms: the sign of the margin that tells a two-loop section apart, and the order in which the pair's reading lists its lines.
- `chord_join.rs` no longer names the boolean. `arc_sheet` (now `bezier_arc_sheet`) and `spiric_cap` moved to the root `test_support_fixtures`, and census's private `spiric_cap` twin folded into the moved one.
- The engine-leaves pins fell (`chord_join` names-boolean 15 → 2, the remaining 2 being the row's predicted miscount) and none rose.
- Filed from the review: `a-branch-picks-its-loop-in-two-matches` (P2).
- This row's premise named `chart_region` and `offset_derive` as readers to re-point. Neither named the boolean's `Branch`, so they were left alone.

Model-compare row: `docs/model-compare-log/rows/PR-6929.md`.
