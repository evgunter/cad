---
id: a-pierce-whose-wide-run-pinches-its-intersection-refuses
kind: issue
title: A two-run pierce whose second run holds a face's bisector and the -z edge refuses its intersection (derived ring role order)
status: open
opened: 2026-10-04
priority: P0
cost: H
refs: [a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op]
---


## What

Found by the sweep of `a-pierce-with-two-out-runs-at-one-vertex-refuses-every-op`.

The pose is `join_pierce_strut_facing.rs`'s L-prism and `cube_beyond(m)`,
tilted so that the reflex top corner `v = (1, 1, 1)` has two Out runs:
the top sector's bisector alone, and the x = 1 face's bisector with
the −z edge. Examples are `m = (2, 0.4, 1)` and `(1, 0.2, 0.5)` (the
row `a_wide_run_builds_its_union_and_difference`). In
`join_pierce_runs_sweep.rs`, the face placement's poses
`i=0 j=3..5` and `i=2 j=5` are this family.

The union and the difference build `SOUND` in both orders. The
intersection refuses in both orders with
`JoinDesync { "derived ring role order separates a loose scaffolding pair" }`.
If the ring struts are faced the other way, it refuses
`Euler(SelfLoopEdge)` from `zip::zip_seam` instead.

PR 4026's review r1 (m3) also reads "derived ring role order" on L-corner
∩ poses where only the top bisector and the −z edge read Out, outside
this family (*likely*, by the bisector reading). The row's family is
the ∩ residue of the L corner's two-run poses, whatever widens the
runs. The notch and shallow corners' ∩ residue is
`a-two-run-pierce-on-a-notch-or-shallow-corner-refuses-its-intersection-every-chord-arc`.

The intersection pinches at `v`: two lumps of the prism's In part
meet there. The piercing vertex keeps both In runs, so its In section
face passes `v` twice. Where the second run is the −z edge alone,
the ring struts face their start germ with the half leaving the ring
vertex, and the zip then fuses the pinch (`vtxfac`, step 3). In this
family the join's ring lane refuses that facing (`join::ring_order`,
read from the ring run's winding). The walk's own facing leaves both
In seams passing the pinch vertex twice, and the zip cannot fuse a
vertex a second time.

## The shape to give

Diagnose first. Two questions: which face of the result carries the
pinch's crossed corner, and whether the zip can be taught a second
visit of a fused vertex. The zip's second fusion is a zero-length
self-loop, so the `kev` refuses.

## Built

Branch `join/pierce-pinch-families`.

**First wrong state: the intersection's ring facing, then the zip.**
Main faced an intersection's ring struts toward the start germ by a
measured rule. In this family the walk about the pierced face's normal
reads the end germ, and the join's ring lane refuses the start facing
("derived ring role order"). With the walk's facing the join completes,
and the seam is one figure-eight section polygon that both operands
keep through one vertex at `v`, passed twice. The zip fuses the first
meeting, and the second would fuse the vertex to itself
(`Euler(SelfLoopEdge)`). Before the zips, no kept face through `v`
lies in both cones of the pinch: the cube face's two lobes are two
faces, and so are the prism top's two sectors. So nothing yet carries
the crossed corner that keeps two cones on one vertex.

**What changed.**
- The ring struts face by the walk in every op; the intersection rule
  is gone (`vtxfac.rs`, step 3).
- `zip::cross_pinches` runs before the zips. It simulates their fusion
  order and, where a pair would fuse a vertex to itself, splits the
  vertex across two corners of kept faces first. Here those are the
  outer corners of two faces of one surface: `mev` between them, then
  `kef`, makes them one face whose boundary meets itself at `v`, and
  that face carries the crossed corner. The zips then fuse a tree.
- The absorption is recorded (`Descendants`, `BooleanNaming::merge_groups`).

Rows: `join_pierce_strut_facing::a_wide_run_builds_in_every_op` (every
op, both orders, `SOUND`, the closed-form volume, one vertex at `v`) and
`a_crossed_pinch_names_the_faces_it_merged`.
