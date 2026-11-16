---
id: main-red-quad-lanes-endpoint-census-reads-27-against-a-pin-of-25
kind: issue
title: main red (slow set): the endpoint census reads 27 sites in quad_lane.rs against a pin of 25
status: open
opened: 2026-11-16
priority: P0
cost: E
---


`geom-core::all certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
(`crates/geom-core/tests/certified_endpoint_census.rs:1056`, the roster
assertion) is red on main at `253d3bbd58`. It was measured there on a
checkout of main, 2026-11-16, while ONEHOME ran its workspace suite. This
row records the red; it does not fix it.

    cargo nextest run -p geom-core -E 'test(every_production_endpoint_read_is_counted_and_dispositioned)'

The census's roster differs in one entry only:

- measured `("crates/topo/src/props/quad_lane.rs", 27, 27)`
- pinned `("crates/topo/src/props/quad_lane.rs", 25, 25, "")`
  (`certified_endpoint_census.rs:740`)

So the file now holds 27 endpoint read sites, and all 27 sit in a fn that
asks `is_certified()`. The census's own message asks for the pin, and the
file's remainder note if it has one, to follow the reads. It does not ask
for a number to be silenced: each new read is to be checked as one that
asks by name.

**Likely source, not bisected.** The last first-parent change to
`quad_lane.rs` on main is PR 6982 (`boxfinish/rectangle-runs`, merge
`a868d3a2ab`, 2026-11-15). As text, its diff of the file adds 13
`.lo()`/`.hi()` calls and removes 4. Some of the added calls are in test
code, which the census skips. The census is in the slow set, so that PR's
gate did not run it (`work/ciw/tree-wide-source-censuses-in-the-slow-set-run-on-no-pr-that-moves-a-site.md`).

## Done when

The census is green on main. Either the two new sites are confirmed to ask
`is_certified()` and the pin reads `(27, 27)`, or a site that does not ask
is fixed or argued safe in place.
