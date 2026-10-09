---
id: rim-wedge-walks-the-second-order-stations-by-hand
kind: issue
title: boolean::rim_wedge::classify_shared_rim hand-rolls the material arm's per-station body and the second-order walk that check 4 reads through geom_brep::second_order_walk, differing only in schedule and transport
status: review
branch: encl/rim-wedge-one-walk
pr: 4433
priority: P3
cost: M
opened: 2026-10-09
---


## Finding (review of PR 4423)

`topo::boolean::rim_wedge::classify_shared_rim`
(`crates/topo/src/boolean/rim_wedge.rs`, ~`:968`) runs, at each rim
station, the body that tier 3's check 4 now runs through
`geom_brep::second_order_walk` with its `MaterialStations` hook
(`crates/topo/src/validate.rs`): `classify_material_pairing` at the
folded arm, then `geom_brep::tangent_second_order`'s verdict, where the
first station not `Positive` decides, then `material_cusp_side` on a
`Positive` one, folded by `material_arm_outcome`. It differs in two
ways only:

- **schedule** — a closed rim has no endpoints to exclude, so it reads
  all `CERT_SAMPLES` stations at uniform phase (`0..n`, its own
  `station` closure), where the walker reads `geom_brep::interior_stations`;
- **transport** — escalations leave through `?` as an `Indeterminate`,
  where tier 3 pushes `SliverDihedral` — the shape the walker's
  `StationHook::Break = Indeterminate` already carries.

So the second-order walk is still spelled twice, kept in step by
prose (`material_arm_outcome`'s "two callers … two different sample
schedules").

## Repair shape

Let `second_order_walk` take its stations as an iterator of `(p, τ)`
(the edge callers passing `interior_stations`, the rim its uniform
phase), and give `MaterialStations` one home both callers use — its
`Break` is already the rim's `?` payload. Neither answer may move: the
rim rows in `rim_wedge.rs`'s `redfirst` module and the corpus
k-stream are the evidence.
