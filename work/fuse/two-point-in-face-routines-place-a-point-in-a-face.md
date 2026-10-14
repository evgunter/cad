---
id: two-point-in-face-routines-place-a-point-in-a-face
kind: issue
title: Two point-in-face routines place a point in a face: the lattice row's holds-point check reads the one the carry does not
status: closed
branch: fuse/census-tier3-corners
closed: 2026-10-14
opened: 2026-10-12
priority: P3
cost: M
---


Found by lane A's review of PR 4893. Read, not measured.

## What

Two routines answer "is this point inside this face":

- `boolean::solid_contain::point_in_face`
  (`crates/topo/src/boolean/solid_contain.rs:3822`) is the
  point-in-solid walk's face test. It is planar only, and reports a
  boundary as `None`. `test_support::point_in_face`
  (`crates/topo/src/lib.rs:821`) is a thin door onto it.
- `boolean::contain::face_placement`
  (`crates/topo/src/boolean/contain.rs:870`) is the census's confirm of
  a vertex-on-face record (`census.rs:6601`). It places on a plane
  through `contfp`, on a curved face through the chart trim. The
  boolean's carry reads through it twice: in the census's record
  confirm, which `records::landings` asks for each pair of a divided
  face's pieces, and in `boolean::ops::span_held`, through
  `piece_holds`.

The lattice corpus row
`every_fused_vertex_of_the_lattice_corpus_settles_on_a_live_cell`
(`crates/topo/tests/fused_into_live_cells.rs:76`) checks that a settled
face holds its point through the first routine. The carry and the
census decide through the second. So the row confirms the carry with a
read the carry does not make, and the two could disagree at a boundary
or an in-band margin without either side noticing.

## Owed

Either one routine, or the row reads the census's door
(`face_placement`, exposed to tests), so that what the row checks is
what the census confirms. Then state why the walk keeps its own test,
if it does.

## 2026-10-14, closed: branch `fuse/census-tier3-corners`

**Two readings, one core.** Both routines walk each loop on
`splitting::containment::carrier_walk`. They differ around it:

- `solid_contain::point_in_face` reads the loops' projection, without
  certifying `q` on the plane, and reports any boundary as `None`. That
  is what the ray walk needs: a hit on a re-described face's own plane,
  and a graze it retries.
- `contain::face_placement` decides the point on the plane first. It
  then runs a boundary pre-pass (vertices, then edges) and reads the
  interior only past it.

**Measured** (a scratch probe, not committed): the lattice corpus's
planar faces (6³ brick placements × 3 ops), with points at every vertex,
edge midpoint and face centre on the face's plane, offset in-plane by 0
to 20ε in six directions. That is about 3.08M pairs.

| Pairs | Readings |
|---|---|
| 1 877 451 | agree exactly (`in`, `out` and boundary alike) |
| 1 050 585 | both refuse |
| 151 356 | `None` (a boundary) from the walk, an escalation (`bool_contact_vertex`) from the placement |

The last group are points on an edge within the band of one of its
vertices: the placement's pre-pass reads the vertex first, the walk the
edge. Neither answers inside. **There is no in/out disagreement.**

**The row now reads the census's door.**
`fused_into_live_cells.rs`'s corpus row reads
`test_support::face_holds`, the census's confirm read, so it checks
what the census confirms. `point_in_face`'s doc says why the walk keeps
its own test.
