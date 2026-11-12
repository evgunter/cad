---
id: chord-join-names-the-booleans-section-branch
kind: issue
title: "[topo.homes] chord_join, a root construction, names the boolean's Branch and borrows its test fixtures"
status: open
opened: 2026-11-12
---


Found by `topo-shared-cores-hosted-in-one-half` (branch
`onedoor/shared-cores`), which emptied `JoinError`'s `Display` of engine
readings and left these.

## What

`[topo.homes]` (`crates/topo/README.md`) makes `chord_join` a crate-root
construction, which names no type declared under an operation's
directory. Two things in `crates/topo/src/chord_join.rs` still do, and
they are its whole `names-boolean` pin in
`scripts/gates/engine-leaves.counts`:

- `crate::boolean::Branch` (declared in `boolean/mod.rs`, `pub enum
  Branch`): the field `SectionCtx::branch`, `JoinLane::BoolPlanar`'s
  `branch`, `section_loop`'s parameter and its match on
  `Branch::{Positive, Negative, Only, Component, Line}`, and
  `bool_planar_chord_spec`'s parameter. Eight sites.
- its test module `run_edge_carrier_rows` imports
  `crate::boolean::reduce::planar_lane_carrier_rows::{arc_sheet, spiric_cap}`.

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
leaves the counts file.
