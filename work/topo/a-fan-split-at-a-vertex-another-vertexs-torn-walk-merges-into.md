---
id: a-fan-split-at-a-vertex-another-vertexs-torn-walk-merges-into
kind: issue
title: a fan mev at v proves v's walk, but another vertex's rho-shaped torn walk can merge into v's cycle, and the split leaves a minted key in an orbit error
status: open
opened: 2026-10-03
priority: P3
cost: E
refs: [vertex-orbit-reads-no-start-vertex]
---


## What

The residue `vertex-orbit-reads-no-start-vertex` recorded and left
open. `Body::orbit_walk` (`crates/topo/src/body.rs`) now refuses a
walk with a member that starts at another vertex, so every reader of
`Body::vertex_orbit`, `edges_of_vertex` and `faces_of_vertex` gets
`None` for a walk that leaves its vertex. A fan `mev` / `mev_null`
(`Body::mev_fan_plan`, `euler.rs`) reads that walk. It refuses
`FanOrbitBroken` when the walk from `he1` leaves `v`.

That proof covers only the walk that starts at `he1`. A torn `next` at
another vertex `u` can make `u`'s walk rho-shaped: it leaves `u` and
runs into `v`'s cycle, which stays closed and stays at `v`. The split
at `v` is planned on a walk the proof accepts, and it splices new
halves into a cycle that `u`'s torn walk also enters. The result's
`validate` then names a minted key (the new vertex, `he_plus` or
`he_minus`) in an orbit error: `OrbitForeignMember`,
`SplitVertexOrbit` or `VertexOrbitOverrun`.

**Measured at the head of the PR that filed this row.** The probe is
not committed. On `review_d18`'s `FIXTURES`, seeds 1..=3,000, it plants
one and then two `Tear::NextForeign` tears (`plant`, `Rng::from_seed`).
It calls `mev_null(MevSite::Fan { he1, he2 }, Above)` inside a surgery
scope for every ordered pair of distinct half-edges with the same
start:

- 1,260,000 calls;
- 1,103,470 `Ok`;
- 965 `Ok` results whose `validate` names a minted key in one of those
  three errors.

The first measurement, in the row above, was 975 of 1,101,856. It used
a different call set, so the two counts are not directly comparable.

## The shape to give

A plan would have to prove every orbit that ends in `v`'s cycle, not
only `v`'s own. Two candidates, neither tried yet: a whole-arena check that nothing
outside the walk steps into it, or a predecessor check on each member
(`mate(prev(·))` lands back in the walk). Both are O(arena) or
O(valence) reads per split. Weigh that cost against the tier-1
contract before choosing: a torn body is invalid input, and today the
validator reports this one.
