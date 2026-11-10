---
id: funnel-row-bundles-have-no-cross-reader-roster
kind: issue
title: The funnel's other row bundles (SubdivisionRows, QuarticRows, HalfAngleRows, ...) are built field by field by each reader, and nothing checks their names across readers
status: closed
opened: 2026-11-09
priority: P3
closed: 2026-11-10
branch: onecopy/funnel-bundles
pr: 6723
---


Filed by the ONECOPY `onecopy/torus-ring-names` lane as option C of
`ring-sites-names-are-not-unique-across-readers`. That row closed the
ring-convention and spiric-regime bundles: `geom::RingSites` and
`geom::SpiricRegimeSites` are now minted only by their reader enums
(`geom::RingReader`, `geom::SpiricReader`). One roster of every reader's
rows (`geom`'s `reader_rows`) is checked distinct when `geom` compiles.

## What

The same hazard stands for every other bundle of funnel rows a reader
passes to a shared predicate. Two readers naming one row merge their
samples in the `k_stats` stream and under one ledger row of
`docs/predicate-dimension-audit.md`, and only a reading of the ledger
would show it. These bundles are structs with public `&'static str`
fields, built literally by each reader:

- `geom_brep::half_angle::SubdivisionRows` (`crates/geom-brep/src/half_angle.rs:328`), about 9 sites spelling it;
- `geom_brep::quartic::QuarticRows` (`crates/geom-brep/src/quartic.rs:233`), about 3;
- `geom_brep::half_angle::HalfAngleRows` (`half_angle.rs:312`), about 2;
- `geom_brep::half_angle::TouchRows` (`half_angle.rs:348`), 1;
- `geom_brep::curve_face::circle_roots::FirstHarmonicRows` (`circle_roots.rs:222`), 1;
- `geom_core::period::TurnRows` (`crates/geom-core/src/period.rs:678`).

**The partial answer that exists today** is the `predicate_census`
rosters (`test_utils::source::predicate_census`). Each lists the
predicate names one crate decides and why:
- `crates/sweep/tests/recourse_roster.rs`;
- `crates/profile/tests/recourse_roster.rs`;
- `crates/topo/tests/recourse_roster.rs`;
- `geom-core`'s `ladder.rs` (moved from geom-brep) and geom-brep's `curve_face/rows.rs`.

Each is per crate, and none checks one name across two crates' readers. (This row once said topo had no such roster; it has one now, and `crates/geom-core/src/ladder.rs:744-748` names sweep's and topo's rosters as the ones that list `read_identity`'s calls.)

Beyond bundles, a plain `&'static str` site passed to a `k_stats` door can
be spelled by two readers alike. `crates/geom-core/tests/flagged_census.rs`
checks that every shipped site cites a ledger row. It does not check that
two readers do not cite one row meaning two readings.

## What closing it takes

Pick one:

- give each bundle the reader-enum shape `RingSites` now has, its rows
  joining one roster checked distinct;
- or one funnel-wide check, a census like `flagged_census`, that refuses
  a row literal spelled at two readers unless the ledger says they are
  one reading.

The second covers plain sites too, but needs the ledger to say which rows
are shared on purpose.

## Closed

PR 6723. Every crate that mints row bundles holds them in one registry: `geom::rows`, `geom_brep::rows`, `topo::rows`, `sweep::rows`.

- The bundle types' fields and constructors are private to the registry, so no reader builds one field by field.
- Every reader's bundle is a `const` given to `geom_core::row_registry!`, which derives the crate's `ROSTER` from exactly those consts and fails the build on a shared name.
- geom's roster is read off its reader enums.
- editor-core's `every_predicate_name_is_minted_once_tree_wide` walks all four rosters together.

topo's `ConicRows` (a `TurnRows` with two names of its reader's own) and check 7's `RoleNames` are registered bundles. Still unregistered, as wrappers of a name minted elsewhere:
- `geom_core::period::TurnRows::exact` / `::bounded` called with a passed-in name;
- `RoleNames::one`;
- `SignNames`;
- `half_angle::RootSlack`'s `row`.

Their uniqueness, and that of literals at `k_stats` doors, is `plain-predicate-sites-have-no-tree-wide-census`.
