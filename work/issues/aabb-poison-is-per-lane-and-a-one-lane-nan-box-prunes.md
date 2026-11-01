---
id: aabb-poison-is-per-lane-and-a-one-lane-nan-box-prunes
kind: issue
title: bvh::Aabb promises a NaN bound poisons the whole box, but from_points folds per lane and overlaps prunes on a finite lane
status: open
opened: 2026-10-10
priority: P1
cost: E
---


## What

`crates/bvh/src/aabb.rs` states the contract three times — the module
docs ("poison (NaN anywhere) yields a poison box, and a poison box can
never prove disjointness — `Aabb::overlaps` answers `true`"), `Aabb`'s
docs ("a NaN bound makes the box *poison* — it overlaps everything")
and `Aabb::from_points`' docs ("Any poisoned coordinate poisons the
whole box"). The code keeps it per LANE only:

- `Aabb::from_points` folds each lane with `pmin`/`pmax`, so a NaN `x`
  makes `min_x`/`max_x` NaN and leaves `y`/`z` finite (`union` the same).
- `Aabb::overlaps` is six raw `<` tests; a NaN lane's tests are all
  `false`, so it neither proves nor blocks disjointness, and a finite
  lane that separates still answers `false`.

Executed (a scratch test in `aabb.rs`, not committed): `from_points`
over `(NaN, 0, 0), (NaN, 1, 1)` gives
`Aabb { min_x: NaN, min_y: 0, min_z: 0, max_x: NaN, max_y: 1, max_z: 1 }`,
and `overlaps` against `[0,1] × [100,101] × [0,1]` answers `false` — the
box PRUNES. Only `Aabb::poison()` (NaN on every lane) is the
never-prunes box the docs describe.

This is the root of PIPE's S350 class (PR 4482): every door that hands
`from_points` a point set carrying poison in one channel gets a box
that prunes on the others. S350 screened the census reach arm and
`geom`'s NURBS box doors screen `any_poison`, but the callers that pass
possibly-poisoned points straight in still inherit the gap, e.g.:

- `crates/topo/src/census.rs` — the `Trees` vertex boxes
  (`Aabb::from_points([p])` over vertex points), arm 1's `reach_boxes`
  (`from_points([lo, hi])` over a face reach) and arm 2's
  `extent_boxes` (`from_points([lo, hi, rlo, rhi])`);
- `crates/topo/src/boolean/boxes.rs` — the vertex box and the
  chord box (`from_points([a, b])`);
- `crates/bvh/src/tree.rs`'s padded leaf boxes;
- `crates/geom/src/curves/boxes.rs`' chord-hull boxes over end points.

Not traced: which of these can actually see a one-lane NaN at rest
(tier 1/3 checks refuse many such points), and the viewer/editor-core
callers.

## Fix shape

Make the docs true in one place: `from_points` (and `union`, `padded`)
answer `Aabb::poison()` when any lane is NaN, or `overlaps` answers
`true` when any bound is NaN. Either repairs every caller at once; the
first also makes the stored box say what it is. A row should pin the
one-lane-NaN case against a disjoint box.

## Home

`work/issues/`: `crates/bvh/src/aabb.rs` is on no program's `paths:`
(`work.py territory --files -`).
