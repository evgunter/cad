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
