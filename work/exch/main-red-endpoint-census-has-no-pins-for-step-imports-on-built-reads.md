---
id: main-red-endpoint-census-has-no-pins-for-step-imports-on-built-reads
kind: issue
title: Main red (slow set): the certified-endpoint census has no pins for step-import's on_built.rs (24 reads) and recognize_curve.rs (5 reads)
status: open
opened: 2026-11-22
priority: P2
cost: E
---


Found by SECTCONE's PR 7350 lane, 2026-11-22.

`crates/geom-core/tests/certified_endpoint_census.rs` `every_production_endpoint_read_is_counted_and_dispositioned` reads `crates/step-import/src/on_built.rs` at (24, 0) and `crates/step-import/src/recognize_curve.rs` at (5, 0), and has no pinned row for either file. It is red on main's nightly (run 44989234943, eps 1e-12 3of3). The new reads came with EXCH's printed-on-built certificate commits (`651aed96ea`, `92f8deba09`, `1259b698be`).

**What closes it:** each read asks `is_certified()` by name or is argued safe at its site, and the census pins both files with their remainder notes.
