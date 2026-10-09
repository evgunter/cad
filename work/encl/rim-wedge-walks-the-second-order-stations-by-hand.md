---
id: rim-wedge-walks-the-second-order-stations-by-hand
kind: issue
title: boolean::rim_wedge::classify_shared_rim hand-rolls the material arm's per-station body and the second-order walk that check 4 reads through geom_brep::second_order_walk, differing only in schedule and transport
status: closed
closed: 2026-10-09
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

## Closed

2026-10-09. PR 4433 merged at `0bdb2baeae` after a full review (verdict: merge, with notes only) and a small fix pass; hosted CI was green.
- **The walker takes its stations from the caller.** `geom_brep::second_order_walk` now takes `stations: impl IntoIterator<Item = (Point3, Vec3)>`. The edge callers pass `interior_stations`; the rim passes its uniform-phase `0..CERT_SAMPLES`.
- **`validate::MaterialStations` (`pub(crate)`) is the one home of the material reads.** Tier 3 uses it and pushes `SliverDihedral` at its one site. `classify_shared_rim` uses it and leaves through `?`. The rim's hand-rolled loop and its hand-minted cusp-side Zero arm are gone.
- **No answer or text moved.** The rim decision trace is byte-identical at 1e-9, 1e-6 and 1e-12 over every rim-reaching row, and the tier-3 k-stream is identical.
- **Pin:** `tier3_tests::check_4_and_the_rim_route_their_second_order_reading_through_the_one_walk` scans the production code of `validate.rs` and `rim_wedge.rs`.
- **Found in review, pre-existing:** topo P2 `mfkrh-leaves-an-orphan-surface-under-the-per-op-scalpel`.
