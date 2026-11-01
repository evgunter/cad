---
id: kev-kills-a-far-vertex-whose-fan-it-reads-by-the-walk
kind: issue
title: kev removes its far vertex on the strength of its orbit walk: a half-edge the walk never reached keeps a start naming the dead vertex through Ok
status: closed
pr: 3570
branch: topo/kill-proves-removals
opened: 2026-09-30
refs: [kef-and-mekr-kill-a-loop-whose-members-they-read-by-the-walk, kef-and-kev-take-a-mate-whose-own-edge-is-another]
priority: P3
cost: E
closed: 2026-09-30
---

## What

Found by the receipt of the walk-proofs unit (PR 3511,
`kef-and-mekr-kill-a-loop-whose-members-they-read-by-the-walk`),
whose probe row gained a column for a half-edge naming a dead loop or
vertex.

`Body::kev` (`crates/topo/src/euler_kill.rs`, `kev_plan` ~:926,
shared by `kev_describing` and `kev_merged_members`) kills the far
vertex `w = end(he)` and re-bases its merged fan, which it takes from
`vertex_orbit(m)` (~:967). `require_orbit_starts_at` proves every
member of that walk starts at `w`; nothing proves the converse, that
no half-edge outside the walk and the killed halves starts at `w`. A
torn `next` closes the orbit early, a torn start puts a half-edge of
elsewhere on `w`, and a torn bijection reads a foreign mate; each
leaves a half-edge whose start is the vertex the kill removes. It is
the vertex dual of the dying-loop proof PR 3511 adds to `kef` and
`mekr` (`Body::require_run_of` with `RunExtent::Whole`), and the
removal-side sibling of `kvfs`'s start check.

## Measured

`review_d18::kill_anchors_on_torn_bodies` at PR 3511's head (seeds
1..=2,000, one and two tears, eight bodies, release, debug assertions
off), the `Ok, dangling` column of `kev`: `NextForeign` 851 of
880,000 calls, `EdgeBijection` 836, `StartForeign` 73,573,
`ParentLoopForeign` 0. The probe admits this cell by name
(`FILED_CELLS`) and asserts every other one is 0. First witnesses:
`ops_strut_cube`, one `EdgeBijection` tear, `kev(halves[8])`;
`ops_ring_bridge`, `halves[5].next = halves[37]` and
`halves[11].next = halves[1]`, `kev(halves[0])`. The `EdgeBijection`
part overlaps `kef-and-kev-take-a-mate-whose-own-edge-is-another`.

## The shape to give

`kev_plan` proves that no half-edge but the merged fan and the killed
two starts at `w`, bounded by the arena as the kill anchors' `Lone`
proof is, and refuses `OrbitBroken` naming `m` otherwise. Pin a row
per tear kind, and drop the `FILED_CELLS` entry.
