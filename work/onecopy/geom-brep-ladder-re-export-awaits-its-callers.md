---
id: geom-brep-ladder-re-export-awaits-its-callers
kind: issue
title: geom_brep::ladder is a transitional re-export of geom_core::ladder: two paths to one item until its callers move
status: open
opened: 2026-11-09
priority: P3
cost: C
---


Filed by QML tranche 3 (`onecopy/refusal-standing`), on the
orchestrator's ruling A of 2026-11-09.

## What

The ladder moved from `geom_brep::ladder` to `geom_core::ladder`, so
that a crate below geom-brep (profile's `seg::arc_arc`) can climb.
`ladder.rs` imports only geom-core, and no ratified text places it. To
keep in-flight lanes' diffs from conflicting, `geom_brep::ladder` stays
as `pub use geom_core::ladder::*`, marked `#[doc(hidden)]` and
documented as transitional (`crates/geom-brep/src/lib.rs`). That is two
paths to one item: a small copy of a name.

New code imports `geom_core::ladder` directly.

## Done when

The ladder PRs in flight at the move have merged. Then every caller of
`geom_brep::ladder` is ported to `geom_core::ladder`, the re-export is
deleted, and `rung-holds-one-home.sh`'s `LADDER` pattern drops its
`geom_brep::` alternative.
