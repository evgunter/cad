---
id: levered-same-direction-parallelism-is-spelled-twice
kind: issue
title: One levered same-direction parallelism check is spelled twice: sectors.rs parallel_same and recl.rs parallel_same_dir
status: dispatched
priority: P1
cost: M
opened: 2026-10-28
---

Found by PR 5949's review (the crescent short-arc lane).

## What

Two functions read whether two directions are parallel and point the
same way, each a cross-norm levered at an arm decided Zero, then the
cosine's sense (`sectors::direction_sense`):

- `boolean/sectors.rs` `parallel_same`: `u`, `v` as given, predicate
  `bool_dir_parallel`, its escalation `Coincide::Sectors`;
- `boolean/recl.rs` `parallel_same_dir`: `u`, `v` normalized first,
  predicate `bool_ee_collinear`, its escalation `Coincide::EdgeOnEdge`.

They differ in whether the inputs are normalized (`parallel_same` takes
them "unit-ish", so a caller passing a non-unit direction moves its
margin) and in the coincidence the in-band reading names. One check,
one spelling: the subject a refusal names can be the caller's argument.

## Swept

- `boolean/insert.rs`: no third copy. Its nearest are `bool_germ_line`
  (the two faces' normals' cross, parallelism only: `pair_search`'s
  margin, read again) and `tied_held`'s bare `direction_sense`.
- Outside the pair, the same two-step reading of NORMALS:
  `boolean/plane_eq.rs` (`bool_plane_parallel` then `bool_plane_orient`)
  and `boolean/carrier_eq.rs` (axes' cross-norm). Those read carriers,
  not bounds, and are listed here as the shape's neighbours, not its
  copies.

