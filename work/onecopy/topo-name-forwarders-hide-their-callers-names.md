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

**A closed type's `predicate()` is behind a method, out of the census's reach.** Since main's tangent-row change, `boolean/contact_verify.rs` decides its off-surface rows through `TangentRow::Off(side).predicate()`. The names `contact_tangent_off_1` and `contact_tangent_off_2` are spelled in that method's match, which the census does not read; the roster lists the site as `boolean/contact_verify.rs: row.predicate(` (`CLOSED_TYPE`). `clearance.rs`'s `bound.predicate()` and `self.as_bound()` are the same shape. A carrier kind that reads a declared function's literals (`NameCarrier::Fn("predicate")`), or a closed type's `ALL` iterated by the roster as sweep's does, would close these.

**INDIRECT keys a forwarded site by its expression, so one entry stands for every future site in the file.** A site whose name is a forwarded parameter is keyed `<file>: <expression>` (`link.rs: name`). The census's `indirect` is a set, so every forwarding site in that file that spells the same expression is one entry. A new forwarder under a common word in a file that already has a `name` entry does not red the roster, and its callers' names go unread. `read_identity` sites are keyed by their ordinal in the file for this reason. Doing the same for forwarded `decide*` sites (`<file>: name#3`) would make each site its own entry, at the cost of renumbering when one is added above.
