---
id: geom-runs-no-predicate-name-roster
kind: issue
title: geom decides predicate names and runs no roster: 11 names read, 6 indirect sites unlisted
status: open
opened: 2026-11-10
priority: P3
cost: E
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
