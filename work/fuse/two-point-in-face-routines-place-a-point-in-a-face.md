---
id: two-point-in-face-routines-place-a-point-in-a-face
kind: issue
title: Two point-in-face routines place a point in a face: the lattice row's holds-point check reads the one the carry does not
status: open
opened: 2026-10-12
priority: P3
cost: M
---


Found by lane A's review of PR 4893. Read, not measured.

## What

Two routines answer "is this point inside this face":

- `boolean::solid_contain::point_in_face`
  (`crates/topo/src/boolean/solid_contain.rs:3751`) is the
  point-in-solid walk's face test. It is planar only, and reports a
  boundary as `None`. `test_support::point_in_face`
  (`crates/topo/src/lib.rs:821`) is a thin door onto it.
- `boolean::contain::face_placement`
  (`crates/topo/src/boolean/contain.rs:870`) is the census's confirm of
  a vertex-on-face record. It places on a plane through `contfp`, on a
  curved face through the chart trim, and is what the boolean's carry
  reads (`boolean::ops::piece_holds`).

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
