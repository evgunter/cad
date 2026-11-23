---
id: the-join-outside-the-boolean-records-no-class-zero
kind: issue
title: Every door but the boolean that ends with the join makes edges one on a class reading and records nothing
status: open
opened: 2026-11-04
priority: P2
cost: M
---


## What

The join is one predicate (`crates/topo/src/boolean/edge_join.rs`,
`joinable`) run by one loop (`join_all`), which hands back a `ClassZero`
for each join it makes on an iso-family reading. Only the boolean's
output stage (`join_stage`, into `boolean::ops::join_rows`) records it.
Every other door drops it:

- `Body::join_edges` (`edge_join.rs`), and so the merge door
  (`crates/topo/src/merge_faces.rs`, `merge_coplanar_faces`), which the
  split's finish ends with (`crates/topo/src/splitting/finish.rs`) and
  which the step-import assembly calls directly
  (`crates/step-import/src/assemble.rs`, `join_edges`);
- `Body::join_edges_within` (`edge_join.rs`), which
  `crates/topo/src/replace_face.rs` (`staged_join`, and through it
  `crates/topo/src/offset_together.rs`), `crates/topo/src/shell.rs` and
  `crates/sweep/src/blend/surgery.rs` call.

D10 `[coincidence.door]`: "a door that makes cells one on a class it
read (the boolean's edge join) records it". The split records its ON
verdicts (`DecisionSite::SplitOn`) and the blend its battery's
decisions, but a join either makes on an iso family reaches no row, so
the `unproven-coincidence` lint does not see it. The pcert E3 probe
saw such joins at the step-import door
(`chart_singularity_vertex::every_chart_singularity_a_file_puts_in_a_face_imports_as_a_pole_form_vertex`,
two circles on distinct carriers) and at the split/merge doors on
pieces of one edge.

## Done when

Each door that carries coincidence rows records the `ClassZero`s its
join hands back between two of its inputs' cells, or says why its
inputs make them structure.
