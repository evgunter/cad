---
id: levered-same-direction-parallelism-is-spelled-twice
kind: issue
title: One levered same-direction parallelism check is spelled twice: sectors.rs parallel_same and recl.rs parallel_same_dir
status: closed
priority: P1
cost: M
opened: 2026-10-28
closed: 2026-10-29
pr: 5983
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


## Closed (2026-10-29, PR 5983)

There is one spelling, `sectors.rs` `parallel_same`. It normalizes both directions, levers the sine at the arm, decides it Zero, then reads `direction_sense`. Each caller passes its `Parallelism { predicate, coincide }`, and the consts live in `refusal_routes.rs`, which `decision_words` also reads. The site census credits the real raise sites, and `the_census_names_every_parallelism_reading` holds that true.

No verdict moves up to tolerance, because a corner's chords lie in its face's tangent plane. `fate.rs` `tip_wraps` is a neighbour, not a copy.

FULL single review: APPROVE. A fix pass made the pair's home single and the census honest.
