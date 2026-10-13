---
id: pcurve-cache-gained-three-endpoint-reads-the-census-does-not-count
kind: issue
title: pcurve_cache.rs reads five certification endpoints where the census counts two, so main's endpoint census is red
status: closed
opened: 2026-10-13
priority: P0
cost: E
closed: 2026-10-13
---


Found by the FLUX lanes (`flux/iso-parse-incidence`, PR 4932), whose
`test` job went red on the merge ref with a failure their diff does
not touch. It reproduces on `origin/main` alone (`95d344176`):

```
cargo nextest run -p geom-core -E 'test(every_production_endpoint_read_is_counted_and_dispositioned)'
left:  ("crates/geom-brep/src/pcurve_cache.rs", 5, 2)
right: ("crates/geom-brep/src/pcurve_cache.rs", 2, 2)
```

`crates/geom-brep/src/pcurve_cache.rs` now reads five certification
endpoints where `crates/geom-core/tests/certified_endpoint_census.rs`
counts two. The likeliest source is PR 4900
(`pcert/fitted-row-reads-carrier-and-chart`), merged just before the
census went red. The census's own message asks for each new read to be
counted and dispositioned, not for the number to be moved: a
one-sided read of a bracket that may not certify takes the certifying
branch.

Closed 2026-10-13: fixed by PR 4953 (`pcert/endpoint-census-fitted-domain-gate`), which put the roster entry at (5, 2) with dispositions; `every_production_endpoint_read_is_counted_and_dispositioned` is green on main at `68b07af9`.
