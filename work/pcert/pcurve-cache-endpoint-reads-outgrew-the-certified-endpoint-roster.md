---
id: pcurve-cache-endpoint-reads-outgrew-the-certified-endpoint-roster
kind: issue
title: pcurve_cache.rs has 5 production endpoint reads where the certified-endpoint roster counts 2, so geom-core's endpoint census is red on main
status: closed
opened: 2026-10-13
priority: P0
cost: E
closed: 2026-10-13
---

Filed by the TANG orchestrator. TANG's PR 4413 lane found it, and checked it
red on main at `95d344176` in a clean worktree.

`geom-core` `certified_endpoint_census::every_production_endpoint_read_is_counted_and_dispositioned`
fails. `crates/geom-brep/src/pcurve_cache.rs` now has 5 production endpoint
reads, but its roster line in
`crates/geom-core/tests/certified_endpoint_census.rs` (the
`("crates/geom-brep/src/pcurve_cache.rs", 2, 2, "")` entry) counts 2.

Two merges reshaped the file, and neither touched the roster:
- PCERT PR 4900 (`fitted-row-reads-carrier-and-chart`, merge `9397996836`);
- NURBS PR 4928 (`rational-inf-floor`, merge `03a0fec33c`).

The fix is a disposition for each of the three new reads, not a recount:
certified, or why not. That is the owner's call, so no fix is guessed here.
Main's push CI does not select this row unless geom-core is seeded, which
is why main reads green.

Closed 2026-10-13: fixed by PR 4953 (`pcert/endpoint-census-fitted-domain-gate`), which put the roster entry at (5, 2) with dispositions; `every_production_endpoint_read_is_counted_and_dispositioned` is green on main at `68b07af9`.
