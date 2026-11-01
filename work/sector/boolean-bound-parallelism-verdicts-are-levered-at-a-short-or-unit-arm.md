---
id: boolean-bound-parallelism-verdicts-are-levered-at-a-short-or-unit-arm
kind: issue
title: The boolean's bound-parallelism verdicts (parallel_same, levered at the shorter of two sectors' arms) read a Zero as 'same ray' for long bounds that part by many bands
status: open
opened: 2026-09-29
priority: P3
cost: M
---


Filed by CONTACT-9's class sweep. Unverified: no fixture has reached
it.

Two verdicts in the boolean's vertex lanes still read a direction
levered at a length that is not the bounds' own:
- `sectors.rs` `parallel_same` (`bool_dir_parallel`, `bool_dir_same`)
  levers two coplanar sectors' bounds at the shorter arm of the pair.
  Its Zero means "these bounds are one ray", and `sector_overlap` then
  reads the identical-region case.
- `recl.rs` `parallel_same_dir` (`bool_ee_collinear`) levers two On
  mentions at `T::one()`. Its Zero groups them into one ray event, so
  the edge-edge engine runs where two edge-sector events were due.

Two 10 m bounds 1e-9 rad apart part by 10 bands at their far ends, and
both readings call them one ray.

Both are downstream of side codes that CONTACT-9 now reads at the far
vertex. A pair reaching `sector_overlap` has all four bounds On in
metres. A mention reaching `parallel_same_dir` is a metric On. So the
two bounds already lie within the band of each other's planes, which
narrows the pose but does not close it. The metric reading here is the
far vertices' distance from each other's line, or each far vertex's
distance from the other bound's ray.

## 2026-10-29 — one check now, both readings at the shorter arm (SPLITDOOR)

`recl.rs` `parallel_same_dir` is gone: PR 5983 merged it into
`sectors.rs` `parallel_same`, which both readings now call with their
own `Parallelism` (`refusal_routes.rs`): `BOUNDS_PARALLEL`
(`bool_dir_parallel`, `Coincide::Sectors`) and `RAY_COLLINEAR`
(`bool_ee_collinear`, `Coincide::EdgeOnEdge`), both normalized before
the lever. The second bullet's "at `T::one()`" premise was already
stale before that merge: `recl_edges` levers the ray-event grouping at
`mentions[i].arm.min(mentions[j].arm)`, the shorter mentioning sector's
arm. So both readings are now the first bullet's case, a Zero at the
shorter arm where each bound's own reach is known. The pose above
stands for both.
