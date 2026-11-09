---
id: topo-name-forwarders-hide-their-callers-names
kind: issue
title: topo's generic name forwarders hide their callers' predicate names from the census
status: open
opened: 2026-11-09
priority: P3
cost: M
---


Found closing `topo-has-no-predicate-name-roster` (step 2 of that row, as the orchestrator split it).

`crates/topo/tests/recourse_roster.rs` lists every site whose name the census cannot read at the call (`INDIRECT`). The `FORWARDS` entries there are funnels that hand on a name parameter. Where the funnel has a distinctive name, the roster declares it as a `NameCarrier::Call` and its callers' literals are read (`magnitude_door`, `enclosure_reading`, `gap_zero_reading`, `relation_step`, …). The rest cannot be declared, because a `Call` carrier is crate-wide and these tokens are common words in topo's src. Declaring one sweeps in every unrelated call of that word: `link.rs`'s `decided` (111 calls crate-wide), `read` (611), `zero` (922), `sign` (114), `on` (219), `shift` (35), `escalated` (95), `why`. The names those callers pass are invisible to the roster, so a name drifting at one of them is caught only by the runtime K-telemetry census.

**Preference: rename the generic forwarders** to spellings that say they forward a predicate name, so each is declared as an ordinary `Call` carrier. Fall back to a file-scoped carrier in the reader (`NameCarrier::CallIn(file, token)` in `test_utils::source`) only where a rename is impossible. That reader extension would be shared with sweep's and profile's rosters, which use the same `NameCarrier` set.

The same row covers the rest of what the roster's `INDIRECT` table names as unread:
- carrier_eq's `reading` pairs (`("carrier_sphere_reach", "carrier_sphere_reach_straddle")` and its siblings), handed to `decide_reported(row, …)` as a tuple field (`boolean/carrier_eq.rs: row`);
- `geom_core::k_stats::straddle(name, …)`, which reports under a name the census does not scan for (`census.rs`'s `enclosure_reading`);
- `census/in_plane.rs`'s `REGION_ON_PLANE`, a name const no decide site the census reads hands on.

Closing it shrinks `INDIRECT`'s `FORWARDS` entries to the forwarders whose callers are all declared.
