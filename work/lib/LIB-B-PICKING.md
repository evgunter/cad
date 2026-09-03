---
id: LIB-B-PICKING
kind: unit
title: binding census family B-PICKING
status: review
pr: 1661
opened: 2026-09-03
branch: lib/b-picking
---

Queued mechanical census family (the B-READBACK/B-CHECKS shape): sweep the
family's bindings against the census contract, construct the previously
unconstructible pins where the surface now allows, and re-cut the census
rows honestly. Families share the census/tags/test files, so at most two
run concurrently, staggered.

## Derived scope (stated before any code changed)

`crates/pncad-py/tests/test_binding_census.py` charters `B-PICKING`
in `FAMILIES` and seven `NOT_BOUND` entries cite it, all under one
`gap:` prose ("ray onto a name"): `pick_face`, `PickTarget`,
`PickHit`, `NodePick`, `NodePickError`, `HitTestError`, `Ray`. That
roster IS the family; nothing else in the census cites the id, and the
`NOT_BOUND` docstring's `B-PICKING` bullet names the same seven.

`MeshPick` and `MeshPickError` are NOT in scope: CUR3 recorded them
DECIDED absent from the façade, argued structurally in
`crates/pncad/src/select.rs` and `crates/pncad/tests/all.rs`
(`NOT_CARRIED`). That decision is honored, not relitigated — with one
consequence banked as a finding rather than acted on.
