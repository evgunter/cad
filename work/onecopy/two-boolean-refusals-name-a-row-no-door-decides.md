---
id: two-boolean-refusals-name-a-row-no-door-decides
kind: issue
title: Two boolean refusals hand-build their result under a row no k_stats door decides: unglued_coincidence's cause and the bisector's Zero
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by the review of ONECOPY's roster-reach (#6715, M5 and M6).

Two production sites in topo build a decision's result by hand, under a row name no `k_stats` door decides:

1. **`crates/topo/src/boolean/mod.rs::unglued_coincidence`** builds an `Indeterminate` under `carrier_unglued_coincidence`, with the margin of the glue reading it is handed. It is raised from `boolean/recl.rs:168` and `vtxfac.rs:2949`. The name is now the const `CARRIER_UNGLUED_COINCIDENCE` at the raise site, and topo's roster declares it a carrier. So the census reads the name, and a reference to it resolves. No door decides it: the margin it reports was decided under the glue reading's own row.
2. **`crates/topo/src/boolean/recl.rs:2232-2238`** builds `Decided { sign: Zero, margin: ±zero }` under `bool_sector_bisector_side`, handed to `sectors::bisector_zero_refusal` in an `OnReading`. The Zero is the one every `On` side code states, and no reading is made. The roster no longer declares `OnReading` a carrier, since a carrier there turns a decision's output into a decided name. So the name is read nowhere, and the unit test that asserts it (`recl.rs`, the graze between two `Out` keys) is marked `UNDECIDED-NAME`, citing this row.

**The question for each:** route it through a door (`k_stats::decide`/`choose` over the margin it has, or a `straddle`-style mint the K recorder logs), or state at the site that the row is a refusal's label rather than a decision's.

**Done** when each site either goes through a named door, or is a declared label the census reads as such, and the marker at `recl.rs` is gone.

## Site 1 retired (SEAL PR 6693, 2026-11-10)

SEAL's `a-corner-reads-only-its-own-sectors` deletes `unglued_coincidence` with C22. An unglued corner now refuses on its own reading through a door: `gate_rejection` (`fan::unglued_parallel`, `boolean::unglued_corner`) or `straddle` (`bool_corner_side_slack_straddle`). The const `CARRIER_UNGLUED_COINCIDENCE` and its roster entry go with it. Site 2, the bisector's `OnReading` in `recl.rs`, remains (`by: TiedBy::Band`).
