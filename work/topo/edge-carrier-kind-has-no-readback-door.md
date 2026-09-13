---
id: edge-carrier-kind-has-no-readback-door
kind: issue
title: query::edge_carrier_kind has no readback twin, so the two-homes ruling does not reach the edge side
status: open
opened: 2026-09-06
---


## What

Ev's ruling on `face-kind-read-has-two-homes` (PR 1948): the typed
readback door is the one reading of a stored tag and the predicate seat
flattens it. `query::edge_carrier_kind` (`crates/topo/src/query.rs:296`)
reads an edge's certified carrier kind with no typed twin in
`readback.rs`, so on the edge side the query seat IS the only reading
and there is nothing to flatten. Either the shape is fine (one reading,
it just lives in `query`) or `readback` owes `edge_carrier_kind ->
Result<CurveKind, ReadbackError>` for the read-back consumers that
want to know which lookup missed, with the query seat then flattening
it as the face side does. Not a defect today; a question about where
the one reading lives. Reported by the two-homes lane; filed by the
TOPO orchestrator, 2026-09-06.
