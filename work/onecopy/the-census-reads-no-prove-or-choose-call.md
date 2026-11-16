---
id: the-census-reads-no-prove-or-choose-call
kind: issue
title: The predicate census reads decide* calls only: a name handed to k_stats::prove or k_stats::choose is read by no roster
status: open
opened: 2026-11-10
priority: P3
cost: M
---


Found by the review of #6755, item 2, the `FORWARDS` cross-check.

`k_stats::prove` and `k_stats::choose` are doors like `decide*`: each takes a predicate name first and records under it. `test_utils::source::predicate_census` finds funnel sites by the `decide` spelling only (`funnel_sites`), so a name handed to either is read by no roster. A reference to one does not resolve unless something else decides it.
- `prove(` is called in editor-core, geom, geom-brep (6), geom-core, pncad-py, profile (5) and topo.
- `choose(` is called in geom (4), geom-brep (4), geom-core (6) and topo.
- `k_stats::straddle(` is the same kind of door: it records an escalation under the name it is handed. It is called in geom and geom-brep. geom's roster declares it a `Call` carrier for now (ONECOPY census-tables, #6793, when main's `curved_edges.rs:448` fixture referenced geom's `curve_carrier_straddle`). That holds geom's literal sites only; geom-brep's are read by no roster.

geom-core's roster lists `prove`'s own forwarding site under `PROVE_DOOR`, citing this row.

**The fix:** the census reads `prove(`, `choose(` and `straddle(` as funnel sites, whole-word, alongside `decide*`. Each roster then lists the sites that appear: names read at literal calls, and indirect sites keyed by their `fn`.

**Done** when all three are funnel sites, geom's `straddle` carrier and `PROVE_DOOR` are gone, and a literal handed to `prove` in a test fixture resolves on the coverage row.
