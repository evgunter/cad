---
id: pcurve-cache-fitted-row-endpoint-reads-are-uncounted-in-the-endpoint-census
kind: issue
title: pcurve_cache.rs gained three one-sided .lo()/.hi() reads (PR 4900) with no endpoint-census note, so every_production_endpoint_read_is_counted_and_dispositioned is red on main
status: open
opened: 2026-10-13
priority: P0
cost: E
---


## What

`geom-core`'s `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
fails on `origin/main` `eb96a01d4b`. The tree reads
`("crates/geom-brep/src/pcurve_cache.rs", 5, 2)`, and the roster
(`crates/geom-core/tests/certified_endpoint_census.rs:299`) says
`(2, 2, "")`.

The three new one-sided reads are the ones the fitted-row unit
(PR 4900) added outside the `is_certified()` asks:

- `pcurve_cache.rs:2305-2306`: `params.0.lo().min(params.1.lo())` and
  `params.0.hi().max(params.1.hi())`, from commit `1bb6082e8e`;
- `pcurve_cache.rs:2672`: the containment test
  `d0 <= t0.lo().min(t1.lo()) && t0.hi().max(t1.hi()) <= d1`, from
  commit `08ba749b06`.

The census counts a line, so the five lines are 2305, 2306, 2448,
2455 and 2672. 2448 and 2455 ask. Main's own CI did not run the
census because the change filter runs geom-core's suite only on a diff
that touches geom-core. DECIDE-10's PR 4929 touches geom-core, merged
main, and reproduces it locally without touching `pcurve_cache.rs`'s
production code:

`cargo nextest run -p geom-core --features sym-profile-testing -E 'test(/every_production_endpoint_read_is_counted_and_dispositioned/)'`

## What closes it

Each of the three reads asks `is_certified()` by name, or the roster
line carries the remainder note arguing it safe at the site, as the
census's docs require. That is PCERT's call.
