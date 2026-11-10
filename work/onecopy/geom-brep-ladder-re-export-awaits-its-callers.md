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

Inside geom-brep the ladder is spelled `crate::ladder` until then, one
spelling through the re-export (the review of PR 6701).

## Done when

Nothing outside geom-brep's library spells `geom_brep::ladder`: every
spelling is ported to `geom_core::ladder` (156 at filing, counted raw,
code, test modules, `tests/`, doc links and prose alike: `crates/topo`
153, `crates/sweep` 2, geom-brep's own integration tests 1).
`scripts/gates/ladder-re-export-callers.sh` holds that count per crate
and lets it only fall: it reds a new spelling, and refuses a module
alias (`use geom_brep::ladder;`, `as`, or `ladder` in a group) that would
let a file drop its count without porting. At none, geom-brep's own
`crate::ladder` spellings move to `geom_core::ladder`; the re-export in
`crates/geom-brep/src/lib.rs`, that gate and its register are deleted;
and `rung-holds-one-home.sh`'s `LADDER` pattern drops its `geom_brep::`
alternative.

**The open rows that cite an item by the old path** move with it (the
gate reads `crates/` only, so these are listed here):
- `work/pcert/gate-loops-return-on-an-in-band-gate-before-a-definite-one.md`;
- `work/ciw/probe-gated-lib-unit-tests-never-run-in-ci.md`;
- `work/germ/in-sector-escalates-on-an-in-band-bound-before-a-definite-one-reads-outside.md`;
- `work/onecopy/a-solid-touching-itself-at-a-spline-crease-corner-finishes-undetected.md`;
- `work/onecopy/side-at-infinity-aimed-rays-disagree.md`;
- `work/onecopy/door-searches-left-on-the-hand-rolled-ladder.md`;
- `work/splitdoor/census-interiors-fold-a-meetings-fault-into-unbuilt.md`.

`a-walk-refusals-standing-lives-on-the-ladder-not-its-type` names the
old path only in its record of what was moved.
