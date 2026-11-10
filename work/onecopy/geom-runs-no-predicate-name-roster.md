---
id: geom-runs-no-predicate-name-roster
kind: issue
title: geom decides predicate names and runs no roster: 11 names read, 6 indirect sites unlisted
status: closed
opened: 2026-11-10
priority: P3
cost: E
closed: 2026-11-10
pr: 6715
---


Found by ONECOPY's roster-reach unit. `crates/test-utils/tests/predicate_name_coverage.rs` lists `geom` in `KNOWN_MISSING` with this row.

geom decides predicate names in its `src` (`predicate_census` reads 11), and no `tests/recourse_roster.rs` holds that nothing it decides is invisible. The census reports 6 indirect sites:

- `azimuth.rs: row`
- `chart_point.rs: name`
- `curves.rs: site`
- `curves.rs: sites.off_axis`
- `curves.rs: sites.two_ovals`
- `surfaces.rs: name`

Two names the census does not read are referenced from topo: `spiric_two_ovals` and `pt_spiric_two_ovals`, at `crates/topo/src/boolean/slit.rs:3738` and `:3744`. They are spelled at `crates/geom/src/curves.rs:482` and `:555` as a funnel's argument. Until this row lands, the coverage row holds both through geom's entry.

**Done** when `crates/geom/tests/recourse_roster.rs` runs `test_utils::source::roster_violations`, with its carriers and an `INDIRECT` list naming what carries each site's name, and geom's entry is gone from `KNOWN_MISSING`. The ratchet reds on the entry once the roster runs. topo's two references must then resolve.

**Closed** (2026-11-10, ONECOPY roster-reach, at its review). `crates/geom/tests/recourse_roster.rs` runs `roster_violations` over geom's whole `src`.
- geom's forwarders take the `_as` suffix and are declared carriers: `stable_azimuth_as`, chart_point's `off_as`, `tube_half_as`, and the spindle's `read_as`.
- The two regime-row constructors, `SpiricRegimeSites::new` and `RingSites::new`, are renamed `rows_as`. That makes every reader's spiric and ring rows read, topo's `spiric_two_ovals` and `pt_spiric_two_ovals` included.
- `Ray::ahead` stays listed, since its callers name the row in their own crates.
- geom's entry is gone from `KNOWN_MISSING`.
