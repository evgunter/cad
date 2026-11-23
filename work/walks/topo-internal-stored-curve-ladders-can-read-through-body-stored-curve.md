---
id: topo-internal-stored-curve-ladders-can-read-through-body-stored-curve
kind: issue
title: topo's own edge-to-certified-curve ladders could read through Body::stored_curve
status: closed
opened: 2026-10-20
closed: 2026-11-22
parent: edge-carrier-walks-and-tag-matches-outside-the-one-door
---


(Filed by the ENCL lane that added `Body::stored_curve`, from its class sweep; see `work/encl/the-stored-curve-read-ladder-is-written-three-times.md`.)

## What

`Body::stored_curve` (`crates/topo/src/body.rs`) is topo's one read of a stored edge's certified carrier and interval: a stale key is the caller's `BadArgument::Stale`, a null edge `NullScaffoldCurve`, and a live edge's dangling curve link panics. These topo sites still spell the ladder themselves ("get the edge, get its curve row, `.certified()`, read `carrier()`/`params()`"):

- `boolean/reduce.rs` (~3076), `boolean/join.rs` (~2461), `boolean/slit.rs` (~413, ~479, ~611), `coherence.rs` (~433), `splitting/section.rs` (~488), `splitting/finish.rs` (~750, ~1415), `splitting/section_loops.rs` (~89), `census.rs` (~2979), `stands.rs` (~319), `seqgen.rs` (~1095, ~1273), `wedge_reach.rs` `WedgeReach::edge` (~140), `boolean/mod.rs` (~7400, ~7686).
- `props.rs` `run_edges` (~3290): `corrupt("curve key does not resolve")`, then `LoopEdgesError::NullScaffoldEdge`.
- The `CurveGeom::Certified(..)` pattern-match spellings: `chart_region.rs` (~2523), `solid_contain.rs` (~2085), `boolean/reduce.rs` (~5281).

## Why it was not routed with the sweep and blend copies

Each of these answers a miss its own way: a dangling curve link as a typed `Corrupt`/`desync`/`lost`/`None` rather than a panic, a null edge as `None`, `continue` or a `HalfCarrier::Chord`. Whether each should take `stored_curve`'s contract (dangling link is a kernel bug, so it panics) is a per-site call for this program; where it should, the site becomes `stored_curve(e)` plus a `map_err`, or `.ok()` for the tolerant readers.

Not in scope here:
- `validate.rs` `certified_carrier` (~9262) keeps its own answer. The validator reports a link miss as a finding, and `stored_curve` panics on one.
- Reads that resolve the edge through `proven`/`Link` (the plan-phase contract, e.g. `attach.rs` `carried_spec`), and test code (`tier3_tests.rs`, `test_support_*.rs`, `boolean/boxes.rs` tests, `boolean/node_probe_rows.rs`, `boolean/carrier_cross.rs` tests).

- 2026-11-21 (TESSLIE) — the chart-coherence examination is retired: `work/tesslie/chart-coherence-ships-off-and-nothing-schedules-turning-it-on.md`. The `coherence.rs` site is gone.

- 2026-11-22 (WALKS, `walks/edge-view`) — closed by the edge view: topo's production ladders read an edge's curve through `EdgeView` (`Body::edge` / `proven_edge` / `linked_edge`, then `curve()` / `certified()`), and a live edge's dangling curve link panics there naming the edge. A key a record holds on a body tier 1 vouches for resolves through `linked_edge`; the census and the validator keep `Body::edge`'s typed edge miss. Left raw on purpose, and listed by the source pin `live::tests::production_reads_an_edges_curve_only_through_the_view`: `props.rs` `run_edges` (the raw mass-properties door refuses a removed edge or curve `Corrupt`), `validate.rs` `certified_carrier`, `closed_by_tier` and the tier-3 local checks, `census/curved.rs`'s tangent read, `transform.rs`'s arena remap, and `euler_kill.rs` `kev_merge_moves`, which already panics naming the edge on a link miss.
