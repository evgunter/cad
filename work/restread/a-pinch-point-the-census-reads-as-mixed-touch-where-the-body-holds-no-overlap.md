---
id: a-pinch-point-the-census-reads-as-mixed-touch-where-the-body-holds-no-overlap
kind: issue
title: At a three-cone pinch the census refuses a union MixedTouch where the body's volume, cones, keys and mesh match the oracle (214 four_pairs poses, three, tripod)
status: closed
opened: 2026-10-12
priority: P1
cost: H
closed: 2026-10-19
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
  `join_pierce_runs_sweep::three_pairs_at_one_vertex_build_on_one_key_and_their_unions_fail_mixed_touch`.

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
three_pairs_at_one_vertex_build_on_one_key_and_their_unions_fail_mixed_touch` (the
pin), or add a probe printing `Star` per site (the lane's probe, not
committed, put a trace in `TouchSite::verdict` and a direction sampler
over the result's vertices at `v`).

## Owed

Decide which reading is wrong. If the star: read a pinch vertex's
material at `v` so a cone's star and the containment door agree; the
196 `four_pairs` lines and the two pinned unions should then pass tier
3′, and the pin's `MixedTouch` arm goes. If the body: file it on the
op's program with this pose.

## Tally (PR 4823's review R2, 2026-10-13) and priority

R2 re-ran `four_pairs_battery` on PR 4823's head. All 196 `OK BAD`
lines fail tier 3′ on one finding, `CensusUndecidable` with
`MixedTouch`, and 18 more poses the battery's volume filter skips fail
the same way: 214 in all. In every one the volume oracle matches and
the cone and key oracle is clean.

Raised to **P1**: since PR 4823 these are public-path unions
(`topo::union` of an operand an earlier union pinched) that ship as
at-rest bodies tier 3′ refuses, where main refused them typed
(`PinchConesOnSeparateKeys`) or shipped 24 of them `OK BAD` with the
cones on two keys. PR 4823 accepts that cost: the typed refusal it
retires read keys an earlier op had left apart, so it refused 287
unions the key rule now builds (115 `SOUND`), and this row is where
the census's half is owed.

## 2026-10-18 — measured, weighed, decided (RESTREAD orchestrator)

**Measured** (pinch lane, probe on `three i=4 j=3 k=1 ab U`). At `v`, solid 1 has
two cycles: 11v1 (`CoConvex`, a convex sliver hole) and 49v1 (a saddle
bounding a non-convex pocket). Solid 2's cube corner 48v3 lies in the pocket.
(11, 48) reads `Crossing` → `MixedTouch`; (49, 48) reads `Unanalysed` (never
seen, `blocks` returns first). The star at 11 is right and so is the body; the
fault is the unit. `TouchSite::stars` takes one vertex's cycle as its solid's
whole material, which holds only where the solid has one cycle at the point —
and since PR 4057 a pinch is one vertex per cone. Solid 1's material exceeds a
hemisphere at `v`, so no plane certificate can separate, and the pocket is
non-convex, so the complement certificate fails too.

**Weighed** by two designers (concurrent, same problem, two rounds). They
converged: one designer moved to the other's final state, the other kept it
with a correction. Not a fork for Ev: no ratified clause changes (README C2
gains one sentence; the census's doors rule and `touch_verdict`'s completeness
claim are agent text, CONTACT-7, commit 2b080332db).

**Decision.**
- The census reads a touch once per (solid A, solid B, point key). Every
  finding at the point (vertex–vertex, vertex on edge, vertex on face) shares
  that verdict, and `blocks` reads it once.
- Each side is its solid's **whole link** there: every vertex of the solid on
  the key, plus the edges and faces its own records tie to the point, gathered
  structurally. That also closes
  `census-reads-only-the-recorded-vertex-pair-at-a-shared-point`, which rides
  this row.
- One test, complete for two polygonal cones, decides the two links:
  1. **Crossing** if a face of one crosses a face of the other along a ray, or
     coplanar sectors facing the same way overlap in their plane (C1's aligned
     case; read the in-plane sector overlap, not coplanarity alone);
  2. **Crossing** if, for any face sector of either link cut at every ray of
     the other link lying on it, a direction just off that piece on the
     material side reads inside the other link's material — by arc
     crossing-parity, no convexity assumed (one direction per *piece*, not per
     cycle);
  3. **Rest** otherwise. Undecided readings refuse by name (`TouchInBand` for
     an in-band parity exit), never Rest or Crossing.
- The plane and complement certificates become special cases and retire, and
  with them `Shape` and `TouchUnanalysed`'s "neither convex nor concave" text.
- **One reader, through the census's door.** The direction-side question
  ("is this direction on this polygon cone's material side") gets one reader
  in `topo`, deciding in metres through `metric::Distance` (the census's
  doors rule, `link::tests::the_link_decides_only_through_its_doors`), over a cone
  given as faces with real points on them. `boolean::sectors::cone_side`
  (levered deviations over `BoolSector`) is not lifted as it stands: the
  boolean calls the new reader if its side can move in the unit, and otherwise
  `cone_side` is filed as the duplicate to retire.
- The 214 unions should read Rest (likely, unmeasured). Re-baseline the pin
  and this row's "Owed" on measurement.

Absorbed: INSIDE's `a-touch-at-a-saddle-corner-refuses-unanalysed` and
`a-solid-touching-itself-at-a-vertex-reads-its-star-from-the-vertex-alone`
are the same missing reading; they move here and ride this row. Class H; dual
review, concurrent.

## Closed

2026-10-19, RESTREAD's link-reading lane (branch `restread/link-touch-reading`).
The census reads a touch once per solid pair and touch point
(`census::TouchUnit`), each side its solid's whole link there
(`census::Link::at_point`: every vertex of the solid on the point's keys,
closed under vertex–vertex ties, and the edges and faces its records and
findings tie there), and decides the two links by one test
(`census::rest::judge`): a piece of one crossing a face of the other along
a ray, or a coplanar same-facing overlap, or a piece's probe inside the
other's material by arc crossing-parity (`Link::reads`), is a crossing;
otherwise a rest; anything undecided refuses by name. The plane and
complement certificates, `Shape` and the saddle text retire.

Measured on main `aea60c929` and on the branch head:
- `four_pairs_battery`: main 196 `OK BAD`, 4012 `OK SOUND`, 982
  `ERR SharedVertexCrossings`, 143 `SKIP`; head 0 `OK BAD`, 4208 `OK SOUND`,
  982, 143. Exactly the 196 moved, each to `OK SOUND`. The 18 poses the
  battery's volume filter skips are not run by it and were not measured.
- The pin (`three i=4 j=3 k=1`, `tripod i=0 j=4 t=1 k=0`): main both unions
  `OK BAD` on `MixedTouch`; head every op of both poses `OK SOUND`, re-pinned
  as `three_pairs_at_one_vertex_build_on_one_key_and_every_op_is_sound`.
