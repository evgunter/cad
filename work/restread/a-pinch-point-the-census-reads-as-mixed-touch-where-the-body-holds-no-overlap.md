---
id: a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap
kind: issue
title: At a three-cone pinch the census refuses a union MixedTouch where the body's volume, cones, keys and mesh match the oracle (196 four_pairs lines, three, tripod)
status: open
opened: 2026-10-12
priority: P2
cost: M
---


## What

Found by FLUSH's touch-key lane (branch `flush/touch-key`). Once a
boolean puts the vertices it decided one point on one key
(`crates/topo/src/boolean/zip.rs` `record_classes`), the pinched
operands of `join_pierce_runs_sweep`'s `four_pairs`, `three` and
`tripod` sets build where they refused `PinchConesOnSeparateKeys`.
Many of their unions with `notch343` then ship `OK BAD`: the oracle's
volume, tier 2, the certificate, one vertex per cone at `v` on one key,
and a clean mesh, while tier 3′ refuses `CensusUndecidable` with
`MixedTouch` ("one passes into the other where they touch") between
the two solids of the result that meet at `v`.

- `four_pairs_battery`: 196 lines `OK BAD`, all with tier 3′ false and
  no key or cone finding (172 that refused `PinchConesOnSeparateKeys`
  on main, and the 24 that shipped `OK BAD` there with their cones on
  two keys). Their reasons were not tallied line by line.
- `three i=4 j=3 k=1` and `tripod i=0 j=4 t=1 k=0`, `ab U` and `ba U`:
  `MixedTouch` alone, pinned by
  `join_pierce_runs_sweep::three_pairs_whose_operands_pinch_the_touch_unions_decided_build`.

## Read at `three i=4 j=3 k=1 ab U`

The result is two solids. At `v` solid 1 holds vertices 11 and 49 and
solid 2 holds vertex 48 (a cube's corner). The census reads the
vertex-vertex site (11, 48) `Crossing` (`census.rs` `TouchSite::verdict`
→ `touch_verdict`), which `blocks` turns into `MixedTouch`:

- `Star::vertex(11)` is `CoConvex`: four faces, all four edges reflex,
  their outward normals pointing into the convex wedge the four edge
  rays span (53 of 4 000 sampled directions). So the star claims every
  other direction as solid 1's material.
- `Star::vertex(48)` is `Convex`, a cube corner whose three rays lie
  outside that wedge. So by the two stars the materials overlap, and
  no plane or complement candidate certifies a rest.

The body says otherwise. Sampling 4 000 directions at 1e-4 from `v`,
the oracle (the notch's and the cubes' convex pieces) and
`topo::point_in_solid_of` agree: the wedge is empty, 1 321 directions
outside it are empty too, 48's cone is solid 2 alone, and solid 1 holds
2 125 directions, none of them in 48's cone. Volume, cone count and
mesh match the oracle. So the per-vertex star at 11 does not describe
solid 1's material at `v`: either `Star::vertex` misreads vertex 11's
cone, or the body's faces at 11 are oriented against its containment
in a way volume and the mesh check cannot see. Which one is not
settled: no edge of solid 1 runs through `v`, and two faces of solid 1
(`FaceKey(3v3)`, `FaceKey(5v3)`) hold `v`'s plane without a vertex
there.

## Replay

`cargo test -p sweep --release --test all
three_pairs_whose_operands_pinch_the_touch_unions_decided_build` (the
pin), or add a probe printing `Star` per site (the lane's probe, not
committed, put a trace in `TouchSite::verdict` and a direction sampler
over the result's vertices at `v`).

## Owed

Decide which reading is wrong. If the star: read a pinch vertex's
material at `v` so a cone's star and the containment door agree; the
196 `four_pairs` lines and the two pinned unions should then pass tier
3′, and the pin's `MixedTouch` arm goes. If the body: file it on the
op's program with this pose.
