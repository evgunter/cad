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

geom-core's roster lists `prove`'s own forwarding site under `PROVE_DOOR`, citing this row.

**The fix:** the census reads `prove(` and `choose(` as funnel sites, whole-word, alongside `decide*`. Each roster then lists the sites that appear: names read at literal calls, and indirect sites keyed by their `fn`.

**Done** when both are funnel sites, `PROVE_DOOR` is gone, and a literal handed to `prove` in a test fixture resolves on the coverage row.
