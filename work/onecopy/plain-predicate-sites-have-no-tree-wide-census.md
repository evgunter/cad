---
id: plain-predicate-sites-have-no-tree-wide-census
kind: issue
title: Plain predicate sites (literals at k_stats doors, and names passed through wrappers) have no tree-wide check that no two readers share a name
status: open
opened: 2026-11-10
priority: P3
---


Filed by the ONECOPY `onecopy/funnel-bundles` lane, at the FULL review of PR 6723 (F4).

## What

PR 6723 put every crate's row bundles in one registry (`geom::rows`, `geom_brep::rows`, `topo::rows`, `sweep::rows`). editor-core's `every_predicate_name_is_minted_once_tree_wide` (`crates/editor-core/tests/every_predicate_name_is_minted_once.rs`) walks their rosters together.

A predicate name minted as a **plain site** is in no roster:
- a string literal at a `k_stats` door;
- a name a wrapper passes through: `geom_core::period::TurnRows::exact` / `::bounded` (`crates/geom-core/src/period.rs`), `topo::RoleNames::one`, `topo::SignNames` (`crates/topo/src/props/settle.rs`), and `half_angle::RootSlack`'s `row` (`crates/geom-brep/src/half_angle.rs`).

Two readers spelling one such name merge their samples in the `k_stats` stream and under one ledger row of `docs/predicate-dimension-audit.md`, and nothing goes red.

The partial answers today are per crate:
- the `predicate_census` rosters: `crates/sweep/tests/recourse_roster.rs`, `crates/profile/tests/recourse_roster.rs`, `crates/topo/tests/recourse_roster.rs`, and geom-brep's `ladder.rs` / `curve_face/rows.rs`;
- `crates/geom-core/tests/flagged_census.rs`, which checks that a door cites a ledger row.

None of them checks one name across two crates' readers. PR 6715 (roster-reach) adds geom and geom-core rosters.

## What closing it takes

Feed the plain sites the census already reads into editor-core's tree-wide test. It is named for the question so they can join it. Then refuse a name two readers spell, unless the ledger says they are one reading. Read the per-crate censuses' output rather than walking the sources a second time.
