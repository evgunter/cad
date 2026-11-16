---
id: the-census-reads-no-closed-type-name-table
kind: issue
title: The predicate census reads no closed type's name table: profile's Gate macro, CcDecision, Row::name and meter.predicate are invisible to it
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by ONECOPY's roster-reach unit, at the tree-wide reference check (`crates/test-utils/tests/predicate_name_coverage.rs`). Its `KNOWN_MISSING` ratchet carries a `profile` entry citing this row.

`test_utils::source::predicate_census` reads a name at a `decide*` call or out of a declared carrier. It does not read a name spelled in a closed type's own table. The tables are:
- `crates/profile/src/gate.rs`'s gate macro: `CollinearOverlap => "collinear_overlap", …`. Every profile decision is a `Gate`, so profile's census reads no name at all.
- geom-brep's `pair_section/cylinders.rs` `CcDecision::name`/`floor`, and `pair_section/ruling.rs`'s row `name()`/`straddle()`. These are its roster's `CLOSED_TYPE` entries: `pair_section.rs: decision.name()`, `pair_section/cylinders.rs: CcDecision::Radius.name()`, `[decision.name(), decision.floor()]`, `decision.name()`, `pair_section/ruling.rs: row.name()`, `row.straddle()`.
- geom-brep's `offset_meters.rs: meter.predicate()`.
- topo's `CLOSED_TYPE` sites: `boolean/contact_verify.rs: row.predicate()`, `clearance.rs: bound.predicate()`, `self.as_bound().predicate()`.

sweep's blend decisions (`blend/mod.rs`: `Self::RingClearance => "fillet3_ring_clearance"`, …) are the same shape. So a reference to one of these names cannot resolve against the census union. profile, sweep and geom-brep each carry a `ClosedTable` entry in `KNOWN_MISSING` citing this row, and each holds its table's arms. The references held today are `collinear_overlap` (profile), the `fillet3_*` rows (sweep's suites) and `cs_coaxial`, `cs_wall_reach`, `cc_coaxial` and `carrier_torus_minor_radius` (geom-brep's suites). `collinear_overlap`, referenced at `crates/profile/src/seg.rs` in two unit tests, is the instance the coverage row meets today, and profile's entry holds it.

**The shape to read**: `Variant => "lit"` arms inside a `fn name`/`fn predicate`/`fn straddle`/`fn floor` body returning `&'static str`, plus profile's gate macro, whose rows put the name first after `=>`. Reading them makes each roster's `CLOSED_TYPE` entries read sites rather than listed ones, and moves what topo's and geom-brep's rosters see. That is why it is its own row and not part of roster-reach.

**Done** when the census reads those tables, the rosters' `CLOSED_TYPE` entries are gone, and the three entries are gone from `KNOWN_MISSING`.

**Two additions** (2026-11-10, ONECOPY roster-rest, at #6736's review):

- **editor-core's own `decisions!` table** (`crates/editor-core/src/decision.rs:91-102`) is a closed table of the same kind. It is decided through `decision.name()`, for example at `crates/editor-core/src/eval/wire.rs:4568` (`turns_off`'s `decide_flagged(decision.name(), …)`). Most of its rows are consts the roster reads (`EVAL_DIRECTION_NORM`, `ON_MEMBER_EDGE`, `CHORD_ON_RIM`, `SEL_PLANE_DISTANCE`), but `RevolveFullTurn => "revolve_full_vs_partial"` is spelled only in the table, so no census reads it and no hold covers it. editor-core's roster lists the site as `CLOSED_TYPE` and claims completeness only up to this row.
- **The hold's known over-hold.** `source::held_spellings` takes every `Variant => "lit"` arm inside a `fn name/predicate/straddle/floor/row` body and inside any braced macro invocation. `fn name` in particular is also how a type spells a human label, so labels and prose are held as if they were decided names. Four such sites:
  - `crates/sweep/src/blend/arms.rs:247`: blend-arm shape labels ("plane–plane → cylinder", …);
  - `crates/geom-brep/src/pcurve_cache.rs:1904`: pcurve kind labels ("Harmonic", "IsoLine", …);
  - `crates/geom-brep/src/ssi/certify.rs:323`: limb prose ("limb 1 (on-locus residual)", …);
  - `crates/geom-brep/src/ssi/exhaust.rs:383`: floor prose ("seeding floor", …).

  A held label can only make a reference resolve that should not. A fixture naming "Harmonic" as a predicate would be green, which is the permissive direction. Reading the tables by what they are, not by the fn's name, fixes it with the rest of this row.

