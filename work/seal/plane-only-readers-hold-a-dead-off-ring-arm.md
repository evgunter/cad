---
id: plane-only-readers-hold-a-dead-off-ring-arm
kind: issue
title: topo: PlaneEqError is CarrierEqError, so plane-only readers hold a dead OffRing arm
status: open
opened: 2026-11-09
priority: P3
cost: E
---


Found by PR 6634's review (SEAL, `carrier-readings-keep-the-margins-they-decided-on`).

## What

`PlaneEqError` is a re-export of `CarrierEqError` (`crates/topo/src/boolean/plane_eq.rs`), and since PR 6634 that error carries `OffRing { torus, refusal }`. Readers that only ever compare two planes therefore hold an arm no plane pair reaches:
- `reduce.rs`'s maximal-faces gate (`oriented_plane_eq` between neighbours), routed to `off_ring_torus`;
- `vtxfac.rs`'s coplanar corner (`carrier_eq_reading` of two `CarrierDesc::Plane`), routed to `off_ring_torus`;
- `merge_faces.rs`'s plane call of `unglued_pair_verdict` (`oriented_plane_eq`), routed to `MergeCoplanarError::DegenerateTorus`.

Each routes the arm the way a real off-ring pair would end, so no wrong answer is reachable. The arm is dead code that the type forces.

## Shape

Give `oriented_plane_eq` its own error type without `OffRing`, or a narrowing from `CarrierEqError`. Step 4 (`a-corner-reads-only-its-own-sectors`) removes vtxfac's ladder call either way.
